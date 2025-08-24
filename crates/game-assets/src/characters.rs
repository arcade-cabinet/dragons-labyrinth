//! Character definitions and personalities for Dragon's Labyrinth
//!
//! This is the single source of truth for all character data

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum CompanionArchetype {
    Therapist,
    Child,
    Medic,
    Scholar,
    Warrior,
    Fool,
    Priest,
    Thief,
    Noble,
    Hermit,
    Artist,
    Betrayer,
}

pub struct CompanionData {
    pub archetype: CompanionArchetype,
    pub name: &'static str,
    pub age: u8,
    pub personality: &'static str,
    pub backstory: &'static str,
    pub fear: &'static str,
    pub breaking_point: &'static str,
}

impl CompanionArchetype {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Therapist, Self::Child, Self::Medic, Self::Scholar,
            Self::Warrior, Self::Fool, Self::Priest, Self::Thief,
            Self::Noble, Self::Hermit, Self::Artist, Self::Betrayer,
        ]
    }
    
    pub fn data(&self) -> CompanionData {
        match self {
            Self::Therapist => CompanionData {
                archetype: *self,
                name: "Dr. Helena Cross",
                age: 42,
                personality: "Professional, analytical, hiding deep trauma",
                backstory: "Former military psychologist who witnessed unspeakable horrors",
                fear: "Losing control of her carefully constructed facade",
                breaking_point: "When forced to confront her own repressed memories",
            },
            Self::Child => CompanionData {
                archetype: *self,
                name: "Timmy",
                age: 8,
                personality: "Innocent but eerily observant",
                backstory: "Orphan who sees things others cannot",
                fear: "Being alone in the dark",
                breaking_point: "Witnessing violence against those he trusts",
            },
            Self::Medic => CompanionData {
                archetype: *self,
                name: "Marcus Reid",
                age: 35,
                personality: "Heals others but can't heal himself",
                backstory: "Battlefield surgeon haunted by those he couldn't save",
                fear: "Being unable to help when needed most",
                breaking_point: "Having to choose who lives and who dies",
            },
            Self::Scholar => CompanionData {
                archetype: *self,
                name: "Professor Aldric Blackwood",
                age: 67,
                personality: "Knowledge brings madness",
                backstory: "Discovered forbidden texts that revealed terrible truths",
                fear: "The implications of what he knows",
                breaking_point: "When his theories prove horrifyingly correct",
            },
            Self::Warrior => CompanionData {
                archetype: *self,
                name: "Captain Elara Thorne",
                age: 38,
                personality: "Strong exterior, crumbling interior",
                backstory: "Decorated soldier struggling with PTSD",
                fear: "Showing weakness to those who depend on her",
                breaking_point: "Failing to protect her companions",
            },
            Self::Fool => CompanionData {
                archetype: *self,
                name: "Jasper the Jester",
                age: 29,
                personality: "Wisdom through apparent madness",
                backstory: "Court fool who spoke truth through jest, exiled for knowing too much",
                fear: "Being taken seriously",
                breaking_point: "When laughter can no longer mask the horror",
            },
            Self::Priest => CompanionData {
                archetype: *self,
                name: "Father Benedict",
                age: 55,
                personality: "Faith tested by horror",
                backstory: "Witnessed his congregation consumed by darkness",
                fear: "That God has abandoned the world",
                breaking_point: "Being forced to commit sin to survive",
            },
            Self::Thief => CompanionData {
                archetype: *self,
                name: "Silas Crow",
                age: 31,
                personality: "Survivor with guilty past",
                backstory: "Stole to survive, left others to die",
                fear: "Karma catching up",
                breaking_point: "Having to sacrifice himself for others",
            },
            Self::Noble => CompanionData {
                archetype: *self,
                name: "Lady Evangeline Ashford",
                age: 26,
                personality: "Privilege becomes burden",
                backstory: "Sheltered aristocrat thrust into nightmare",
                fear: "Losing her identity and purpose",
                breaking_point: "Realizing her family's dark involvement",
            },
            Self::Hermit => CompanionData {
                archetype: *self,
                name: "Old Tom",
                age: 73,
                personality: "Isolated wisdom, social decay",
                backstory: "Fled society after foreseeing its doom",
                fear: "Being forced back into civilization",
                breaking_point: "When isolation is no longer possible",
            },
            Self::Artist => CompanionData {
                archetype: *self,
                name: "Celeste Moreau",
                age: 24,
                personality: "Sees too much beauty and horror",
                backstory: "Painter whose art predicted the catastrophe",
                fear: "Losing the ability to create",
                breaking_point: "When reality becomes worse than her visions",
            },
            Self::Betrayer => CompanionData {
                archetype: *self,
                name: "Vincent Drake",
                age: 45,
                personality: "Destined to turn against you",
                backstory: "Made a deal he can't escape",
                fear: "The moment of inevitable betrayal",
                breaking_point: "When the price must be paid",
            },
        }
    }
}
