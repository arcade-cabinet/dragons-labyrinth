//! Asset management models for Dragon's Labyrinth
//! 
//! Models for tracking CC0 assets, their attribution, and build-time metadata.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Asset metadata entity - tracks all assets in the library
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    
    #[sea_orm(column_type = "Text", unique)]
    pub path: String,
    
    #[sea_orm(column_type = "Text")]
    pub category: String,
    
    #[sea_orm(column_type = "Text")]
    pub asset_type: String,
    
    #[sea_orm(column_type = "Text")]
    pub filename: String,
    
    #[sea_orm(column_type = "Text")]
    pub display_name: String,
    
    #[sea_orm(column_type = "Text")]
    pub tags: String, // JSON array as string
    
    pub file_size_bytes: i64,
    
    pub dread_level: Option<i32>,
    
    pub mobile_compatible: bool,
    
    pub performance_score: f32,
    
    pub last_modified: DateTime<Utc>,
    
    pub indexed_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "attribution::Entity")]
    Attribution,
}

impl Related<attribution::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attribution.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// Asset attribution entity - tracks CC0 source information
pub mod attribution {
    use super::*;
    
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "asset_attribution")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        
        pub asset_id: Uuid,
        
        #[sea_orm(column_type = "Text")]
        pub source_library: String,
        
        #[sea_orm(column_type = "Text")]
        pub vendor_prefix: String,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub original_filename: Option<String>,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub original_path: Option<String>,
        
        #[sea_orm(column_type = "Text")]
        pub license_type: String,
        
        pub attribution_required: bool,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub attribution_text: Option<String>,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub source_url: Option<String>,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub converted_from: Option<String>,
        
        #[sea_orm(column_type = "Text", nullable)]
        pub conversion_manifest_path: Option<String>,
        
        pub indexed_at: DateTime<Utc>,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::Entity",
            from = "Column::AssetId",
            to = "super::Column::Id"
        )]
        Asset,
    }

    impl Related<super::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Asset.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
