use std::{collections::HashMap, pin::Pin};

use rocket::serde::{Deserialize, json::Json};
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[get("/validate/<id>")]
async fn validate(id: &str) -> Json<bool> {
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
            .validate(vec![Box::new(MassLandDenialValidator {})])
            .await,
    )
}

#[get("/<id>")]
async fn print(id: &str) -> Json<List> {
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

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![validate, print])
}

#[derive(Serialize, Deserialize, Debug)]
struct List {
    id: String,
    name: String,
    format: String,
    visibility: String,
    #[serde(rename = "createdByUser")]
    created_by_user: User,
    boards: Boards,
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    #[serde(rename = "userName")]
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Card {
    quantity: u32,
    card: CardDetails,
}

#[derive(Serialize, Deserialize, Debug)]
struct Boards {
    mainboard: Board,
    commanders: Board,
}

#[derive(Serialize, Deserialize, Debug)]
struct Board {
    count: u32,
    cards: HashMap<String, Card>,
}

#[derive(Serialize, Deserialize, Debug)]
struct CardDetails {
    id: String,
    name: String,
    legalities: HashMap<String, String>,
    type_line: String,
}

impl List {
    async fn validate(&self, validators: Vec<Box<dyn Validator>>) -> bool {
        for validator in validators {
            if !validator.check(self).await {
                return false;
            }
        }
        true
    }
}

trait Validator: Send + Sync {
    fn check<'a>(&'a self, list: &'a List)
    -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>>;
}
struct MassLandDenialValidator {}

impl Validator for MassLandDenialValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut mld_found = false;
            for card in list.boards.mainboard.cards.values().chain(list.boards.commanders.cards.values()) {
                let card_name = &card.card.name;
                let response = client.get("https://api.scryfall.com/cards/search?q=f:edh+otag:mass-land-denial+".to_string() + card_name.replace(" ", "+").as_str())
                    .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:40.0) Gecko/20100101 Firefox/40.0")
                    .header("Accept", "application/json")
                    .send()
                    .await;

                if let Ok(value) = response
                    && value.status().is_success() && !card.card.type_line.contains("Planeswalker") {
                        println!("Card {} is banned due to mass land denial policy.", card_name);
                        mld_found = true;
                    }
            }
            !mld_found
        })
    }
}