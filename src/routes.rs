use rocket::serde::json::Json;

use crate::{
    models::{List, Report},
    validators::{
        CommanderTutorValidator, GamechangerValidator, InfiniteTurnsValidator, MassLandDenialValidator, NonLandTutorValidator, TwoCardComboValidator
    },
};

#[get("/validate/<id>")]
pub async fn validate(id: &str) -> Json<Report> {
    let client = reqwest::Client::new();

    let response: List = client
        .get("https://api2.moxfield.com/v3/decks/all/".to_string() + id)
        .header("User-Agent", "MoxKey; JoeRJoe 6e9f5aa56c63")
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Json(
        response
            .validate(vec![
                Box::new(MassLandDenialValidator),
                Box::new(NonLandTutorValidator),
                Box::new(CommanderTutorValidator),
                Box::new(GamechangerValidator),
                Box::new(InfiniteTurnsValidator),
                Box::new(TwoCardComboValidator),
            ])
            .await,
    )
}

#[post("/validate/batch", data = "<id_lists>")]
pub async fn validate_batch(id_lists: Json<Vec<&str>>) -> Json<Vec<Report>> {
    let mut reports = Vec::new();
    for id in id_lists.iter() {
        let client = reqwest::Client::new();
        let response: List = client
            .get("https://api2.moxfield.com/v3/decks/all/".to_string() + id)
            .header("User-Agent", "MoxKey; JoeRJoe 6e9f5aa56c63")
            .header("Accept", "application/json")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        reports.push(
            response
                .validate(vec![
                    Box::new(MassLandDenialValidator),
                    Box::new(NonLandTutorValidator),
                    Box::new(CommanderTutorValidator),
                    Box::new(GamechangerValidator),
                    Box::new(InfiniteTurnsValidator),
                    Box::new(TwoCardComboValidator),
                ])
                .await,
        );
    }
    Json(reports)
}