"""
Dialogue Agent - AI-powered narrative dialogue generation for Dragon's Labyrinth
Uses OpenAI to generate YarnSpinner dialogue trees with horror progression
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
class DialogueConfig:
    """Configuration for dialogue generation"""
    character_name: str  # einar, mira, sorin, tamara, villager, etc.
    dread_level: int  # 0-4
    trauma_level: float = 0.0  # 0.0-1.0 for companions
    dialogue_type: str = "conversation"  # conversation, monologue, moral_choice, farewell
    emotional_state: str = "neutral"  # neutral, scared, angry, hopeful, broken
    context: str = ""  # Additional context for the dialogue


class DialogueAgent:
    """AI agent for generating character dialogue"""
    
    def __init__(self, db_path: str = "assets/assets.db"):
        self.client = OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))
        self.db_path = db_path
        self._init_database()
    
    def _init_database(self):
        """Initialize dialogue generation tracking"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS dialogue_generations (
                generation_id TEXT PRIMARY KEY,
                character_name TEXT NOT NULL,
                dread_level INTEGER,
                trauma_level REAL,
                prompt TEXT,
                response TEXT,
                dialogue_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
    
    def generate_dialogue_tree(self, config: DialogueConfig) -> dict[str, any]:
        """
        Generate dialogue tree using AI
        
        Returns:
            Dictionary with YarnSpinner dialogue data
        """
        # Create horror-aware prompt
        prompt = self._create_dialogue_prompt(config)
        
        try:
            # Call OpenAI for dialogue generation
            response = self.client.chat.completions.create(
                model="gpt-4o",
                messages=[
                    {"role": "system", "content": self._get_system_prompt()},
                    {"role": "user", "content": prompt}
                ],
                temperature=0.8,
                response_format={"type": "json_object"}
            )
            
            # Parse response
            dialogue_data = json.loads(response.choices[0].message.content)
            
            # Record generation
            generation_id = f"dialogue_{config.character_name}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._record_generation(generation_id, config, prompt, response.choices[0].message.content, dialogue_data)
            
            return {
                "success": True,
                "generation_id": generation_id,
                "dialogue_data": dialogue_data,
                "config": asdict(config)
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "config": asdict(config)
            }
    
    def _get_system_prompt(self) -> str:
        """Get system prompt for dialogue generation"""
        return """You are a narrative writer for a horror RPG called Dragon's Labyrinth.
        
        The game has 5 dread levels (0-4):
        - 0: Peace - Normal conversations, hope and humor
        - 1: Unease - Subtle hints something is wrong
        - 2: Dread - Open fear, people fleeing
        - 3: Terror - Breakdown of sanity and relationships
        - 4: Horror - Complete despair or absence
        
        Companion character arcs:
        - Einar: Loyal warrior → violent paranoid → broken/dead
        - Mira: Optimist healer → desperate helper → loses hope/leaves
        - Sorin: Curious scholar → obsessed → betrays for knowledge
        - Tamara: Innocent baker → traumatized → hardened/catatonic
        
        Generate YarnSpinner-compatible dialogue that reflects horror progression.
        
        Return JSON with:
        {
            "nodes": [
                {
                    "title": "Start",
                    "tags": ["dread_0", "peaceful"],
                    "body": [
                        "Einar: The road ahead looks clear.",
                        "-> That's good news",
                        "    Einar: Indeed! We should reach the village by sundown.",
                        "-> I have a bad feeling",
                        "    Einar: Nonsense! It's a beautiful day for travel."
                    ]
                }
            ],
            "variables": {
                "companion_trust": 1.0,
                "moral_alignment": 0,
                "trauma_witnessed": 0
            },
            "conditions": [
                {
                    "node": "BetrayalChoice",
                    "requires": "dread_level >= 3 && companion == 'sorin'"
                }
            ],
            "emotional_progression": {
                "start_emotion": "hopeful",
                "end_emotion": "fearful",
                "turning_points": ["sees_first_corpse", "companion_breaks"]
            }
        }
        """
    
    def _create_dialogue_prompt(self, config: DialogueConfig) -> str:
        """Create prompt for dialogue generation"""
        character_profiles = {
            "einar": "Loyal warrior, protective, becomes violent under stress",
            "mira": "Optimistic healer, caring, loses hope as horror grows",
            "sorin": "Intellectual scholar, curious, will betray for knowledge",
            "tamara": "Young baker's apprentice, innocent, becomes hardened",
            "villager": "Common folk, varies from helpful to terrified",
            "merchant": "Pragmatic trader, flees when danger comes",
            "hollow_caretaker": "First boss, was once kind, now corrupted"
        }
        
        trauma_descriptions = {
            0.0: "no trauma, normal state",
            0.25: "slightly shaken but functional",
            0.5: "visibly traumatized, struggling",
            0.75: "severely damaged, breaking down",
            1.0: "completely broken or absent"
        }
        
        emotional_states = {
            "neutral": "calm and collected",
            "scared": "frightened and anxious",
            "angry": "aggressive and confrontational",
            "hopeful": "optimistic despite circumstances",
            "broken": "given up, no hope left"
        }
        
        # Get closest trauma description
        trauma_desc = "no trauma"
        for threshold, desc in trauma_descriptions.items():
            if config.trauma_level >= threshold:
                trauma_desc = desc
        
        return f"""Generate dialogue for Dragon's Labyrinth.
        
        Character: {config.character_name} - {character_profiles.get(config.character_name, 'Unknown character')}
        Dread Level: {config.dread_level}
        Trauma Level: {config.trauma_level:.1%} - {trauma_desc}
        Dialogue Type: {config.dialogue_type}
        Emotional State: {config.emotional_state} - {emotional_states.get(config.emotional_state, '')}
        Context: {config.context if config.context else 'General interaction'}
        
        Create dialogue that:
        1. Reflects the character's personality and current mental state
        2. Shows appropriate horror progression for dread level {config.dread_level}
        3. Includes {3 if config.dialogue_type == 'moral_choice' else 2} player response options
        4. Demonstrates trauma effects at {config.trauma_level:.1%}
        5. Uses YarnSpinner format with proper branching
        
        The dialogue should feel natural but increasingly unsettling.
        """
    
    def _record_generation(self, generation_id: str, config: DialogueConfig, prompt: str, response: str, dialogue_data: dict):
        """Record generation in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO dialogue_generations 
            (generation_id, character_name, dread_level, trauma_level, prompt, response, dialogue_data)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        ''', (generation_id, config.character_name, config.dread_level, config.trauma_level, 
              prompt, response, json.dumps(dialogue_data)))
        
        conn.commit()
        conn.close()
    
    def generate_yarn_file(self, dialogue_data: dict, output_path: str = "assets/generated/dialogue/") -> str:
        """
        Generate YarnSpinner .yarn file
        
        Returns:
            Path to generated .yarn file
        """
        Path(output_path).mkdir(parents=True, exist_ok=True)
        
        character = dialogue_data.get("config", {}).get("character_name", "unknown")
        dread = dialogue_data.get("config", {}).get("dread_level", 0)
        
        yarn_content = f"""# Generated dialogue for {character} at dread level {dread}
"""
        
        # Add each node
        for node in dialogue_data.get("dialogue_data", {}).get("nodes", []):
            yarn_content += f"""
title: {node.get("title", "Node")}
tags: {" ".join(node.get("tags", []))}
---
"""
            for line in node.get("body", []):
                yarn_content += f"{line}\n"
            yarn_content += "===\n"
        
        # Write to file
        file_name = f"{character}_dread{dread}_{datetime.now().strftime('%Y%m%d_%H%M%S')}.yarn"
        file_path = Path(output_path) / file_name
        file_path.write_text(yarn_content)
        
        return str(file_path)
    
    def generate_companion_arc(self, companion: str) -> list[dict]:
        """
        Generate complete dialogue arc for a companion (all dread levels)
        
        Returns:
            List of generation results for each dread level
        """
        results = []
        
        # Trauma progression per companion
        trauma_progression = {
            "einar": [0.0, 0.2, 0.4, 0.7, 1.0],
            "mira": [0.0, 0.1, 0.3, 0.6, 1.0],
            "sorin": [0.0, 0.15, 0.35, 0.8, 1.0],
            "tamara": [0.0, 0.25, 0.5, 0.75, 1.0]
        }
        
        for dread_level in range(5):
            config = DialogueConfig(
                character_name=companion,
                dread_level=dread_level,
                trauma_level=trauma_progression.get(companion, [0.0] * 5)[dread_level],
                dialogue_type="conversation" if dread_level < 3 else "farewell" if dread_level == 4 else "moral_choice",
                emotional_state=["hopeful", "scared", "angry", "broken", "broken"][dread_level]
            )
            
            result = self.generate_dialogue_tree(config)
            results.append(result)
            
            if result["success"]:
                # Generate Yarn file
                yarn_path = self.generate_yarn_file(result)
                result["yarn_path"] = yarn_path
        
        return results
