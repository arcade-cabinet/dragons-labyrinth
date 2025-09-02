"""
World Generator - Top-level world generation with complete bevy_ecs_tilemap setup.

Generates the complete world module with all regions, physics layers, and tilemap configuration.
Orchestrates region, tile, and area generators to create the full game world.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from generator.entities.generators.base import BaseGenerator
from generator.entities.models import ProcessingResult


class WorldGenerator(BaseGenerator):
    """
    World-level generator that orchestrates complete world generation.
    
    Responsible for:
    - World-level mod.rs with all region imports
    - Global physics configuration
    - Tilemap layer setup for bevy_ecs_tilemap
    - World resource initialization
    - Cross-region coordination
    """
    
    def __init__(self):
        super().__init__("world")
    
    def generate_world_module(self, regions: list[ProcessingResult], 
                             output_dir: Path) -> Path | None:
        """
        Generate complete world module with all regions and global setup.
        
        Creates:
        - apps/game/src/world/mod.rs with all region imports
        - Global tilemap configuration
        - Physics layer definitions
        - World resource setup
        """
        
        # Prepare context for world module template
        context = {
            'regions': [self._prepare_region_context(region) for region in regions],
            'total_entities': sum(r.entity_count for r in regions),
            'physics_layers': self._generate_physics_layers(regions),
            'tilemap_config': self._generate_tilemap_config(regions),
            'world_bounds': self._calculate_world_bounds(regions)
        }
        
        # Generate world mod.rs
        world_mod_content = self.render_template('world_mod.rs.j2', context)
        world_mod_path = output_dir / "mod.rs"
        
        if self.write_rust_module(world_mod_content, world_mod_path):
            return world_mod_path
        
        return None
    
    def generate_world_plugin(self, regions: list[ProcessingResult], 
                             output_dir: Path) -> Path | None:
        """
        Generate Bevy world plugin that sets up the complete world system.
        
        Creates a WorldPlugin that:
        - Initializes bevy_ecs_tilemap
        - Sets up Avian physics
        - Loads all regions
        - Configures world resources
        """
        
        context = {
            'regions': [self._prepare_region_context(region) for region in regions],
            'tilemap_layers': self._generate_all_tilemap_layers(regions),
            'physics_setup': self._generate_global_physics_setup(regions),
            'world_resources': self._generate_world_resources(regions)
        }
        
        plugin_content = self.render_template('world_plugin.rs.j2', context)
        plugin_path = output_dir / "plugin.rs"
        
        if self.write_rust_module(plugin_content, plugin_path):
            return plugin_path
        
        return None
    
    def generate_world_resources(self, regions: list[ProcessingResult], 
                                output_dir: Path) -> Path | None:
        """
        Generate world resources struct for runtime data.
        
        Creates WorldResources with:
        - Region lookup tables
        - Entity spawning data  
        - Tilemap handles
        - Physics configuration
        """
        
        context = {
            'regions': [self._prepare_region_context(region) for region in regions],
            'entity_lookup': self._generate_entity_lookup_table(regions),
            'spawn_tables': self._generate_spawn_tables(regions),
            'texture_atlas_data': self._generate_texture_atlas_data(regions)
        }
        
        resources_content = self.render_template('world_resources.rs.j2', context)
        resources_path = output_dir / "resources.rs"
        
        if self.write_rust_module(resources_content, resources_path):
            return resources_path
        
        return None
    
    def _prepare_region_context(self, region: ProcessingResult) -> dict[str, Any]:
        """Prepare region data for template context."""
        return {
            'name': region.cluster_name,
            'module_name': self._to_rust_module_name(region.cluster_name),
            'struct_name': self._to_rust_struct_name(region.cluster_name),
            'entity_count': region.entity_count,
            'category': region.cluster_category,
            'biome_data': region.specific_data.get('biome_info', {}),
            'corruption_level': region.specific_data.get('corruption_level', 0.0),
            'threat_level': region.specific_data.get('threat_level', 0.0)
        }
    
    def _generate_physics_layers(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate global physics layer configuration for Avian."""
        layers = {
            'TERRAIN': {'id': 0, 'collides_with': ['PLAYER', 'ENTITIES']},
            'PLAYER': {'id': 1, 'collides_with': ['TERRAIN', 'ENTITIES', 'BARRIERS']}, 
            'ENTITIES': {'id': 2, 'collides_with': ['TERRAIN', 'PLAYER', 'BARRIERS']},
            'BARRIERS': {'id': 3, 'collides_with': ['PLAYER', 'ENTITIES']},
            'TRIGGERS': {'id': 4, 'collides_with': []},  # Only trigger events
        }
        
        return layers
    
    def _generate_tilemap_config(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate bevy_ecs_tilemap configuration."""
        return {
            'tile_size': 64.0,  # Standard hex tile size
            'chunk_size': 64,   # Chunk size for streaming
            'map_type': 'hexagonal',
            'coordinate_system': 'row',
            'layers': {
                'background': {'z_index': 0, 'name': 'terrain'},
                'entities': {'z_index': 10, 'name': 'objects'},
                'ui': {'z_index': 100, 'name': 'interface'}
            }
        }
    
    def _calculate_world_bounds(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Calculate world bounds from all regions."""
        # For now, use default bounds - this could be enhanced with actual region data
        return {
            'min_q': -100,
            'max_q': 100,
            'min_r': -100,
            'max_r': 100,
            'center_q': 0,
            'center_r': 0
        }
    
    def _generate_all_tilemap_layers(self, regions: list[ProcessingResult]) -> list[dict[str, Any]]:
        """Generate tilemap layer definitions for all regions."""
        layers = []
        
        # Background terrain layer
        layers.append({
            'name': 'terrain',
            'z_index': 0,
            'tile_size': 64,
            'chunk_size': 64,
            'layer_type': 'hexagonal'
        })
        
        # Entity object layer
        layers.append({
            'name': 'objects',
            'z_index': 10,
            'tile_size': 64,
            'chunk_size': 64,
            'layer_type': 'hexagonal'
        })
        
        # UI overlay layer
        layers.append({
            'name': 'ui_overlay',
            'z_index': 100,
            'tile_size': 64,
            'chunk_size': 64,
            'layer_type': 'square'  # UI typically uses square grid
        })
        
        return layers
    
    def _generate_global_physics_setup(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate global physics setup for Avian."""
        return {
            'gravity': [0.0, 0.0],  # Top-down game, no gravity
            'physics_scale': 1.0,
            'collision_layers': self._generate_physics_layers(regions),
            'default_material': {
                'restitution': 0.0,
                'friction': 0.5
            }
        }
    
    def _generate_world_resources(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate world resource definitions."""
        return {
            'region_count': len(regions),
            'total_entities': sum(r.entity_count for r in regions),
            'world_seed': 42,  # Could be configurable
            'generation_timestamp': 'generated_at_build_time'
        }
    
    def _generate_entity_lookup_table(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate entity lookup table for runtime spawning."""
        lookup = {}
        
        for region in regions:
            region_name = self._to_rust_module_name(region.cluster_name)
            lookup[region_name] = {
                'spawn_function': f"spawn_{region_name}",
                'entity_count': region.entity_count,
                'biome': region.specific_data.get('biome', 'forest'),
                'corruption_level': region.specific_data.get('corruption_level', 0.0)
            }
        
        return lookup
    
    def _generate_spawn_tables(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate spawn probability tables for each region."""
        spawn_tables = {}
        
        for region in regions:
            region_name = self._to_rust_module_name(region.cluster_name)
            spawn_tables[region_name] = {
                'encounter_rate': 0.1,  # Base encounter rate
                'poi_density': 0.05,    # POI spawn density
                'resource_density': 0.03, # Resource spawn density
            }
        
        return spawn_tables
    
    def _generate_texture_atlas_data(self, regions: list[ProcessingResult]) -> dict[str, Any]:
        """Generate texture atlas configuration for bevy_ecs_tilemap."""
        atlas_data = {
            'atlas_size': [1024, 1024],  # Standard atlas size
            'tile_size': [64, 64],       # Individual tile size
            'tiles_per_row': 16,         # 1024 / 64
            'biome_tiles': {},
            'entity_sprites': {},
            'ui_elements': {}
        }
        
        tile_index = 0
        
        # Assign tile indices for each region's biomes
        for region in regions:
            region_name = self._to_rust_module_name(region.cluster_name)
            biome = region.specific_data.get('biome', 'forest')
            
            if biome not in atlas_data['biome_tiles']:
                atlas_data['biome_tiles'][biome] = {
                    'base_index': tile_index,
                    'variant_count': 3,  # 3 variants per biome
                    'regions': []
                }
                tile_index += 3
            
            atlas_data['biome_tiles'][biome]['regions'].append(region_name)
        
        return atlas_data
