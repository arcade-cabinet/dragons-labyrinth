use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;

use crate::mcp_client::MCPClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub dread_level: u8,
    pub opacity: f32,
    pub corruption_overlay: f32,
    pub trauma_indicators: Vec<TraumaIndicator>,
    pub interface_elements: HashMap<String, InterfaceElement>,
    pub color_scheme: ColorScheme,
    pub degradation_effects: Vec<DegradationEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaIndicator {
    pub companion_id: String,
    pub trauma_level: f32,
    pub visual_state: String,
    pub position: (f32, f32),
    pub opacity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceElement {
    pub element_type: String,
    pub visibility: f32,
    pub corruption_level: f32,
    pub interactive: bool,
    pub dread_responsive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub primary: String,
    pub secondary: String,
    pub text: String,
    pub warning: String,
    pub error: String,
    pub corruption_tint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationEffect {
    pub effect_type: String,
    pub intensity: f32,
    pub trigger_conditions: Vec<String>,
    pub visual_impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraumaStates {
    pub companions: Vec<CompanionTrauma>,
    pub world_corruption: f32,
    pub player_stress: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionTrauma {
    pub companion_id: String,
    pub trauma_level: f32,
    pub breakdown_risk: f32,
    pub current_state: String,
}

pub struct UIAgent {
    dread_level: u8,
    out_dir: PathBuf,
    mcp_client: MCPClient,
}

impl UIAgent {
    pub fn new(dread_level: u8, out_dir: &std::path::Path, mcp_client: MCPClient) -> Self {
        Self {
            dread_level,
            out_dir: out_dir.to_path_buf(),
            mcp_client,
        }
    }
    
    pub async fn generate(&self) -> Result<UIConfig> {
        println!("UIAgent: Generating UI configuration for dread level {}", self.dread_level);
        
        // Query game state via MCP client
        let trauma_states = self.query_companion_trauma().await?;
        let corruption = self.query_world_corruption().await?;
        
        // Generate horror-responsive UI configuration
        let ui_config = self.generate_dread_ui(trauma_states, corruption)?;
        
        // Save to OUT_DIR
        let ui_dir = self.out_dir.join("ui");
        std::fs::create_dir_all(&ui_dir)?;
        let ui_path = ui_dir.join(format!("dread_{}.toml", self.dread_level));
        std::fs::write(ui_path, toml::to_string(&ui_config)?)?;
        
        println!("UIAgent: Generated UI config for dread level {} with {} trauma indicators", 
                self.dread_level, ui_config.trauma_indicators.len());
        
        Ok(ui_config)
    }
    
    async fn query_companion_trauma(&self) -> Result<TraumaStates> {
        // Try to query via MCP, fallback to generated data
        match self.mcp_client.query_companion_trauma_states().await {
            Ok(states) => Ok(states),
            Err(_) => {
                // Fallback to generated trauma states based on dread level
                Ok(self.generate_fallback_trauma_states())
            }
        }
    }
    
    async fn query_world_corruption(&self) -> Result<f32> {
        // Try to query via MCP, fallback to dread-based calculation
        match self.mcp_client.query_world_corruption_level().await {
            Ok(level) => Ok(level),
            Err(_) => {
                // Fallback calculation based on dread level
                Ok(self.dread_level as f32 * 0.2)
            }
        }
    }
    
    fn generate_fallback_trauma_states(&self) -> TraumaStates {
        let companions = vec![
            CompanionTrauma {
                companion_id: "einar".to_string(),
                trauma_level: self.dread_level as f32 * 0.15,
                breakdown_risk: (self.dread_level as f32 * 0.1).min(1.0),
                current_state: self.get_companion_state("einar"),
            },
            CompanionTrauma {
                companion_id: "mira".to_string(),
                trauma_level: self.dread_level as f32 * 0.25,
                breakdown_risk: (self.dread_level as f32 * 0.2).min(1.0),
                current_state: self.get_companion_state("mira"),
            },
            CompanionTrauma {
                companion_id: "sorin".to_string(),
                trauma_level: self.dread_level as f32 * 0.3,
                breakdown_risk: (self.dread_level as f32 * 0.25).min(1.0),
                current_state: self.get_companion_state("sorin"),
            },
            CompanionTrauma {
                companion_id: "tamara".to_string(),
                trauma_level: self.dread_level as f32 * 0.2,
                breakdown_risk: (self.dread_level as f32 * 0.15).min(1.0),
                current_state: self.get_companion_state("tamara"),
            },
        ];
        
        TraumaStates {
            companions,
            world_corruption: self.dread_level as f32 * 0.2,
            player_stress: self.dread_level as f32 * 0.18,
        }
    }
    
    fn get_companion_state(&self, companion_id: &str) -> String {
        match (companion_id, self.dread_level) {
            (_, 0) => "stable".to_string(),
            (_, 1) => "uneasy".to_string(),
            ("mira", 2) => "questioning".to_string(),
            ("sorin", 2) => "analytical".to_string(),
            (_, 2) => "worried".to_string(),
            ("mira", 3) => "fearful".to_string(),
            ("sorin", 3) => "suspicious".to_string(),
            (_, 3) => "stressed".to_string(),
            ("mira", 4) => "fled".to_string(),
            ("sorin", 4) => "hostile".to_string(),
            (_, 4) => "broken".to_string(),
        }
    }
    
    fn generate_dread_ui(&self, trauma: TraumaStates, corruption: f32) -> Result<UIConfig> {
        // Base configuration that degrades with dread level
        let base_opacity = 1.0 - (self.dread_level as f32 * 0.15);
        let corruption_overlay = corruption.min(1.0);
        
        // Generate trauma indicators for each companion
        let trauma_indicators = trauma.companions.iter().enumerate().map(|(i, companion)| {
            TraumaIndicator {
                companion_id: companion.companion_id.clone(),
                trauma_level: companion.trauma_level,
                visual_state: companion.current_state.clone(),
                position: (20.0 + (i as f32 * 60.0), 20.0), // Spread across top
                opacity: (1.0 - companion.trauma_level).max(0.1),
            }
        }).collect();
        
        // Generate interface elements with dread-responsive degradation
        let mut interface_elements = HashMap::new();
        
        // Health UI - becomes more unreliable as dread increases
        interface_elements.insert("health_bar".to_string(), InterfaceElement {
            element_type: "health_display".to_string(),
            visibility: base_opacity,
            corruption_level: corruption * 0.5,
            interactive: self.dread_level < 4, // Non-interactive in horror stage
            dread_responsive: true,
        });
        
        // Companion portraits - fade as they break down
        for companion in &trauma.companions {
            interface_elements.insert(
                format!("portrait_{}", companion.companion_id),
                InterfaceElement {
                    element_type: "companion_portrait".to_string(),
                    visibility: (1.0 - companion.trauma_level).max(0.0),
                    corruption_level: companion.trauma_level,
                    interactive: companion.trauma_level < 0.8,
                    dread_responsive: true,
                }
            );
        }
        
        // Inventory UI - becomes corrupted
        interface_elements.insert("inventory".to_string(), InterfaceElement {
            element_type: "inventory_panel".to_string(),
            visibility: base_opacity * 0.9,
            corruption_level: corruption * 0.3,
            interactive: true,
            dread_responsive: true,
        });
        
        // Map UI - becomes less reliable
        interface_elements.insert("map".to_string(), InterfaceElement {
            element_type: "world_map".to_string(),
            visibility: (base_opacity * 0.8).max(0.3),
            corruption_level: corruption * 0.6,
            interactive: self.dread_level < 3,
            dread_responsive: true,
        });
        
        // Generate color scheme that shifts toward horror
        let color_scheme = self.generate_horror_color_scheme();
        
        // Generate degradation effects
        let degradation_effects = self.generate_degradation_effects(corruption);
        
        Ok(UIConfig {
            dread_level: self.dread_level,
            opacity: base_opacity,
            corruption_overlay,
            trauma_indicators,
            interface_elements,
            color_scheme,
            degradation_effects,
        })
    }
    
    fn generate_horror_color_scheme(&self) -> ColorScheme {
        match self.dread_level {
            0 => ColorScheme {
                primary: "#4A90E2".to_string(),      // Bright blue
                secondary: "#7ED321".to_string(),    // Green
                text: "#333333".to_string(),         // Dark gray
                warning: "#F5A623".to_string(),      // Orange
                error: "#D0021B".to_string(),        // Red
                corruption_tint: "#000000".to_string(), // No tint
            },
            1 => ColorScheme {
                primary: "#6B5B95".to_string(),      // Muted purple
                secondary: "#88C999".to_string(),    // Muted green
                text: "#444444".to_string(),         // Darker gray
                warning: "#E67E22".to_string(),      // Darker orange
                error: "#C0392B".to_string(),        // Darker red
                corruption_tint: "#1A0F1A".to_string(), // Very dark purple tint
            },
            2 => ColorScheme {
                primary: "#8B7D8B".to_string(),      // Gray-purple
                secondary: "#7A8471".to_string(),    // Gray-green
                text: "#555555".to_string(),         // Medium gray
                warning: "#D68910".to_string(),      // Amber
                error: "#A93226".to_string(),        // Dark red
                corruption_tint: "#2D1B2D".to_string(), // Dark purple tint
            },
            3 => ColorScheme {
                primary: "#704A4A".to_string(),      // Dark brownish
                secondary: "#5D6A5D".to_string(),    // Dark green-gray
                text: "#666666".to_string(),         // Darker gray
                warning: "#B7950B".to_string(),      // Dark yellow
                error: "#922B21".to_string(),        // Very dark red
                corruption_tint: "#3D2B3D".to_string(), // Purple-black tint
            },
            4 => ColorScheme {
                primary: "#2C1810".to_string(),      // Very dark brown
                secondary: "#1F2A1F".to_string(),    // Very dark green
                text: "#777777".to_string(),         // Light gray on dark
                warning: "#856404".to_string(),      // Very dark yellow
                error: "#7B241C".to_string(),        // Very dark red
                corruption_tint: "#4D3B4D".to_string(), // Dark corruption overlay
            },
            _ => ColorScheme {
                primary: "#000000".to_string(),
                secondary: "#111111".to_string(),
                text: "#888888".to_string(),
                warning: "#444444".to_string(),
                error: "#222222".to_string(),
                corruption_tint: "#000000".to_string(),
            }
        }
    }
    
    fn generate_degradation_effects(&self, corruption: f32) -> Vec<DegradationEffect> {
        let mut effects = Vec::new();
        
        if self.dread_level >= 1 {
            effects.push(DegradationEffect {
                effect_type: "texture_corruption".to_string(),
                intensity: corruption * 0.3,
                trigger_conditions: vec!["always".to_string()],
                visual_impact: "adds noise to UI textures".to_string(),
            });
        }
        
        if self.dread_level >= 2 {
            effects.push(DegradationEffect {
                effect_type: "flicker".to_string(),
                intensity: (corruption * 0.4).min(0.6),
                trigger_conditions: vec!["companion_stress_high".to_string()],
                visual_impact: "interface elements flicker".to_string(),
            });
        }
        
        if self.dread_level >= 3 {
            effects.push(DegradationEffect {
                effect_type: "false_information".to_string(),
                intensity: corruption * 0.5,
                trigger_conditions: vec!["world_corruption_high".to_string()],
                visual_impact: "displays incorrect health/status".to_string(),
            });
        }
        
        if self.dread_level >= 4 {
            effects.push(DegradationEffect {
                effect_type: "interface_breakdown".to_string(),
                intensity: corruption * 0.8,
                trigger_conditions: vec!["horror_stage".to_string()],
                visual_impact: "UI elements randomly disappear or move".to_string(),
            });
        }
        
        effects
    }
}