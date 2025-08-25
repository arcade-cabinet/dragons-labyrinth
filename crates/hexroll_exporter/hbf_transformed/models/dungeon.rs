//! SeaORM model for Dungeon

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "dungeons")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hex_id: Option<i32>,
    pub name: String,
    pub theme: String,
    pub danger_level: i32,
    pub treasure_value: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
