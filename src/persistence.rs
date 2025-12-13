use crate::entities::{prelude::*, report};
use crate::errors::AppError;
use crate::models::Report as ReportModel;
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Clone)]
pub struct HistoryStore {
    conn: DatabaseConnection,
}

impl HistoryStore {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub async fn save(&self, report: ReportModel) -> Result<(), AppError> {
        let active_model = report::ActiveModel {
            is_valid: Set(report.is_valid),
            name: Set(report.name),
            author: Set(report.author),
            non_land_tutors: Set(serde_json::to_value(report.non_land_tutors).unwrap()),
            mass_land_denial_cards: Set(
                serde_json::to_value(report.mass_land_denial_cards).unwrap()
            ),
            commander_tutors: Set(serde_json::to_value(report.commander_tutors).unwrap()),
            two_card_combos: Set(serde_json::to_value(report.two_card_combos).unwrap()),
            gamechangers: Set(serde_json::to_value(report.gamechangers).unwrap()),
            infinite_turns_combos: Set(serde_json::to_value(report.infinite_turns_combos).unwrap()),
            combos: Set(serde_json::to_value(report.combos).unwrap()),
            deck_list: Set(serde_json::to_value(report.deck_list).unwrap()),
            ..Default::default()
        };

        Report::insert(active_model)
            .exec(&self.conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<ReportModel>, AppError> {
        let reports = Report::find()
            .all(&self.conn)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(reports
            .into_iter()
            .map(|r| ReportModel {
                is_valid: r.is_valid,
                name: r.name,
                author: r.author,
                non_land_tutors: serde_json::from_value(r.non_land_tutors).unwrap_or_default(),
                mass_land_denial_cards: serde_json::from_value(r.mass_land_denial_cards)
                    .unwrap_or_default(),
                commander_tutors: serde_json::from_value(r.commander_tutors).unwrap_or_default(),
                two_card_combos: serde_json::from_value(r.two_card_combos).unwrap_or_default(),
                gamechangers: serde_json::from_value(r.gamechangers).unwrap_or_default(),
                infinite_turns_combos: serde_json::from_value(r.infinite_turns_combos)
                    .unwrap_or_default(),
                combos: serde_json::from_value(r.combos).unwrap_or_default(),
                deck_list: serde_json::from_value(r.deck_list).unwrap_or_default(),
            })
            .rev()
            .collect())
    }
}
