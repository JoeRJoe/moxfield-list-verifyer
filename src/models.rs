use std::collections::HashMap;

use crate::validators::Validator;
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct List {
    pub id: String,
    pub name: String,
    pub format: String,
    pub visibility: String,
    #[serde(rename = "createdByUser")]
    pub created_by_user: User,
    pub boards: Boards,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "userName")]
    pub user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub quantity: u32,
    pub card: CardDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Boards {
    pub mainboard: Board,
    pub commanders: Board,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub count: u32,
    pub cards: HashMap<String, Card>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardDetails {
    pub id: String,
    pub name: String,
    pub legalities: HashMap<String, String>,
    pub type_line: String,
}

impl List {
    pub async fn validate(&self, validators: Vec<Box<dyn Validator>>) -> bool {
        let mut is_valid =  true;
        println!("Validating list {}...", self.name);
        for validator in validators {
            if !validator.check(self).await {
                is_valid = false;
            }
        }
        is_valid
    }
}
