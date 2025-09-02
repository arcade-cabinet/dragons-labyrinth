"""
Area Generator - Dungeon area generation for Dragon's Labyrinth.

Generates dungeon area modules with room layouts, connections, treasure, 
and bevy_ecs_tilemap integration using actual DungeonData from DungeonsProcessor.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from generator.entities.generators.base import BaseGenerator
from generator.entities.models import ProcessingResult


class AreaGenerator(BaseGenerator):
    """
    Dungeon area-level generator for individual dungeon rooms/areas.
    
    Responsible for:
    - Individual {area_uuid}.rs modules for each dungeon area/room
    - Area-specific encounters, treasure, and environmental features
    - Room connections and dungeon layout
    - bevy_ecs_tilemap configuration for dungeon tiles
    - Physics setup for dungeon exploration
    """
    
    def __init__(self):
        super().__init__("area")
    
    def generate_dungeon_module(self, dungeon_result: ProcessingResult,
                               output_dir: Path,
                               dungeon_uuid: str) -> Path:
        """
        Generate complete dungeon module using actual DungeonData from DungeonsProcessor.
        
        Creates:
        - dungeons/{dungeon_name}/mod.rs with all area imports
        - Dungeon-wide configuration from threat_assessment
        - Room generation based on structural_analysis
        - Treasure distribution based on actual analysis
        """
        
        # Extract actual DungeonData from processor
        dungeon_data = dungeon_result.specific_data
        if not hasattr(dungeon_data, 'name'):
            raise ValueError(f"Expected DungeonData from DungeonsProcessor, got: {type(dungeon_data)}")
        
        dungeon_name = self._to_rust_module_name(dungeon_data.name)
        dungeon_dir = output_dir / "dungeons" / dungeon_name
        
        # Use actual analyzed data from DungeonsProcessor
        context = {
            'dungeon_name': dungeon_data.name,
            'dungeon_module': dungeon_name,
            'dungeon_struct': self._to_rust_struct_name(dungeon_data.name),
            'dungeon_uuid': dungeon_uuid,
            'dungeon_type': dungeon_data.dungeon_type.value,
            'threat_assessment': {
                'threat_level': dungeon_data.threat_assessment.threat_level,
                'enemy_count': dungeon_data.threat_assessment.enemy_count,
                'has_boss_encounter': dungeon_data.threat_assessment.has_boss_encounter,
                'trap_count': dungeon_data.threat_assessment.trap_count,
                'undead_presence': dungeon_data.threat_assessment.undead_presence,
                'threat_density': dungeon_data.threat_assessment.threat_density,
                'hazard_distribution': dungeon_data.threat_assessment.hazard_distribution
            },
            'structural_analysis': {
                'estimated_rooms': dungeon_data.structural_analysis.estimated_rooms,
                'complexity_level': dungeon_data.structural_analysis.complexity_level.value,
                'is_multi_level': dungeon_data.structural_analysis.is_multi_level,
                'navigation_difficulty': dungeon_data.structural_analysis.navigation_difficulty.value,
                'layout_density': dungeon_data.structural_analysis.layout_density
            },
            'accessibility': dungeon_data.accessibility,
            'corruption_influence': dungeon_data.corruption_influence,
            'exploration_difficulty': dungeon_data.exploration_difficulty.value,
            'areas': self._generate_area_list_from_data(dungeon_data),
            'tilemap_config': self._generate_dungeon_tilemap_config_from_data(dungeon_data),
            'physics_config': self._generate_dungeon_physics_from_data(dungeon_data)
        }
        
        # Generate dungeon mod.rs
        dungeon_mod_content = self.render_template('dungeon_mod.rs.j2', context)
        dungeon_mod_path = dungeon_dir / "mod.rs"
        
        return self.write_rust_module(dungeon_mod_content, dungeon_mod_path)
    
    def generate_dungeon_area_module(self, dungeon_data, area_data: dict[str, Any],
                                    output_dir: Path, 
                                    dungeon_uuid: str,
                                    area_uuid: str) -> Path:
        """
        Generate individual dungeon area module using actual room data.
        
        Creates:
        - dungeons/{dungeon_name}/{area_uuid}.rs with room details
        - Area-specific encounters based on threat_assessment
        - Treasure spawns based on structural analysis
        - Connection points to other areas
        """
        
        dungeon_name = self._to_rust_module_name(dungeon_data.name)
        dungeon_dir = output_dir / "dungeons" / dungeon_name
        
        # Use actual analyzed data for area generation
        context = {
            'area_uuid': area_uuid,
            'area_name': area_data['name'],
            'area_struct': self._to_rust_struct_name(f"area_{area_uuid}"),
            'dungeon_name': dungeon_data.name,
            'dungeon_uuid': dungeon_uuid,
            'area_type': area_data['type'],
            'room_size': area_data['size'],
            'encounter_data': self._generate_area_encounters_from_threat_data(dungeon_data.threat_assessment, area_data),
            'treasure_data': self._generate_area_treasure_from_data(dungeon_data, area_data),
            'connections': area_data.get('connections', []),
            'environmental_features': area_data.get('features', []),
            'tilemap_setup': self._generate_area_tilemap_setup(dungeon_data, area_data),
            'physics_setup': self._generate_area_physics_setup(dungeon_data, area_data)
        }
        
        # Generate area module
        area_mod_content = self.render_template('dungeon_area.rs.j2', context)
        area_mod_path = dungeon_dir / f"{area_uuid}.rs"
        
        return self.write_rust_module(area_mod_content, area_mod_path)
    
    def generate_dungeon_environment_images(self, dungeon_result: ProcessingResult,
                                          output_dir: Path) -> list[Path]:
        """
        Generate dungeon environment images using actual DungeonData.
        
        Creates:
        - Dungeon entrance images based on accessibility
        - Room environment tiles based on dungeon_type
        - Threat-appropriate enemy sprites
        - Treasure chest and item sprites
        """
        
        dungeon_data = dungeon_result.specific_data
        dungeon_name = self._to_rust_module_name(dungeon_data.name)
        
        generated_images = []
        image_dir = output_dir / "assets" / "textures" / "dungeons" / dungeon_name
        
        # Generate entrance image based on actual accessibility
        entrance_prompt = self._generate_entrance_image_prompt(dungeon_data)
        entrance_path = image_dir / f"{dungeon_name}_entrance.png"
        generated_images.append(self.generate_image(entrance_prompt, entrance_path))
        
        # Generate room environment tiles based on dungeon type and corruption
        room_variants = ['floor', 'wall', 'door']
        for variant in room_variants:
            room_prompt = self._generate_room_tile_prompt(dungeon_data, variant)
            room_path = image_dir / f"{dungeon_name}_{variant}.png"
            generated_images.append(self.generate_image(room_prompt, room_path, size="64x64"))
        
        # Generate threat-appropriate enemy sprites if high enemy count
        if dungeon_data.threat_assessment.enemy_count > 5:
            enemy_prompt = self._generate_enemy_sprite_prompt(dungeon_data)
            enemy_path = image_dir / f"{dungeon_name}_enemy.png"
            generated_images.append(self.generate_image(enemy_prompt, enemy_path, size="32x32"))
        
        return generated_images
    
    def _generate_area_list_from_data(self, dungeon_data) -> list[dict[str, Any]]:
        """Generate area list based on actual structural analysis."""
        
        areas = []
        estimated_rooms = dungeon_data.structural_analysis.estimated_rooms
        complexity_level = dungeon_data.structural_analysis.complexity_level.value
        is_multi_level = dungeon_data.structural_analysis.is_multi_level
        
        # Generate areas based on actual room count analysis
        for room_idx in range(estimated_rooms):
            area_type = self._determine_area_type(room_idx, estimated_rooms, dungeon_data)
            area_size = self._determine_area_size(complexity_level, area_type)
            
            area_data = {
                'name': f"Area {room_idx + 1}",
                'type': area_type,
                'size': area_size,
                'level': 0 if not is_multi_level else (room_idx // (estimated_rooms // 2)),
                'threat_factor': self._calculate_area_threat_factor(room_idx, estimated_rooms, dungeon_data),
                'features': self._generate_area_features(area_type, dungeon_data)
            }
            
            areas.append(area_data)
        
        return areas
    
    def _determine_area_type(self, room_idx: int, total_rooms: int, dungeon_data) -> str:
        """Determine area type based on position and dungeon characteristics."""
        
        # First room is always entrance
        if room_idx == 0:
            return "entrance"
        
        # Last room gets boss if has_boss_encounter
        if room_idx == total_rooms - 1 and dungeon_data.threat_assessment.has_boss_encounter:
            return "boss_chamber"
        
        # Treasure rooms based on dungeon analysis
        treasure_rooms = max(1, total_rooms // 5)  # ~20% treasure rooms
        if room_idx <= treasure_rooms:
            return "treasure_room"
        
        # Trap rooms based on trap count
        if dungeon_data.threat_assessment.trap_count > 0:
            trap_interval = max(3, total_rooms // dungeon_data.threat_assessment.trap_count)
            if room_idx % trap_interval == 0:
                return "trap_room"
        
        # Default combat encounter rooms
        return "combat_room"
    
    def _determine_area_size(self, complexity_level: str, area_type: str) -> str:
        """Determine area size based on complexity and type."""
        
        if area_type in ["boss_chamber", "treasure_room"]:
            return "large" if complexity_level in ["complex", "very_complex"] else "medium"
        elif area_type == "entrance":
            return "medium"
        else:
            return "small" if complexity_level == "simple" else "medium"
    
    def _calculate_area_threat_factor(self, room_idx: int, total_rooms: int, dungeon_data) -> float:
        """Calculate threat factor for individual area."""
        
        base_threat = dungeon_data.threat_assessment.threat_level / 5.0
        
        # Threat increases deeper into dungeon
        depth_factor = room_idx / total_rooms
        
        # Boss rooms have maximum threat
        if room_idx == total_rooms - 1 and dungeon_data.threat_assessment.has_boss_encounter:
            return 1.0
        
        return min(1.0, base_threat + depth_factor * 0.5)
    
    def _generate_area_features(self, area_type: str, dungeon_data) -> list[str]:
        """Generate environmental features for area based on type and dungeon data."""
        
        features = []
        
        if area_type == "entrance":
            features.extend(["entrance_door", "natural_light"])
            if dungeon_data.accessibility in ["difficult", "very_difficult"]:
                features.append("hidden_entrance")
        
        elif area_type == "boss_chamber":
            features.extend(["boss_altar", "dramatic_lighting", "treasure_pile"])
            
        elif area_type == "treasure_room":
            features.extend(["treasure_chest", "protective_wards"])
            if dungeon_data.threat_assessment.trap_count > 0:
                features.append("treasure_trap")
        
        elif area_type == "trap_room":
            features.extend(["pressure_plate", "hidden_mechanism"])
        
        # Add corruption-based features
        if dungeon_data.corruption_influence >= 3:
            features.extend(["corruption_stains", "unnatural_shadows"])
        
        # Add undead-specific features
        if dungeon_data.threat_assessment.undead_presence:
            features.extend(["bone_debris", "cold_aura"])
        
        return features
    
    def _generate_area_encounters_from_threat_data(self, threat_assessment, area_data: dict[str, Any]) -> list[dict[str, Any]]:
        """Generate encounters for area using actual threat assessment data."""
        
        encounters = []
        area_threat = area_data['threat_factor']
        
        # Scale encounters based on actual threat density
        if threat_assessment.threat_density >= 15:  # Very high threat
            encounter_count = 3
        elif threat_assessment.threat_density >= 10:  # High threat
            encounter_count = 2
        else:
            encounter_count = 1
        
        for enc_idx in range(encounter_count):
            encounter_type = self._determine_encounter_type(area_data['type'], threat_assessment)
            encounters.append({
                'type': encounter_type,
                'difficulty': min(10, int(area_threat * threat_assessment.threat_level)),
                'spawn_probability': 0.8 if area_data['type'] == 'combat_room' else 0.3,
                'is_undead': threat_assessment.undead_presence and enc_idx == 0,
                'has_traps': area_data['type'] == 'trap_room' or threat_assessment.trap_count > enc_idx
            })
        
        return encounters
    
    def _determine_encounter_type(self, area_type: str, threat_assessment) -> str:
        """Determine encounter type based on area and threat data."""
        
        if area_type == "boss_chamber":
            return "boss_encounter"
        elif area_type == "trap_room":
            return "trap_encounter"
        elif threat_assessment.undead_presence:
            return "undead_encounter"
        else:
            return "combat_encounter"
    
    def _generate_area_treasure_from_data(self, dungeon_data, area_data: dict[str, Any]) -> dict[str, Any]:
        """Generate treasure data for area based on actual dungeon analysis."""
        
        base_treasure_chance = 0.1
        
        # Treasure rooms have guaranteed treasure
        if area_data['type'] == "treasure_room":
            treasure_chance = 1.0
        elif area_data['type'] == "boss_chamber":
            treasure_chance = 0.9
        else:
            treasure_chance = base_treasure_chance
        
        # Scale treasure quality based on threat and complexity
        treasure_quality = "common"
        if dungeon_data.threat_assessment.threat_level >= 4:
            treasure_quality = "rare"
        elif dungeon_data.threat_assessment.threat_level >= 3:
            treasure_quality = "uncommon"
        
        return {
            'spawn_chance': treasure_chance,
            'quality': treasure_quality,
            'quantity': max(1, dungeon_data.threat_assessment.threat_level // 2),
            'is_cursed': dungeon_data.corruption_influence >= 4
        }
    
    def _generate_dungeon_tilemap_config_from_data(self, dungeon_data) -> dict[str, Any]:
        """Generate tilemap configuration using actual dungeon data."""
        
        return {
            'tile_size': 32,  # Smaller tiles for dungeon details
            'chunk_size': 16, # Smaller chunks for room-based loading
            'dungeon_type': dungeon_data.dungeon_type.value,
            'corruption_level': dungeon_data.corruption_influence,
            'room_count': dungeon_data.structural_analysis.estimated_rooms,
            'is_multi_level': dungeon_data.structural_analysis.is_multi_level,
            'lighting_type': 'torch' if dungeon_data.corruption_influence < 3 else 'cursed'
        }
    
    def _generate_dungeon_physics_from_data(self, dungeon_data) -> dict[str, Any]:
        """Generate physics configuration using actual dungeon data."""
        
        # Movement speed affected by complexity and corruption
        complexity_modifier = {
            'simple': 1.0,
            'moderate': 0.9,
            'complex': 0.8,
            'very_complex': 0.7
        }.get(dungeon_data.structural_analysis.complexity_level.value, 1.0)
        
        corruption_modifier = max(0.5, 1.0 - (dungeon_data.corruption_influence / 10.0))
        
        return {
            'movement_speed_modifier': complexity_modifier * corruption_modifier,
            'visibility_range': max(3, 8 - dungeon_data.corruption_influence),
            'encounter_rate': dungeon_data.threat_assessment.threat_density / 100.0,
            'trap_detection_difficulty': dungeon_data.threat_assessment.trap_count / 5.0,
            'ambient_lighting': dungeon_data.corruption_influence < 2,
            'supernatural_effects': dungeon_data.corruption_influence >= 3
        }
    
    def _generate_area_tilemap_setup(self, dungeon_data, area_data: dict[str, Any]) -> dict[str, Any]:
        """Generate tilemap setup for individual area."""
        
        return {
            'area_size': area_data['size'],
            'tile_variant': self._get_dungeon_tile_variant(dungeon_data.dungeon_type.value),
            'lighting_level': max(0.2, 1.0 - (dungeon_data.corruption_influence / 5.0)),
            'has_special_tiles': area_data['type'] in ['boss_chamber', 'treasure_room'],
            'corruption_overlay': dungeon_data.corruption_influence >= 3
        }
    
    def _generate_area_physics_setup(self, dungeon_data, area_data: dict[str, Any]) -> dict[str, Any]:
        """Generate physics setup for individual area."""
        
        return {
            'collision_boundaries': True,  # Dungeon rooms always have walls
            'door_positions': len(area_data.get('connections', [])),
            'trap_zones': area_data['type'] == 'trap_room',
            'boss_arena': area_data['type'] == 'boss_chamber',
            'environmental_hazards': dungeon_data.threat_assessment.trap_count > 0
        }
    
    def _get_dungeon_tile_variant(self, dungeon_type: str) -> str:
        """Get tile variant based on dungeon type."""
        
        tile_variants = {
            'crypt': 'stone_tomb',
            'cave': 'natural_stone',
            'temple': 'carved_stone',
            'lair': 'rough_stone',
            'hideout': 'wooden_planks',
            'pit': 'metal_grating'
        }
        
        return tile_variants.get(dungeon_type, 'generic_stone')
    
    def _generate_entrance_image_prompt(self, dungeon_data) -> str:
        """Generate prompt for dungeon entrance image."""
        
        type_prompts = {
            'crypt': 'ancient stone crypt entrance with weathered carvings, ominous archway',
            'cave': 'natural cave mouth with rocky formations, dark opening',
            'temple': 'ornate temple entrance with columns and mystical symbols',
            'lair': 'beast lair entrance with claw marks and bones scattered',
            'hideout': 'hidden entrance concealed behind foliage or rocks',
            'pit': 'deep pit entrance with metal grating or stone rim'
        }
        
        base_prompt = type_prompts.get(dungeon_data.dungeon_type.value, type_prompts['cave'])
        
        # Add accessibility modifiers
        if dungeon_data.accessibility in ['difficult', 'very_difficult']:
            accessibility_desc = ', partially hidden and difficult to access'
        else:
            accessibility_desc = ', clearly visible entrance'
        
        # Add corruption effects
        if dungeon_data.corruption_influence >= 4:
            corruption_desc = ', heavily corrupted with unnatural darkness emanating'
        elif dungeon_data.corruption_influence >= 2:
            corruption_desc = ', showing signs of corruption and decay'
        else:
            corruption_desc = ', relatively pristine'
        
        return f"Create a {base_prompt}{accessibility_desc}{corruption_desc}. Top-down perspective for dungeon entrance, 64x64 pixel art style, clear entrance marker."
    
    def _generate_room_tile_prompt(self, dungeon_data, tile_type: str) -> str:
        """Generate prompt for room tile variants."""
        
        dungeon_type = dungeon_data.dungeon_type.value
        corruption = dungeon_data.corruption_influence
        
        base_descriptions = {
            'floor': f'{dungeon_type} floor tile with appropriate texture',
            'wall': f'{dungeon_type} wall tile with structural details',
            'door': f'{dungeon_type} doorway tile with passage indication'
        }
        
        base_prompt = base_descriptions[tile_type]
        
        if corruption >= 3:
            corruption_mod = ', heavily corrupted with dark stains and unnatural growths'
        elif corruption >= 1:
            corruption_mod = ', showing wear and minor corruption'
        else:
            corruption_mod = ', clean and well-maintained'
        
        return f"Create a {base_prompt}{corruption_mod}. Seamless tileable texture, 32x32 pixel art, dungeon interior style."
    
    def _generate_enemy_sprite_prompt(self, dungeon_data) -> str:
        """Generate prompt for enemy sprite."""
        
        if dungeon_data.threat_assessment.undead_presence:
            enemy_type = "undead creature (skeleton, zombie, or wraith)"
        elif dungeon_data.dungeon_type.value == 'lair':
            enemy_type = "beast or monster appropriate for lair"
        elif dungeon_data.corruption_influence >= 3:
            enemy_type = "corrupted creature with dark magical influence"
        else:
            enemy_type = "dungeon guardian or hostile creature"
        
        threat_level = dungeon_data.threat_assessment.threat_level
        if threat_level >= 4:
            difficulty_desc = ", powerful and intimidating"
        elif threat_level >= 3:
            difficulty_desc = ", moderately threatening"
        else:
            difficulty_desc = ", basic threat level"
        
        return f"Create a {enemy_type}{difficulty_desc}. 32x32 pixel art sprite, front-facing view, dungeon creature style, clear details."
