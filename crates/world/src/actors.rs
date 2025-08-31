use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ActorsFile { pub party: Vec<ActorSpec>, pub npcs: Vec<ActorSpec> }

#[derive(Debug, Clone, Deserialize)]
pub struct ActorSpec { pub id: String, pub name: String, pub axial: String, pub sprite: String }
