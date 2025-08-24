"""
Maps Agent - AI-powered hex world generation for Dragon's Labyrinth
Uses OpenAI to generate tile layouts with horror progression
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
class HexWorldConfig:
    """Configuration for hex world generation"""
    world_name: str
    dread_level: int  # 0-4
    size: int = 50  # Number of hex tiles
    biome: str = "grassland"  # Starting biome
    corruption_radius: int = 0  # How far corruption spreads
    seed: Optional[int] = None


class MapsAgent:
    """AI agent for generating hex world maps"""
    
    def __init__(self, db_path: str = "assets/assets.db"):
        self.client = OpenAI(api_key=os.environ.get("OPENAI_API_KEY"))
        self.db_path = db_path
        self._init_database()
    
    def _init_database(self):
        """Initialize maps generation tracking"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            CREATE TABLE IF NOT EXISTS map_generations (
                generation_id TEXT PRIMARY KEY,
                world_name TEXT NOT NULL,
                dread_level INTEGER,
                prompt TEXT,
                response TEXT,
                tile_data TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ''')
        
        conn.commit()
        conn.close()
    
    def generate_world_layout(self, config: HexWorldConfig) -> dict[str, any]:
        """
        Generate a hex world layout using AI
        
        Returns:
            Dictionary with world data and tile assignments
        """
        # Create horror-aware prompt
        prompt = self._create_world_prompt(config)
        
        try:
            # Call OpenAI for world generation
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
            world_data = json.loads(response.choices[0].message.content)
            
            # Record generation
            generation_id = f"world_{config.world_name}_{datetime.now().strftime('%Y%m%d_%H%M%S')}"
            self._record_generation(generation_id, config, prompt, response.choices[0].message.content, world_data)
            
            return {
                "success": True,
                "generation_id": generation_id,
                "world_data": world_data,
                "config": asdict(config)
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "config": asdict(config)
            }
    
    def _get_system_prompt(self) -> str:
        """Get system prompt for world generation"""
        return """You are a hex world generator for a horror RPG called Dragon's Labyrinth.
        
        The game has 5 dread levels (0-4):
        - 0: Peace - Beautiful, safe world
        - 1: Unease - Subtle wrongness
        - 2: Dread - Visible corruption
        - 3: Terror - Reality breaking
        - 4: Horror - Nightmare realm
        
        Generate hex tile layouts that reflect the current dread level.
        Each tile should have:
        - q, r coordinates (axial hex coordinates)
        - tile_type: grass, stone, water, corrupted, void, etc.
        - corruption_level: 0-1 float indicating corruption
        - features: list of features on this tile
        
        Return JSON with:
        {
            "tiles": [
                {"q": 0, "r": 0, "tile_type": "grass", "corruption_level": 0.0, "features": ["village"]},
                ...
            ],
            "points_of_interest": [
                {"name": "Abandoned Mill", "q": 2, "r": -1, "description": "..."}
            ],
            "corruption_pattern": "radial" or "spreading" or "random",
            "narrative_notes": "Description of how this layout supports the horror progression"
        }
        """
    
    def _create_world_prompt(self, config: HexWorldConfig) -> str:
        """Create prompt for world generation"""
        dread_descriptions = {
            0: "peaceful and beautiful, with thriving villages and clear streams",
            1: "subtly unsettling, shadows too long, birds too quiet",
            2: "visibly corrupted, dying vegetation, fleeing animals",
            3: "reality breaking down, impossible geometries, time distortions",
            4: "pure nightmare, everything corrupted, dragon's presence overwhelming"
        }
        
        return f"""Generate a hex world layout for Dragon's Labyrinth.
        
        World Name: {config.world_name}
        Dread Level: {config.dread_level} - {dread_descriptions[config.dread_level]}
        Size: {config.size} hex tiles
        Starting Biome: {config.biome}
        Corruption Radius: {config.corruption_radius} tiles from center
        
        Create a layout that:
        1. Reflects the current dread level in tile types and features
        2. Has appropriate points of interest for horror progression
        3. Shows corruption spreading from specific sources
        4. Includes environmental storytelling elements
        
        Generate exactly {config.size} tiles in a roughly circular pattern.
        """
    
    def _record_generation(self, generation_id: str, config: HexWorldConfig, prompt: str, response: str, world_data: dict):
        """Record generation in database"""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        cursor.execute('''
            INSERT INTO map_generations 
            (generation_id, world_name, dread_level, prompt, response, tile_data)
            VALUES (?, ?, ?, ?, ?, ?)
        ''', (generation_id, config.world_name, config.dread_level, prompt, response, json.dumps(world_data)))
        
        conn.commit()
        conn.close()
    
    def generate_tile_configs(self, world_data: dict) -> list[dict]:
        """
        Convert AI world data to tile generation configs
        
        Returns:
            List of tile configurations for batch processing
        """
        tile_configs = []
        
        for tile in world_data.get("tiles", []):
            # Map AI tile types to our tile system
            tile_type = tile.get("tile_type", "grass")
            corruption = tile.get("corruption_level", 0.0)
            
            # Calculate effective dread level for this tile
            tile_dread = min(4, int(corruption * 5))
            
            config = {
                "tile_type": tile_type,
                "dread_level": tile_dread,
                "hex_size": 1.0,
                "height": 0.2 if tile_type != "water" else 0.1,
                "position": {"q": tile.get("q", 0), "r": tile.get("r", 0)}
            }
            
            tile_configs.append(config)
        
        return tile_configs
    
    def generate_rust_loader(self, world_data: dict, output_path: str = "src/maps/generated/") -> str:
        """
        Generate Rust code to load this world
        
        Returns:
            Path to generated Rust file
        """
        Path(output_path).mkdir(parents=True, exist_ok=True)
        
        world_name = world_data.get("config", {}).get("world_name", "unnamed")
        safe_name = world_name.lower().replace(" ", "_").replace("-", "_")
        
        rust_code = f'''// Auto-generated world loader for {world_name}
use bevy::prelude::*;
use hexx::{{Hex, HexLayout}};

pub fn load_{safe_name}_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {{
    let layout = HexLayout::flat(1.0);
    
'''
        
        # Add tile spawning code
        for tile in world_data.get("world_data", {}).get("tiles", []):
            q = tile.get("q", 0)
            r = tile.get("r", 0)
            tile_type = tile.get("tile_type", "grass")
            
            rust_code += f'''    // Spawn {tile_type} tile at ({q}, {r})
    commands.spawn((
        SceneBundle {{
            scene: asset_server.load("tiles/{tile_type}_dread0.glb"),
            transform: Transform::from_translation(
                layout.hex_to_world_pos(Hex::new({q}, {r})).extend(0.0)
            ),
            ..default()
        }},
        HexPosition(Hex::new({q}, {r})),
        TileType::{tile_type.capitalize()},
    ));
    
'''
        
        rust_code += "}\n"
        
        # Write to file
        file_path = Path(output_path) / f"{safe_name}_loader.rs"
        file_path.write_text(rust_code)
        
        return str(file_path)
