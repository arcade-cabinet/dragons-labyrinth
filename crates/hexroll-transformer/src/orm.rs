use sea_orm::entity::prelude::*;

pub mod realm {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "realm")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub name: String,
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)] pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod region {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "region")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub name: String,
        pub description: Option<String>,
        pub hex_count: Option<i64>,
        pub biome_counts_json: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)] pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod biome {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "biome")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub region_id: String,
        pub name: String,
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Region }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Region => Entity::belongs_to(super::region::Entity)
                    .from(Column::RegionId)
                    .to(super::region::Column::Id)
                    .into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod hex {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "hex")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String, // original uuid
        pub realm_id: String,
        pub region_id: Option<String>,
        pub biome_id: Option<String>,
        pub coord: String,                   // "N2"
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Realm, Region, Biome }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Realm => Entity::belongs_to(super::realm::Entity)
                    .from(Column::RealmId).to(super::realm::Column::Id).into(),
                Relation::Region => Entity::belongs_to(super::region::Entity)
                    .from(Column::RegionId).to(super::region::Column::Id).into(),
                Relation::Biome => Entity::belongs_to(super::biome::Entity)
                    .from(Column::BiomeId).to(super::biome::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod settlement {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "settlement")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub name: String,
        pub kind: String, // City/Town/Village
        pub hex_id: Option<String>,
        pub region_id: Option<String>,
        pub biome_id: Option<String>,
        pub population: Option<i64>,
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex, Region, Biome }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
                Relation::Region => Entity::belongs_to(super::region::Entity)
                    .from(Column::RegionId).to(super::region::Column::Id).into(),
                Relation::Biome => Entity::belongs_to(super::biome::Entity)
                    .from(Column::BiomeId).to(super::biome::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod inn {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "inn")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub hex_id: String,
        pub name: String,
        pub description: Option<String>,
        pub has_healer: Option<bool>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod dwelling {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "dwelling")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub hex_id: String,
        pub kind: String, // FarmsCabins or Stronghold
        pub name: Option<String>,
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod dungeon {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "dungeon")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub hex_id: String,
        pub name: String,
        pub kind: String, // Cave/Temple/Tomb
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod monster {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "monster")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub hex_id: Option<String>,
        pub dungeon_id: Option<String>,
        pub name: String,
        pub cr: Option<String>,
        pub ac: Option<String>,
        pub hp: Option<String>,
        pub speed: Option<String>,
        pub abilities_json: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex, Dungeon }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
                Relation::Dungeon => Entity::belongs_to(super::dungeon::Entity)
                    .from(Column::DungeonId).to(super::dungeon::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod npc {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "npc")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub name: String,
        pub settlement_id: Option<String>,
        pub hex_id: Option<String>,
        pub description: Option<String>,
        pub level: Option<i64>,
        pub ac: Option<String>,
        pub hp: Option<String>,
        pub speed: Option<String>,
        pub abilities_json: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Settlement, Hex }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Settlement => Entity::belongs_to(super::settlement::Entity)
                    .from(Column::SettlementId).to(super::settlement::Column::Id).into(),
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod faction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "faction")]
    pub struct Model {
        #[sea_orm(primary_key, column_type = "String")] pub id: String,
        pub name: String,
        pub kind: String, // Cult/Militia/Syndicate
        pub description: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)] pub enum Relation {}
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod npc_faction {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "npc_faction")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub npc_id: String,
        pub faction_id: String,
        pub role: Option<String>,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Npc, Faction }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Npc => Entity::belongs_to(super::npc::Entity)
                    .from(Column::NpcId).to(super::npc::Column::Id).into(),
                Relation::Faction => Entity::belongs_to(super::faction::Entity)
                    .from(Column::FactionId).to(super::faction::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod shop {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "shop")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub settlement_id: String,
        pub name: String,
        pub kind: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Settlement }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Settlement => Entity::belongs_to(super::settlement::Entity)
                    .from(Column::SettlementId).to(super::settlement::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod rumor {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "rumor")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub hex_id: Option<String>,
        pub settlement_id: Option<String>,
        pub text: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex, Settlement }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
                Relation::Settlement => Entity::belongs_to(super::settlement::Entity)
                    .from(Column::SettlementId).to(super::settlement::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod weather {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "weather")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub hex_id: String,
        pub table_json: String,
        pub flood_one_in_6_weekly: bool,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Hex }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Hex => Entity::belongs_to(super::hex::Entity)
                    .from(Column::HexId).to(super::hex::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}

pub mod dialogue {
    use super::*;
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "dialogue")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)] pub id: i32,
        pub npc_id: String,
        pub node_name: String,
        pub yarn_text: String,
    }
    #[derive(Copy, Clone, Debug, EnumIter)]
    pub enum Relation { Npc }
    impl RelationTrait for Relation {
        fn def(&self) -> RelationDef {
            match self {
                Relation::Npc => Entity::belongs_to(super::npc::Entity)
                    .from(Column::NpcId).to(super::npc::Column::Id).into(),
            }
        }
    }
    impl ActiveModelBehavior for ActiveModel {}
}
