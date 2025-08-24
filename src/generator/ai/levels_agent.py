"""
Levels Agent - AI-powered encounter and object placement for Dragon's Labyrinth
Uses OpenAI to generate level designs with horror progression
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
class LevelConfig:
    """Configuration for level generation"""
    level_name: str
    dread_level: int  # 0-4
    encounter_density: float = 0.3  # 0.0-1.0
    boss_encounter: bool = False
    companion_present: str = None  # einar, mira, sorin, tamara
    moral_choices: int = 0  # Number of moral decision points


class LevelsAgent:
    """AI agent for generating level encounters and objects"""
    
    def __init__(self, db_path: str = "assets/assets.db"):
        self.client = OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))
        self.db_path = db_path
        self._init_database()
    
    def _init_database(self):
        """Initialize levels generation tracking"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS level_generations (
                generation_id TEXT PRIMARY KEY,
                level_name TEXT NOT NULL,
                dread_level INTEGER,
                prompt TEXT,
                response TEXT,
                encounter_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
    
    def generate_level_encounters(self, config: LevelConfig, world_data: dict = None) -> dict[str, any]:
        """
        Generate level encounters and objects using AI
        
        Args:
            config: Level configuration
            world_data: Optional world data from MapsAgent
        
        Returns:
            Dictionary with encounter placements and objects
        """
        # Create horror-aware prompt
        prompt = self._create_level_prompt(config, world_data)
        
        try:
            # Call OpenAI for level generation
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
            level_data = json.loads(response.choices[0].message.content)
            
            # Record generation
            generation_id = f"level_{config.level_name}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._record_generation(generation_id, config, prompt, response.choices[0].message.content, level_data)
            
            return {
                "success": True,
                "generation_id": generation_id,
                "level_data": level_data,
                "config": asdict(config)
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "config": asdict(config)
            }
    
    def _get_system_prompt(self) -> str:
        """Get system prompt for level generation"""
        return """You are a level designer for a horror RPG called Dragon's Labyrinth.
        
        The game has 5 dread levels (0-4):
        - 0: Peace - Safe encounters, helpful NPCs
        - 1: Unease - Strange behaviors, unsettling events
        - 2: Dread - Hostile encounters, fleeing NPCs
        - 3: Terror - Reality distortions, companion trauma
        - 4: Horror - Dragon presence, survival mode
        
        Companions and their arcs:
        - Einar: Loyal warrior, trauma makes him violent
        - Mira: Optimist healer, loses hope and leaves
        - Sorin: Scholar, betrays for knowledge
        - Tamara: Young baker, loses innocence
        
        Generate level encounters that reflect the current dread level.
        Each encounter should have:
        - position: hex coordinates (q, r)
        - type: combat, dialogue, puzzle, moral_choice, environmental
        - entities: list of NPCs/enemies
        - companion_reaction: how companion responds
        
        Return JSON with:
        {
            "encounters": [
                {
                    "id": "enc_001",
                    "position": {"q": 0, "r": 0},
                    "type": "dialogue",
                    "entities": ["villager_scared"],
                    "description": "...",
                    "companion_reaction": "...",
                    "moral_choice": null or {...}
                }
            ],
            "interactive_objects": [
                {
                    "id": "obj_001",
                    "position": {"q": 1, "r": -1},
                    "type": "chest",
                    "contents": ["healing_potion"],
                    "corruption_state": 0.0
                }
            ],
            "narrative_triggers": [
                {
                    "id": "trig_001",
                    "position": {"q": 2, "r": 0},
                    "condition": "enter_hex",
                    "effect": "companion_dialogue",
                    "content": "..."
                }
            ],
            "horror_elements": {
                "ambient_sounds": ["whispers", "breathing"],
                "visual_distortions": ["shadow_figures"],
                "false_positives": 2
            }
        }
        """
    
    def _create_level_prompt(self, config: LevelConfig, world_data: dict = None) -> str:
        """Create prompt for level generation"""
        dread_descriptions = {
            0: "peaceful exploration with helpful encounters",
            1: "subtle unease with NPCs acting strangely",
            2: "active danger with hostile encounters and fleeing villagers",
            3: "reality breaking with companion trauma and betrayals",
            4: "pure survival horror with dragon hunting the player"
        }
        
        companion_states = {
            "einar": {
                0: "protective and jovial",
                1: "tense and watchful",
                2: "aggressive and paranoid",
                3: "violent and unstable",
                4: "completely broken or dead"
            },
            "mira": {
                0: "cheerful and healing",
                1: "worried but hopeful",
                2: "desperate to help",
                3: "losing all hope",
                4: "gone, left a farewell note"
            },
            "sorin": {
                0: "curious and helpful",
                1: "obsessed with ancient texts",
                2: "hiding something",
                3: "making dark bargains",
                4: "fully betrayed the party"
            },
            "tamara": {
                0: "innocent and eager",
                1: "frightened but brave",
                2: "traumatized by violence",
                3: "hardened and cold",
                4: "catatonic or missing"
            }
        }
        
        companion_state = ""
        if config.companion_present:
            state = companion_states.get(config.companion_present, {}).get(config.dread_level, "")
            companion_state = f"\nCompanion {config.companion_present.capitalize()} is present and {state}."
        
        world_context = ""
        if world_data:
            world_context = f"\nWorld context: {world_data.get('narrative_notes', 'Unknown world state')}"
        
        return f"""Generate level encounters for Dragon's Labyrinth.
        
        Level Name: {config.level_name}
        Dread Level: {config.dread_level} - {dread_descriptions[config.dread_level]}
        Encounter Density: {config.encounter_density} (0=sparse, 1=dense)
        Boss Encounter: {config.boss_encounter}
        Moral Choices: {config.moral_choices} decision points{companion_state}{world_context}
        
        Create encounters that:
        1. Reflect the current dread level in type and intensity
        2. Show companion degradation if present
        3. Include {config.moral_choices} meaningful moral choices
        4. Build horror through environmental storytelling
        5. Use false positives and misdirection at higher dread levels
        
        Generate 10-20 encounters based on density.
        """
    
    def _record_generation(self, generation_id: str, config: LevelConfig, prompt: str, response: str, level_data: dict):
        """Record generation in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO level_generations 
            (generation_id, level_name, dread_level, prompt, response, encounter_data)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (generation_id, config.level_name, config.dread_level, prompt, response, json.dumps(level_data)))
        
        conn.commit()
        conn.close()
    
    def generate_yoleck_config(self, level_data: dict, output_path: str = "assets/generated/levels/") -> str:
        """
        Generate Yoleck-compatible level file
        
        Returns:
            Path to generated level file
        """
        Path(output_path).mkdir(parents=True, exist_ok=True)
        
        level_name = level_data.get("config", {}).get("level_name", "unnamed")
        safe_name = level_name.lower().replace(" ", "_").replace("-", "_")
        
        # Create Yoleck format
        yoleck_data = {
            "format_version": 1,
            "level_name": level_name,
            "entities": []
        }
        
        # Add encounters as entities
        for enc in level_data.get("level_data", {}).get("encounters", []):
            entity = {
                "type": "Encounter",
                "name": enc.get("id"),
                "position": enc.get("position"),
                "data": {
                    "encounter_type": enc.get("type"),
                    "entities": enc.get("entities", []),
                    "description": enc.get("description", ""),
                    "companion_reaction": enc.get("companion_reaction", ""),
                    "moral_choice": enc.get("moral_choice")
                }
            }
            yoleck_data["entities"].append(entity)
        
        # Add interactive objects
        for obj in level_data.get("level_data", {}).get("interactive_objects", []):
            entity = {
                "type": "InteractiveObject",
                "name": obj.get("id"),
                "position": obj.get("position"),
                "data": {
                    "object_type": obj.get("type"),
                    "contents": obj.get("contents", []),
                    "corruption_state": obj.get("corruption_state", 0.0)
                }
            }
            yoleck_data["entities"].append(entity)
        
        # Add narrative triggers
        for trig in level_data.get("level_data", {}).get("narrative_triggers", []):
            entity = {
                "type": "NarrativeTrigger",
                "name": trig.get("id"),
                "position": trig.get("position"),
                "data": {
                    "condition": trig.get("condition"),
                    "effect": trig.get("effect"),
                    "content": trig.get("content")
                }
            }
            yoleck_data["entities"].append(entity)
        
        # Write to file
        file_path = Path(output_path) / f"{safe_name}.yol"
        with open(file_path, 'w') as f:
            json.dump(yoleck_data, f, indent=2)
        
        return str(file_path)
