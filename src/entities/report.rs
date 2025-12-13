use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "report")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub is_valid: bool,
    pub name: String,
    pub author: String,
    #[sea_orm(column_type = "Json")]
    pub non_land_tutors: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub mass_land_denial_cards: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub commander_tutors: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub two_card_combos: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub gamechangers: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub infinite_turns_combos: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub combos: serde_json::Value,
    #[sea_orm(column_type = "Json")]
    pub deck_list: serde_json::Value,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
