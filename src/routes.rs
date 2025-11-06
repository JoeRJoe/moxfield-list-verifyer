use rocket::serde::json::Json;

use crate::{models::List, validators::{MassLandDenialValidator, NonLandTutorValidator}};

#[get("/validate/<id>")]
pub async fn validate(id: &str) -> Json<bool> {
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
                Box::new(MassLandDenialValidator {}),
                Box::new(NonLandTutorValidator {}),
            ])
            .await,
    )
}

