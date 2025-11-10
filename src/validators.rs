use crate::models::{CardList, CardListUnit, ComboListRequest, List, Report};
use std::pin::Pin;

pub trait Validator: Send + Sync {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>>;
}

pub struct MassLandDenialValidator;
pub struct NonLandTutorValidator;
pub struct CommanderTutorValidator;
pub struct RepeatableTutorValidator;
pub struct MultipleTutorValidator;
pub struct TwoCardComboValidator;
pub struct GamechangerValidator;
pub struct InfiniteTurnsValidator;

impl Validator for MassLandDenialValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for mass land denial cards...");
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut mld_found = false;
            for card in list
                .boards
                .mainboard
                .cards
                .values()
                .chain(list.boards.commanders.cards.values())
            {
                let card_name = &card.card.name;
                let response = client.get(format!("https://api.scryfall.com/cards/search?q=f:edh+otag:mass-land-denial+!\"{}\"", card_name.replace(" ", "+")))
                    .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:40.0) Gecko/20100101 Firefox/40.0")
                    .header("Accept", "application/json")
                    .send()
                    .await;

                if let Ok(value) = response
                    && value.status().is_success()
                    && !card.card.type_line.contains("Planeswalker")
                {
                    println!(
                        "Card {} is banned due to mass land denial policy.",
                        card_name
                    );
                    mld_found = true;
                    report.mass_land_denial_cards.push(card_name.to_string());
                }
            }
            !mld_found
        })
    }
}

impl Validator for NonLandTutorValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for non-land tutors...");
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut tutor_count = 0;
            for card in list.boards.mainboard.cards.values() {
                let card_name = &card.card.name;
                let response = client.get(format!("https://api.scryfall.com/cards/search?q=f:edh+otag:tutor+-otag:tutor-land+!\"{}\"", card_name.replace(" ", "+")))
                    .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:40.0) Gecko/20100101 Firefox/40.0")
                    .header("Accept", "application/json")
                    .send()
                    .await;

                if let Ok(value) = response
                    && value.status().is_success()
                {
                    println!("Card {} is a non-land tutor.", card_name);
                    tutor_count += 1;
                    report.non_land_tutors.push(card_name.to_string());
                }
            }
            println!("Total non-land tutors found: {}", tutor_count);
            tutor_count <= 3
        })
    }
}

impl Validator for CommanderTutorValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for tutors in command zone...");
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut commander_tutor_found = false;
            for card in list.boards.commanders.cards.values() {
                let card_name = &card.card.name;
                let response = client.get(format!("https://api.scryfall.com/cards/search?q=f:edh+otag:tutor+-otag:tutor-land+!\"{}\"", card_name.replace(" ", "+")))
                    .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:40.0) Gecko/20100101 Firefox/40.0")
                    .header("Accept", "application/json")
                    .send()
                    .await;

                if let Ok(value) = response
                    && value.status().is_success()
                {
                    println!("Commander {} is a tutor.", card_name);
                    commander_tutor_found = true;
                    report.commander_tutors.push(card_name.to_string());
                }
            }
            !commander_tutor_found
        })
    }
}

impl Validator for RepeatableTutorValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        todo!()
    }
}

impl Validator for MultipleTutorValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        todo!()
    }
}

impl Validator for TwoCardComboValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for two card combos...");
        Box::pin(async move {
            let client = reqwest::Client::new();

            let card_names: Vec<String> = list
                .boards
                .mainboard
                .cards
                .values()
                .chain(list.boards.commanders.cards.values())
                .map(|card| card.card.name.clone())
                .collect();
            let cards: Vec<CardListUnit> = card_names
                .into_iter()
                .map(|card| CardListUnit { card, quantity: 1 })
                .collect();

            let card_list = CardList { main: cards };
            
            let combo_list = client.post("https://backend.commanderspellbook.com/find-my-combos")
                .header("Content-Type", "application/json")
                .json(&card_list)
                .send()
                .await
                .unwrap()
                .json::<ComboListRequest>()
                .await
                .unwrap();

            let two_card_combos = combo_list.results.check_two_card_combos();
            let has_two_card_combos = !two_card_combos.is_empty();
            report.two_card_combos = two_card_combos;
            !has_two_card_combos
        })
    }
}

impl Validator for GamechangerValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for gamechanger cards...");
        Box::pin(async move {
            let client = reqwest::Client::new();
            let mut gamechanger_found = false;
            for card in list
                .boards
                .mainboard
                .cards
                .values()
                .chain(list.boards.commanders.cards.values())
            {
                let card_name = &card.card.name;
                let response = client.get(format!("https://api.scryfall.com/cards/search?q=f:edh+is:gamechanger+!\"{}\"", card_name.replace(" ", "+")))
                    .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:40.0) Gecko/20100101 Firefox/40.0")
                    .header("Accept", "application/json")
                    .send()
                    .await;

                if let Ok(value) = response
                    && value.status().is_success()
                {
                    println!("Card {} is a gamechanger.", card_name);
                    gamechanger_found = true;
                    report.gamechangers.push(card_name.to_string());
                }
            }
            !gamechanger_found
        })
    }
}

impl Validator for InfiniteTurnsValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
        report: &'a mut Report,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
        println!("Checking for infinite turns combos...");
        Box::pin(async move {
            let client = reqwest::Client::new();

            let card_names: Vec<String> = list
                .boards
                .mainboard
                .cards
                .values()
                .chain(list.boards.commanders.cards.values())
                .map(|card| card.card.name.clone())
                .collect();
            let cards: Vec<CardListUnit> = card_names
                .into_iter()
                .map(|card| CardListUnit { card, quantity: 1 })
                .collect();

            let card_list = CardList { main: cards };
            
            let combo_list = client.post("https://backend.commanderspellbook.com/find-my-combos")
                .header("Content-Type", "application/json")
                .json(&card_list)
                .send()
                .await
                .unwrap()
                .json::<ComboListRequest>()
                .await
                .unwrap();

            let infinite_turn_combos = combo_list.results.check_infinite_turns_combos();
            let has_infinite_turns = !infinite_turn_combos.is_empty();
            report.infinite_turns_combos = infinite_turn_combos;
            !has_infinite_turns
        })
    }
}
