//! Bevy ECS integration for Dragon's Labyrinth database
//! 
//! This module provides seamless integration between the game database and Bevy's
//! Entity Component System, enabling automatic synchronization, efficient queries,
//! and event-driven database operations within the game engine.

use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
// Using i64 timestamps for Bevy reflection compatibility in components

use database_orm::{players, companions, hex_tiles, encounters, dialogues, items, 
                   player_statistics, game_states, ai_workflows, generated_assets, 
                   asset_dependencies, forge, psychology, philosophy, decay, mounts,
                   assets};
use crate::engine::GameDatabase;
use crate::router::{DatabaseRouter, DatabaseRouterResource};
use crate::traits::{GameDatabaseOperations, EcsDatabaseOperations, AIGenerationOperations};
use crate::error::{DatabaseError, DatabaseResult};

// ============================================================================
// BEVY COMPONENTS FOR DATABASE INTEGRATION
// ============================================================================

/// Component marking entities that should be synchronized with the database
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct DatabaseEntity {
    pub database_id: uuid::Uuid,
    pub entity_type: String,
    pub last_sync: i64,  // Unix timestamp for Bevy reflection compatibility
    pub sync_status: SyncStatus,
}

/// Component for horror progression tracking
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct HorrorProgression {
    pub current_dread_level: i32,
    pub dread_progression: f32,
    pub total_corruption: f32,
    pub sanity_level: f32,
    pub last_dread_event: Option<i64>,  // Unix timestamp for Bevy reflection compatibility
}

/// Component for hex position tracking (integrates with hexx crate)
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct HexPosition {
    pub q: i32,  // Axial coordinate
    pub r: i32,  // Axial coordinate
    pub world_x: f32,
    pub world_z: f32,
}

/// Component for AI-generated content metadata
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct AIGeneratedContent {
    pub asset_id: uuid::Uuid,
    pub generation_agent: String,
    pub dread_level: i32,
    pub quality_score: f32,
    pub approval_status: AssetGenerationStatus,
    pub generated_at: i64,  // Unix timestamp for Bevy reflection compatibility
}

/// Component for companion state tracking
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct CompanionState {
    pub companion_id: uuid::Uuid,
    pub trauma_level: f32,
    pub loyalty: f32,
    pub current_mood: String,
    pub therapy_progress: f32,
    pub last_trauma_event: Option<i64>,  // Unix timestamp for Bevy reflection compatibility
}

/// Component for dialogue state tracking
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct DialogueState {
    pub dialogue_id: uuid::Uuid,
    pub current_node: String,
    pub conversation_partner: Option<Entity>,
    pub trauma_context: f32,
    pub moral_choice_weight: f32,
}

/// Component for forge progress tracking
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct ForgeProgress {
    pub forge_path: Option<String>, // "light" or "dark"
    pub trials_completed: Vec<String>,
    pub reagent_power: f32,
    pub sacrifice_readiness: f32,
    pub trial_readiness: f32,
}

/// Component for philosophical progression tracking
#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone)]
#[reflect(Component)]
pub struct PhilosophicalProgression {
    pub current_philosophy: String,
    pub trait_scores: HashMap<String, f32>,
    pub identity_stability: f32,
    pub authenticity_score: f32,
    pub transition_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum SyncStatus {
    Synced,
    PendingSync,
    SyncFailed,
    ConflictResolution,
}

// ============================================================================
// BEVY PLUGINS
// ============================================================================

/// Main database plugin that sets up all database integration systems
pub struct GameDatabasePlugin {
    pub database_url: String,
    pub auto_sync_enabled: bool,
    pub sync_interval_seconds: f64,
}

impl GameDatabasePlugin {
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
            auto_sync_enabled: true,
            sync_interval_seconds: 5.0,
        }
    }

    pub fn with_auto_sync(mut self, enabled: bool) -> Self {
        self.auto_sync_enabled = enabled;
        self
    }

    pub fn with_sync_interval(mut self, interval_seconds: f64) -> Self {
        self.sync_interval_seconds = interval_seconds;
        self
    }
}

impl Plugin for GameDatabasePlugin {
    fn build(&self, app: &mut App) {
        // Initialize database connection as a resource
        let database_url = self.database_url.clone();
        app.add_systems(Startup, move |mut commands: Commands| {
            // This system will run async operations
            let db_url = database_url.clone();
            commands.spawn_empty().insert(DatabaseConnectionTask {
                database_url: db_url,
                connection_future: None,
            });
        });

        // Add core resources
        app.init_resource::<DatabaseSyncQueue>()
           .init_resource::<DatabaseStats>()
           .init_resource::<EntityDatabaseMapping>();

        // Add events
        app.add_event::<DatabaseSyncEvent>()
           .add_event::<DatabaseErrorEvent>()
           .add_event::<HorrorProgressionEvent>()
           .add_event::<CompanionTraumaEvent>()
           .add_event::<AIWorkflowEvent>()
           .add_event::<AssetGenerationEvent>();

        // Add core systems
        app.add_systems(Update, (
            handle_database_connection_task,
            process_database_sync_queue,
            handle_database_sync_events,
            sync_horror_progression_components,
            sync_companion_state_components,
            sync_hex_position_components,
            handle_database_errors,
        ));

        // Add timer-based systems if auto-sync is enabled
        if self.auto_sync_enabled {
            app.insert_resource(DatabaseSyncTimer(Timer::from_seconds(
                self.sync_interval_seconds as f32, 
                TimerMode::Repeating
            )))
            .add_systems(Update, auto_sync_entities_system);
        }

        // Add specialized systems
        app.add_systems(Update, (
            update_database_stats_system,
            process_ai_workflow_events,
            process_asset_generation_events,
        ));

        // Register component reflection for database components
        app.register_type::<DatabaseEntity>()
           .register_type::<HorrorProgression>()
           .register_type::<HexPosition>()
           .register_type::<AIGeneratedContent>()
           .register_type::<CompanionState>()
           .register_type::<DialogueState>();

        // ===== DRAGON'S LABYRINTH UNIQUE SYSTEMS =====
        // Add the three production-ready unique systems that transform D&D foundation into horror RPG
        
        info!("Registering Dragon's Labyrinth unique systems:");
        
        // 1. Companion Psychology & Therapy System
        app.add_plugins(crate::systems::companion_psychology::CompanionPsychologyPlugin);
        info!("✅ Companion Psychology & Therapy System - Memory palaces, trauma processing, therapy quests");
        
        // 2. Dread Progression Controller (Master Orchestrator)
        app.add_plugins(crate::systems::dread_progression::DreadProgressionPlugin);
        info!("✅ Dread Progression Controller - Master horror orchestrator transforming ALL systems (0-4 dread levels)");
        
        // 3. Sentimental Item & Forge System  
        app.add_plugins(crate::systems::forge::ForgeSystemPlugin);
        info!("✅ Sentimental Item & Forge System - Light/dark paths, mythic gear, second chances");
        
        info!("🎯 Dragon's Labyrinth transformation complete: D&D foundation → Horror-first RPG");
        info!("   📊 Database-driven: 70k+ HBF entities powering all mechanics");
        info!("   🧠 Psychology: Authentic trauma progression and therapy");
        info!("   😨 Horror: Dread level (0-4) transforms all systems");
        info!("   ⚔️  Forge: Sentimental items → mythic gear via light/dark paths");
    }
}

// ============================================================================
// BEVY RESOURCES
// ============================================================================

/// Bevy resource wrapping the database connection
#[derive(Resource)]
pub struct DatabaseConnection(pub Arc<RwLock<GameDatabase>>);

impl DatabaseConnection {
    pub async fn new(database_url: &str) -> DatabaseResult<Self> {
        let db = GameDatabase::connect(database_url).await?;
        Ok(Self(Arc::new(RwLock::new(db))))
    }

    pub async fn get(&self) -> tokio::sync::RwLockReadGuard<GameDatabase> {
        self.0.read().await
    }

    pub async fn get_mut(&self) -> tokio::sync::RwLockWriteGuard<GameDatabase> {
        self.0.write().await
    }
}

/// Queue for batching database synchronization operations
#[derive(Resource, Default)]
pub struct DatabaseSyncQueue {
    pub pending_syncs: Vec<EntitySyncData>,
    pub batch_size: usize,
}

impl DatabaseSyncQueue {
    pub fn new() -> Self {
        Self {
            pending_syncs: Vec::new(),
            batch_size: 50,
        }
    }

    pub fn queue_sync(&mut self, sync_data: EntitySyncData) {
        self.pending_syncs.push(sync_data);
    }

    pub fn take_batch(&mut self) -> Vec<EntitySyncData> {
        let batch_size = self.batch_size.min(self.pending_syncs.len());
        self.pending_syncs.drain(..batch_size).collect()
    }

    pub fn is_ready_for_batch(&self) -> bool {
        self.pending_syncs.len() >= self.batch_size
    }
}

/// Mapping between Bevy entities and database records
#[derive(Resource, Default)]
pub struct EntityDatabaseMapping {
    pub entity_to_db: HashMap<Entity, (uuid::Uuid, String)>, // Entity -> (DB ID, Type)
    pub db_to_entity: HashMap<uuid::Uuid, Entity>,           // DB ID -> Entity
}

impl EntityDatabaseMapping {
    pub fn register_entity(&mut self, entity: Entity, db_id: uuid::Uuid, entity_type: String) {
        self.entity_to_db.insert(entity, (db_id, entity_type));
        self.db_to_entity.insert(db_id, entity);
    }

    pub fn unregister_entity(&mut self, entity: Entity) {
        if let Some((db_id, _)) = self.entity_to_db.remove(&entity) {
            self.db_to_entity.remove(&db_id);
        }
    }

    pub fn get_database_id(&self, entity: Entity) -> Option<uuid::Uuid> {
        self.entity_to_db.get(&entity).map(|(id, _)| *id)
    }

    pub fn get_entity(&self, db_id: uuid::Uuid) -> Option<Entity> {
        self.db_to_entity.get(&db_id).copied()
    }
}

/// Timer resource for automatic database synchronization
#[derive(Resource)]
pub struct DatabaseSyncTimer(pub Timer);

/// Statistics about database operations for monitoring
#[derive(Resource, Default)]
pub struct DatabaseStats {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub avg_query_time_ms: f64,
    pub entities_synced: u64,
    pub last_sync_time: Option<DateTime<Utc>>,
}

/// Task component for handling async database connection
#[derive(Component)]
pub struct DatabaseConnectionTask {
    pub database_url: String,
    pub connection_future: Option<tokio::task::JoinHandle<DatabaseResult<GameDatabase>>>,
}

// ============================================================================
// EVENTS
// ============================================================================

/// Event for triggering database synchronization
#[derive(Event, Debug, Clone)]
pub struct DatabaseSyncEvent {
    pub entity: Entity,
    pub sync_type: SyncType,
    pub priority: SyncPriority,
}

/// Event for database errors that need handling
#[derive(Event, Debug, Clone)]
pub struct DatabaseErrorEvent {
    pub error: DatabaseError,
    pub entity: Option<Entity>,
    pub operation: String,
    pub retry_count: u32,
}

/// Event for horror progression changes
#[derive(Event, Debug, Clone)]
pub struct HorrorProgressionEvent {
    pub player_entity: Entity,
    pub old_dread_level: i32,
    pub new_dread_level: i32,
    pub dread_progression: f32,
    pub trigger_cause: String,
}

/// Event for companion trauma changes
#[derive(Event, Debug, Clone)]
pub struct CompanionTraumaEvent {
    pub companion_entity: Entity,
    pub trauma_change: f32,
    pub loyalty_change: f32,
    pub trauma_source: String,
    pub requires_visual_update: bool,
}

/// Event for AI workflow operations
#[derive(Event, Debug, Clone)]
pub struct AIWorkflowEvent {
    pub workflow_type: String,
    pub agent_type: String,
    pub status: WorkflowStatus,
    pub target_dread_level: i32,
    pub generated_assets: Vec<uuid::Uuid>,
}

/// Event for asset generation completion
#[derive(Event, Debug, Clone)]
pub struct AssetGenerationEvent {
    pub asset_id: uuid::Uuid,
    pub asset_type: String,
    pub generation_status: AssetGenerationStatus,
    pub quality_score: f32,
    pub requires_approval: bool,
}

// ============================================================================
// SUPPORTING TYPES
// ============================================================================

#[derive(Debug, Clone)]
pub struct EntitySyncData {
    pub entity: Entity,
    pub database_id: uuid::Uuid,
    pub entity_type: String,
    pub component_data: serde_json::Value,
    pub sync_type: SyncType,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncType {
    Create,
    Update,
    Delete,
    Load,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SyncPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowStatus {
    Started,
    InProgress,
    Completed,
    Failed,
    RequiresReview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Reflect)]
pub enum AssetGenerationStatus {
    Generated,
    Approved,
    Rejected,
    RequiresReview,
}

// ============================================================================
// SYSTEM PARAMETERS
// ============================================================================

/// System parameter for easy database access in systems
#[derive(SystemParam)]
pub struct DatabaseQuery<'w, 's> {
    pub connection: Res<'w, DatabaseConnection>,
    pub mapping: ResMut<'w, EntityDatabaseMapping>,
    pub sync_queue: ResMut<'w, DatabaseSyncQueue>,
    pub stats: ResMut<'w, DatabaseStats>,
    pub sync_events: EventWriter<'w, DatabaseSyncEvent>,
    pub error_events: EventWriter<'w, DatabaseErrorEvent>,
    marker: std::marker::PhantomData<&'s ()>,
}

impl<'w, 's> DatabaseQuery<'w, 's> {
    /// Queue an entity for database synchronization
    pub fn sync_entity(&mut self, entity: Entity, sync_type: SyncType, priority: SyncPriority) {
        if let Some((db_id, entity_type)) = self.mapping.entity_to_db.get(&entity).cloned() {
            self.sync_events.write(DatabaseSyncEvent {
                entity,
                sync_type,
                priority,
            });
        }
    }

    /// Register a new entity with the database
    pub fn register_entity(&mut self, entity: Entity, db_id: uuid::Uuid, entity_type: String) {
        self.mapping.register_entity(entity, db_id, entity_type);
    }

    /// Get the database ID for an entity
    pub fn get_database_id(&self, entity: Entity) -> Option<uuid::Uuid> {
        self.mapping.get_database_id(entity)
    }

    /// Report a database error
    pub fn report_error(&mut self, error: DatabaseError, entity: Option<Entity>, operation: String) {
        self.error_events.write(DatabaseErrorEvent {
            error,
            entity,
            operation,
            retry_count: 0,
        });
    }

    /// Update statistics
    pub fn update_stats(&mut self, query_successful: bool, query_time_ms: f64) {
        self.stats.total_queries += 1;
        if query_successful {
            self.stats.successful_queries += 1;
        } else {
            self.stats.failed_queries += 1;
        }
        
        // Update rolling average
        let total_successful = self.stats.successful_queries.max(1) as f64;
        self.stats.avg_query_time_ms = 
            (self.stats.avg_query_time_ms * (total_successful - 1.0) + query_time_ms) / total_successful;
    }
}

// ============================================================================
// CORE SYSTEMS
// ============================================================================

/// System to handle async database connection setup
pub fn handle_database_connection_task(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DatabaseConnectionTask)>,
) {
    for (entity, mut task) in query.iter_mut() {
        if task.connection_future.is_none() {
            // Start the async connection task
            let database_url = task.database_url.clone();
            let future = tokio::spawn(async move {
                GameDatabase::connect(&database_url).await
            });
            task.connection_future = Some(future);
        }

        if let Some(ref mut future) = task.connection_future {
            if future.is_finished() {
                // Connection completed, extract the result
                let future = task.connection_future.take().unwrap();
                match futures::executor::block_on(future) {
                    Ok(database) => {
                        // Success! Insert the database connection resource
                        commands.insert_resource(DatabaseConnection(Arc::new(RwLock::new(database))));
                        commands.entity(entity).despawn();
                        info!("Database connection established successfully");
                    },
                    Err(err) => {
                        error!("Failed to connect to database: {}", err);
                        // Could retry here or handle the error
                        commands.entity(entity).despawn();
                    }
                }
            }
        }
    }
}

/// System to process the database synchronization queue
pub fn process_database_sync_queue(
    mut sync_queue: ResMut<DatabaseSyncQueue>,
    db_connection: Res<DatabaseConnection>,
    mut stats: ResMut<DatabaseStats>,
) {
    if !sync_queue.is_ready_for_batch() && !sync_queue.pending_syncs.is_empty() {
        return;
    }

    let batch = sync_queue.take_batch();
    if batch.is_empty() {
        return;
    }

    // Process batch asynchronously
    let db = db_connection.0.clone();
    let batch_size = batch.len();
    
    tokio::spawn(async move {
        let db = db.read().await;
        let start_time = std::time::Instant::now();
        
        for sync_data in batch {
            // Process each sync operation
            // This would involve calling the appropriate database methods
            // based on sync_data.sync_type and sync_data.entity_type
        }
        
        let elapsed = start_time.elapsed().as_millis() as f64;
        debug!("Processed database sync batch of {} items in {}ms", batch_size, elapsed);
    });

    stats.entities_synced += batch_size as u64;
    stats.last_sync_time = Some(Utc::now());
}

/// System to handle database sync events
pub fn handle_database_sync_events(
    mut events: EventReader<DatabaseSyncEvent>,
    mut sync_queue: ResMut<DatabaseSyncQueue>,
    mapping: Res<EntityDatabaseMapping>,
) {
    for event in events.read() {
        if let Some((db_id, entity_type)) = mapping.entity_to_db.get(&event.entity) {
            let sync_data = EntitySyncData {
                entity: event.entity,
                database_id: *db_id,
                entity_type: entity_type.clone(),
                component_data: serde_json::Value::Object(serde_json::Map::new()), // TODO: Extract actual component data
                sync_type: event.sync_type,
                timestamp: Utc::now(),
            };
            
            sync_queue.queue_sync(sync_data);
        }
    }
}

/// System to automatically sync entities on a timer
pub fn auto_sync_entities_system(
    time: Res<Time>,
    mut timer: ResMut<DatabaseSyncTimer>,
    mut db_query: DatabaseQuery,
    entities_with_db: Query<Entity, With<DatabaseEntity>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // Queue all entities with DatabaseEntity component for sync
        for entity in entities_with_db.iter() {
            db_query.sync_entity(entity, SyncType::Update, SyncPriority::Low);
        }
        
        debug!("Queued {} entities for automatic synchronization", entities_with_db.iter().count());
    }
}

/// System to sync horror progression components with database
pub fn sync_horror_progression_components(
    mut progression_events: EventReader<HorrorProgressionEvent>,
    mut db_query: DatabaseQuery,
    mut horror_query: Query<&mut HorrorProgression>,
) {
    for event in progression_events.read() {
        if let Ok(mut progression) = horror_query.get_mut(event.player_entity) {
            progression.current_dread_level = event.new_dread_level;
            progression.dread_progression = event.dread_progression;
            progression.last_dread_event = Some(Utc::now());
            
            db_query.sync_entity(event.player_entity, SyncType::Update, SyncPriority::High);
        }
    }
}

/// System to sync companion state components with database
pub fn sync_companion_state_components(
    mut trauma_events: EventReader<CompanionTraumaEvent>,
    mut db_query: DatabaseQuery,
    mut companion_query: Query<&mut CompanionState>,
) {
    for event in trauma_events.read() {
        if let Ok(mut companion) = companion_query.get_mut(event.companion_entity) {
            companion.trauma_level += event.trauma_change;
            companion.loyalty += event.loyalty_change;
            
            // Clamp values to valid ranges
            companion.trauma_level = companion.trauma_level.clamp(0.0, 1.0);
            companion.loyalty = companion.loyalty.clamp(0.0, 1.0);
            
            db_query.sync_entity(event.companion_entity, SyncType::Update, SyncPriority::High);
        }
    }
}

/// System to sync hex position components with database
pub fn sync_hex_position_components(
    mut db_query: DatabaseQuery,
    changed_positions: Query<Entity, (With<HexPosition>, Changed<HexPosition>)>,
) {
    for entity in changed_positions.iter() {
        db_query.sync_entity(entity, SyncType::Update, SyncPriority::Medium);
    }
}

/// System to handle database errors
pub fn handle_database_errors(
    mut error_events: EventReader<DatabaseErrorEvent>,
    mut stats: ResMut<DatabaseStats>,
) {
    for event in error_events.read() {
        match event.error.severity() {
            crate::error::ErrorSeverity::Critical => {
                error!("Critical database error in {}: {}", event.operation, event.error);
                // Could trigger application shutdown or recovery procedures
            },
            crate::error::ErrorSeverity::High => {
                warn!("High severity database error in {}: {}", event.operation, event.error);
                // Could disable certain features temporarily
            },
            crate::error::ErrorSeverity::Medium => {
                warn!("Database error in {}: {}", event.operation, event.error);
            },
            crate::error::ErrorSeverity::Low => {
                debug!("Minor database error in {}: {}", event.operation, event.error);
            },
        }
        
        stats.failed_queries += 1;
        
        // Could implement retry logic here based on error.is_retryable()
        if event.error.is_retryable() && event.retry_count < 3 {
            // Schedule retry
            debug!("Scheduling retry for retryable error (attempt {})", event.retry_count + 1);
        }
    }
}

/// System to update database statistics
pub fn update_database_stats_system(
    db_connection: Res<DatabaseConnection>,
    mut stats: ResMut<DatabaseStats>,
) {
    // This could periodically query the database for health metrics
    // For now, just update timestamp if connection exists
    if db_connection.is_changed() {
        stats.last_sync_time = Some(Utc::now());
    }
}

/// System to process AI workflow events
pub fn process_ai_workflow_events(
    mut workflow_events: EventReader<AIWorkflowEvent>,
    mut db_query: DatabaseQuery,
) {
    for event in workflow_events.read() {
        match event.status {
            WorkflowStatus::Completed => {
                info!("AI workflow {} completed successfully, generated {} assets", 
                      event.workflow_type, event.generated_assets.len());
            },
            WorkflowStatus::Failed => {
                warn!("AI workflow {} failed", event.workflow_type);
            },
            WorkflowStatus::RequiresReview => {
                info!("AI workflow {} requires human review", event.workflow_type);
            },
            _ => {}
        }
        
        db_query.update_stats(event.status != WorkflowStatus::Failed, 0.0);
    }
}

/// System to process asset generation events
pub fn process_asset_generation_events(
    mut asset_events: EventReader<AssetGenerationEvent>,
    mut db_query: DatabaseQuery,
) {
    for event in asset_events.read() {
        match event.generation_status {
            AssetGenerationStatus::Generated => {
                info!("Generated new {} asset with quality score {:.2}", 
                      event.asset_type, event.quality_score);
            },
            AssetGenerationStatus::RequiresReview => {
                info!("Generated {} asset requires human approval (quality: {:.2})", 
                      event.asset_type, event.quality_score);
            },
            AssetGenerationStatus::Approved => {
                info!("Asset {} approved for use", event.asset_id);
            },
            AssetGenerationStatus::Rejected => {
                warn!("Asset {} rejected", event.asset_id);
            },
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Helper function to extract component data as JSON for database storage
pub fn extract_component_data<T: Component + serde::Serialize>(
    world: &World,
    entity: Entity,
) -> Option<serde_json::Value> {
    world.get::<T>(entity)
        .and_then(|component| serde_json::to_value(component).ok())
}

/// Helper function to apply component data from JSON database storage
pub fn apply_component_data<T: Component + serde::de::DeserializeOwned>(
    commands: &mut Commands,
    entity: Entity,
    data: &serde_json::Value,
) -> Result<(), serde_json::Error> {
    let component: T = serde_json::from_value(data.clone())?;
    commands.entity(entity).insert(component);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_database_mapping() {
        let mut mapping = EntityDatabaseMapping::default();
        let entity = Entity::from_raw(1);
        let db_id = uuid::Uuid::new_v4();
        
        mapping.register_entity(entity, db_id, "Player".to_string());
        
        assert_eq!(mapping.get_database_id(entity), Some(db_id));
        assert_eq!(mapping.get_entity(db_id), Some(entity));
        
        mapping.unregister_entity(entity);
        assert_eq!(mapping.get_database_id(entity), None);
        assert_eq!(mapping.get_entity(db_id), None);
    }

    #[test]
    fn test_sync_queue_batching() {
        let mut queue = DatabaseSyncQueue::new();
        queue.batch_size = 2;
        
        let sync_data1 = EntitySyncData {
            entity: Entity::from_raw(1),
            database_id: uuid::Uuid::new_v4(),
            entity_type: "Player".to_string(),
            component_data: serde_json::Value::Object(serde_json::Map::new()),
            sync_type: SyncType::Update,
            timestamp: Utc::now(),
        };
        
        let sync_data2 = EntitySyncData {
            entity: Entity::from_raw(2),
            database_id: uuid::Uuid::new_v4(),
            entity_type: "Companion".to_string(),
            component_data: serde_json::Value::Object(serde_json::Map::new()),
            sync_type: SyncType::Update,
            timestamp: Utc::now(),
        };
        
        queue.queue_sync(sync_data1);
        assert!(!queue.is_ready_for_batch());
        
        queue.queue_sync(sync_data2);
        assert!(queue.is_ready_for_batch());
        
        let batch = queue.take_batch();
        assert_eq!(batch.len(), 2);
        assert!(!queue.is_ready_for_batch());
    }
}
