"""
UI Agent - AI-powered horror-responsive interface generation for Dragon's Labyrinth
Uses OpenAI to generate UI degradation patterns and horror elements
"""

import os
import json
import sqlite3
from pathlib import Path
from typing import Optional
from dataclasses import dataclass, asdict
from datetime import datetime

from openai import OpenAI


@dataclass
class UIConfig:
    """Configuration for UI generation"""
    ui_element: str  # health_bar, inventory, compass, etc.
    dread_level: int  # 0-4
    degradation_type: str = "visual"  # visual, functional, both
    companion_trauma: float = 0.0  # 0.0-1.0
    sanity_level: float = 1.0  # 1.0-0.0


class UIAgent:
    """AI agent for generating horror-responsive UI elements"""
    
    def __init__(self, db_path: str = "assets/assets.db"):
        self.client = OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))
        self.db_path = db_path
        self._init_database()
    
    def _init_database(self):
        """Initialize UI generation tracking"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS ui_generations (
                generation_id TEXT PRIMARY KEY,
                ui_element TEXT NOT NULL,
                dread_level INTEGER,
                prompt TEXT,
                response TEXT,
                ui_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
    
    def generate_ui_element(self, config: UIConfig) -> dict[str, any]:
        """
        Generate UI element with horror degradation using AI
        
        Returns:
            Dictionary with UI specifications and degradation patterns
        """
        # Create horror-aware prompt
        prompt = self._create_ui_prompt(config)
        
        try:
            # Call OpenAI for UI generation
            response = self.client.chat.completions.create(
                model="gpt-4o",
                messages=[
                    {"role": "system", "content": self._get_system_prompt()},
                    {"role": "user", "content": prompt}
                ],
                temperature=0.7,
                response_format={"type": "json_object"}
            )
            
            # Parse response
            ui_data = json.loads(response.choices[0].message.content)
            
            # Record generation
            generation_id = f"ui_{config.ui_element}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._record_generation(generation_id, config, prompt, response.choices[0].message.content, ui_data)
            
            return {
                "success": True,
                "generation_id": generation_id,
                "ui_data": ui_data,
                "config": asdict(config)
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "config": asdict(config)
            }
    
    def _get_system_prompt(self) -> str:
        """Get system prompt for UI generation"""
        return """You are a UI designer for a horror RPG called Dragon's Labyrinth.
        
        The game has 5 dread levels (0-4):
        - 0: Peace - Clean, functional UI
        - 1: Unease - Subtle glitches, minor distortions
        - 2: Dread - Visible corruption, functionality degrading
        - 3: Terror - Major distortions, UI becoming unreliable
        - 4: Horror - UI almost unusable, nightmare mode
        
        UI degradation types:
        - Visual: Cracks, blood, corruption overlays, flickering
        - Functional: Wrong values, delayed updates, false information
        - Both: Complete breakdown of UI reliability
        
        Generate UI specifications that reflect the current dread level.
        
        Return JSON with:
        {
            "base_design": {
                "layout": "...",
                "colors": {...},
                "typography": {...},
                "animations": [...]
            },
            "degradation_effects": [
                {
                    "trigger": "dread_level_2",
                    "effect_type": "visual",
                    "description": "Blood seeps from corners",
                    "implementation": "overlay_texture",
                    "intensity": 0.3
                }
            ],
            "sanity_effects": [
                {
                    "sanity_threshold": 0.5,
                    "effect": "false_readings",
                    "description": "Health bar shows wrong values randomly"
                }
            ],
            "companion_reactions": {
                "einar": "Health bar turns red when he's violent",
                "mira": "Healing items grayed out when she leaves",
                "sorin": "Map shows false locations after betrayal",
                "tamara": "Inventory items appear corrupted"
            },
            "cobweb_config": {
                "component_type": "...",
                "reactive_properties": [...],
                "horror_bindings": [...]
            }
        }
        """
    
    def _create_ui_prompt(self, config: UIConfig) -> str:
        """Create prompt for UI generation"""
        ui_elements = {
            "health_bar": "Player health indicator",
            "inventory": "Item management interface",
            "compass": "Navigation tool",
            "dialogue_box": "Conversation interface",
            "quest_log": "Objective tracker",
            "companion_status": "Companion health and mood",
            "sanity_meter": "Mental state indicator",
            "proximity_warning": "Dragon distance indicator"
        }
        
        dread_descriptions = {
            0: "clean and fully functional",
            1: "slightly unsettling with minor glitches",
            2: "visibly corrupted with degrading functionality",
            3: "severely distorted and unreliable",
            4: "nightmare state, barely usable"
        }
        
        return f"""Generate UI specifications for Dragon's Labyrinth.
        
        UI Element: {config.ui_element} - {ui_elements.get(config.ui_element, 'Custom element')}
        Dread Level: {config.dread_level} - {dread_descriptions[config.dread_level]}
        Degradation Type: {config.degradation_type}
        Companion Trauma: {config.companion_trauma:.1%}
        Sanity Level: {config.sanity_level:.1%}
        
        Create UI design that:
        1. Starts clean at dread 0 and degrades progressively
        2. Shows {config.degradation_type} degradation at current level
        3. Reflects companion trauma level in visual/functional changes
        4. Incorporates sanity effects below {config.sanity_level}
        5. Uses Cobweb UI reactive patterns for horror progression
        
        Include specific implementation details for Cobweb UI.
        """
    
    def _record_generation(self, generation_id: str, config: UIConfig, prompt: str, response: str, ui_data: dict):
        """Record generation in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO ui_generations 
            (generation_id, ui_element, dread_level, prompt, response, ui_data)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (generation_id, config.ui_element, config.dread_level, prompt, response, json.dumps(ui_data)))
        
        conn.commit()
        conn.close()
    
    def generate_cobweb_component(self, ui_data: dict, output_path: str = "src/ui/generated/") -> str:
        """
        Generate Cobweb UI component code
        
        Returns:
            Path to generated Rust component file
        """
        Path(output_path).mkdir(parents=True, exist_ok=True)
        
        element_name = ui_data.get("config", {}).get("ui_element", "unknown")
        safe_name = element_name.lower().replace("_", "")
        
        rust_code = f'''// Auto-generated Cobweb UI component for {element_name}
use bevy::prelude::*;
use cobweb_ui::prelude::*;

#[derive(Component)]
pub struct {safe_name.capitalize()}UI {{
    dread_level: u8,
    degradation_active: bool,
    sanity_level: f32,
}}

pub fn spawn_{safe_name}_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    dread: Res<DreadState>,
) {{
    commands.spawn((
        NodeBundle {{
            style: Style {{
                position_type: PositionType::Absolute,
                ..default()
            }},
            ..default()
        }},
        {safe_name.capitalize()}UI {{
            dread_level: dread.level,
            degradation_active: dread.level > 0,
            sanity_level: 1.0,
        }},
    )).with_children(|parent| {{
        // Base UI structure
        parent.spawn(NodeBundle {{
            style: Style {{
                width: Val::Px(200.0),
                height: Val::Px(50.0),
                ..default()
            }},
            background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
            ..default()
        }});
    }});
}}

pub fn update_{safe_name}_degradation(
    mut query: Query<(&mut {safe_name.capitalize()}UI, &mut BackgroundColor)>,
    dread: Res<DreadState>,
    time: Res<Time>,
) {{
    for (mut ui, mut bg_color) in query.iter_mut() {{
        if ui.dread_level != dread.level {{
            ui.dread_level = dread.level;
            ui.degradation_active = dread.level > 0;
        }}
        
        // Apply dread-based degradation
        match dread.level {{
            0 => {{
                // Clean state
                *bg_color = Color::srgba(0.1, 0.1, 0.1, 0.8).into();
            }},
            1 => {{
                // Subtle flicker
                let flicker = (time.elapsed_secs() * 10.0).sin() * 0.05;
                *bg_color = Color::srgba(0.1, 0.1 - flicker, 0.1, 0.8).into();
            }},
            2 => {{
                // Corruption spreading
                let corruption = (time.elapsed_secs() * 2.0).sin() * 0.2;
                *bg_color = Color::srgba(0.2 + corruption, 0.05, 0.1, 0.85).into();
            }},
            3 => {{
                // Major distortion
                let distortion = (time.elapsed_secs() * 5.0).sin() * 0.3;
                *bg_color = Color::srgba(0.3 + distortion, 0.0, 0.05, 0.9).into();
            }},
            4 => {{
                // Nightmare mode
                let nightmare = (time.elapsed_secs() * 20.0).sin().abs();
                *bg_color = Color::srgba(nightmare, 0.0, 0.0, 0.95).into();
            }},
            _ => {{}}
        }}
    }}
}}
'''
        
        # Write to file
        file_path = Path(output_path) / f"{safe_name}_ui.rs"
        file_path.write_text(rust_code)
        
        return str(file_path)
    
    def generate_ui_assets_batch(self, dread_level: int) -> list[dict]:
        """
        Generate all UI elements for a specific dread level
        
        Returns:
            List of generation results
        """
        ui_elements = [
            "health_bar",
            "inventory",
            "compass",
            "dialogue_box",
            "quest_log",
            "companion_status",
            "sanity_meter",
            "proximity_warning"
        ]
        
        results = []
        for element in ui_elements:
            config = UIConfig(
                ui_element=element,
                dread_level=dread_level,
                degradation_type="both" if dread_level >= 3 else "visual"
            )
            
            result = self.generate_ui_element(config)
            results.append(result)
            
            if result["success"]:
                # Generate Cobweb component
                component_path = self.generate_cobweb_component(result)
                result["component_path"] = component_path
        
        return results
