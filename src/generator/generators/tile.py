"""
Tile Generator - Individual hex tile generation for Dragon's Labyrinth.

Generates individual hex tile modules with encounters, loot, NPCs, and 
bevy_ecs_tilemap integration for seamless world rendering.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from generator.entities.generators.base import BaseGenerator
from generator.entities.models import ProcessingResult


class TileGenerator(BaseGenerator):
    """
    Hex tile-level generator for individual world tiles.
    
    Responsible for:
    - Individual {uuid}.rs modules for each hex tile
    - Tile-specific encounters, loot, and NPCs
    - Tile terrain and biome configuration
    - Local physics and collision setup
    - bevy_ecs_tilemap tile definitions
    """
    
    def __init__(self):
        super().__init__("tile")
    
    def generate_hex_tile_module(self, tile_result: ProcessingResult,
                                output_dir: Path,
                                tile_uuid: str) -> Path:
        """
        Generate individual hex tile module.
        
        Creates:
        - regions/{region}/{tile_uuid}.rs with encounters, loot, NPCs
        - Tile-specific bevy_ecs_tilemap configuration
        - Local physics colliders and triggers
        - Spawn tables and event handlers
        """
        
        # Extract coordinates and region info
        coordinates = self._extract_hex_coordinates(tile_result.specific_data)
        region_name = self._extract_region_name(tile_result)
        
        # Prepare context for tile module template
        context = {
            'tile_uuid': tile_uuid,
            'tile_name': tile_result.cluster_name,
            'tile_struct': self._to_rust_struct_name(f"tile_{tile_uuid}"),
            'coordinates': coordinates,
            'region_name': region_name,
            'terrain_type': tile_result.specific_data.get('terrain', 'plains'),
            'biome': tile_result.specific_data.get('biome', 'forest'),
            'encounters': self._generate_tile_encounters(tile_result),
            'loot_tables': self._generate_loot_tables(tile_result),
            'npcs': self._generate_tile_npcs(tile_result),
            'poi_list': self._generate_poi_list(tile_result),
            'physics_setup': self._generate_tile_physics(tile_result, coordinates),
            'tilemap_config': self._generate_tile_tilemap_config(tile_result, coordinates)
        }
        
        # Generate hex tile module
        tile_mod_content = self.render_template('hex_tile.rs.j2', context)
        
        # Determine output path based on region structure
        if region_name:
            region_dir = output_dir / "regions" / self._to_rust_module_name(region_name)
        else:
            region_dir = output_dir / "regions" / "unknown"
            
        tile_mod_path = region_dir / f"{tile_uuid}.rs"
        
        return self.write_rust_module(tile_mod_content, tile_mod_path)
    
    def generate_tile_sprite_images(self, tile_result: ProcessingResult,
                                   output_dir: Path,
                                   tile_uuid: str) -> list[Path]:
        """
        Generate sprite images for entities on this tile.
        
        Creates:
        - POI icons (villages, dungeons, shrines, etc.)
        - NPC sprites
        - Item/loot sprites
        - Environmental detail sprites
        """
        
        generated_images = []
        sprite_dir = output_dir / "assets" / "textures" / "sprites" / tile_uuid
        
        # Generate POI icons
        poi_list = self._generate_poi_list(tile_result)
        for poi in poi_list:
            poi_prompt = self._generate_poi_sprite_prompt(poi)
            poi_image_path = sprite_dir / f"poi_{poi['type']}_{poi['id']}.png"
            generated_path = self.generate_image(poi_prompt, poi_image_path, size="256x256")
            generated_images.append(generated_path)
        
        # Generate NPC sprites
        npcs = self._generate_tile_npcs(tile_result)
        for npc in npcs:
            npc_prompt = self._generate_npc_sprite_prompt(npc)
            npc_image_path = sprite_dir / f"npc_{npc['id']}.png"
            generated_path = self.generate_image(npc_prompt, npc_image_path, size="128x128")
            generated_images.append(generated_path)
        
        return generated_images
    
    def _extract_region_name(self, tile_result: ProcessingResult) -> str | None:
        """Extract region name from tile result data."""
        return tile_result.specific_data.get('region', 
               tile_result.cluster_category if tile_result.cluster_category != 'hex_tile' 
               else None)
    
    def _generate_tile_encounters(self, tile_result: ProcessingResult) -> list[dict[str, Any]]:
        """Generate encounter table for this hex tile."""
        
        terrain = tile_result.specific_data.get('terrain', 'plains')
        corruption_level = tile_result.specific_data.get('corruption_level', 0.0)
        
        encounters = []
        
        # Base encounters by terrain type
        terrain_encounters = {
            'forest': ['wolf_pack', 'lost_traveler', 'forest_spirit'],
            'plains': ['bandit_patrol', 'merchant_caravan', 'wild_horses'],
            'swamp': ['marsh_wraith', 'corrupted_toad', 'will_o_wisp'],
            'mountain': ['stone_giant', 'cave_in', 'mountain_goat_herd'],
            'desert': ['sandstorm', 'desert_raiders', 'oasis_mirage']
        }
        
        base_encounters = terrain_encounters.get(terrain, terrain_encounters['plains'])
        
        for encounter_type in base_encounters:
            encounters.append({
                'type': encounter_type,
                'probability': max(0.1, 0.3 - corruption_level * 0.2),
                'corruption_modified': corruption_level > 0.3,
                'threat_level': min(10, int(corruption_level * 10) + 1)
            })
        
        # Add corruption-specific encounters
        if corruption_level > 0.5:
            encounters.extend([
                {
                    'type': 'shadow_manifestation',
                    'probability': corruption_level * 0.4,
                    'corruption_modified': True,
                    'threat_level': min(10, int(corruption_level * 15))
                },
                {
                    'type': 'cursed_ground',
                    'probability': corruption_level * 0.3,
                    'corruption_modified': True,
                    'threat_level': min(10, int(corruption_level * 12))
                }
            ])
        
        return encounters
    
    def _generate_loot_tables(self, tile_result: ProcessingResult) -> list[dict[str, Any]]:
        """Generate loot tables for this hex tile."""
        
        terrain = tile_result.specific_data.get('terrain', 'plains')
        corruption_level = tile_result.specific_data.get('corruption_level', 0.0)
        
        loot_tables = []
        
        # Base loot by terrain
        terrain_loot = {
            'forest': ['herbs', 'wood', 'berries', 'pelts'],
            'plains': ['grain', 'cloth', 'leather', 'simple_tools'],
            'swamp': ['rare_herbs', 'marsh_gas', 'bog_iron', 'decay_essence'],
            'mountain': ['metals', 'gems', 'stone', 'crystal_shards'],
            'desert': ['sand_glass', 'dried_goods', 'precious_stones', 'water']
        }
        
        base_loot = terrain_loot.get(terrain, terrain_loot['plains'])
        
        for loot_type in base_loot:
            loot_tables.append({
                'item_type': loot_type,
                'rarity': 'common',
                'spawn_rate': 0.3,
                'corruption_tainted': corruption_level > 0.6
            })
        
        # Add rare/corrupted loot
        if corruption_level > 0.4:
            loot_tables.append({
                'item_type': 'cursed_artifact',
                'rarity': 'rare',
                'spawn_rate': corruption_level * 0.2,
                'corruption_tainted': True
            })
        
        return loot_tables
    
    def _generate_tile_npcs(self, tile_result: ProcessingResult) -> list[dict[str, Any]]:
        """Generate NPC list for this hex tile."""
        
        npcs = []
        entity_count = tile_result.entity_count
        
        # Generate NPCs based on entity count and type
        if entity_count > 5:  # Populated area
            npcs.extend([
                {
                    'id': 'local_guide',
                    'name': 'Local Guide',
                    'type': 'merchant',
                    'spawn_probability': 0.7,
                    'dialogue_tree': 'guide_dialogue'
                },
                {
                    'id': 'traveling_scholar',
                    'name': 'Traveling Scholar',
                    'type': 'questgiver',
                    'spawn_probability': 0.4,
                    'dialogue_tree': 'scholar_dialogue'
                }
            ])
        
        if entity_count > 10:  # Major settlement
            npcs.extend([
                {
                    'id': 'settlement_elder',
                    'name': 'Settlement Elder',
                    'type': 'authority',
                    'spawn_probability': 0.9,
                    'dialogue_tree': 'elder_dialogue'
                },
                {
                    'id': 'merchant_trader',
                    'name': 'Merchant Trader',
                    'type': 'vendor',
                    'spawn_probability': 0.8,
                    'dialogue_tree': 'trader_dialogue'
                }
            ])
        
        return npcs
    
    def _generate_poi_list(self, tile_result: ProcessingResult) -> list[dict[str, Any]]:
        """Generate Points of Interest for this hex tile."""
        
        poi_list = []
        
        # Extract POIs from entity data
        entity_data = tile_result.specific_data
        
        # Common POI types
        if 'village' in str(entity_data).lower() or 'settlement' in str(entity_data).lower():
            poi_list.append({
                'id': 'village_center',
                'type': 'village',
                'name': 'Village Center',
                'interactive': True,
                'services': ['shop', 'rest', 'information']
            })
        
        if 'dungeon' in str(entity_data).lower() or 'cave' in str(entity_data).lower():
            poi_list.append({
                'id': 'dungeon_entrance',
                'type': 'dungeon',
                'name': 'Dungeon Entrance',
                'interactive': True,
                'services': ['exploration', 'combat']
            })
        
        if 'shrine' in str(entity_data).lower() or 'temple' in str(entity_data).lower():
            poi_list.append({
                'id': 'shrine',
                'type': 'shrine',
                'name': 'Ancient Shrine',
                'interactive': True,
                'services': ['forge', 'blessing']
            })
        
        # Add generic exploration points
        poi_list.append({
            'id': 'exploration_point',
            'type': 'exploration',
            'name': 'Point of Interest',
            'interactive': False,
            'services': []
        })
        
        return poi_list
    
    def _generate_tile_physics(self, tile_result: ProcessingResult, 
                              coordinates: tuple[int, int] | None) -> dict[str, Any]:
        """Generate physics setup for this hex tile."""
        
        terrain = tile_result.specific_data.get('terrain', 'plains')
        
        # Terrain-specific physics properties
        physics_config = {
            'forest': {'movement_speed': 0.8, 'visibility': 0.7, 'collision': 'partial'},
            'plains': {'movement_speed': 1.0, 'visibility': 1.0, 'collision': 'none'},
            'swamp': {'movement_speed': 0.5, 'visibility': 0.6, 'collision': 'difficult'},
            'mountain': {'movement_speed': 0.6, 'visibility': 0.9, 'collision': 'blocked'},
            'water': {'movement_speed': 0.3, 'visibility': 0.8, 'collision': 'water'}
        }
        
        config = physics_config.get(terrain, physics_config['plains'])
        
        return {
            'coordinates': coordinates or (0, 0),
            'movement_speed_modifier': config['movement_speed'],
            'visibility_modifier': config['visibility'],
            'collision_type': config['collision'],
            'avian_collider': terrain in ['mountain', 'water'],
            'trigger_zones': len(self._generate_poi_list(tile_result)) > 0
        }
    
    def _generate_tile_tilemap_config(self, tile_result: ProcessingResult,
                                    coordinates: tuple[int, int] | None) -> dict[str, Any]:
        """Generate bevy_ecs_tilemap configuration for this hex tile."""
        
        terrain = tile_result.specific_data.get('terrain', 'plains')
        biome = tile_result.specific_data.get('biome', 'forest')
        
        return {
            'coordinates': coordinates or (0, 0),
            'terrain_type': terrain,
            'biome_type': biome,
            'tile_index': self._get_terrain_tile_index(terrain, biome),
            'layer_id': 0,  # Background terrain layer
            'z_order': 0,
            'rotation': 0,
            'flip_x': False,
            'flip_y': False
        }
    
    def _get_terrain_tile_index(self, terrain: str, biome: str) -> int:
        """Get tile index in texture atlas for terrain/biome combination."""
        
        # Map terrain/biome combinations to texture atlas indices
        terrain_indices = {
            ('forest', 'wet_meadow'): 0,
            ('forest', 'ashen_forest'): 1,
            ('plains', 'wet_meadow'): 2,
            ('plains', 'rust_plains'): 3,
            ('swamp', 'black_swamp'): 4,
            ('swamp', 'fungal_cathedral'): 5,
            ('mountain', 'rust_plains'): 6,
            ('mountain', 'bone_forest'): 7,
            ('water', 'flooded_village'): 8,
            ('desert', 'famine_fields'): 9
        }
        
        return terrain_indices.get((terrain, biome), 0)
    
    def _generate_poi_sprite_prompt(self, poi: dict[str, Any]) -> str:
        """Generate prompt for POI sprite image."""
        
        poi_prompts = {
            'village': 'small medieval village icon with houses and smoke, cozy settlement marker',
            'dungeon': 'dark cave entrance icon with ominous shadows, dungeon entrance marker',
            'shrine': 'ancient stone shrine icon with mystical glow, sacred site marker',
            'exploration': 'question mark or magnifying glass icon, exploration point marker'
        }
        
        base_prompt = poi_prompts.get(poi['type'], poi_prompts['exploration'])
        
        return f"Create a {base_prompt}. 32x32 pixel art icon, clear simple design, no text, transparent background, high contrast for visibility."
    
    def _generate_npc_sprite_prompt(self, npc: dict[str, Any]) -> str:
        """Generate prompt for NPC sprite image."""
        
        npc_prompts = {
            'merchant': 'friendly merchant character with trade goods, welcoming appearance',
            'questgiver': 'wise scholar or elder with scrolls or books, knowledgeable look',
            'authority': 'important village leader with formal robes, authoritative presence',
            'vendor': 'shopkeeper character with merchant supplies, business-like appearance'
        }
        
        base_prompt = npc_prompts.get(npc['type'], npc_prompts['merchant'])
        
        return f"Create a {base_prompt}. 64x64 pixel art character sprite, front-facing view, medieval fantasy style, clear details, no text."
