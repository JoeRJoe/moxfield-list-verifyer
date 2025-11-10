use std::collections::HashMap;

use crate::validators::Validator;
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub is_valid: bool,
    pub name: String,
    pub author: String,
    pub non_land_tutors: Vec<String>,
    pub mass_land_denial_cards: Vec<String>,
    pub commander_tutors: Vec<String>,
    pub repeatable_tutors: Vec<String>,
    pub multiple_tutors: Vec<String>,
    pub two_card_combos: Vec<Vec<String>>,
    pub gamechangers: Vec<String>,
    pub infinite_turns_combos: Vec<Vec<String>>,
}

impl Report {
    pub fn new(name: String, author: String) -> Self {
        Self {
            is_valid: false,
            author,
            name,
            non_land_tutors: Vec::new(),
            mass_land_denial_cards: Vec::new(),
            commander_tutors: Vec::new(),
            repeatable_tutors: Vec::new(),
            multiple_tutors: Vec::new(),
            two_card_combos: Vec::new(),
            gamechangers: Vec::new(),
            infinite_turns_combos: Vec::new(),
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct CardList {
    pub main: Vec<CardListUnit>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardListUnit {
    pub card: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComboListRequest {
    pub results: ComboList,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComboList {
    pub included: Vec<ComboListIncluded>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComboListIncluded {
    pub id: String,
    pub uses: Vec<Ingredient>,
    pub produces: Vec<ComboEffect>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ingredient {
    pub card: ComboCard,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComboCard {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ComboEffect {
    pub feature: Effect,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub uncountable: bool,
}

impl List {
    pub async fn validate(&self, validators: Vec<Box<dyn Validator>>) -> Report {
        let mut is_valid = true;
        let mut report = Report::new(self.name.clone(), self.created_by_user.user_name.clone());

        println!(
            "Validating list {} by {}",
            self.name, self.created_by_user.user_name
        );
        for validator in validators {
            if !validator.check(self, &mut report).await {
                is_valid = false;
            }
        }
        report.is_valid = is_valid;
        report
    }
}

impl ComboList {
    pub fn check_infinite_turns_combos(&self) -> Vec<Vec<String>> {
        let mut infinite_turns_combos: Vec<Vec<String>> = Vec::new();

        for included in self.included.iter() {
            if included.produces.iter().any(|effect| effect.feature.name.contains("turns")) {
                infinite_turns_combos.push(
                    included
                        .uses
                        .iter()
                        .map(|ingredient| ingredient.card.name.clone())
                        .collect(),
                );
            }
        }
        infinite_turns_combos
    }

    pub fn check_two_card_combos(&self) -> Vec<Vec<String>> {
        let mut two_cards_combos: Vec<Vec<String>> = Vec::new();

        for included in self.included.iter() {
            if included.uses.len() <= 2 {
                two_cards_combos.push(
                    included
                        .uses
                        .iter()
                        .map(|ingredient| ingredient.card.name.clone())
                        .collect(),
                );
            }
        }
        two_cards_combos
    }
}
