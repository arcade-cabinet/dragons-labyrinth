//! Memory system for maintaining agent context and conversation history

use std::collections::{HashMap, VecDeque};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Maximum number of conversation turns to keep in memory
const MAX_CONVERSATION_HISTORY: usize = 20;

/// Maximum number of generation results to cache
const MAX_GENERATION_CACHE: usize = 100;

/// Agent memory system for maintaining context
#[derive(Debug, Clone, Default)]
pub struct AgentMemory {
    /// Conversation history with the AI
    conversation_history: VecDeque<ConversationTurn>,
    
    /// Cache of generated assets by ID
    generation_cache: HashMap<String, GenerationCacheEntry>,
    
    /// Current context variables
    context_variables: HashMap<String, Value>,
    
    /// Horror progression state
    horror_state: HorrorState,
}

/// A single turn in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    /// Role (user, assistant, system)
    pub role: String,
    
    /// Message content
    pub content: String,
    
    /// Tool calls made in this turn (if any)
    pub tool_calls: Option<Vec<ToolCall>>,
    
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// A tool call made during conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name
    pub name: String,
    
    /// Tool arguments
    pub arguments: Value,
    
    /// Tool result
    pub result: Option<Value>,
}

/// Cached generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationCacheEntry {
    /// Asset ID
    pub asset_id: String,
    
    /// Asset type
    pub asset_type: String,
    
    /// Dread level
    pub dread_level: u8,
    
    /// Description used for generation
    pub description: String,
    
    /// Generated result
    pub result: Value,
    
    /// Timestamp of generation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Horror progression state tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HorrorState {
    /// Current dread level (0-4)
    pub current_dread_level: u8,
    
    /// Companion states
    pub companion_states: HashMap<String, CompanionState>,
    
    /// World corruption percentage
    pub world_corruption: f32,
    
    /// Dragon proximity (0.0 = far, 1.0 = immediate danger)
    pub dragon_proximity: f32,
}

impl Default for HorrorState {
    fn default() -> Self {
        Self {
            current_dread_level: 0,
            companion_states: Self::init_companions(),
            world_corruption: 0.0,
            dragon_proximity: 0.0,
        }
    }
}

impl HorrorState {
    fn init_companions() -> HashMap<String, CompanionState> {
        let mut companions = HashMap::new();
        
        companions.insert("Einar".to_string(), CompanionState {
            name: "Einar".to_string(),
            loyalty: 100.0,
            trauma_level: 0.0,
            is_present: true,
            has_betrayed: false,
        });
        
        companions.insert("Mira".to_string(), CompanionState {
            name: "Mira".to_string(),
            loyalty: 90.0,
            trauma_level: 0.0,
            is_present: true,
            has_betrayed: false,
        });
        
        companions.insert("Sorin".to_string(), CompanionState {
            name: "Sorin".to_string(),
            loyalty: 80.0,
            trauma_level: 0.0,
            is_present: true,
            has_betrayed: false,
        });
        
        companions.insert("Tamara".to_string(), CompanionState {
            name: "Tamara".to_string(),
            loyalty: 95.0,
            trauma_level: 0.0,
            is_present: true,
            has_betrayed: false,
        });
        
        companions
    }
}

/// Individual companion state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionState {
    /// Companion name
    pub name: String,
    
    /// Loyalty level (0-100)
    pub loyalty: f32,
    
    /// Trauma level (0.0-1.0)
    pub trauma_level: f32,
    
    /// Whether companion is still with party
    pub is_present: bool,
    
    /// Whether companion has betrayed
    pub has_betrayed: bool,
}

impl AgentMemory {
    /// Create a new memory system
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a conversation turn
    pub fn add_conversation_turn(&mut self, turn: ConversationTurn) {
        if self.conversation_history.len() >= MAX_CONVERSATION_HISTORY {
            self.conversation_history.pop_front();
        }
        self.conversation_history.push_back(turn);
    }
    
    /// Get conversation history
    pub fn conversation_history(&self) -> &VecDeque<ConversationTurn> {
        &self.conversation_history
    }
    
    /// Add a generation to cache
    pub fn cache_generation(&mut self, entry: GenerationCacheEntry) {
        // Remove oldest entries if cache is full
        if self.generation_cache.len() >= MAX_GENERATION_CACHE {
            // Find and remove oldest entry
            if let Some(oldest_key) = self.generation_cache
                .iter()
                .min_by_key(|(_, v)| v.timestamp)
                .map(|(k, _)| k.clone())
            {
                self.generation_cache.remove(&oldest_key);
            }
        }
        
        self.generation_cache.insert(entry.asset_id.clone(), entry);
    }
    
    /// Check if an asset is cached
    pub fn get_cached_generation(&self, asset_id: &str) -> Option<&GenerationCacheEntry> {
        self.generation_cache.get(asset_id)
    }
    
    /// Find cached generations by type and dread level
    pub fn find_cached_generations(&self, asset_type: &str, dread_level: u8) -> Vec<&GenerationCacheEntry> {
        self.generation_cache
            .values()
            .filter(|entry| entry.asset_type == asset_type && entry.dread_level == dread_level)
            .collect()
    }
    
    /// Set a context variable
    pub fn set_context(&mut self, key: String, value: Value) {
        self.context_variables.insert(key, value);
    }
    
    /// Get a context variable
    pub fn get_context(&self, key: &str) -> Option<&Value> {
        self.context_variables.get(key)
    }
    
    /// Get all context variables
    pub fn context_variables(&self) -> &HashMap<String, Value> {
        &self.context_variables
    }
    
    /// Get horror state
    pub fn horror_state(&self) -> &HorrorState {
        &self.horror_state
    }
    
    /// Get mutable horror state
    pub fn horror_state_mut(&mut self) -> &mut HorrorState {
        &mut self.horror_state
    }
    
    /// Update dread level
    pub fn set_dread_level(&mut self, level: u8) {
        self.horror_state.current_dread_level = level.min(4);
        
        // Update world corruption based on dread level
        self.horror_state.world_corruption = match level {
            0 => 0.0,
            1 => 0.15,
            2 => 0.40,
            3 => 0.70,
            4 => 1.0,
            _ => 1.0,
        };
        
        // Update companion trauma based on dread level
        for companion in self.horror_state.companion_states.values_mut() {
            companion.trauma_level = match level {
                0 => 0.0,
                1 => 0.1,
                2 => 0.3,
                3 => 0.6,
                4 => 0.9,
                _ => 1.0,
            };
            
            // Loyalty decreases with dread
            companion.loyalty = (100.0 - (level as f32 * 20.0)).max(0.0);
            
            // Some companions leave or betray at high dread
            if level >= 3 && companion.name == "Mira" {
                companion.is_present = false; // Mira leaves at Terror
            }
            if level >= 4 && companion.name == "Sorin" {
                companion.has_betrayed = true; // Sorin betrays at Horror
            }
        }
        
        // Update dragon proximity
        self.horror_state.dragon_proximity = match level {
            0 => 0.0,
            1 => 0.05,
            2 => 0.20,
            3 => 0.50,
            4 => 0.95,
            _ => 1.0,
        };
    }
    
    /// Clear conversation history
    pub fn clear_conversation(&mut self) {
        self.conversation_history.clear();
    }
    
    /// Clear generation cache
    pub fn clear_cache(&mut self) {
        self.generation_cache.clear();
    }
    
    /// Reset memory to initial state
    pub fn reset(&mut self) {
        self.conversation_history.clear();
        self.generation_cache.clear();
        self.context_variables.clear();
        self.horror_state = HorrorState::default();
    }
}
