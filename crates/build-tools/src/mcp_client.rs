//! MCP Client for Runtime Intelligence Queries
//! 
//! This module enables build-tools to query the game-database MCP server during AI generation,
//! providing access to runtime game state for intelligent content generation decisions.
//! Key capability identified in vision integration analysis for balancing core code vs prompt generation.

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use tokio::time::{timeout, Duration};
use uuid::Uuid;

// ============================================================================
// MCP CLIENT CONFIGURATION
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpClientConfig {
    pub server_host: String,
    pub server_port: u16,
    pub timeout_seconds: u64,
    pub retry_attempts: u32,
    pub connection_pool_size: u32,
}

impl Default for McpClientConfig {
    fn default() -> Self {
        Self {
            server_host: "127.0.0.1".to_string(),
            server_port: 8091,
            timeout_seconds: 30,
            retry_attempts: 3,
            connection_pool_size: 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameDatabaseMcpClient {
    config: McpClientConfig,
    client: reqwest::Client,
    base_url: String,
}

impl GameDatabaseMcpClient {
    pub fn new(config: McpClientConfig) -> Self {
        let base_url = format!("http://{}:{}", config.server_host, config.server_port);
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            config,
            client,
            base_url,
        }
    }
    
    pub fn with_default_config() -> Self {
        Self::new(McpClientConfig::default())
    }
}

// ============================================================================
// FORGE SYSTEM INTELLIGENCE QUERIES
// ============================================================================

impl GameDatabaseMcpClient {
    /// Query forge readiness for intelligent forge-related content generation
    pub async fn query_forge_readiness(
        &self,
        player_id: Option<Uuid>,
        include_companion_states: bool,
        include_sentimental_items: bool,
        include_trial_progress: bool,
    ) -> Result<ForgeReadinessIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "analyze_forge_readiness",
            "params": {
                "player_id": player_id,
                "include_companion_states": include_companion_states,
                "include_sentimental_items": include_sentimental_items,
                "include_trial_progress": include_trial_progress
            },
            "id": 1
        });
        
        let response = self.send_request(request).await?;
        let intelligence: ForgeReadinessIntelligence = serde_json::from_value(response)
            .context("Failed to parse forge readiness response")?;
        
        tracing::info!("Forge readiness queried: {:.2} readiness score", intelligence.readiness_score);
        Ok(intelligence)
    }
    
    /// Query sentimental reagents for intelligent reagent-related content generation
    pub async fn query_sentimental_reagents(
        &self,
        player_id: Option<Uuid>,
        category_filter: Option<String>,
        include_forge_power: bool,
    ) -> Result<SentimentalReagentsIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "get_sentimental_reagents",
            "params": {
                "player_id": player_id,
                "category_filter": category_filter,
                "include_forge_power": include_forge_power
            },
            "id": 2
        });
        
        let response = self.send_request(request).await?;
        let intelligence: SentimentalReagentsIntelligence = serde_json::from_value(response)
            .context("Failed to parse sentimental reagents response")?;
        
        tracing::info!("Sentimental reagents queried: {} reagents found", intelligence.reagent_count);
        Ok(intelligence)
    }
    
    /// Evaluate sacrifice candidates for intelligent companion-related content generation
    pub async fn evaluate_sacrifice_candidates(
        &self,
        player_id: Uuid,
        forge_path: Option<String>,
        include_emotional_cost: bool,
    ) -> Result<SacrificeIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "evaluate_sacrifice_candidates",
            "params": {
                "player_id": player_id,
                "forge_path": forge_path,
                "include_emotional_cost": include_emotional_cost
            },
            "id": 3
        });
        
        let response = self.send_request(request).await?;
        let intelligence: SacrificeIntelligence = serde_json::from_value(response)
            .context("Failed to parse sacrifice candidates response")?;
        
        tracing::info!("Sacrifice candidates evaluated: {} candidates", intelligence.candidate_count);
        Ok(intelligence)
    }
}

// ============================================================================
// PSYCHOLOGY SYSTEM INTELLIGENCE QUERIES
// ============================================================================

impl GameDatabaseMcpClient {
    /// Query companion trauma for intelligent trauma-aware content generation
    pub async fn query_companion_trauma(
        &self,
        companion_id: Option<Uuid>,
        include_therapy_progress: bool,
        include_relationships: bool,
        include_support_network: bool,
        analyze_group_dynamics: bool,
    ) -> Result<CompanionTraumaIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "analyze_companion_trauma",
            "params": {
                "companion_id": companion_id,
                "include_therapy_progress": include_therapy_progress,
                "include_relationships": include_relationships,
                "include_support_network": include_support_network,
                "analyze_group_dynamics": analyze_group_dynamics
            },
            "id": 4
        });
        
        let response = self.send_request(request).await?;
        let intelligence: CompanionTraumaIntelligence = serde_json::from_value(response)
            .context("Failed to parse companion trauma response")?;
        
        tracing::info!("Companion trauma analyzed: {:.2} average trauma", intelligence.average_trauma_level);
        Ok(intelligence)
    }
    
    /// Get therapy recommendations for intelligent therapy content generation
    pub async fn get_therapy_recommendations(
        &self,
        companion_id: Option<Uuid>,
        trauma_threshold: Option<f32>,
        include_group_therapy: bool,
    ) -> Result<TherapyIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "get_therapy_recommendations",
            "params": {
                "companion_id": companion_id,
                "trauma_threshold": trauma_threshold,
                "include_group_therapy": include_group_therapy
            },
            "id": 5
        });
        
        let response = self.send_request(request).await?;
        let intelligence: TherapyIntelligence = serde_json::from_value(response)
            .context("Failed to parse therapy recommendations response")?;
        
        tracing::info!("Therapy recommendations queried: {} recommendations", intelligence.recommendation_count);
        Ok(intelligence)
    }
    
    /// Analyze support network for intelligent relationship content generation
    pub async fn analyze_support_network(
        &self,
        player_id: Uuid,
        include_effectiveness_scores: bool,
        analyze_vulnerabilities: bool,
    ) -> Result<SupportNetworkIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "analyze_support_network",
            "params": {
                "player_id": player_id,
                "include_effectiveness_scores": include_effectiveness_scores,
                "analyze_vulnerabilities": analyze_vulnerabilities
            },
            "id": 6
        });
        
        let response = self.send_request(request).await?;
        let intelligence: SupportNetworkIntelligence = serde_json::from_value(response)
            .context("Failed to parse support network response")?;
        
        tracing::info!("Support network analyzed: {:.2} network strength", intelligence.network_strength);
        Ok(intelligence)
    }
}

// ============================================================================
// PHILOSOPHY SYSTEM INTELLIGENCE QUERIES
// ============================================================================

impl GameDatabaseMcpClient {
    /// Query philosophical progression for intelligent philosophy-aware content generation
    pub async fn query_philosophical_progression(
        &self,
        player_id: Option<Uuid>,
        include_trait_analysis: bool,
        include_transition_history: bool,
        include_consistency_metrics: bool,
    ) -> Result<PhilosophicalIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "analyze_philosophical_progression",
            "params": {
                "player_id": player_id,
                "include_trait_analysis": include_trait_analysis,
                "include_transition_history": include_transition_history,
                "include_consistency_metrics": include_consistency_metrics
            },
            "id": 7
        });
        
        let response = self.send_request(request).await?;
        let intelligence: PhilosophicalIntelligence = serde_json::from_value(response)
            .context("Failed to parse philosophical progression response")?;
        
        tracing::info!("Philosophical progression analyzed: {} dominant philosophy", 
                      intelligence.dominant_philosophy);
        Ok(intelligence)
    }
    
    /// Get transition opportunities for intelligent choice content generation
    pub async fn get_transition_opportunities(
        &self,
        player_id: Uuid,
        current_act: u32,
        current_dread_level: u32,
    ) -> Result<TransitionIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "get_transition_opportunities",
            "params": {
                "player_id": player_id,
                "current_act": current_act,
                "current_dread_level": current_dread_level
            },
            "id": 8
        });
        
        let response = self.send_request(request).await?;
        let intelligence: TransitionIntelligence = serde_json::from_value(response)
            .context("Failed to parse transition opportunities response")?;
        
        tracing::info!("Transition opportunities queried: {} available transitions", 
                      intelligence.available_transition_count);
        Ok(intelligence)
    }
}

// ============================================================================
// CROSS-SYSTEM INTELLIGENCE QUERIES
// ============================================================================

impl GameDatabaseMcpClient {
    /// Get comprehensive game state intelligence for holistic content generation
    pub async fn get_game_state_intelligence(
        &self,
        player_id: Uuid,
        include_predictions: bool,
        include_recommendations: bool,
        include_system_interactions: bool,
    ) -> Result<GameStateIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "get_game_state_intelligence",
            "params": {
                "player_id": player_id,
                "include_predictions": include_predictions,
                "include_recommendations": include_recommendations,
                "include_system_interactions": include_system_interactions
            },
            "id": 9
        });
        
        let response = self.send_request(request).await?;
        let intelligence: GameStateIntelligence = serde_json::from_value(response)
            .context("Failed to parse game state intelligence response")?;
        
        tracing::info!("Game state intelligence retrieved: {} system interconnections", 
                      intelligence.system_interaction_count);
        Ok(intelligence)
    }
    
    /// Analyze system interconnections for intelligent cross-system content generation
    pub async fn analyze_system_interconnections(
        &self,
        focus_systems: Vec<String>,
        interaction_depth: u32,
        include_predictions: bool,
    ) -> Result<SystemInterconnectionIntelligence> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "analyze_system_interconnections",
            "params": {
                "focus_systems": focus_systems,
                "interaction_depth": interaction_depth,
                "include_predictions": include_predictions
            },
            "id": 10
        });
        
        let response = self.send_request(request).await?;
        let intelligence: SystemInterconnectionIntelligence = serde_json::from_value(response)
            .context("Failed to parse system interconnection response")?;
        
        tracing::info!("System interconnections analyzed: {} dependencies found", 
                      intelligence.dependency_count);
        Ok(intelligence)
    }
}

// ============================================================================
// INTELLIGENCE DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeReadinessIntelligence {
    pub readiness_score: f32,               // 0.0-1.0 overall readiness
    pub missing_requirements: Vec<String>,
    pub forge_path_chosen: Option<String>,
    pub trial_completion_status: HashMap<String, f32>,
    pub sentimental_reagent_count: u32,
    pub total_reagent_power: f32,
    pub sacrifice_candidate_count: u32,
    pub recommendations: Vec<String>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionTraumaIntelligence {
    pub average_trauma_level: f32,          // 0.0-5.0 average companion trauma
    pub companions_in_therapy: u32,
    pub companions_at_breakdown_risk: u32,
    pub group_cohesion: f32,                // 0.0-1.0 group support strength
    pub therapy_effectiveness: f32,         // 0.0-1.0 therapy effectiveness
    pub intervention_priorities: Vec<InterventionPriority>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophicalIntelligence {
    pub dominant_philosophy: String,        // Current strongest philosophy
    pub philosophy_scores: HashMap<String, f32>,
    pub identity_stability: f32,            // 0.0-1.0 identity stability
    pub philosophical_conflict: f32,        // 0.0-1.0 internal conflict
    pub current_act: u32,                   // 1-3 current act
    pub transitions_completed: u32,
    pub authenticity_score: f32,            // 0.0-1.0 authenticity to philosophy
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStateIntelligence {
    pub dread_level: u32,                   // 0-4 current dread level
    pub corruption_level: f32,              // 0.0-1.0 world corruption
    pub companion_count: u32,               // Active companions
    pub system_health: HashMap<String, f32>, // Health of each system
    pub system_interaction_count: u32,      // Number of system interactions
    pub predictions: Vec<GameStatePrediction>,
    pub recommendations: Vec<ContentRecommendation>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentalReagentsIntelligence {
    pub reagent_count: u32,
    pub total_power: f32,
    pub categories_represented: Vec<String>,
    pub light_path_compatibility: f32,      // 0.0-1.0 compatibility with light forge
    pub dark_path_compatibility: f32,       // 0.0-1.0 compatibility with dark forge
    pub memory_trigger_count: u32,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SacrificeIntelligence {
    pub candidate_count: u32,
    pub total_power_potential: f32,
    pub emotional_cost_assessment: f32,     // 0.0-1.0 emotional cost to player
    pub companion_willingness_average: f32, // 0.0-1.0 average willingness
    pub sacrifice_recommendations: Vec<String>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TherapyIntelligence {
    pub recommendation_count: u32,
    pub urgent_interventions: u32,
    pub group_therapy_opportunities: u32,
    pub overall_therapy_effectiveness: f32, // 0.0-1.0 effectiveness
    pub therapeutic_alliance_strength: f32, // 0.0-1.0 alliance strength
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportNetworkIntelligence {
    pub network_strength: f32,              // 0.0-1.0 overall network strength
    pub support_flow_efficiency: f32,       // 0.0-1.0 how efficiently support flows
    pub network_vulnerability_count: u32,
    pub network_strength_count: u32,
    pub optimization_opportunities: Vec<String>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransitionIntelligence {
    pub available_transition_count: u32,
    pub transition_requirements: HashMap<String, Vec<String>>,
    pub philosophical_impact_predictions: HashMap<String, f32>,
    pub recommended_next_transition: Option<String>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInterconnectionIntelligence {
    pub dependency_count: u32,
    pub cascade_effect_count: u32,
    pub optimization_opportunity_count: u32,
    pub system_interaction_map: HashMap<String, Vec<String>>,
    pub critical_dependencies: Vec<String>,
    pub content_generation_guidance: ContentGenerationGuidance,
}

// ============================================================================
// CONTENT GENERATION GUIDANCE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentGenerationGuidance {
    // Generation priorities based on game state
    pub high_priority_content: Vec<String>,  // Content that should be generated first
    pub medium_priority_content: Vec<String>, // Content that would be helpful
    pub low_priority_content: Vec<String>,   // Content that can wait
    
    // Content themes based on current state
    pub recommended_themes: Vec<String>,     // Themes that fit current state
    pub avoid_themes: Vec<String>,           // Themes that don't fit current state
    pub emotional_tone_guidance: String,     // Recommended emotional tone
    
    // System-specific guidance
    pub forge_content_guidance: Option<ForgeContentGuidance>,
    pub trauma_content_guidance: Option<TraumaContentGuidance>,
    pub philosophy_content_guidance: Option<PhilosophyContentGuidance>,
    pub decay_content_guidance: Option<DecayContentGuidance>,
    
    // Generation parameters
    pub dread_level_context: u32,           // Current dread level for context
    pub horror_intensity_guidance: f32,     // 0.0-1.0 recommended horror intensity
    pub complexity_level_guidance: f32,     // 0.0-1.0 recommended complexity
    pub player_readiness_assessment: f32,   // 0.0-1.0 readiness for complex content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgeContentGuidance {
    pub emphasize_trials: bool,             // Should content emphasize trial preparation?
    pub emphasize_reagents: bool,           // Should content emphasize reagent collection?
    pub emphasize_sacrifice: bool,          // Should content explore sacrifice themes?
    pub forge_path_bias: Option<String>,    // Bias toward light or dark path
    pub trial_focus_areas: Vec<String>,     // Which trials need content support
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaContentGuidance {
    pub emphasize_therapy: bool,            // Should content emphasize therapeutic healing?
    pub emphasize_support: bool,            // Should content emphasize companion support?
    pub trauma_categories_to_address: Vec<String>, // Which trauma types need attention
    pub therapeutic_approaches_needed: Vec<String>, // Therapy approaches to include
    pub relationship_repair_opportunities: Vec<String>, // Relationship repair content
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyContentGuidance {
    pub emphasize_identity_development: bool, // Should content support identity growth?
    pub emphasize_consistency_testing: bool,  // Should content test philosophical consistency?
    pub philosophy_path_to_reinforce: Option<String>, // Which philosophy to reinforce
    pub trait_development_opportunities: Vec<String>, // Traits that could be developed
    pub authenticity_challenge_level: f32,   // 0.0-1.0 recommended authenticity challenge
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayContentGuidance {
    pub emphasize_corruption_effects: bool,  // Should content show corruption effects?
    pub emphasize_social_isolation: bool,    // Should content show social breakdown?
    pub emphasize_economic_collapse: bool,   // Should content show economic effects?
    pub emphasize_reality_distortion: bool,  // Should content include distortions?
    pub decay_stage_appropriate_content: Vec<String>, // Content appropriate for decay stage
}

// ============================================================================
// HELPER DATA STRUCTURES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionPriority {
    pub companion_id: Uuid,
    pub intervention_type: String,
    pub priority_level: f32,                // 0.0-1.0 priority
    pub time_sensitivity: String,           // "immediate", "urgent", "moderate", "low"
    pub intervention_description: String,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameStatePrediction {
    pub prediction_type: String,            // Type of prediction
    pub prediction_description: String,     // What is predicted
    pub confidence_level: f32,              // 0.0-1.0 confidence in prediction
    pub time_horizon: String,               // When prediction applies
    pub influencing_factors: Vec<String>,   // What factors influence this prediction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRecommendation {
    pub content_type: String,               // Type of content recommended
    pub recommendation_description: String, // What content should be created
    pub priority_level: f32,                // 0.0-1.0 priority of this content
    pub target_systems: Vec<String>,        // Which systems this content supports
    pub generation_parameters: HashMap<String, Value>, // Parameters for generation
}

// ============================================================================
// MCP CLIENT IMPLEMENTATION
// ============================================================================

impl GameDatabaseMcpClient {
    async fn send_request(&self, request: Value) -> Result<Value> {
        let mut last_error = None;
        
        for attempt in 1..=self.config.retry_attempts {
            match self.send_request_once(&request).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    last_error = Some(error);
                    if attempt < self.config.retry_attempts {
                        tracing::warn!("MCP request failed, retrying attempt {}/{}: {}", 
                                     attempt, self.config.retry_attempts, last_error.as_ref().unwrap());
                        tokio::time::sleep(Duration::from_millis(100 * attempt as u64)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
    
    async fn send_request_once(&self, request: &Value) -> Result<Value> {
        let response = timeout(
            Duration::from_secs(self.config.timeout_seconds),
            self.client.post(&self.base_url)
                .header("Content-Type", "application/json")
                .json(request)
                .send()
        ).await
        .context("MCP request timeout")?
        .context("Failed to send MCP request")?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("MCP server returned error: {}", response.status()));
        }
        
        let response_json: Value = response.json().await
            .context("Failed to parse MCP response JSON")?;
        
        // Check for JSON-RPC error
        if let Some(error) = response_json.get("error") {
            return Err(anyhow::anyhow!("MCP server error: {}", error));
        }
        
        // Extract result
        response_json.get("result")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No result in MCP response"))
    }
    
    /// Test MCP server connectivity
    pub async fn test_connection(&self) -> Result<bool> {
        let test_request = json!({
            "jsonrpc": "2.0",
            "method": "get_game_state_intelligence",
            "params": {
                "player_id": Uuid::new_v4(),
                "include_predictions": false,
                "include_recommendations": false,
                "include_system_interactions": false
            },
            "id": 0
        });
        
        match self.send_request_once(&test_request).await {
            Ok(_) => {
                tracing::info!("MCP server connection test successful");
                Ok(true)
            },
            Err(error) => {
                tracing::error!("MCP server connection test failed: {}", error);
                Ok(false)
            }
        }
    }
}

// ============================================================================
// INTELLIGENT CONTENT GENERATION HELPERS
// ============================================================================

/// High-level interface for AI agents to query game state for intelligent generation
pub struct IntelligentContentGenerator {
    mcp_client: GameDatabaseMcpClient,
}

impl IntelligentContentGenerator {
    pub fn new(config: McpClientConfig) -> Self {
        Self {
            mcp_client: GameDatabaseMcpClient::new(config),
        }
    }
    
    pub fn with_default_config() -> Self {
        Self::new(McpClientConfig::default())
    }
    
    /// Get comprehensive intelligence for holistic content generation
    pub async fn get_comprehensive_intelligence(&self, player_id: Uuid) -> Result<ComprehensiveIntelligence> {
        // Query all sophisticated systems for complete picture
        let forge_intelligence = self.mcp_client.query_forge_readiness(
            Some(player_id), true, true, true
        ).await?;
        
        let trauma_intelligence = self.mcp_client.query_companion_trauma(
            None, true, true, true, true
        ).await?;
        
        let philosophy_intelligence = self.mcp_client.query_philosophical_progression(
            Some(player_id), true, true, true
        ).await?;
        
        let game_state_intelligence = self.mcp_client.get_game_state_intelligence(
            player_id, true, true, true
        ).await?;
        
        let system_interconnections = self.mcp_client.analyze_system_interconnections(
            vec!["forge".to_string(), "psychology".to_string(), "philosophy".to_string(), "decay".to_string()],
            3, // Deep analysis
            true
        ).await?;
        
        Ok(ComprehensiveIntelligence {
            forge_intelligence,
            trauma_intelligence,
            philosophy_intelligence,
            game_state_intelligence,
            system_interconnections,
            generation_timestamp: chrono::Utc::now(),
            intelligence_completeness: 0.9, // High completeness with all systems
        })
    }
    
    /// Get targeted intelligence for specific content generation needs
    pub async fn get_targeted_intelligence(
        &self,
        player_id: Uuid,
        focus_systems: Vec<String>,
        content_type: String,
    ) -> Result<TargetedIntelligence> {
        let mut intelligence = TargetedIntelligence {
            content_type: content_type.clone(),
            focus_systems: focus_systems.clone(),
            primary_guidance: ContentGenerationGuidance::default(),
            system_specific_intelligence: HashMap::new(),
            generation_timestamp: chrono::Utc::now(),
        };
        
        // Query only requested systems for efficiency
        for system in &focus_systems {
            match system.as_str() {
                "forge" => {
                    let forge_intel = self.mcp_client.query_forge_readiness(
                        Some(player_id), true, true, true
                    ).await?;
                    intelligence.system_specific_intelligence.insert(
                        "forge".to_string(), 
                        serde_json::to_value(forge_intel)?
                    );
                },
                "psychology" => {
                    let trauma_intel = self.mcp_client.query_companion_trauma(
                        None, true, true, true, true
                    ).await?;
                    intelligence.system_specific_intelligence.insert(
                        "psychology".to_string(),
                        serde_json::to_value(trauma_intel)?
                    );
                },
                "philosophy" => {
                    let philosophy_intel = self.mcp_client.query_philosophical_progression(
                        Some(player_id), true, true, true
                    ).await?;
                    intelligence.system_specific_intelligence.insert(
                        "philosophy".to_string(),
                        serde_json::to_value(philosophy_intel)?
                    );
                },
                _ => {
                    tracing::warn!("Unknown system requested for intelligence: {}", system);
                }
            }
        }
        
        Ok(intelligence)
    }
    
    /// Test MCP server connectivity
    pub async fn test_connection(&self) -> Result<bool> {
        self.mcp_client.test_connection().await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveIntelligence {
    pub forge_intelligence: ForgeReadinessIntelligence,
    pub trauma_intelligence: CompanionTraumaIntelligence,
    pub philosophy_intelligence: PhilosophicalIntelligence,
    pub game_state_intelligence: GameStateIntelligence,
    pub system_interconnections: SystemInterconnectionIntelligence,
    pub generation_timestamp: chrono::DateTime<chrono::Utc>,
    pub intelligence_completeness: f32,     // 0.0-1.0 completeness of intelligence
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetedIntelligence {
    pub content_type: String,
    pub focus_systems: Vec<String>,
    pub primary_guidance: ContentGenerationGuidance,
    pub system_specific_intelligence: HashMap<String, Value>,
    pub generation_timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS
// ============================================================================

impl Default for ContentGenerationGuidance {
    fn default() -> Self {
        Self {
            high_priority_content: vec!["Core gameplay".to_string()],
            medium_priority_content: vec!["Narrative enhancement".to_string()],
            low_priority_content: vec!["Polish and details".to_string()],
            recommended_themes: vec!["Horror progression".to_string()],
            avoid_themes: vec!["Comedy".to_string(), "Light-hearted content".to_string()],
            emotional_tone_guidance: "Growing dread and unease".to_string(),
            forge_content_guidance: None,
            trauma_content_guidance: None,
            philosophy_content_guidance: None,
            decay_content_guidance: None,
            dread_level_context: 0,
            horror_intensity_guidance: 0.3,
            complexity_level_guidance: 0.5,
            player_readiness_assessment: 0.5,
        }
    }
}

// ============================================================================
// INTELLIGENT GENERATION WORKFLOWS
// ============================================================================

impl IntelligentContentGenerator {
    /// Generate content guidance based on comprehensive game state analysis
    pub async fn generate_intelligent_content_guidance(
        &self,
        player_id: Uuid,
        content_type: String,
        target_systems: Vec<String>,
    ) -> Result<ContentGenerationGuidance> {
        // Get comprehensive intelligence
        let comprehensive = self.get_comprehensive_intelligence(player_id).await?;
        
        let mut guidance = ContentGenerationGuidance::default();
        
        // Set dread level context
        guidance.dread_level_context = comprehensive.game_state_intelligence.dread_level;
        guidance.horror_intensity_guidance = (comprehensive.game_state_intelligence.dread_level as f32) / 4.0;
        
        // Generate forge-specific guidance if needed
        if target_systems.contains(&"forge".to_string()) {
            guidance.forge_content_guidance = Some(ForgeContentGuidance {
                emphasize_trials: comprehensive.forge_intelligence.readiness_score < 0.5,
                emphasize_reagents: comprehensive.forge_intelligence.sentimental_reagent_count < 5,
                emphasize_sacrifice: comprehensive.forge_intelligence.sacrifice_candidate_count > 0,
                forge_path_bias: comprehensive.forge_intelligence.forge_path_chosen,
                trial_focus_areas: comprehensive.forge_intelligence.missing_requirements.clone(),
            });
        }
        
        // Generate trauma-specific guidance if needed
        if target_systems.contains(&"psychology".to_string()) {
            guidance.trauma_content_guidance = Some(TraumaContentGuidance {
                emphasize_therapy: comprehensive.trauma_intelligence.companions_in_therapy > 0,
                emphasize_support: comprehensive.trauma_intelligence.group_cohesion < 0.5,
                trauma_categories_to_address: vec!["Combat".to_string(), "Loss".to_string()], // TODO: Get from intelligence
                therapeutic_approaches_needed: vec!["Cognitive Behavioral Therapy".to_string()], // TODO: Get from intelligence
                relationship_repair_opportunities: vec!["Inter-companion support".to_string()], // TODO: Get from intelligence
            });
        }
        
        // Generate philosophy-specific guidance if needed
        if target_systems.contains(&"philosophy".to_string()) {
            guidance.philosophy_content_guidance = Some(PhilosophyContentGuidance {
                emphasize_identity_development: comprehensive.philosophy_intelligence.identity_stability < 0.7,
                emphasize_consistency_testing: comprehensive.philosophy_intelligence.authenticity_score < 0.6,
                philosophy_path_to_reinforce: Some(comprehensive.philosophy_intelligence.dominant_philosophy),
                trait_development_opportunities: vec!["Moral reasoning".to_string(), "Emotional resilience".to_string()], // TODO: Get from intelligence
                authenticity_challenge_level: 1.0 - comprehensive.philosophy_intelligence.authenticity_score,
            });
        }
        
        // Generate decay-specific guidance if needed
        if target_systems.contains(&"decay".to_string()) {
            let corruption_level = comprehensive.game_state_intelligence.corruption_level;
            guidance.decay_content_guidance = Some(DecayContentGuidance {
                emphasize_corruption_effects: corruption_level > 0.3,
                emphasize_social_isolation: corruption_level > 0.4,
                emphasize_economic_collapse: corruption_level > 0.5,
                emphasize_reality_distortion: corruption_level > 0.6,
                decay_stage_appropriate_content: match comprehensive.game_state_intelligence.dread_level {
                    0 => vec!["Beautiful world".to_string(), "Hope".to_string()],
                    1 => vec!["Subtle wrongness".to_string(), "Growing unease".to_string()],
                    2 => vec!["Visible corruption".to_string(), "Social breakdown".to_string()],
                    3 => vec!["Reality distortion".to_string(), "Economic collapse".to_string()],
                    4 => vec!["Complete breakdown".to_string(), "Survival horror".to_string()],
                    _ => vec!["Unknown state".to_string()],
                },
            });
        }
        
        // Set overall priorities based on game state
        guidance.high_priority_content = self.calculate_high_priority_content(&comprehensive);
        guidance.medium_priority_content = self.calculate_medium_priority_content(&comprehensive);
        guidance.low_priority_content = self.calculate_low_priority_content(&comprehensive);
        
        // Set emotional tone based on dread level and system states
        guidance.emotional_tone_guidance = match comprehensive.game_state_intelligence.dread_level {
            0 => "Hopeful and peaceful".to_string(),
            1 => "Subtle unease and growing concern".to_string(),
            2 => "Mounting dread and visible problems".to_string(),
            3 => "Terror and desperation".to_string(),
            4 => "Complete horror and survival".to_string(),
            _ => "Unknown emotional state".to_string(),
        };
        
        // Set complexity guidance based on player readiness
        guidance.complexity_level_guidance = (
            comprehensive.philosophy_intelligence.identity_stability * 0.3 +
            comprehensive.forge_intelligence.readiness_score * 0.3 +
            (1.0 - comprehensive.trauma_intelligence.average_trauma_level / 5.0) * 0.4
        ).min(1.0);
        
        guidance.player_readiness_assessment = guidance.complexity_level_guidance;
        
        Ok(guidance)
    }
    
    fn calculate_high_priority_content(&self, intelligence: &ComprehensiveIntelligence) -> Vec<String> {
        let mut priorities = Vec::new();
        
        // High priority: Systems that need immediate attention
        if intelligence.forge_intelligence.readiness_score < 0.3 {
            priorities.push("Forge trial preparation content".to_string());
        }
        
        if intelligence.trauma_intelligence.companions_at_breakdown_risk > 0 {
            priorities.push("Emergency trauma intervention content".to_string());
        }
        
        if intelligence.philosophy_intelligence.identity_stability < 0.4 {
            priorities.push("Identity stabilization content".to_string());
        }
        
        if intelligence.game_state_intelligence.corruption_level > 0.7 {
            priorities.push("Corruption response content".to_string());
        }
        
        priorities
    }
    
    fn calculate_medium_priority_content(&self, intelligence: &ComprehensiveIntelligence) -> Vec<String> {
        let mut priorities = Vec::new();
        
        // Medium priority: Systems that would benefit from attention
        if intelligence.trauma_intelligence.therapy_effectiveness < 0.6 {
            priorities.push("Therapy effectiveness improvement content".to_string());
        }
        
        if intelligence.philosophy_intelligence.authenticity_score < 0.7 {
            priorities.push("Philosophical authenticity testing content".to_string());
        }
        
        if intelligence.forge_intelligence.sentimental_reagent_count < 5 {
            priorities.push("Sentimental reagent collection content".to_string());
        }
        
        priorities
    }
    
    fn calculate_low_priority_content(&self, intelligence: &ComprehensiveIntelligence) -> Vec<String> {
        vec![
            "Polish and detail content".to_string(),
            "Optional side content".to_string(),
            "Aesthetic improvements".to_string(),
        ]
    }
}

// ============================================================================
// ERROR HANDLING
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum McpClientError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Request timeout after {timeout}s")]
    RequestTimeout { timeout: u64 },
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

// ============================================================================
// USAGE EXAMPLES FOR AI AGENTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Example usage for AI agents
    async fn example_intelligent_generation_workflow() -> Result<()> {
        let generator = IntelligentContentGenerator::with_default_config();
        
        // Test connection first
        if !generator.test_connection().await? {
            return Err(anyhow::anyhow!("MCP server not available"));
        }
        
        let player_id = Uuid::new_v4();
        
        // Get comprehensive intelligence for holistic content generation
        let intelligence = generator.get_comprehensive_intelligence(player_id).await?;
        
        // Generate intelligent content guidance
        let guidance = generator.generate_intelligent_content_guidance(
            player_id,
            "dialogue".to_string(),
            vec!["psychology".to_string(), "philosophy".to_string()]
        ).await?;
        
        // Use guidance to inform AI content generation
        tracing::info!("Content guidance: {}", guidance.emotional_tone_guidance);
        tracing::info!("High priority: {:?}", guidance.high_priority_content);
        
        // Example: Use trauma guidance for dialogue generation
        if let Some(trauma_guidance) = &guidance.trauma_content_guidance {
            if trauma_guidance.emphasize_therapy {
                tracing::info!("AI should generate therapy-focused dialogue");
            }
            
            for trauma_category in &trauma_guidance.trauma_categories_to_address {
                tracing::info!("AI should address {} trauma in dialogue", trauma_category);
            }
        }
        
        // Example: Use philosophy guidance for choice generation
        if let Some(philosophy_guidance) = &guidance.philosophy_content_guidance {
            if philosophy_guidance.emphasize_consistency_testing {
                tracing::info!("AI should generate choices that test philosophical consistency");
            }
            
            if let Some(philosophy_to_reinforce) = &philosophy_guidance.philosophy_path_to_reinforce {
                tracing::info!("AI should create content that reinforces {} philosophy", philosophy_to_reinforce);
            }
        }
        
        Ok(())
    }
    
    // ============================================================================
    // AGENT WRAPPER METHODS - For compatibility with agents
    // ============================================================================
    
    /// Wrapper method for agents - query companion trauma states
    pub async fn query_companion_trauma_states(&self) -> Result<Value> {
        // Call the actual MCP server method added for agents
        let request = json!({
            "jsonrpc": "2.0",
            "method": "query_companion_trauma_states",
            "params": {},
            "id": 101
        });
        
        let response = self.send_request(request).await?;
        Ok(response)
    }
    
    /// Wrapper method for agents - query world corruption level
    pub async fn query_world_corruption_level(&self) -> Result<f32> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "query_world_corruption_level", 
            "params": {},
            "id": 102
        });
        
        let response = self.send_request(request).await?;
        let corruption_level = response.as_f64().unwrap_or(0.0) as f32;
        Ok(corruption_level)
    }
    
    /// Wrapper method for agents - query NPC fear states
    pub async fn query_npc_fear_states(&self) -> Result<HashMap<String, f32>> {
        let request = json!({
            "jsonrpc": "2.0", 
            "method": "query_npc_fear_states",
            "params": {},
            "id": 103
        });
        
        let response = self.send_request(request).await?;
        let npc_states: HashMap<String, f32> = serde_json::from_value(response)
            .context("Failed to parse NPC fear states")?;
        Ok(npc_states)
    }
    
    /// Wrapper method for agents - query philosophical progression  
    pub async fn query_philosophical_progression(&self) -> Result<HashMap<String, f32>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "query_philosophical_progression",
            "params": {},
            "id": 104
        });
        
        let response = self.send_request(request).await?;
        let progression: HashMap<String, f32> = serde_json::from_value(response)
            .context("Failed to parse philosophical progression")?;
        Ok(progression)
    }
    
    /// Wrapper method for agents - query companion states
    pub async fn query_companion_states(&self) -> Result<HashMap<String, String>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "query_companion_states", 
            "params": {},
            "id": 105
        });
        
        let response = self.send_request(request).await?;
        let states: HashMap<String, String> = serde_json::from_value(response)
            .context("Failed to parse companion states")?;
        Ok(states)
    }
    
    /// Wrapper method for agents - query forge readiness
    pub async fn query_forge_readiness(&self) -> Result<HashMap<String, f32>> {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "query_forge_readiness",
            "params": {},
            "id": 106
        });
        
        let response = self.send_request(request).await?;
        let readiness: HashMap<String, f32> = serde_json::from_value(response)
            .context("Failed to parse forge readiness")?;
        Ok(readiness)
    }
}

// Default implementation for fallback mode when MCP server is not available
impl Default for GameDatabaseMcpClient {
    fn default() -> Self {
        Self::new(McpClientConfig::default())
    }
}

pub type MCPClient = GameDatabaseMcpClient;
