//! SeaORM model for Hex

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "hexes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub realm_id: i32,
    pub x: i32,
    pub y: i32,
    pub terrain_type: String,
    pub features: Json,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
