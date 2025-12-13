use crate::errors::AppError;
use crate::models::List;
use crate::persistence::HistoryStore;
use crate::progress::ProgressTracker;
use crate::validators::{
    CommanderTutorValidator, GamechangerValidator, InfiniteTurnsValidator, MassLandDenialValidator,
    NonLandTutorValidator, TwoCardComboValidator,
};
use rocket::State;
use rocket::futures::{SinkExt, StreamExt};
use rocket_ws::{Channel, Message, WebSocket};
use std::sync::{Arc, Mutex};

#[get("/ws/validate/<id>")]
pub fn validate_ws(
    id: String,
    ws: WebSocket,
    client: &State<reqwest::Client>,
    store: &State<HistoryStore>,
) -> Channel<'static> {
    let client = client.inner().clone();
    let store = store.inner().clone();
    let id_clone = id.clone();

    ws.channel(move |stream| {
        Box::pin(async move {
            let tracker = Arc::new(Mutex::new(ProgressTracker::new(6)));

            let mut rx = {
                let tracker_guard = tracker.lock().unwrap();
                tracker_guard.subscribe()
            };

            let (mut sender, _receiver) = stream.split();

            let tracker_clone = tracker.clone();

            let mut validation_task = Box::pin(tokio::spawn(async move {
                let user_agent = std::env::var("MOXFIELD_USER_AGENT")
                    .unwrap();

                let response = client
                    .get("https://api2.moxfield.com/v3/decks/all/".to_string() + &id_clone)
                    .header("User-Agent", &user_agent)
                    .header("Accept", "application/json")
                    .send()
                    .await
                    .map_err(|e| AppError::ScryfallApiError(e.to_string()))?;

                let list = response
                    .json::<List>()
                    .await
                    .map_err(|e| AppError::ScryfallApiError(e.to_string()))?;

                let report = list
                    .validate_with_progress(
                        &client,
                        vec![
                            Box::new(MassLandDenialValidator),
                            Box::new(NonLandTutorValidator),
                            Box::new(CommanderTutorValidator),
                            Box::new(GamechangerValidator),
                            Box::new(InfiniteTurnsValidator),
                            Box::new(TwoCardComboValidator),
                        ],
                        Some(tracker_clone),
                    )
                    .await?;

                store.save(report.clone()).await?;
                Ok::<_, AppError>(report)
            }));

            loop {
                tokio::select! {
                    res = &mut validation_task => {
                        match res {
                             Ok(Ok(report)) => {
                                 if let Ok(json) = serde_json::to_string(&report) {
                                      let _ = sender.send(Message::Text(json)).await;
                                 }
                             },
                             Ok(Err(e)) => {
                                 let error_msg = serde_json::json!({
                                     "type": "error",
                                     "message": format!("{:?}", e)
                                 });
                                 let _ = sender.send(Message::Text(error_msg.to_string())).await;
                             },
                             Err(e) => {
                                 let error_msg = serde_json::json!({
                                     "type": "error",
                                     "message": format!("Task failed: {:?}", e)
                                 });
                                 let _ = sender.send(Message::Text(error_msg.to_string())).await;
                             }
                        }
                        break;
                    },
                    msg = rx.recv() => {
                        if let Ok(progress_msg) = msg {
                            if let Ok(json) = serde_json::to_string(&progress_msg) {
                                 let _ = sender.send(Message::Text(json)).await;
                            }
                        } else {
                        }
                    }
                }
            }

            Ok(())
        })
    })
}
