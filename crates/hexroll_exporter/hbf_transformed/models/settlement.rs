//! SeaORM model for Settlement

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "settlements")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub hex_id: Option<i32>,
    pub name: String,
    pub population: Option<i32>,
    pub description: Text,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
