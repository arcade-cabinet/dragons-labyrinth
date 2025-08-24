//! Dungeon Systems - Database-driven dungeon navigation and room mechanics
//!
//! This system queries dungeon room and doorway data to create 3D navigation
//! and exploration experiences using HBF-imported dungeon layouts.

use anyhow::Result;
use database_orm::*;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;
use super::{HexPosition, DungeonLayout, DungeonRoomData, DoorwayData};

pub struct DungeonSystems {
    db: DatabaseConnection,
    dungeon_cache: HashMap<Uuid, DungeonLayout>,
    room_connections: HashMap<Uuid, Vec<Uuid>>, // room_id -> connected_room_ids
}

impl DungeonSystems {
    pub async fn new(db: &DatabaseConnection) -> Result<Self> {
        let mut system = Self {
            db: db.clone(),
            dungeon_cache: HashMap::new(),
            room_connections: HashMap::new(),
        };
        
        // Pre-load dungeon layouts for navigation
        system.load_dungeon_layouts().await?;
        system.build_room_connections().await?;
        
        info!("Dungeon systems initialized with {} dungeons and {} room connections",
              system.dungeon_cache.len(), system.room_connections.len());
        
        Ok(system)
    }
    
    /// Get all dungeons within a hex radius for map display
    pub async fn get_dungeons_in_area(&self, center: HexPosition, radius: u32) -> Result<Vec<DungeonSummary>> {
        let dungeons = dungeons::Entity::find()
            .all(&self.db)
            .await?;
        
        let mut nearby_dungeons = Vec::new();
        
        for dungeon in dungeons {
            if let (Some(hbf_x), Some(hbf_y)) = (dungeon.hbf_x, dungeon.hbf_y) {
                let dungeon_pos = HexPosition::from_hbf_coords(hbf_x, hbf_y);
                let distance = center.distance_to(&dungeon_pos);
                
                if distance <= radius {
                    nearby_dungeons.push(DungeonSummary {
                        id: dungeon.id,
                        name: dungeon.name,
                        dungeon_type: dungeon.dungeon_type,
                        position: dungeon_pos,
                        total_rooms: dungeon.total_rooms,
                        danger_level: dungeon.danger_level,
                        discovered: dungeon.discovered,
                        partially_explored: dungeon.partially_explored,
                        fully_cleared: dungeon.fully_cleared,
                    });
                }
            }
        }
        
        debug!("Found {} dungeons within {} hexes of ({}, {})", 
               nearby_dungeons.len(), radius, center.q, center.r);
        
        Ok(nearby_dungeons)
    }
    
    /// Enter a dungeon and get navigation interface
    pub async fn enter_dungeon(&self, dungeon_id: Uuid, entry_point: Option<Uuid>) -> Result<DungeonNavigationInterface> {
        let dungeon_layout = self.dungeon_cache.get(&dungeon_id)
            .ok_or_else(|| anyhow::anyhow!("Dungeon not found in cache"))?;
        
        // Determine starting room (entrance or specific entry point)
        let starting_room_id = match entry_point {
            Some(room_id) => room_id,
            None => self.find_dungeon_entrance(dungeon_id).await?,
        };
        
        let current_room = dungeon_layout.rooms.iter()
            .find(|room| room.id == starting_room_id)
            .ok_or_else(|| anyhow::anyhow!("Starting room not found"))?;
        
        // Get connected rooms for navigation options
        let connected_rooms = self.get_connected_rooms(starting_room_id).await?;
        
        // Update dungeon discovery state
        self.mark_room_discovered(starting_room_id).await?;
        
        info!("Player entered {}, room: {} (area {})", 
              dungeon_layout.name, current_room.title, current_room.area_number);
        
        Ok(DungeonNavigationInterface {
            dungeon_name: dungeon_layout.name.clone(),
            current_room: current_room.clone(),
            connected_rooms,
            available_actions: self.get_room_actions(current_room),
            environmental_description: self.get_environmental_description(current_room),
        })
    }
    
    /// Move through a doorway to another room
    pub async fn move_through_doorway(&self, current_room_id: Uuid, direction: &str) -> Result<DungeonNavigationResult> {
        // Find the doorway in the specified direction
        let doorway = dungeons::doorways::Entity::find()
            .filter(dungeons::doorways::Column::RoomId.eq(current_room_id))
            .filter(dungeons::doorways::Column::Direction.eq(direction))
            .one(&self.db)
            .await?;
        
        let doorway = match doorway {
            Some(d) => d,
            None => return Ok(DungeonNavigationResult::NoExit { direction: direction.to_string() }),
        };
        
        // Check if doorway is passable
        if doorway.locked && !doorway.opened {
            return Ok(DungeonNavigationResult::Blocked {
                reason: "locked".to_string(),
                unlock_method: doorway.unlock_method,
            });
        }
        
        if doorway.condition == "barricaded" {
            return Ok(DungeonNavigationResult::Blocked {
                reason: "barricaded".to_string(),
                unlock_method: Some("force".to_string()),
            });
        }
        
        // Find destination room
        let destination_room_id = match doorway.leads_to_room_id {
            Some(id) => id,
            None => {
                // Try to find by area number
                if let Some(area_number) = doorway.leads_to_area_number {
                    let room = dungeons::rooms::Entity::find()
                        .filter(dungeons::rooms::Column::DungeonId.eq(self.get_dungeon_id_for_room(current_room_id).await?))
                        .filter(dungeons::rooms::Column::AreaNumber.eq(area_number))
                        .one(&self.db)
                        .await?;
                    
                    match room {
                        Some(r) => r.id,
                        None => return Ok(DungeonNavigationResult::UnknownDestination),
                    }
                } else {
                    return Ok(DungeonNavigationResult::UnknownDestination);
                }
            }
        };
        
        // Mark new room as discovered
        self.mark_room_discovered(destination_room_id).await?;
        
        // Get new room data
        let new_room = dungeons::rooms::Entity::find_by_id(destination_room_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Destination room not found"))?;
        
        let room_data = self.convert_room_to_data(&new_room);
        let connected_rooms = self.get_connected_rooms(destination_room_id).await?;
        
        Ok(DungeonNavigationResult::MovedToRoom {
            room: room_data,
            connected_rooms,
            movement_description: format!("You pass through the {} {} door", doorway.material, doorway.shape),
        })
    }
    
    /// Search a room for secrets, treasure, or clues
    pub async fn search_room(&self, room_id: Uuid, search_type: SearchType) -> Result<SearchResult> {
        let room = dungeons::rooms::Entity::find_by_id(room_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
        
        // Check if room has already been searched
        if room.searched {
            return Ok(SearchResult::AlreadySearched);
        }
        
        let result = match search_type {
            SearchType::Treasure => self.search_for_treasure(&room).await?,
            SearchType::Secrets => self.search_for_secrets(&room).await?,
            SearchType::Clues => self.search_for_clues(&room).await?,
            SearchType::Traps => self.search_for_traps(&room).await?,
        };
        
        // Mark room as searched
        dungeons::rooms::Entity::update_many()
            .col_expr(dungeons::rooms::Column::Searched, sea_orm::sea_query::Expr::value(true))
            .filter(dungeons::rooms::Column::Id.eq(room_id))
            .exec(&self.db)
            .await?;
        
        info!("Player searched {} for {:?}: {:?}", room.title, search_type, result);
        
        Ok(result)
    }
    
    /// Get dungeon completion status
    pub async fn get_dungeon_progress(&self, dungeon_id: Uuid) -> Result<DungeonProgress> {
        let rooms = dungeons::rooms::Entity::find()
            .filter(dungeons::rooms::Column::DungeonId.eq(dungeon_id))
            .all(&self.db)
            .await?;
        
        let total_rooms = rooms.len();
        let discovered_rooms = rooms.iter().filter(|r| r.discovered).count();
        let searched_rooms = rooms.iter().filter(|r| r.searched).count();
        let cleared_rooms = rooms.iter().filter(|r| r.cleared).count();
        
        Ok(DungeonProgress {
            total_rooms,
            discovered_rooms,
            searched_rooms,
            cleared_rooms,
            completion_percentage: (cleared_rooms as f32 / total_rooms as f32 * 100.0) as u32,
        })
    }
    
    /// Private helper methods
    
    async fn load_dungeon_layouts(&mut self) -> Result<()> {
        let dungeons = dungeons::Entity::find().all(&self.db).await?;
        
        for dungeon in dungeons {
            let rooms = dungeons::rooms::Entity::find()
                .filter(dungeons::rooms::Column::DungeonId.eq(dungeon.id))
                .all(&self.db)
                .await?;
            
            let room_data: Vec<DungeonRoomData> = rooms.into_iter()
                .map(|room| self.convert_room_to_data(&room))
                .collect();
            
            let discovered_rooms: Vec<Uuid> = room_data.iter()
                .filter(|room| room.discovered)
                .map(|room| room.id)
                .collect();
            
            let layout = DungeonLayout {
                id: dungeon.id,
                name: dungeon.name,
                total_rooms: dungeon.total_rooms,
                current_room: None,
                rooms: room_data,
                discovered_rooms,
            };
            
            self.dungeon_cache.insert(dungeon.id, layout);
        }
        
        Ok(())
    }
    
    async fn build_room_connections(&mut self) -> Result<()> {
        let doorways = dungeons::doorways::Entity::find().all(&self.db).await?;
        
        for doorway in doorways {
            let connections = self.room_connections.entry(doorway.room_id).or_insert_with(Vec::new);
            
            if let Some(leads_to) = doorway.leads_to_room_id {
                if !connections.contains(&leads_to) {
                    connections.push(leads_to);
                }
            }
        }
        
        Ok(())
    }
    
    fn convert_room_to_data(&self, room: &dungeons::rooms::Model) -> DungeonRoomData {
        let doorways = if let Some(doorway_json) = &room.doorways {
            serde_json::from_value(doorway_json.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };
        
        let features = if let Some(features_json) = &room.features {
            serde_json::from_value(features_json.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };
        
        let encounters = if let Some(encounters_json) = &room.encounters {
            serde_json::from_value(encounters_json.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };
        
        DungeonRoomData {
            id: room.id,
            area_number: room.area_number,
            title: room.title.clone(),
            description: room.description.clone(),
            doorways,
            features,
            encounters,
            discovered: room.discovered,
            cleared: room.cleared,
        }
    }
    
    async fn find_dungeon_entrance(&self, dungeon_id: Uuid) -> Result<Uuid> {
        // Find room with lowest area number (usually entrance)
        let entrance_room = dungeons::rooms::Entity::find()
            .filter(dungeons::rooms::Column::DungeonId.eq(dungeon_id))
            .order_by_asc(dungeons::rooms::Column::AreaNumber)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("No rooms found in dungeon"))?;
        
        Ok(entrance_room.id)
    }
    
    async fn get_connected_rooms(&self, room_id: Uuid) -> Result<Vec<ConnectedRoom>> {
        let doorways = dungeons::doorways::Entity::find()
            .filter(dungeons::doorways::Column::RoomId.eq(room_id))
            .all(&self.db)
            .await?;
        
        let mut connected_rooms = Vec::new();
        
        for doorway in doorways {
            let is_passable = !doorway.locked && doorway.condition != "barricaded";
            
            connected_rooms.push(ConnectedRoom {
                direction: doorway.direction,
                material: doorway.material,
                shape: doorway.shape,
                condition: doorway.condition,
                is_passable,
                requires_key: doorway.locked,
                key_location: doorway.unlock_method,
                leads_to_area: doorway.leads_to_area_number,
            });
        }
        
        Ok(connected_rooms)
    }
    
    async fn mark_room_discovered(&self, room_id: Uuid) -> Result<()> {
        dungeons::rooms::Entity::update_many()
            .col_expr(dungeons::rooms::Column::Discovered, sea_orm::sea_query::Expr::value(true))
            .col_expr(dungeons::rooms::Column::FirstEnteredAt, sea_orm::sea_query::Expr::current_timestamp())
            .filter(dungeons::rooms::Column::Id.eq(room_id))
            .exec(&self.db)
            .await?;
        
        Ok(())
    }
    
    async fn get_dungeon_id_for_room(&self, room_id: Uuid) -> Result<Uuid> {
        let room = dungeons::rooms::Entity::find_by_id(room_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Room not found"))?;
        
        Ok(room.dungeon_id)
    }
    
    fn get_room_actions(&self, room: &DungeonRoomData) -> Vec<String> {
        let mut actions = vec!["look_around".to_string(), "search".to_string()];
        
        if !room.encounters.is_empty() && !room.cleared {
            actions.push("fight_monsters".to_string());
        }
        
        if !room.features.is_empty() {
            actions.push("interact_with_features".to_string());
        }
        
        for doorway in &room.doorways {
            actions.push(format!("go_{}", doorway.direction));
        }
        
        actions
    }
    
    fn get_environmental_description(&self, room: &DungeonRoomData) -> String {
        let mut description = room.description.clone();
        
        // Add atmospheric details
        if !room.features.is_empty() {
            description.push_str(&format!(" Notable features: {}.", room.features.join(", ")));
        }
        
        if !room.encounters.is_empty() && !room.cleared {
            description.push_str(" You sense hostile presence here.");
        }
        
        if !room.doorways.is_empty() {
            let directions: Vec<String> = room.doorways.iter().map(|d| d.direction.clone()).collect();
            description.push_str(&format!(" Exits: {}.", directions.join(", ")));
        }
        
        description
    }
    
    async fn search_for_treasure(&self, room: &dungeons::rooms::Model) -> Result<SearchResult> {
        // Check room features for treasure hints
        if let Some(features) = &room.features {
            if let Ok(feature_list) = serde_json::from_value::<Vec<String>>(features.clone()) {
                for feature in feature_list {
                    if feature.to_lowercase().contains("treasure") || feature.to_lowercase().contains("gold") {
                        return Ok(SearchResult::TreasureFound {
                            description: feature,
                            value: rand::thread_rng().gen_range(10..=100),
                        });
                    }
                }
            }
        }
        
        // Random chance of finding something small
        if rand::thread_rng().gen::<f32>() < 0.3 {
            Ok(SearchResult::TreasureFound {
                description: "A few scattered coins".to_string(),
                value: rand::thread_rng().gen_range(1..=10),
            })
        } else {
            Ok(SearchResult::Nothing)
        }
    }
    
    async fn search_for_secrets(&self, room: &dungeons::rooms::Model) -> Result<SearchResult> {
        // Check for secret doors or passages
        let doorways = dungeons::doorways::Entity::find()
            .filter(dungeons::doorways::Column::RoomId.eq(room.id))
            .filter(dungeons::doorways::Column::Secret.eq(true))
            .all(&self.db)
            .await?;
        
        if !doorways.is_empty() {
            let secret_doorway = &doorways[0];
            Ok(SearchResult::SecretFound {
                description: format!("You discover a secret {} door to the {}", 
                                   secret_doorway.material, secret_doorway.direction),
                secret_type: "hidden_passage".to_string(),
            })
        } else {
            Ok(SearchResult::Nothing)
        }
    }
    
    async fn search_for_clues(&self, room: &dungeons::rooms::Model) -> Result<SearchResult> {
        // Extract clues from room description or features
        if room.description.to_lowercase().contains("journal") || 
           room.description.to_lowercase().contains("note") ||
           room.description.to_lowercase().contains("inscription") {
            Ok(SearchResult::ClueFound {
                description: "You find cryptic writing that may be important".to_string(),
                clue_type: "textual_evidence".to_string(),
            })
        } else {
            Ok(SearchResult::Nothing)
        }
    }
    
    async fn search_for_traps(&self, room: &dungeons::rooms::Model) -> Result<SearchResult> {
        // Check doorways for traps
        let trapped_doorways = dungeons::doorways::Entity::find()
            .filter(dungeons::doorways::Column::RoomId.eq(room.id))
            .filter(dungeons::doorways::Column::Trapped.eq(true))
            .all(&self.db)
            .await?;
        
        if !trapped_doorways.is_empty() {
            Ok(SearchResult::TrapFound {
                description: "You notice signs of a trap mechanism".to_string(),
                trap_type: "door_trap".to_string(),
                disarm_difficulty: 15,
            })
        } else {
            Ok(SearchResult::Nothing)
        }
    }
}

#[derive(Debug, Clone)]
pub struct DungeonSummary {
    pub id: Uuid,
    pub name: String,
    pub dungeon_type: String,
    pub position: HexPosition,
    pub total_rooms: i32,
    pub danger_level: i32,
    pub discovered: bool,
    pub partially_explored: bool,
    pub fully_cleared: bool,
}

#[derive(Debug, Clone)]
pub struct DungeonNavigationInterface {
    pub dungeon_name: String,
    pub current_room: DungeonRoomData,
    pub connected_rooms: Vec<ConnectedRoom>,
    pub available_actions: Vec<String>,
    pub environmental_description: String,
}

#[derive(Debug, Clone)]
pub struct ConnectedRoom {
    pub direction: String,
    pub material: String,
    pub shape: String,
    pub condition: String,
    pub is_passable: bool,
    pub requires_key: bool,
    pub key_location: Option<String>,
    pub leads_to_area: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum DungeonNavigationResult {
    MovedToRoom {
        room: DungeonRoomData,
        connected_rooms: Vec<ConnectedRoom>,
        movement_description: String,
    },
    Blocked {
        reason: String,
        unlock_method: Option<String>,
    },
    NoExit {
        direction: String,
    },
    UnknownDestination,
}

#[derive(Debug, Clone)]
pub enum SearchType {
    Treasure,
    Secrets,
    Clues,
    Traps,
}

#[derive(Debug, Clone)]
pub enum SearchResult {
    TreasureFound { description: String, value: i32 },
    SecretFound { description: String, secret_type: String },
    ClueFound { description: String, clue_type: String },
    TrapFound { description: String, trap_type: String, disarm_difficulty: i32 },
    Nothing,
    AlreadySearched,
}

#[derive(Debug, Clone)]
pub struct DungeonProgress {
    pub total_rooms: usize,
    pub discovered_rooms: usize,
    pub searched_rooms: usize,
    pub cleared_rooms: usize,
    pub completion_percentage: u32,
}
