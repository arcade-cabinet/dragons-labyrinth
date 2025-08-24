//! DOOM-style 3D labyrinth system
//!
//! This system generates and manages the 3D first-person labyrinths
//! that players explore between hex overworld sections.

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::{HashMap, HashSet};

/// Plugin for 3D labyrinth system
pub struct Labyrinth3DPlugin;

impl Plugin for Labyrinth3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_labyrinth_renderer)
            .add_systems(Update, (
                update_labyrinth_corruption,
                handle_room_transitions,
                spawn_labyrinth_enemies,
                update_void_effects,
            ))
            .insert_resource(LabyrinthState::default());
    }
}

/// Current labyrinth state
#[derive(Resource, Default)]
pub struct LabyrinthState {
    pub current_room: u32,
    pub layout: LabyrinthLayout,
    pub corruption_level: f32,
    pub explored_rooms: HashSet<u32>,
    pub active_effects: Vec<VoidEffect>,
}

/// Complete labyrinth layout
#[derive(Default, Clone)]
pub struct LabyrinthLayout {
    pub seed: u64,
    pub rooms: HashMap<u32, Room>,
    pub connections: HashMap<u32, Vec<u32>>,
    pub entrance_room: u32,
    pub exit_room: u32,
    pub boss_room: Option<u32>,
}

/// Individual room in the labyrinth
#[derive(Clone)]
pub struct Room {
    pub id: u32,
    pub room_type: RoomType,
    pub size: Vec3,
    pub position: Vec3,
    pub theme: RoomTheme,
    pub corruption: f32,
    pub loot: Vec<LootItem>,
    pub enemies: Vec<EnemySpawn>,
    pub puzzles: Vec<Puzzle>,
    pub cc0_models: Vec<CC0Model>,
}

/// Types of rooms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomType {
    Entrance,
    Corridor,
    Chamber,
    Arena,
    Puzzle,
    Treasure,
    Boss,
    Secret,
    Void,
}

/// Visual themes for rooms (using CC0 assets)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoomTheme {
    Medieval,      // Stone walls, torches
    Dungeon,       // Prison cells, chains
    Crypt,         // Tombs, bones
    Library,       // Bookshelves, scrolls
    Armory,        // Weapon racks, armor stands
    Temple,        // Altars, statues
    Laboratory,    // Alchemy equipment
    VoidTouched,   // Corrupted geometry
    FleshHorror,   // Organic walls
}

/// CC0 model reference
#[derive(Clone)]
pub struct CC0Model {
    pub model_path: String,
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub corruption_variant: f32,
}

/// Loot that can be found
#[derive(Clone)]
pub struct LootItem {
    pub item_type: String,
    pub rarity: LootRarity,
    pub position: Vec3,
}

/// Loot rarity tiers
#[derive(Debug, Clone, Copy)]
pub enum LootRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Cursed,
}

/// Enemy spawn point
#[derive(Clone)]
pub struct EnemySpawn {
    pub enemy_type: String,
    pub position: Vec3,
    pub patrol_path: Vec<Vec3>,
    pub aggression: f32,
}

/// Puzzle in a room
#[derive(Clone)]
pub struct Puzzle {
    pub puzzle_type: PuzzleType,
    pub difficulty: u8,
    pub reward: Option<LootItem>,
}

/// Types of puzzles
#[derive(Debug, Clone, Copy)]
pub enum PuzzleType {
    PressurePlates,
    RotatingStatues,
    LightBeams,
    RuneSequence,
    TimedSwitch,
    VoidRiddle,
}

/// Void effects that can occur
#[derive(Clone)]
pub struct VoidEffect {
    pub effect_type: VoidEffectType,
    pub intensity: f32,
    pub duration: f32,
}

/// Types of void effects
#[derive(Debug, Clone, Copy)]
pub enum VoidEffectType {
    WallsBreathing,
    GeometryShift,
    GravityInversion,
    TimeDistortion,
    EchoingVoices,
    ShadowDoppelganger,
}

/// Generate a labyrinth layout
pub fn generate_labyrinth(
    seed: u64,
    level: u32,
    corruption: f32,
    boss: bool,
) -> LabyrinthLayout {
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut layout = LabyrinthLayout {
        seed,
        rooms: HashMap::new(),
        connections: HashMap::new(),
        entrance_room: 0,
        exit_room: 0,
        boss_room: None,
    };
    
    // Determine room count based on level
    let room_count = calculate_room_count(level);
    
    // Generate entrance
    let entrance = generate_entrance_room(&mut rng, corruption);
    layout.rooms.insert(0, entrance);
    layout.entrance_room = 0;
    
    // Generate main path using spanning tree
    let main_path = generate_main_path(&mut rng, room_count);
    
    // Generate rooms along main path
    for (i, room_type) in main_path.iter().enumerate() {
        let room_id = i as u32 + 1;
        let room = generate_room(
            &mut rng,
            room_id,
            *room_type,
            level,
            corruption,
        );
        layout.rooms.insert(room_id, room);
        
        // Connect to previous room
        if i > 0 {
            let prev_id = i as u32;
            layout.connections.entry(prev_id).or_default().push(room_id);
            layout.connections.entry(room_id).or_default().push(prev_id);
        }
    }
    
    // Add side branches
    add_side_branches(&mut layout, &mut rng, level, corruption);
    
    // Add secret rooms
    add_secret_rooms(&mut layout, &mut rng, corruption);
    
    // Generate boss room if needed
    if boss {
        let boss_room = generate_boss_room(&mut rng, level, corruption);
        let boss_id = layout.rooms.len() as u32;
        layout.rooms.insert(boss_id, boss_room);
        layout.boss_room = Some(boss_id);
        
        // Connect boss room to the second-to-last room
        let pre_boss_id = (layout.rooms.len() - 2) as u32;
        layout.connections.entry(pre_boss_id).or_default().push(boss_id);
        layout.connections.entry(boss_id).or_default().push(pre_boss_id);
    }
    
    // Set exit room
    layout.exit_room = (layout.rooms.len() - 1) as u32;
    
    layout
}

/// Calculate room count based on level
fn calculate_room_count(level: u32) -> usize {
    match level {
        1..=20 => 5 + (level / 5) as usize,
        21..=40 => 10 + (level / 10) as usize,
        41..=60 => 15 + (level / 15) as usize,
        61..=100 => 20 + (level / 20) as usize,
        101..=140 => 25 + (level / 25) as usize,
        141..=180 => 30 + (level / 30) as usize,
        _ => 10,
    }
}

/// Generate the entrance room
fn generate_entrance_room(rng: &mut ChaCha8Rng, corruption: f32) -> Room {
    Room {
        id: 0,
        room_type: RoomType::Entrance,
        size: Vec3::new(10.0, 5.0, 10.0),
        position: Vec3::ZERO,
        theme: if corruption > 0.5 {
            RoomTheme::VoidTouched
        } else {
            RoomTheme::Medieval
        },
        corruption,
        loot: vec![],
        enemies: vec![],
        puzzles: vec![],
        cc0_models: select_cc0_models(RoomTheme::Medieval, corruption),
    }
}

/// Generate main path through labyrinth
fn generate_main_path(rng: &mut ChaCha8Rng, room_count: usize) -> Vec<RoomType> {
    let mut path = Vec::new();
    
    for i in 0..room_count {
        let room_type = if i % 5 == 0 && i > 0 {
            RoomType::Arena  // Combat every 5 rooms
        } else if i % 7 == 0 && i > 0 {
            RoomType::Puzzle  // Puzzle every 7 rooms
        } else if i % 3 == 0 {
            RoomType::Chamber  // Larger room every 3
        } else {
            RoomType::Corridor  // Standard corridor
        };
        
        path.push(room_type);
    }
    
    path
}

/// Generate a single room
fn generate_room(
    rng: &mut ChaCha8Rng,
    id: u32,
    room_type: RoomType,
    level: u32,
    base_corruption: f32,
) -> Room {
    // Add corruption variance
    let corruption = (base_corruption + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
    
    // Determine theme based on corruption and room type
    let theme = select_room_theme(room_type, corruption, rng);
    
    // Generate room size
    let size = match room_type {
        RoomType::Corridor => Vec3::new(5.0, 3.0, 15.0),
        RoomType::Chamber => Vec3::new(15.0, 5.0, 15.0),
        RoomType::Arena => Vec3::new(20.0, 8.0, 20.0),
        RoomType::Boss => Vec3::new(30.0, 10.0, 30.0),
        _ => Vec3::new(10.0, 4.0, 10.0),
    };
    
    // Generate loot
    let loot = if rng.gen_bool(0.3) {
        generate_loot(rng, level, room_type)
    } else {
        vec![]
    };
    
    // Generate enemies
    let enemies = if matches!(room_type, RoomType::Arena | RoomType::Chamber) {
        generate_enemies(rng, level, corruption)
    } else {
        vec![]
    };
    
    // Generate puzzles
    let puzzles = if room_type == RoomType::Puzzle {
        vec![generate_puzzle(rng, level)]
    } else {
        vec![]
    };
    
    Room {
        id,
        room_type,
        size,
        position: Vec3::new(
            id as f32 * 20.0,
            0.0,
            rng.gen_range(-10.0..10.0),
        ),
        theme,
        corruption,
        loot,
        enemies,
        puzzles,
        cc0_models: select_cc0_models(theme, corruption),
    }
}

/// Select room theme based on type and corruption
fn select_room_theme(
    room_type: RoomType,
    corruption: f32,
    rng: &mut ChaCha8Rng,
) -> RoomTheme {
    if corruption > 0.8 {
        if rng.gen_bool(0.7) {
            RoomTheme::FleshHorror
        } else {
            RoomTheme::VoidTouched
        }
    } else if corruption > 0.5 {
        RoomTheme::VoidTouched
    } else {
        match room_type {
            RoomType::Entrance => RoomTheme::Medieval,
            RoomType::Boss => RoomTheme::Temple,
            RoomType::Treasure => RoomTheme::Armory,
            _ => {
                let themes = [
                    RoomTheme::Medieval,
                    RoomTheme::Dungeon,
                    RoomTheme::Crypt,
                    RoomTheme::Library,
                ];
                themes[rng.gen_range(0..themes.len())]
            }
        }
    }
}

/// Select CC0 models for a room
fn select_cc0_models(theme: RoomTheme, corruption: f32) -> Vec<CC0Model> {
    let mut models = Vec::new();
    
    // Base models for theme
    let base_models = match theme {
        RoomTheme::Medieval => vec![
            "architecture/k_architecture_wall_stone.glb",
            "architecture/k_architecture_door_wooden.glb",
            "architecture/k_architecture_torch.glb",
        ],
        RoomTheme::Dungeon => vec![
            "architecture/k_architecture_prison_cell.glb",
            "architecture/k_architecture_chain.glb",
            "architecture/k_architecture_barrel.glb",
        ],
        RoomTheme::Crypt => vec![
            "architecture/k_architecture_tomb.glb",
            "architecture/k_architecture_skull.glb",
            "architecture/k_architecture_candelabra.glb",
        ],
        RoomTheme::Library => vec![
            "architecture/k_architecture_bookshelf.glb",
            "architecture/k_architecture_table.glb",
            "architecture/k_architecture_scroll.glb",
        ],
        RoomTheme::VoidTouched => vec![
            "void/crystal_formation.glb",
            "void/corrupted_pillar.glb",
            "void/rift_tear.glb",
        ],
        _ => vec!["architecture/k_architecture_wall_stone.glb"],
    };
    
    // Add models with corruption variants
    for model_path in base_models {
        models.push(CC0Model {
            model_path: format!("assets/models/{}", model_path),
            position: Vec3::ZERO,  // Would be positioned properly in actual implementation
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            corruption_variant: corruption,
        });
    }
    
    models
}

/// Generate loot for a room
fn generate_loot(
    rng: &mut ChaCha8Rng,
    level: u32,
    room_type: RoomType,
) -> Vec<LootItem> {
    let mut loot = Vec::new();
    
    let loot_count = match room_type {
        RoomType::Treasure => rng.gen_range(3..6),
        RoomType::Boss => rng.gen_range(2..4),
        RoomType::Secret => rng.gen_range(1..3),
        _ => rng.gen_range(0..2),
    };
    
    for _ in 0..loot_count {
        let rarity = if level > 100 {
            if rng.gen_bool(0.3) {
                LootRarity::Legendary
            } else if rng.gen_bool(0.5) {
                LootRarity::Epic
            } else {
                LootRarity::Rare
            }
        } else if level > 50 {
            if rng.gen_bool(0.2) {
                LootRarity::Epic
            } else if rng.gen_bool(0.4) {
                LootRarity::Rare
            } else {
                LootRarity::Uncommon
            }
        } else {
            if rng.gen_bool(0.1) {
                LootRarity::Rare
            } else if rng.gen_bool(0.3) {
                LootRarity::Uncommon
            } else {
                LootRarity::Common
            }
        };
        
        loot.push(LootItem {
            item_type: generate_item_type(rng, rarity),
            rarity,
            position: Vec3::new(
                rng.gen_range(-5.0..5.0),
                0.5,
                rng.gen_range(-5.0..5.0),
            ),
        });
    }
    
    loot
}

/// Generate item type based on rarity
fn generate_item_type(rng: &mut ChaCha8Rng, rarity: LootRarity) -> String {
    match rarity {
        LootRarity::Common => {
            let items = ["Health Potion", "Torch", "Rope", "Food"];
            items[rng.gen_range(0..items.len())].to_string()
        }
        LootRarity::Uncommon => {
            let items = ["Iron Sword", "Leather Armor", "Mana Potion", "Lockpick"];
            items[rng.gen_range(0..items.len())].to_string()
        }
        LootRarity::Rare => {
            let items = ["Enchanted Blade", "Mithril Armor", "Spell Scroll", "Ancient Map"];
            items[rng.gen_range(0..items.len())].to_string()
        }
        LootRarity::Epic => {
            let items = ["Dragon Scale", "Void Crystal", "Phoenix Feather", "Elder Rune"];
            items[rng.gen_range(0..items.len())].to_string()
        }
        LootRarity::Legendary => {
            let items = ["Father's Sword", "Dragon Heart", "Void Key", "Crown of Shadows"];
            items[rng.gen_range(0..items.len())].to_string()
        }
        LootRarity::Cursed => {
            let items = ["Cursed Ring", "Void Fragment", "Soul Stone", "Corrupted Artifact"];
            items[rng.gen_range(0..items.len())].to_string()
        }
    }
}

/// Generate enemies for a room
fn generate_enemies(
    rng: &mut ChaCha8Rng,
    level: u32,
    corruption: f32,
) -> Vec<EnemySpawn> {
    let mut enemies = Vec::new();
    
    let enemy_count = (level / 20 + 1).min(5) as usize;
    
    for _ in 0..enemy_count {
        let enemy_type = if corruption > 0.7 {
            let types = ["Void Spawn", "Corrupted Knight", "Shadow Wraith"];
            types[rng.gen_range(0..types.len())].to_string()
        } else if level > 50 {
            let types = ["Skeleton Warrior", "Dark Mage", "Stone Golem"];
            types[rng.gen_range(0..types.len())].to_string()
        } else {
            let types = ["Bandit", "Wolf", "Spider"];
            types[rng.gen_range(0..types.len())].to_string()
        };
        
        enemies.push(EnemySpawn {
            enemy_type,
            position: Vec3::new(
                rng.gen_range(-8.0..8.0),
                0.0,
                rng.gen_range(-8.0..8.0),
            ),
            patrol_path: vec![],
            aggression: corruption + rng.gen_range(0.2..0.8),
        });
    }
    
    enemies
}

/// Generate a puzzle
fn generate_puzzle(rng: &mut ChaCha8Rng, level: u32) -> Puzzle {
    let puzzle_types = [
        PuzzleType::PressurePlates,
        PuzzleType::RotatingStatues,
        PuzzleType::LightBeams,
        PuzzleType::RuneSequence,
        PuzzleType::TimedSwitch,
    ];
    
    let puzzle_type = if level > 100 {
        PuzzleType::VoidRiddle
    } else {
        puzzle_types[rng.gen_range(0..puzzle_types.len())]
    };
    
    Puzzle {
        puzzle_type,
        difficulty: ((level / 30) + 1).min(5) as u8,
        reward: if rng.gen_bool(0.5) {
            Some(LootItem {
                item_type: "Puzzle Reward Chest".to_string(),
                rarity: LootRarity::Rare,
                position: Vec3::ZERO,
            })
        } else {
            None
        },
    }
}

/// Generate boss room
fn generate_boss_room(
    rng: &mut ChaCha8Rng,
    level: u32,
    corruption: f32,
) -> Room {
    Room {
        id: u32::MAX,  // Will be reassigned
        room_type: RoomType::Boss,
        size: Vec3::new(30.0, 10.0, 30.0),
        position: Vec3::ZERO,
        theme: RoomTheme::Temple,
        corruption: (corruption + 0.2).min(1.0),
        loot: generate_loot(rng, level * 2, RoomType::Boss),
        enemies: vec![generate_boss_spawn(level)],
        puzzles: vec![],
        cc0_models: select_cc0_models(RoomTheme::Temple, corruption),
    }
}

/// Generate boss spawn
fn generate_boss_spawn(level: u32) -> EnemySpawn {
    let boss_type = match level {
        1..=20 => "Bandit Leader",
        21..=40 => "Corrupted Knight",
        41..=60 => "Swamp Witch",
        61..=80 => "Dragon Fragment",
        81..=100 => "Void Herald",
        101..=120 => "Fallen Companion",
        121..=140 => "Mirror Self",
        141..=160 => "Void Dragon",
        161..=180 => "True Dragon",
        _ => "Unknown Boss",
    };
    
    EnemySpawn {
        enemy_type: boss_type.to_string(),
        position: Vec3::ZERO,
        patrol_path: vec![],
        aggression: 1.0,
    }
}

/// Add side branches to the labyrinth
fn add_side_branches(
    layout: &mut LabyrinthLayout,
    rng: &mut ChaCha8Rng,
    level: u32,
    corruption: f32,
) {
    let branch_count = (level / 30 + 1).min(3) as usize;
    
    for _ in 0..branch_count {
        let branch_from = rng.gen_range(1..layout.rooms.len() as u32);
        let new_room_id = layout.rooms.len() as u32;
        
        let branch_room = generate_room(
            rng,
            new_room_id,
            RoomType::Treasure,
            level,
            corruption,
        );
        
        layout.rooms.insert(new_room_id, branch_room);
        layout.connections.entry(branch_from).or_default().push(new_room_id);
        layout.connections.entry(new_room_id).or_default().push(branch_from);
    }
}

/// Add secret rooms
fn add_secret_rooms(
    layout: &mut LabyrinthLayout,
    rng: &mut ChaCha8Rng,
    corruption: f32,
) {
    if rng.gen_bool(0.3 + corruption as f64 * 0.2) {
        let secret_id = layout.rooms.len() as u32;
        let connect_to = rng.gen_range(0..layout.rooms.len() as u32);
        
        let secret_room = Room {
            id: secret_id,
            room_type: RoomType::Secret,
            size: Vec3::new(8.0, 4.0, 8.0),
            position: Vec3::ZERO,
            theme: RoomTheme::VoidTouched,
            corruption: 1.0,
            loot: vec![
                LootItem {
                    item_type: "Void Artifact".to_string(),
                    rarity: LootRarity::Cursed,
                    position: Vec3::ZERO,
                }
            ],
            enemies: vec![],
            puzzles: vec![],
            cc0_models: select_cc0_models(RoomTheme::VoidTouched, 1.0),
        };
        
        layout.rooms.insert(secret_id, secret_room);
        // Secret rooms have hidden connections
        layout.connections.entry(connect_to).or_default().push(secret_id);
    }
}

/// Setup labyrinth renderer
fn setup_labyrinth_renderer(mut commands: Commands) {
    // Camera for first-person view
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.7, 0.0),  // Eye height
        Projection::Perspective(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
    ));
    
    // Atmospheric fog for horror
    commands.insert_resource(ClearColor(Color::srgb(0.02, 0.01, 0.03)));
}

/// Update corruption effects in labyrinth
fn update_labyrinth_corruption(
    state: Res<LabyrinthState>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<CC0Model>>,
) {
    let corruption = state.corruption_level;
    
    if corruption > 0.5 {
        // Make walls breathe
        let breathing = (time.elapsed_secs() * 0.5).sin() * 0.02 * corruption;
        
        for mut transform in &mut query {
            transform.scale = Vec3::ONE + Vec3::new(breathing, breathing, breathing);
        }
    }
}

/// Handle room transitions
fn handle_room_transitions(
    mut state: ResMut<LabyrinthState>,
    player: Query<&Transform, With<Player>>,
) {
    // Check if player has moved to a new room
    // This would check collision with room transition triggers
}

/// Spawn enemies in current room
fn spawn_labyrinth_enemies(
    state: Res<LabyrinthState>,
    mut commands: Commands,
) {
    if let Some(room) = state.layout.rooms.get(&state.current_room) {
        // Spawn enemies if room hasn't been cleared
        if !state.explored_rooms.contains(&room.id) {
            for enemy_spawn in &room.enemies {
                // Spawn enemy entity
                // This would create the actual enemy with AI
            }
        }
    }
}

/// Update void effects
fn update_void_effects(
    mut state: ResMut<LabyrinthState>,
    time: Res<Time>,
) {
    // Update active void effects
    state.active_effects.retain_mut(|effect| {
        effect.duration -= time.delta_secs();
        effect.duration > 0.0
    });
    
    // Apply void effect visuals/mechanics
    for effect in &state.active_effects {
        match effect.effect_type {
            VoidEffectType::TimeDistortion => {
                // Slow or speed up time
            }
            VoidEffectType::GeometryShift => {
                // Non-Euclidean geometry
            }
            _ => {}
        }
    }
}

/// Marker component for player
#[derive(Component)]
struct Player;
