use crate::errors::AppError;
use crate::models::{CardList, CardListUnit, ComboListRequest, List, ScryfallQuery};
use crate::validation_results::ValidationResults;
use async_trait::async_trait;
use moka::future::Cache;
use std::sync::OnceLock;

static SCRYFALL_CACHE: OnceLock<Cache<String, Option<String>>> = OnceLock::new();
static SPELLBOOK_CACHE: OnceLock<Cache<String, ComboListRequest>> = OnceLock::new();

fn get_scryfall_cache() -> &'static Cache<String, Option<String>> {
    SCRYFALL_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(std::time::Duration::from_secs(3600))
            .build()
    })
}

fn get_spellbook_cache() -> &'static Cache<String, ComboListRequest> {
    SPELLBOOK_CACHE.get_or_init(|| {
        Cache::builder()
            .time_to_live(std::time::Duration::from_secs(3600))
            .build()
    })
}

async fn check_scryfall(
    client: &reqwest::Client,
    query: String,
) -> Result<Option<String>, AppError> {
    let cache = get_scryfall_cache();
    if let Some(result) = cache.get(&query).await {
        return Ok(result);
    }

    let user_agent =
        std::env::var("SCRYFALL_USER_AGENT").unwrap_or_else(|_| "Mozilla/5.0".to_string());
    let response = client
        .get(&query)
        .header("User-Agent", &user_agent)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| AppError::ScryfallApiError(e.to_string()))?;

    let result = if response.status().is_success() {
        if let Ok(query_result) = response.json::<ScryfallQuery>().await {
            query_result
                .data
                .first()
                .map(|card| card.oracle_text.clone())
        } else {
            None
        }
    } else {
        None
    };

    cache.insert(query, result.clone()).await;
    Ok(result)
}

async fn get_combos(client: &reqwest::Client, list: &List) -> Result<ComboListRequest, AppError> {
    let mut card_names: Vec<String> = list
        .boards
        .mainboard
        .cards
        .values()
        .chain(list.boards.commanders.cards.values())
        .map(|card| card.card.name.clone())
        .collect();
    card_names.sort();

    let cache_key = card_names.join("|");
    let cache = get_spellbook_cache();

    if let Some(result) = cache.get(&cache_key).await {
        return Ok(result);
    }

    let cards: Vec<CardListUnit> = card_names
        .into_iter()
        .map(|card| CardListUnit { card, quantity: 1 })
        .collect();

    let card_list = CardList { main: cards };

    let response = client
        .post("https://backend.commanderspellbook.com/find-my-combos")
        .header("Content-Type", "application/json")
        .json(&card_list)
        .send()
        .await
        .map_err(|e| AppError::SpellbookApiError(e.to_string()))?;

    let result = if response.status().is_success() {
        response
            .json::<ComboListRequest>()
            .await
            .unwrap_or(ComboListRequest {
                results: crate::models::ComboList { included: vec![] },
            })
    } else {
        ComboListRequest {
            results: crate::models::ComboList { included: vec![] },
        }
    };

    cache.insert(cache_key, result.clone()).await;
    Ok(result)
}

#[async_trait]
pub trait Validator: Send + Sync {
    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError>;

    fn name(&self) -> &'static str;
}

pub struct MassLandDenialValidator;
pub struct NonLandTutorValidator;
pub struct CommanderTutorValidator;
pub struct TwoCardComboValidator;
pub struct GamechangerValidator;
pub struct InfiniteTurnsValidator;

#[async_trait]
impl Validator for MassLandDenialValidator {
    fn name(&self) -> &'static str {
        "Mass Land Denial"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for mass land denial cards...");
        let mut results = ValidationResults::default();

        for card in list
            .boards
            .mainboard
            .cards
            .values()
            .chain(list.boards.commanders.cards.values())
        {
            let card_name = &card.card.name;
            let query = format!(
                "https://api.scryfall.com/cards/search?q=f:edh+otag:mass-land-denial+!\"{}\"",
                card_name.replace(" ", "+")
            );

            if let Some(oracle_text) = check_scryfall(client, query).await? {
                println!(
                    "Card {} is banned due to mass land denial policy.",
                    card_name
                );
                results
                    .mass_land_denial_cards
                    .push((card_name.to_string(), oracle_text));
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Validator for NonLandTutorValidator {
    fn name(&self) -> &'static str {
        "Non-Land Tutors"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for non-land tutors...");
        let mut results = ValidationResults::default();

        for card in list.boards.mainboard.cards.values() {
            let card_name = &card.card.name;
            let query = format!(
                "https://api.scryfall.com/cards/search?q=f:edh+otag:tutor+-otag:tutor-land+!\"{}\"",
                card_name.replace(" ", "+")
            );

            if let Some(oracle_text) = check_scryfall(client, query).await? {
                println!("Card {} is a non-land tutor.", card_name);
                results
                    .non_land_tutors
                    .push((card_name.to_string(), oracle_text));
            }
        }
        println!(
            "Total non-land tutors found: {}",
            results.non_land_tutors.len()
        );
        Ok(results)
    }
}

#[async_trait]
impl Validator for CommanderTutorValidator {
    fn name(&self) -> &'static str {
        "Commander Tutors"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for tutors in command zone...");
        let mut results = ValidationResults::default();

        for card in list.boards.commanders.cards.values() {
            let card_name = &card.card.name;
            let query = format!(
                "https://api.scryfall.com/cards/search?q=f:edh+otag:tutor+-otag:tutor-land+!\"{}\"",
                card_name.replace(" ", "+")
            );

            if check_scryfall(client, query).await?.is_some() {
                println!("Commander {} is a tutor.", card_name);
                results.commander_tutors.push(card_name.to_string());
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Validator for TwoCardComboValidator {
    fn name(&self) -> &'static str {
        "Two-Card Combos"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for two card combos...");
        let combo_list = get_combos(client, list).await?;
        let mut results = ValidationResults::default();

        results.combos = combo_list.results.get_combos();
        results.two_card_combos = combo_list.results.check_two_card_combos();

        Ok(results)
    }
}

#[async_trait]
impl Validator for GamechangerValidator {
    fn name(&self) -> &'static str {
        "Gamechangers"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for gamechanger cards...");
        let mut results = ValidationResults::default();

        for card in list
            .boards
            .mainboard
            .cards
            .values()
            .chain(list.boards.commanders.cards.values())
        {
            let card_name = &card.card.name;
            let query = format!(
                "https://api.scryfall.com/cards/search?q=f:edh+is:gamechanger+!\"{}\"",
                card_name.replace(" ", "+")
            );

            if check_scryfall(client, query).await?.is_some() {
                println!("Card {} is a gamechanger.", card_name);
                results.gamechangers.push(card_name.to_string());
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Validator for InfiniteTurnsValidator {
    fn name(&self) -> &'static str {
        "Infinite Turns"
    }

    async fn check(
        &self,
        client: &reqwest::Client,
        list: &List,
    ) -> Result<ValidationResults, AppError> {
        println!("Checking for infinite turns combos...");
        let combo_list = get_combos(client, list).await?;
        let mut results = ValidationResults::default();

        results.combos = combo_list.results.get_combos();
        results.infinite_turns_combos = combo_list.results.check_infinite_turns_combos();

        Ok(results)
    }
}
