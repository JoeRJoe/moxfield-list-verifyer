use crate::errors::AppError;
use futures::stream::{self, StreamExt};
use rocket::{State, serde::json::Json};

use crate::{
    models::{List, Report},
    persistence::HistoryStore,
    validators::{
        CommanderTutorValidator, GamechangerValidator, InfiniteTurnsValidator,
        MassLandDenialValidator, NonLandTutorValidator, TwoCardComboValidator,
    },
};

#[get("/history")]
pub async fn get_history(store: &State<HistoryStore>) -> Result<Json<Vec<Report>>, AppError> {
    let reports = store.get_all().await?;
    Ok(Json(reports))
}

#[get("/validate/<id>")]
pub async fn validate(
    id: &str,
    client: &State<reqwest::Client>,
    store: &State<HistoryStore>,
) -> Result<Json<Report>, AppError> {
    let user_agent = std::env::var("MOXFIELD_USER_AGENT").map_err(AppError::EnvVarMissing)?;
    let response: List = client
        .get("https://api2.moxfield.com/v3/decks/all/".to_string() + id)
        .header("User-Agent", &user_agent)
        .header("Accept", "application/json")
        .send()
        .await?
        .json()
        .await?;

    let report = response
        .validate(
            client,
            vec![
                Box::new(MassLandDenialValidator),
                Box::new(NonLandTutorValidator),
                Box::new(CommanderTutorValidator),
                Box::new(GamechangerValidator),
                Box::new(InfiniteTurnsValidator),
                Box::new(TwoCardComboValidator),
            ],
        )
        .await?;

    store.save(report.clone()).await?;

    Ok(Json(report))
}

#[post("/validate/batch", data = "<id_lists>")]
pub async fn validate_batch(
    id_lists: Json<Vec<String>>,
    client: &State<reqwest::Client>,
    store: &State<HistoryStore>,
) -> Result<Json<Vec<Report>>, AppError> {
    let reports = stream::iter(id_lists.into_inner())
        .map(|id| async move {
            let user_agent =
                std::env::var("MOXFIELD_USER_AGENT").map_err(AppError::EnvVarMissing)?;
            let response: List = client
                .get("https://api2.moxfield.com/v3/decks/all/".to_string() + &id)
                .header("User-Agent", &user_agent)
                .header("Accept", "application/json")
                .send()
                .await?
                .json()
                .await?;

            let report = response
                .validate(
                    client,
                    vec![
                        Box::new(MassLandDenialValidator),
                        Box::new(NonLandTutorValidator),
                        Box::new(CommanderTutorValidator),
                        Box::new(GamechangerValidator),
                        Box::new(InfiniteTurnsValidator),
                        Box::new(TwoCardComboValidator),
                    ],
                )
                .await?;
            Ok::<Report, AppError>(report)
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    let successful_reports: Vec<Report> = reports
        .into_iter()
        .filter_map(|res| match res {
            Ok(report) => Some(report),
            Err(e) => {
                eprintln!("Error validating list: {}", e);
                None
            }
        })
        .collect();

    for report in &successful_reports {
        store.save(report.clone()).await?;
    }

    Ok(Json(successful_reports))
}
