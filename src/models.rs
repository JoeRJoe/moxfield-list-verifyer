use std::collections::HashMap;

use crate::validators::Validator;
use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Report {
    pub is_valid: bool,
    pub name: String,
    pub author: String,
    pub non_land_tutors: Vec<(String, String)>,
    pub mass_land_denial_cards: Vec<(String, String)>,
    pub commander_tutors: Vec<String>,
    pub two_card_combos: Vec<(Vec<String>, String)>,
    pub gamechangers: Vec<String>,
    pub infinite_turns_combos: Vec<Vec<String>>,
    pub combos: Vec<(Vec<String>, String)>,
    pub deck_list: Vec<CardListUnit>,
}

impl Report {
    pub fn new(name: String, author: String, deck_list: Vec<CardListUnit>) -> Self {
        Self {
            is_valid: false,
            author,
            name,
            non_land_tutors: Vec::new(),
            mass_land_denial_cards: Vec::new(),
            commander_tutors: Vec::new(),
            two_card_combos: Vec::new(),
            gamechangers: Vec::new(),
            infinite_turns_combos: Vec::new(),
            combos: Vec::new(),
            deck_list,
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardList {
    pub main: Vec<CardListUnit>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardListUnit {
    pub card: String,
    pub quantity: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboListRequest {
    pub results: ComboList,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboList {
    pub included: Vec<ComboListIncluded>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboListIncluded {
    pub id: String,
    pub uses: Vec<Ingredient>,
    pub produces: Vec<ComboEffect>,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ingredient {
    pub card: ComboCard,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboCard {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ComboEffect {
    pub feature: Effect,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub id: u32,
    pub name: String,
    pub status: String,
    pub uncountable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScryfallQuery {
    pub data: Vec<ScryfallCard>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScryfallCard {
    pub name: String,
    pub oracle_text: String,
    pub game_changer: bool,
}

use crate::errors::AppError;
use crate::progress::ProgressTracker;
use crate::validation_results::ValidationResults;
use std::sync::{Arc, Mutex};

impl List {
    pub async fn validate(
        &self,
        client: &reqwest::Client,
        validators: Vec<Box<dyn Validator>>,
    ) -> Result<Report, AppError> {
        self.validate_with_progress(client, validators, None).await
    }

    pub async fn validate_with_progress(
        &self,
        client: &reqwest::Client,
        validators: Vec<Box<dyn Validator>>,
        progress_tracker: Option<Arc<Mutex<ProgressTracker>>>,
    ) -> Result<Report, AppError> {
        let deck_list: Vec<CardListUnit> = self
            .boards
            .mainboard
            .cards
            .values()
            .map(|c| CardListUnit {
                card: c.card.name.clone(),
                quantity: c.quantity,
            })
            .collect();

        println!(
            "Validating list {} by {}",
            self.name, self.created_by_user.user_name
        );

        let validation_futures: Vec<_> = validators
            .into_iter()
            .map(|validator| {
                let validator_name = validator.name().to_string();
                let progress_clone = progress_tracker.clone();

                async move {
                    let result = validator.check(client, self).await;

                    if let Some(tracker) = progress_clone {
                        if let Ok(mut tracker_guard) = tracker.lock() {
                            tracker_guard.update(
                                validator_name.clone(),
                                format!("Completed {}", validator_name),
                            );
                        }
                    }

                    result
                }
            })
            .collect();

        let results = futures::future::join_all(validation_futures).await;

        let mut aggregated_results = ValidationResults::default();
        for result in results {
            let validation_result = result?;
            aggregated_results = aggregated_results.merge(validation_result);
        }

        let is_valid = aggregated_results.is_valid();

        let mut report = Report::new(
            self.name.clone(),
            self.created_by_user.user_name.clone(),
            deck_list,
        );

        report.mass_land_denial_cards = aggregated_results.mass_land_denial_cards;
        report.non_land_tutors = aggregated_results.non_land_tutors;
        report.commander_tutors = aggregated_results.commander_tutors;
        report.two_card_combos = aggregated_results.two_card_combos;
        report.gamechangers = aggregated_results.gamechangers;
        report.infinite_turns_combos = aggregated_results.infinite_turns_combos;
        report.combos = aggregated_results.combos;
        report.is_valid = is_valid;

        Ok(report)
    }
}

impl ComboList {
    pub fn check_infinite_turns_combos(&self) -> Vec<Vec<String>> {
        let mut infinite_turns_combos: Vec<Vec<String>> = Vec::new();

        for included in self.included.iter() {
            if included
                .produces
                .iter()
                .any(|effect| effect.feature.name.contains("turns"))
            {
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

    pub fn check_two_card_combos(&self) -> Vec<(Vec<String>, String)> {
        let mut two_cards_combos: Vec<(Vec<String>, String)> = Vec::new();

        for included in self.included.iter() {
            if included.uses.len() <= 2 {
                two_cards_combos.push((
                    included
                        .uses
                        .iter()
                        .map(|ingredient| ingredient.card.name.clone())
                        .collect(),
                    included.description.clone(),
                ));
            }
        }
        two_cards_combos
    }

    pub fn get_combos(&self) -> Vec<(Vec<String>, String)> {
        let mut combos: Vec<(Vec<String>, String)> = Vec::new();

        for included in self.included.iter() {
            combos.push((
                included
                    .uses
                    .iter()
                    .map(|ingredient| ingredient.card.name.clone())
                    .collect(),
                included.description.clone(),
            ));
        }
        println!("Combos found: {}", combos.len());
        combos
    }
}
