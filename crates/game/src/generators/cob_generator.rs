// Cobweb UI .cob file generator
// Generates reactive UI scenes based on game state

use super::ContentGenerator;
use serde::{Deserialize, Serialize};

pub struct CobGenerator;

impl CobGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_biome_ui(&self, biome: &str, dread_level: u8) -> Vec<String> {
        vec![
            self.generate_hud(biome, dread_level),
            self.generate_inventory_ui(dread_level),
            self.generate_dialogue_ui(dread_level),
        ]
    }
    
    fn generate_hud(&self, biome: &str, dread_level: u8) -> String {
        format!(r#"
// HUD for {} at dread level {}
#[cob_scene]
{{
    "root": {{
        "type": "Container",
        "style": {{
            "position_type": "Absolute",
            "width": "100%",
            "height": "100%"
        }},
        "children": [
            {{
                "type": "Panel",
                "id": "health_bar",
                "style": {{
                    "position": [10, 10],
                    "width": 200,
                    "height": 30,
                    "background_color": {}
                }},
                "reactive": {{
                    "bind": "player_health",
                    "update": "width_percent"
                }}
            }},
            {{
                "type": "Text",
                "id": "dread_indicator",
                "text": "{}",
                "style": {{
                    "position": ["50%", 20],
                    "font_size": {},
                    "color": {}
                }}
            }}
        ]
    }}
}}
"#, 
            biome, 
            dread_level,
            self.get_health_color(dread_level),
            self.get_dread_text(dread_level),
            20 + dread_level * 2,
            self.get_dread_color(dread_level)
        )
    }
    
    fn generate_inventory_ui(&self, dread_level: u8) -> String {
        format!(r#"
#[cob_scene]
{{
    "root": {{
        "type": "Grid",
        "columns": 5,
        "spacing": 10,
        "style": {{
            "padding": 20,
            "background_color": "{}"
        }},
        "children": []
    }}
}}
"#, self.get_ui_background(dread_level))
    }
    
    fn generate_dialogue_ui(&self, dread_level: u8) -> String {
        format!(r#"
#[cob_scene]
{{
    "root": {{
        "type": "DialogueBox",
        "style": {{
            "position": ["center", "bottom"],
            "width": "80%",
            "max_height": "30%",
            "background_color": "{}",
            "border_width": {},
            "animation": "{}"
        }}
    }}
}}
"#, 
            self.get_dialogue_bg(dread_level),
            1 + dread_level,
            if dread_level > 2 { "shake" } else { "fade_in" }
        )
    }
    
    fn get_health_color(&self, dread: u8) -> &str {
        match dread {
            0 => "[0, 255, 0, 255]",     // Green
            1 => "[255, 255, 0, 255]",   // Yellow
            2 => "[255, 128, 0, 255]",   // Orange
            3 => "[255, 0, 0, 255]",     // Red
            _ => "[128, 0, 128, 255]",   // Purple
        }
    }
    
    fn get_dread_text(&self, dread: u8) -> &str {
        match dread {
            0 => "Peace",
            1 => "Unease",
            2 => "Dread",
            3 => "Terror",
            _ => "HORROR",
        }
    }
    
    fn get_dread_color(&self, dread: u8) -> &str {
        match dread {
            0..=1 => "[255, 255, 255, 255]",
            2..=3 => "[255, 200, 200, 255]",
            _ => "[255, 0, 0, 255]",
        }
    }
    
    fn get_ui_background(&self, dread: u8) -> &str {
        match dread {
            0 => "[240, 240, 240, 200]",
            1 => "[220, 220, 220, 200]",
            2 => "[180, 180, 180, 200]",
            3 => "[120, 120, 120, 200]",
            _ => "[40, 40, 40, 200]",
        }
    }
    
    fn get_dialogue_bg(&self, dread: u8) -> &str {
        match dread {
            0 => "[255, 255, 255, 230]",
            1 => "[240, 240, 240, 230]",
            2 => "[200, 200, 200, 230]",
            3 => "[150, 150, 150, 230]",
            _ => "[50, 50, 50, 230]",
        }
    }
}

impl ContentGenerator for CobGenerator {
    type Output = String;
    
    fn generate(&self, prompt: &str, dread_level: u8) -> Self::Output {
        self.generate_hud(prompt, dread_level)
    }
    
    fn validate(&self, content: &Self::Output) -> Result<(), String> {
        if content.contains("#[cob_scene]") {
            Ok(())
        } else {
            Err("Invalid .cob format".to_string())
        }
    }
}