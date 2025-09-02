use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct BiomeTextureMap { pub mappings: HashMap<String, u32> }

pub fn register_resources(app: &mut App) {
    // Builder will overwrite with authoritative data; seed minimal mapping
    let mut mappings = HashMap::new();
    mappings.insert("wet_meadow".to_string(), 0);
    app.insert_resource(BiomeTextureMap { mappings });
}


