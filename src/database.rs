// Dual-database system for Dragon's Labyrinth
// - game.db: Distributed read-only ECS content (maps, NPCs, items, quests)
// - player.db: Runtime read-write player state (saves, progress, settings)

use bevy::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter, Set, TransactionTrait,
};
use sea_orm_migration::prelude::*;
use std::path::PathBuf;
use xdg::BaseDirectories;
use crate::components::*;
use crate::resources::*;

pub mod entities;
pub mod migration;

/// Database plugin managing both game content and player state databases
pub struct DatabasePlugin {
    pub game_database_path: PathBuf,    // Distributed with game (read-only)
    pub app_name: String,                // For XDG directory creation
}

impl Plugin for DatabasePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DatabaseConfig {
            game_db_path: self.game_database_path.clone(),
            app_name: self.app_name.clone(),
        })
        .add_systems(Startup, (
            initialize_databases,
            load_game_content,
            load_player_state,
        ))
        .add_systems(Update, (
            save_player_state,
            sync_player_progress,
        ))
        .add_systems(OnExit(AppExit), persist_final_player_state);
    }
}

/// Database configuration resource
#[derive(Resource, Clone)]
pub struct DatabaseConfig {
    pub game_db_path: PathBuf,     // Path to distributed game.db
    pub app_name: String,           // App name for XDG directories
}

/// Dual database connections
#[derive(Resource, Clone)]
pub struct GameDatabases {
    pub game_db: DatabaseConnection,      // Read-only game content
    pub player_db: DatabaseConnection,    // Read-write player state
    pub player_db_path: PathBuf,         // Path to player.db for backups
}

/// Initialize both game and player databases
async fn initialize_databases(
    config: Res<DatabaseConfig>,
    mut commands: Commands,
) {
    // Connect to read-only game database (distributed with game)
    let game_db_url = format!("sqlite://{}?mode=ro", config.game_db_path.display());
    let game_db = Database::connect(&game_db_url)
        .await
        .expect("Failed to connect to game database");
    
    info!("Connected to game database at: {}", config.game_db_path.display());
    
    // Create player database in XDG data directory
    let xdg_dirs = BaseDirectories::with_prefix(&config.app_name)
        .expect("Failed to create XDG directories");
    
    let player_db_path = xdg_dirs.place_data_file("player.db")
        .expect("Failed to create player database path");
    
    let player_db_url = format!("sqlite://{}?mode=rwc", player_db_path.display());
    let player_db = Database::connect(&player_db_url)
        .await
        .expect("Failed to connect to player database");
    
    // Run migrations on player database only (game.db is pre-migrated)
    migration::PlayerMigrator::up(&player_db, None)
        .await
        .expect("Failed to run player database migrations");
    
    info!("Player database initialized at: {}", player_db_path.display());
    
    commands.insert_resource(GameDatabases {
        game_db,
        player_db,
        player_db_path,
    });
}

/// Load game content from read-only game database
async fn load_game_content(
    dbs: Res<GameDatabases>,
    mut commands: Commands,
    mut hex_world: ResMut<HexWorld>,
) {
    // Load static game content from game.db
    load_hex_tiles_from_game(&dbs.game_db, &mut commands, &mut hex_world).await;
    load_npcs_from_game(&dbs.game_db, &mut commands).await;
    load_quests_from_game(&dbs.game_db, &mut commands).await;
    load_items_from_game(&dbs.game_db, &mut commands).await;
    load_dialogue_from_game(&dbs.game_db, &mut commands).await;
    
    info!("Loaded game content from distribution database");
}

/// Load player state from player database
async fn load_player_state(
    dbs: Res<GameDatabases>,
    mut commands: Commands,
    mut dread_state: ResMut<DreadState>,
    mut narrative_state: ResMut<NarrativeState>,
    mut player_state: ResMut<PlayerState>,
    mut companion_state: ResMut<CompanionState>,
) {
    use entities::player_save;
    
    // Load the most recent save
    if let Ok(Some(save)) = player_save::Entity::find()
        .order_by_desc(player_save::Column::UpdatedAt)
        .one(&dbs.player_db)
        .await
    {
        // Restore dread state
        dread_state.level = save.dread_level as u8;
        dread_state.progress = save.dread_progress;
        dread_state.total_events = save.total_events as u32;
        dread_state.corruption_spread = save.corruption_spread;
        
        // Restore player state
        player_state.sanity = save.player_sanity;
        player_state.health = save.player_health;
        player_state.corruption_level = save.player_corruption;
        player_state.current_hex = Hex::new(save.player_hex_q, save.player_hex_r);
        
        // Load companion states specific to this save
        load_companion_states(&dbs.player_db, &mut companion_state, save.id).await;
        
        // Load quest progress
        load_quest_progress(&dbs.player_db, &mut narrative_state, save.id).await;
        
        info!("Loaded player save #{} from player database", save.id);
    } else {
        info!("No existing save found, starting new game");
        
        // Initialize default companions from game.db
        load_default_companions(&dbs.game_db, &mut commands).await;
    }
}

/// Save player state to player database periodically
fn save_player_state(
    dbs: Res<GameDatabases>,
    dread_state: Res<DreadState>,
    player_state: Res<PlayerState>,
    companion_state: Res<CompanionState>,
    narrative_state: Res<NarrativeState>,
    time: Res<Time>,
    mut save_timer: Local<f32>,
    mut current_save_id: Local<Option<i32>>,
) {
    *save_timer += time.delta_seconds();
    
    // Auto-save every 30 seconds
    if *save_timer >= 30.0 {
        *save_timer = 0.0;
        
        // Save asynchronously to player.db only
        let player_db = dbs.player_db.clone();
        let dread = dread_state.clone();
        let player = player_state.clone();
        let companions = companion_state.clone();
        let narrative = narrative_state.clone();
        let save_id = *current_save_id;
        
        bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
            save_player_to_db(&player_db, &dread, &player, &companions, &narrative, save_id).await
                .expect("Failed to save player state");
        }).detach();
        
        info!("Auto-saved player progress to {}", dbs.player_db_path.display());
    }
}

/// Sync player progress to player database
fn sync_player_progress(
    dbs: Res<GameDatabases>,
    quest_progress: Query<(&Quest, &QuestProgress), Changed<QuestProgress>>,
    companion_changes: Query<&Companion, Changed<Companion>>,
    inventory_changes: Query<&Inventory, Changed<Inventory>>,
) {
    // Only sync player-specific changes to player.db
    for (quest, progress) in quest_progress.iter() {
        let player_db = dbs.player_db.clone();
        let quest_id = quest.id.clone();
        let progress_data = progress.clone();
        
        bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
            save_quest_progress(&player_db, &quest_id, &progress_data).await
                .expect("Failed to save quest progress");
        }).detach();
    }
    
    // Sync companion state changes (trauma, loyalty, etc.)
    for companion in companion_changes.iter() {
        let player_db = dbs.player_db.clone();
        let companion_data = companion.clone();
        
        bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
            update_companion_state(&player_db, &companion_data).await
                .expect("Failed to update companion state");
        }).detach();
    }
    
    // Sync inventory changes
    for inventory in inventory_changes.iter() {
        let player_db = dbs.player_db.clone();
        let inventory_data = inventory.clone();
        
        bevy::tasks::AsyncComputeTaskPool::get().spawn(async move {
            save_inventory(&player_db, &inventory_data).await
                .expect("Failed to save inventory");
        }).detach();
    }
}

/// Persist final player state on exit
fn persist_final_player_state(
    dbs: Res<GameDatabases>,
    dread_state: Res<DreadState>,
    player_state: Res<PlayerState>,
    companion_state: Res<CompanionState>,
    narrative_state: Res<NarrativeState>,
) {
    let player_db = dbs.player_db.clone();
    let dread = dread_state.clone();
    let player = player_state.clone();
    let companions = companion_state.clone();
    let narrative = narrative_state.clone();
    
    // Block on final save to ensure it completes
    bevy::tasks::block_on(async move {
        save_player_to_db(&player_db, &dread, &player, &companions, &narrative, None).await
            .expect("Failed to save final player state");
        info!("Final player state persisted to player database");
    });
}

// Database helper functions

// Player database functions (read-write)
async fn save_player_to_db(
    conn: &DatabaseConnection,
    dread: &DreadState,
    player: &PlayerState,
    companions: &CompanionState,
    narrative: &NarrativeState,
    existing_save_id: Option<i32>,
) -> Result<i32, sea_orm::DbErr> {
    use entities::player_save;
    
    let save_model = if let Some(id) = existing_save_id {
        // Update existing save
        player_save::ActiveModel {
            id: Set(id),
            dread_level: Set(dread.level as i32),
            dread_progress: Set(dread.progress),
            total_events: Set(dread.total_events as i32),
            corruption_spread: Set(dread.corruption_spread),
            player_sanity: Set(player.sanity),
            player_health: Set(player.health),
            player_corruption: Set(player.corruption_level),
            player_hex_q: Set(player.current_hex.q),
            player_hex_r: Set(player.current_hex.r),
            current_act: Set(narrative.current_act as i32),
            playtime: Set(0.0), // TODO: Track actual playtime
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
    } else {
        // Create new save
        player_save::ActiveModel {
            dread_level: Set(dread.level as i32),
            dread_progress: Set(dread.progress),
            total_events: Set(dread.total_events as i32),
            corruption_spread: Set(dread.corruption_spread),
            player_sanity: Set(player.sanity),
            player_health: Set(player.health),
            player_corruption: Set(player.corruption_level),
            player_hex_q: Set(player.current_hex.q),
            player_hex_r: Set(player.current_hex.r),
            current_act: Set(narrative.current_act as i32),
            playtime: Set(0.0),
            created_at: Set(chrono::Utc::now()),
            updated_at: Set(chrono::Utc::now()),
            ..Default::default()
        }
    };
    
    let saved = save_model.save(conn).await?;
    Ok(saved.id.unwrap())
}

// Game database functions (read-only, distributed content)
async fn load_hex_tiles_from_game(
    conn: &DatabaseConnection,
    commands: &mut Commands,
    hex_world: &mut HexWorld,
) {
    use entities::hex_tiles;
    
    if let Ok(tiles) = hex_tiles::Entity::find().all(conn).await {
        for tile_data in tiles {
            let hex = Hex::new(tile_data.q, tile_data.r);
            let tile = HexTile {
                hex,
                tile_type: parse_tile_type(&tile_data.tile_type),
                dread_level: tile_data.dread_level as u8,
                corruption: tile_data.corruption,
                elevation: tile_data.elevation,
                passable: tile_data.passable,
            };
            
            hex_world.tiles.insert(hex, tile.clone());
            commands.spawn(tile);
        }
        info!("Loaded {} hex tiles from game database", tiles.len());
    }
}

async fn load_npcs_from_game(
    conn: &DatabaseConnection,
    commands: &mut Commands,
) {
    use entities::npcs;
    
    if let Ok(npcs) = npcs::Entity::find().all(conn).await {
        for npc_data in npcs {
            let npc = NPC {
                name: npc_data.name,
                npc_type: npc_data.npc_type,
                sanity: npc_data.sanity,
                dialogue_tree: npc_data.dialogue_tree,
                flee_threshold: npc_data.flee_threshold,
            };
            
            commands.spawn(npc);
        }
        info!("Loaded {} NPCs from game database", npcs.len());
    }
}

async fn load_quests_from_game(
    conn: &DatabaseConnection,
    commands: &mut Commands,
) {
    use entities::quests;
    
    if let Ok(quests) = quests::Entity::find().all(conn).await {
        for quest_data in quests {
            let quest = Quest {
                id: quest_data.quest_id,
                title: quest_data.title,
                description: quest_data.description,
                quest_type: parse_quest_type(&quest_data.quest_type),
                completion_status: QuestStatus::Available, // All quests start available
                required_dread_level: quest_data.required_dread_level as u8,
                moral_choice: None, // Load separately if present
            };
            
            commands.spawn(quest);
        }
        info!("Loaded {} quests from game database", quests.len());
    }
}

async fn load_items_from_game(
    conn: &DatabaseConnection,
    commands: &mut Commands,
) {
    // TODO: Implement item loading from game.db
    info!("Item loading from game database not yet implemented");
}

async fn load_dialogue_from_game(
    conn: &DatabaseConnection,
    commands: &mut Commands,
) {
    // TODO: Implement dialogue loading from game.db
    info!("Dialogue loading from game database not yet implemented");
}

async fn load_default_companions(
    conn: &DatabaseConnection,
    commands: &mut Commands,
) {
    use entities::companions;
    
    if let Ok(companions) = companions::Entity::find().all(conn).await {
        for companion_data in companions {
            let companion = Companion {
                name: companion_data.name,
                companion_type: parse_companion_type(&companion_data.companion_type),
                sanity: companion_data.sanity,
                loyalty: companion_data.loyalty,
                trauma_level: 0.0, // Start fresh
            };
            
            commands.spawn(companion);
        }
        info!("Loaded {} default companions from game database", companions.len());
    }
}

// Player database helper functions
async fn load_companion_states(
    conn: &DatabaseConnection,
    companion_state: &mut CompanionState,
    save_id: i32,
) {
    // TODO: Load companion states specific to this save
    info!("Loading companion states for save #{}", save_id);
}

async fn load_quest_progress(
    conn: &DatabaseConnection,
    narrative_state: &mut NarrativeState,
    save_id: i32,
) {
    // TODO: Load quest progress specific to this save
    info!("Loading quest progress for save #{}", save_id);
}

async fn save_quest_progress(
    conn: &DatabaseConnection,
    quest_id: &str,
    progress: &QuestProgress,
) -> Result<(), sea_orm::DbErr> {
    // TODO: Save quest progress to player database
    Ok(())
}

async fn update_companion_state(
    conn: &DatabaseConnection,
    companion: &Companion,
) -> Result<(), sea_orm::DbErr> {
    // TODO: Update companion state in player database
    Ok(())
}

async fn save_inventory(
    conn: &DatabaseConnection,
    inventory: &Inventory,
) -> Result<(), sea_orm::DbErr> {
    // TODO: Save inventory to player database
    Ok(())
}

// Parse helpers for converting strings back to enums
fn parse_tile_type(s: &str) -> TileType {
    match s {
        "Grass" => TileType::Grass,
        "Forest" => TileType::Forest,
        "Swamp" => TileType::Swamp,
        "Stone" => TileType::Stone,
        "Corrupted" => TileType::Corrupted,
        _ => TileType::Grass,
    }
}

fn parse_companion_type(s: &str) -> CompanionType {
    match s {
        "Einar" => CompanionType::Einar,
        "Mira" => CompanionType::Mira,
        "Sorin" => CompanionType::Sorin,
        "Tamara" => CompanionType::Tamara,
        _ => CompanionType::Einar,
    }
}

fn parse_quest_type(s: &str) -> QuestType {
    match s {
        "Delivery" => QuestType::Delivery,
        "Investigation" => QuestType::Investigation,
        "Survival" => QuestType::Survival,
        "MoralDilemma" => QuestType::MoralDilemma,
        "BossEncounter" => QuestType::BossEncounter,
        _ => QuestType::Delivery,
    }
}

fn parse_quest_status(s: &str) -> QuestStatus {
    match s {
        "Available" => QuestStatus::Available,
        "Active" => QuestStatus::Active,
        "Completed" => QuestStatus::Completed,
        "Failed" => QuestStatus::Failed,
        "Abandoned" => QuestStatus::Abandoned,
        _ => QuestStatus::Available,
    }
}

pub mod entities {
    // Player database entities (runtime state)
    pub mod player_save {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "player_saves")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub dread_level: i32,
            pub dread_progress: f32,
            pub total_events: i32,
            pub corruption_spread: f32,
            pub player_sanity: f32,
            pub player_health: f32,
            pub player_corruption: f32,
            pub player_hex_q: i32,
            pub player_hex_r: i32,
            pub current_act: i32,
            pub playtime: f32,
            pub created_at: chrono::DateTime<chrono::Utc>,
            pub updated_at: chrono::DateTime<chrono::Utc>,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        
        impl ActiveModelBehavior for ActiveModel {}
    }
    
    pub mod hex_tiles {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "hex_tiles")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub q: i32,
            pub r: i32,
            pub tile_type: String,
            pub dread_level: i32,
            pub corruption: f32,
            pub elevation: f32,
            pub passable: bool,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        
        impl ActiveModelBehavior for ActiveModel {}
    }
    
    pub mod companions {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "companions")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub name: String,
            pub companion_type: String,
            pub sanity: f32,
            pub loyalty: f32,
            pub trauma_level: f32,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        
        impl ActiveModelBehavior for ActiveModel {}
    }
    
    pub mod npcs {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "npcs")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub name: String,
            pub npc_type: String,
            pub sanity: f32,
            pub dialogue_tree: String,
            pub flee_threshold: f32,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        
        impl ActiveModelBehavior for ActiveModel {}
    }
    
    pub mod quests {
        use sea_orm::entity::prelude::*;
        
        #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
        #[sea_orm(table_name = "quests")]
        pub struct Model {
            #[sea_orm(primary_key)]
            pub id: i32,
            pub quest_id: String,
            pub title: String,
            pub description: String,
            pub quest_type: String,
            pub status: String,
            pub required_dread_level: i32,
        }
        
        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}
        
        impl ActiveModelBehavior for ActiveModel {}
    }
}

pub mod migration {
    use sea_orm_migration::prelude::*;
    
    // Player database migrator
    pub struct PlayerMigrator;
    
    #[async_trait::async_trait]
    impl MigratorTrait for PlayerMigrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![
                Box::new(CreatePlayerTables),
            ]
        }
    }
    
    // Game database migrator (for initial setup)
    pub struct GameMigrator;
    
    #[async_trait::async_trait]
    impl MigratorTrait for GameMigrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![
                Box::new(CreateGameTables),
            ]
        }
    }
    
    pub struct CreatePlayerTables;
    
    impl MigrationName for CreatePlayerTables {
        fn name(&self) -> &str {
            "m20240101_000001_create_player_tables"
        }
    }
    
    #[async_trait::async_trait]
    impl MigrationTrait for CreatePlayerTables {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // Create player_saves table
            manager
                .create_table(
                    Table::create()
                        .table(PlayerSaves::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(PlayerSaves::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(PlayerSaves::DreadLevel).integer().not_null())
                        .col(ColumnDef::new(PlayerSaves::DreadProgress).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::TotalEvents).integer().not_null())
                        .col(ColumnDef::new(PlayerSaves::CorruptionSpread).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::PlayerSanity).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::PlayerHealth).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::PlayerCorruption).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::PlayerHexQ).integer().not_null())
                        .col(ColumnDef::new(PlayerSaves::PlayerHexR).integer().not_null())
                        .col(ColumnDef::new(PlayerSaves::CurrentAct).integer().not_null())
                        .col(ColumnDef::new(PlayerSaves::Playtime).float().not_null())
                        .col(ColumnDef::new(PlayerSaves::CreatedAt).timestamp().not_null())
                        .col(ColumnDef::new(PlayerSaves::UpdatedAt).timestamp().not_null())
                        .to_owned(),
                )
                .await?;
            
            // Create quest_progress table  
            manager
                .create_table(
                    Table::create()
                        .table(QuestProgress::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(QuestProgress::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(QuestProgress::SaveId).integer().not_null())
                        .col(ColumnDef::new(QuestProgress::QuestId).string().not_null())
                        .col(ColumnDef::new(QuestProgress::Status).string().not_null())
                        .col(ColumnDef::new(QuestProgress::Progress).float().not_null())
                        .col(ColumnDef::new(QuestProgress::MoralChoice).string())
                        .to_owned(),
                )
                .await?;
            
            // Create companion_states table
            manager
                .create_table(
                    Table::create()
                        .table(CompanionStates::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(CompanionStates::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(CompanionStates::SaveId).integer().not_null())
                        .col(ColumnDef::new(CompanionStates::CompanionName).string().not_null())
                        .col(ColumnDef::new(CompanionStates::Sanity).float().not_null())
                        .col(ColumnDef::new(CompanionStates::Loyalty).float().not_null())
                        .col(ColumnDef::new(CompanionStates::TraumaLevel).float().not_null())
                        .col(ColumnDef::new(CompanionStates::IsActive).boolean().not_null())
                        .to_owned(),
                )
                .await?;
            
            // Create inventory table
            manager
                .create_table(
                    Table::create()
                        .table(PlayerInventory::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(PlayerInventory::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(PlayerInventory::SaveId).integer().not_null())
                        .col(ColumnDef::new(PlayerInventory::ItemId).string().not_null())
                        .col(ColumnDef::new(PlayerInventory::Quantity).integer().not_null())
                        .col(ColumnDef::new(PlayerInventory::Equipped).boolean().not_null())
                        .to_owned(),
                )
                .await?;
            
            Ok(())
        }
        
        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager.drop_table(Table::drop().table(PlayerSaves::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(QuestProgress::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(CompanionStates::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(PlayerInventory::Table).to_owned()).await?;
            Ok(())
        }
    }
    
    pub struct CreateGameTables;
    
    impl MigrationName for CreateGameTables {
        fn name(&self) -> &str {
            "m20240101_000001_create_game_tables"
        }
    }
    
    #[async_trait::async_trait]
    impl MigrationTrait for CreateGameTables {
        async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            // Create hex_tiles table for distributed game content
            manager
                .create_table(
                    Table::create()
                        .table(HexTiles::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(HexTiles::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(HexTiles::Q).integer().not_null())
                        .col(ColumnDef::new(HexTiles::R).integer().not_null())
                        .col(ColumnDef::new(HexTiles::TileType).string().not_null())
                        .col(ColumnDef::new(HexTiles::DreadLevel).integer().not_null())
                        .col(ColumnDef::new(HexTiles::Corruption).float().not_null())
                        .col(ColumnDef::new(HexTiles::Elevation).float().not_null())
                        .col(ColumnDef::new(HexTiles::Passable).boolean().not_null())
                        .to_owned(),
                )
                .await?;
            
            // Create npcs table
            manager
                .create_table(
                    Table::create()
                        .table(Npcs::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Npcs::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Npcs::Name).string().not_null())
                        .col(ColumnDef::new(Npcs::NpcType).string().not_null())
                        .col(ColumnDef::new(Npcs::Sanity).float().not_null())
                        .col(ColumnDef::new(Npcs::DialogueTree).string().not_null())
                        .col(ColumnDef::new(Npcs::FleeThreshold).float().not_null())
                        .to_owned(),
                )
                .await?;
            
            // Create quests table
            manager
                .create_table(
                    Table::create()
                        .table(Quests::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Quests::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Quests::QuestId).string().not_null())
                        .col(ColumnDef::new(Quests::Title).string().not_null())
                        .col(ColumnDef::new(Quests::Description).string().not_null())
                        .col(ColumnDef::new(Quests::QuestType).string().not_null())
                        .col(ColumnDef::new(Quests::RequiredDreadLevel).integer().not_null())
                        .to_owned(),
                )
                .await?;
            
            // Create companions table
            manager
                .create_table(
                    Table::create()
                        .table(Companions::Table)
                        .if_not_exists()
                        .col(
                            ColumnDef::new(Companions::Id)
                                .integer()
                                .not_null()
                                .auto_increment()
                                .primary_key(),
                        )
                        .col(ColumnDef::new(Companions::Name).string().not_null())
                        .col(ColumnDef::new(Companions::CompanionType).string().not_null())
                        .col(ColumnDef::new(Companions::Sanity).float().not_null())
                        .col(ColumnDef::new(Companions::Loyalty).float().not_null())
                        .col(ColumnDef::new(Companions::TraumaLevel).float().not_null())
                        .to_owned(),
                )
                .await?;
            
            Ok(())
        }
        
        async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
            manager.drop_table(Table::drop().table(HexTiles::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(Npcs::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(Quests::Table).to_owned()).await?;
            manager.drop_table(Table::drop().table(Companions::Table).to_owned()).await?;
            Ok(())
        }
    }
    
    // Table identifiers for migrations
    #[derive(Iden)]
    enum PlayerSaves {
        Table,
        Id,
        DreadLevel,
        DreadProgress,
        TotalEvents,
        CorruptionSpread,
        PlayerSanity,
        PlayerHealth,
        PlayerCorruption,
        PlayerHexQ,
        PlayerHexR,
        CurrentAct,
        Playtime,
        CreatedAt,
        UpdatedAt,
    }
    
    #[derive(Iden)]
    enum QuestProgress {
        Table,
        Id,
        SaveId,
        QuestId,
        Status,
        Progress,
        MoralChoice,
    }
    
    #[derive(Iden)]
    enum CompanionStates {
        Table,
        Id,
        SaveId,
        CompanionName,
        Sanity,
        Loyalty,
        TraumaLevel,
        IsActive,
    }
    
    #[derive(Iden)]
    enum PlayerInventory {
        Table,
        Id,
        SaveId,
        ItemId,
        Quantity,
        Equipped,
    }
    
    #[derive(Iden)]
    enum HexTiles {
        Table,
        Id,
        Q,
        R,
        TileType,
        DreadLevel,
        Corruption,
        Elevation,
        Passable,
    }
    
    #[derive(Iden)]
    enum Companions {
        Table,
        Id,
        Name,
        CompanionType,
        Sanity,
        Loyalty,
        TraumaLevel,
    }
    
    #[derive(Iden)]
    enum Npcs {
        Table,
        Id,
        Name,
        NpcType,
        Sanity,
        DialogueTree,
        FleeThreshold,
    }
    
    #[derive(Iden)]
    enum Quests {
        Table,
        Id,
        QuestId,
        Title,
        Description,
        QuestType,
        RequiredDreadLevel,
    }
}