// Auto-generated biome rules
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BiomeRules {
    pub adjacency: HashMap<String, Vec<String>>,    // which biomes prefer neighbors
    pub clustering: HashMap<String, ClusterRule>,    // optional cluster limits
    pub movement: HashMap<String, MovementRule>,    // movement multipliers
    pub hazards: HashMap<String, HazardRule>,       // hazard DPS etc.
    pub passability: HashMap<String, PassRule>,     // passability per movement mode
}

#[derive(Debug, Clone, Default)]
pub struct ClusterRule { pub max_cluster: Option<u32> }

#[derive(Debug, Clone, Default)]
pub struct MovementRule { pub walk: f32, pub ground_mount: f32, pub flying: f32 }

#[derive(Debug, Clone, Default)]
pub struct HazardRule { pub dps: f32 }

#[derive(Debug, Clone, Default)]
pub struct PassRule { pub walk: bool, pub ground_mount: bool, pub flying: bool }

pub fn load_biome_rules() -> BiomeRules {
    let mut adjacency: HashMap<String, Vec<String>> = HashMap::new();
    let mut clustering: HashMap<String, ClusterRule> = HashMap::new();
    let mut movement: HashMap<String, MovementRule> = HashMap::new();
    let mut hazards: HashMap<String, HazardRule> = HashMap::new();
    let mut passability: HashMap<String, PassRule> = HashMap::new();

    // Adjacency
    adjacency.insert("plains".to_string(), vec![
"plains".to_string(),"forest".to_string(),"coast".to_string(),    ]);
    adjacency.insert("forest".to_string(), vec![
"forest".to_string(),"plains".to_string(),"mountain".to_string(),    ]);
    adjacency.insert("mountain".to_string(), vec![
"mountain".to_string(),"forest".to_string(),"snow".to_string(),    ]);
    adjacency.insert("swamp".to_string(), vec![
"swamp".to_string(),"forest".to_string(),"coast".to_string(),    ]);
    adjacency.insert("desert".to_string(), vec![
"desert".to_string(),"plains".to_string(),    ]);

    // Clustering
    clustering.insert("lava".to_string(), ClusterRule{ max_cluster: 6 });

    // Movement
    movement.insert("plains".to_string(), MovementRule{ 
        walk: 1.0, 
        ground_mount: 1.0, 
        flying: 1.0 
    });
    movement.insert("forest".to_string(), MovementRule{ 
        walk: 1.2, 
        ground_mount: 1.1, 
        flying: 1.0 
    });
    movement.insert("swamp".to_string(), MovementRule{ 
        walk: 1.5, 
        ground_mount: 1.3, 
        flying: 1.0 
    });
    movement.insert("mountain".to_string(), MovementRule{ 
        walk: 1.8, 
        ground_mount: 1.6, 
        flying: 1.0 
    });
    movement.insert("desert".to_string(), MovementRule{ 
        walk: 1.2, 
        ground_mount: 1.1, 
        flying: 1.0 
    });

    // Hazards
    hazards.insert("lava".to_string(), HazardRule{ dps: 8.0 });
    hazards.insert("swamp".to_string(), HazardRule{ dps: 1.0 });

    // Passability
    passability.insert("mountain".to_string(), PassRule{ 
        walk: false, 
        ground_mount: false, 
        flying: true 
    });
    passability.insert("plains".to_string(), PassRule{ 
        walk: true, 
        ground_mount: true, 
        flying: true 
    });

    BiomeRules { adjacency, clustering, movement, hazards, passability }
}

