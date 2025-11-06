use crate::models::List;
use std::pin::Pin;

pub trait Validator: Send + Sync {
    fn check<'a>(
        &'a self,
        list: &'a List,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>>;
}

pub struct MassLandDenialValidator {}

impl Validator for MassLandDenialValidator {
    fn check<'a>(
        &'a self,
        list: &'a List,
    ) -> Pin<Box<dyn std::future::Future<Output = bool> + Send + 'a>> {
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
                let response = client.get("https://api.scryfall.com/cards/search?q=f:edh+otag:mass-land-denial+".to_string() + card_name.replace(" ", "+").as_str())
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
                }
            }
            !mld_found
        })
    }
}
