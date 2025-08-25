//! SeaORM model for Npc

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "npcs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub settlement_id: Option<i32>,
    pub name: String,
    pub occupation: Option<String>,
    pub description: Option<Text>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
