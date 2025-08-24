//! Save system with player.db and game.db separation
//!
//! This module implements the dual-database save system:
//! - player.db: Player-specific data that persists across playthroughs
//! - game.db: Current game state that resets with new games

use sea_orm::{Database, DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Player database - persists across all playthroughs
pub struct PlayerDatabase {
    conn: DatabaseConnection,
}

/// Game database - specific to current playthrough
pub struct GameDatabase {
    conn: DatabaseConnection,
}

/// Save game metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveGameMeta {
    pub save_id: Uuid,
    pub player_id: Uuid,
    pub save_name: String,
    pub level: u32,
    pub playtime_seconds: u64,
    pub dread_level: u8,
    pub companion_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Player profile - persists across games
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub player_id: Uuid,
    pub username: String,
    pub total_playtime: u64,
    pub achievements_unlocked: Vec<String>,
    pub highest_level_reached: u32,
    pub total_deaths: u32,
    pub endings_seen: Vec<EndingType>,
    pub philosophy_tendencies: PhilosophyStats,
    pub created_at: DateTime<Utc>,
}

/// Philosophy tracking across playthroughs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PhilosophyStats {
    pub strength_choices: u32,
    pub harmony_choices: u32,
    pub light_choices: u32,
    pub dark_choices: u32,
    pub void_embraces: u32,
}

/// Types of endings the player has achieved
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EndingType {
    SealedVoid,         // Sacrificed self to seal void
    BecameTyrant,       // Ruled with dragon power
    CompanionBetrayal,  // Betrayed by companion
    TrueUnderstanding,  // Understood the cycle
    VoidConsumption,    // Consumed by void
}

/// Current game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub save_id: Uuid,
    pub current_level: u32,
    pub player_position: (f32, f32, f32),
    pub health: f32,
    pub max_health: f32,
    pub dread_level: u8,
    pub corruption: f32,
    pub inventory: Vec<InventoryItem>,
    pub equipped_items: EquippedItems,
    pub companion_state: Option<CompanionState>,
    pub mount_state: Option<MountState>,
    pub quest_progress: Vec<QuestProgress>,
    pub discovered_locations: Vec<String>,
    pub death_scars: Vec<DeathScar>,
}

/// Inventory item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: String,
    pub quantity: u32,
    pub durability: Option<f32>,
    pub enchantments: Vec<String>,
}

/// Equipped items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedItems {
    pub weapon: Option<String>,
    pub armor: Option<String>,
    pub accessory: Option<String>,
    pub mount: Option<String>,
}

/// Companion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionState {
    pub companion_type: CompanionType,
    pub name: String,
    pub relationship_level: f32,
    pub trust: f32,
    pub corruption: f32,
    pub alive: bool,
    pub betrayed: bool,
}

/// Companion types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompanionType {
    Elena,   // Optimistic adventurer
    Marcus,  // Grizzled warrior
    Quinn,   // Mystic scholar
}

/// Mount state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountState {
    pub mount_type: String,
    pub name: String,
    pub bond_level: f32,
    pub health: f32,
    pub stamina: f32,
    pub void_sensitivity: f32,
}

/// Quest progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgress {
    pub quest_id: String,
    pub stage: u32,
    pub completed: bool,
    pub failed: bool,
}

/// Death scar system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathScar {
    pub scar_type: ScarType,
    pub level_acquired: u32,
    pub permanent: bool,
    pub effect: ScarEffect,
}

/// Types of death scars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScarType {
    Physical,    // Visible wound
    Mental,      // Psychological trauma
    Spiritual,   // Soul damage
    Void,        // Void corruption
}

/// Effects of death scars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScarEffect {
    ReducedMaxHealth(f32),
    IncreasedCorruption(f32),
    CombatPenalty(String),
    VoidResistance(f32),
    UnlockAbility(String),
}

impl PlayerDatabase {
    /// Open or create player database
    pub async fn open(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = format!("sqlite://{}?mode=rwc", path.display());
        let conn = Database::connect(&db_url).await?;
        
        // Run migrations for player database
        Self::migrate(&conn).await?;
        
        Ok(Self { conn })
    }
    
    /// Run player database migrations
    async fn migrate(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        // Create player profile table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS player_profiles (
                player_id TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                total_playtime INTEGER NOT NULL DEFAULT 0,
                highest_level_reached INTEGER NOT NULL DEFAULT 0,
                total_deaths INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create achievements table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS player_achievements (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                achievement_id TEXT NOT NULL,
                unlocked_at INTEGER NOT NULL,
                FOREIGN KEY (player_id) REFERENCES player_profiles(player_id),
                UNIQUE(player_id, achievement_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create endings table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS player_endings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                player_id TEXT NOT NULL,
                ending_type TEXT NOT NULL,
                achieved_at INTEGER NOT NULL,
                FOREIGN KEY (player_id) REFERENCES player_profiles(player_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create philosophy stats table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS philosophy_stats (
                player_id TEXT PRIMARY KEY,
                strength_choices INTEGER NOT NULL DEFAULT 0,
                harmony_choices INTEGER NOT NULL DEFAULT 0,
                light_choices INTEGER NOT NULL DEFAULT 0,
                dark_choices INTEGER NOT NULL DEFAULT 0,
                void_embraces INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (player_id) REFERENCES player_profiles(player_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        Ok(())
    }
    
    /// Create new player profile
    pub async fn create_player(&self, username: String) -> Result<PlayerProfile, Box<dyn std::error::Error>> {
        let profile = PlayerProfile {
            player_id: Uuid::new_v4(),
            username,
            total_playtime: 0,
            achievements_unlocked: vec![],
            highest_level_reached: 0,
            total_deaths: 0,
            endings_seen: vec![],
            philosophy_tendencies: PhilosophyStats::default(),
            created_at: Utc::now(),
        };
        
        // Insert into database
        let sql = format!(
            "INSERT INTO player_profiles (player_id, username, created_at) VALUES ('{}', '{}', {})",
            profile.player_id,
            profile.username,
            profile.created_at.timestamp()
        );
        self.conn.execute_unprepared(&sql).await?;
        
        Ok(profile)
    }
    
    /// Update player statistics
    pub async fn update_stats(
        &self, 
        player_id: Uuid,
        playtime_delta: u64,
        level: u32,
        deaths_delta: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sql = format!(
            "UPDATE player_profiles SET 
             total_playtime = total_playtime + {},
             highest_level_reached = MAX(highest_level_reached, {}),
             total_deaths = total_deaths + {}
             WHERE player_id = '{}'",
            playtime_delta, level, deaths_delta, player_id
        );
        self.conn.execute_unprepared(&sql).await?;
        Ok(())
    }
    
    /// Record achievement unlock
    pub async fn unlock_achievement(
        &self,
        player_id: Uuid,
        achievement_id: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sql = format!(
            "INSERT OR IGNORE INTO player_achievements (player_id, achievement_id, unlocked_at) 
             VALUES ('{}', '{}', {})",
            player_id, achievement_id, Utc::now().timestamp()
        );
        self.conn.execute_unprepared(&sql).await?;
        Ok(())
    }
    
    /// Record ending achieved
    pub async fn record_ending(
        &self,
        player_id: Uuid,
        ending: EndingType,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sql = format!(
            "INSERT INTO player_endings (player_id, ending_type, achieved_at) 
             VALUES ('{}', '{:?}', {})",
            player_id, ending, Utc::now().timestamp()
        );
        self.conn.execute_unprepared(&sql).await?;
        Ok(())
    }
}

impl GameDatabase {
    /// Open or create game database for current playthrough
    pub async fn open(path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let db_url = format!("sqlite://{}?mode=rwc", path.display());
        let conn = Database::connect(&db_url).await?;
        
        // Run migrations for game database
        Self::migrate(&conn).await?;
        
        Ok(Self { conn })
    }
    
    /// Run game database migrations
    async fn migrate(conn: &DatabaseConnection) -> Result<(), Box<dyn std::error::Error>> {
        // Create save game metadata table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS save_games (
                save_id TEXT PRIMARY KEY,
                player_id TEXT NOT NULL,
                save_name TEXT NOT NULL,
                level INTEGER NOT NULL,
                playtime_seconds INTEGER NOT NULL,
                dread_level INTEGER NOT NULL,
                companion_name TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create game state table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS game_states (
                save_id TEXT PRIMARY KEY,
                current_level INTEGER NOT NULL,
                player_x REAL NOT NULL,
                player_y REAL NOT NULL,
                player_z REAL NOT NULL,
                health REAL NOT NULL,
                max_health REAL NOT NULL,
                dread_level INTEGER NOT NULL,
                corruption REAL NOT NULL,
                state_json TEXT NOT NULL,
                FOREIGN KEY (save_id) REFERENCES save_games(save_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create inventory table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS inventory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                save_id TEXT NOT NULL,
                item_id TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                durability REAL,
                enchantments TEXT,
                FOREIGN KEY (save_id) REFERENCES save_games(save_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        // Create death scars table
        let sql = r#"
            CREATE TABLE IF NOT EXISTS death_scars (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                save_id TEXT NOT NULL,
                scar_type TEXT NOT NULL,
                level_acquired INTEGER NOT NULL,
                permanent INTEGER NOT NULL,
                effect_json TEXT NOT NULL,
                FOREIGN KEY (save_id) REFERENCES save_games(save_id)
            )
        "#;
        conn.execute_unprepared(sql).await?;
        
        Ok(())
    }
    
    /// Create new save game
    pub async fn create_save(
        &self,
        player_id: Uuid,
        save_name: String,
    ) -> Result<SaveGameMeta, Box<dyn std::error::Error>> {
        let save = SaveGameMeta {
            save_id: Uuid::new_v4(),
            player_id,
            save_name,
            level: 1,
            playtime_seconds: 0,
            dread_level: 0,
            companion_name: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let sql = format!(
            "INSERT INTO save_games (save_id, player_id, save_name, level, playtime_seconds, dread_level, created_at, updated_at) 
             VALUES ('{}', '{}', '{}', {}, {}, {}, {}, {})",
            save.save_id, save.player_id, save.save_name, save.level, 
            save.playtime_seconds, save.dread_level,
            save.created_at.timestamp(), save.updated_at.timestamp()
        );
        self.conn.execute_unprepared(&sql).await?;
        
        Ok(save)
    }
    
    /// Save game state
    pub async fn save_state(&self, state: &GameState) -> Result<(), Box<dyn std::error::Error>> {
        let state_json = serde_json::to_string(state)?;
        
        let sql = format!(
            "INSERT OR REPLACE INTO game_states 
             (save_id, current_level, player_x, player_y, player_z, health, max_health, dread_level, corruption, state_json)
             VALUES ('{}', {}, {}, {}, {}, {}, {}, {}, {}, '{}')",
            state.save_id, state.current_level,
            state.player_position.0, state.player_position.1, state.player_position.2,
            state.health, state.max_health, state.dread_level, state.corruption,
            state_json
        );
        self.conn.execute_unprepared(&sql).await?;
        
        // Update save metadata
        let sql = format!(
            "UPDATE save_games SET level = {}, dread_level = {}, updated_at = {} WHERE save_id = '{}'",
            state.current_level, state.dread_level, Utc::now().timestamp(), state.save_id
        );
        self.conn.execute_unprepared(&sql).await?;
        
        Ok(())
    }
    
    /// Load game state
    pub async fn load_state(&self, save_id: Uuid) -> Result<GameState, Box<dyn std::error::Error>> {
        let sql = format!(
            "SELECT state_json FROM game_states WHERE save_id = '{}'",
            save_id
        );
        
        let result = self.conn.execute_unprepared(&sql).await?;
        // Parse result and deserialize JSON
        // This is simplified - actual implementation would properly handle the query result
        
        todo!("Implement proper query result handling")
    }
    
    /// Delete save game
    pub async fn delete_save(&self, save_id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        // Delete in order due to foreign keys
        let tables = ["death_scars", "inventory", "game_states", "save_games"];
        
        for table in &tables {
            let sql = format!("DELETE FROM {} WHERE save_id = '{}'", table, save_id);
            self.conn.execute_unprepared(&sql).await?;
        }
        
        Ok(())
    }
}

/// Get the save directory path
pub fn get_save_directory() -> PathBuf {
    if let Some(data_dir) = dirs::data_dir() {
        data_dir.join("dragons-labyrinth").join("saves")
    } else {
        PathBuf::from("./saves")
    }
}

/// Get player database path
pub fn get_player_db_path() -> PathBuf {
    get_save_directory().join("player.db")
}

/// Get game database path for a specific save
pub fn get_game_db_path(save_id: Uuid) -> PathBuf {
    get_save_directory().join(format!("game_{}.db", save_id))
}
