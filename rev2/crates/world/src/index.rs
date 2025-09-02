use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct WorldIndex {
    pub axial_to_entity: HashMap<(i32,i32), Entity>,
    pub npc_by_id: HashMap<String, Entity>,
}
