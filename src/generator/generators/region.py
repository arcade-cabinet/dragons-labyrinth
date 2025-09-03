"""
Region Generator - Regional-level world generation for Dragon's Labyrinth.

Generates region modules with hex tile organization, biome management, and 
local physics/tilemap configuration for bevy_ecs_tilemap integration.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from generator.entities.generators.base import BaseGenerator
from generator.entities.models import ProcessingResult


class RegionGenerator(BaseGenerator):
    """
    Region-level generator for organizing hex tiles within regions.
    
    Responsible for:
    - Region mod.rs with all hex tile imports
    - Regional tilemap layer configuration
    - Biome transition management
    - Regional physics setup
    - Cross-tile coordination within region
    """
    
    def __init__(self):
        super().__init__("region")
    
    def generate_region_module(self, region_result: ProcessingResult, 
                              hex_tiles: list[ProcessingResult],
                              output_dir: Path) -> Path:
        """
        Generate complete region module with all hex tiles using actual RegionData from processor.
        
        Creates:
        - regions/{region_name}/mod.rs with hex tile imports
        - Regional biome configuration from actual biome_distribution
        - Local tilemap setup using dominant_biome
        - Region-specific physics layers based on corruption_level
        """
        
        # Extract actual RegionData from processor
        region_data = region_result.specific_data
        if not hasattr(region_data, 'name'):
            raise ValueError(f"Expected RegionData from RegionsProcessor, got: {type(region_data)}")
        
        region_name = self._to_rust_module_name(region_data.name)
        region_dir = output_dir / "regions" / region_name
        
        # Use actual analyzed data from RegionsProcessor
        context = {
            'region_name': region_data.name,
            'region_module': region_name,
            'region_struct': self._to_rust_struct_name(region_data.name),
            'hex_tiles': [self._prepare_hex_tile_context(tile) for tile in hex_tiles],
            'dominant_biome': region_data.dominant_biome.value,
            'biome_distribution': region_data.biome_distribution,
            'corruption_level': region_data.corruption_level.value,
            'settlement_locations': region_data.settlement_locations,
            'geographic_features': {
                'rivers': region_data.geographic_features.rivers,
                'trails': region_data.geographic_features.trails,
                'harbors': region_data.geographic_features.harbors,
                'borders': region_data.geographic_features.borders,
                'bridges': region_data.geographic_features.bridges,
                'roads': region_data.geographic_features.roads
            },
            'total_hexes': region_data.total_hexes,
            'settlement_density': region_data.settlement_density,
            'region_bounds': self._calculate_region_bounds_from_hex_count(region_data.total_hexes),
            'tilemap_layers': self._generate_region_tilemap_layers_from_data(region_data),
            'physics_config': self._generate_region_physics_from_data(region_data)
        }
        
        # Generate region mod.rs
        region_mod_content = self.render_template('region_mod.rs.j2', context)
        region_mod_path = region_dir / "mod.rs"
        
        return self.write_rust_module(region_mod_content, region_mod_path)
    
    def generate_region_biome_images(self, region_result: ProcessingResult,
                                   output_dir: Path) -> list[Path]:
        """
        Generate biome-specific hex tile images for this region using actual RegionData.
        
        Uses OpenAI gpt-image-1 to create:
        - Dominant biome hex tiles
        - Biome distribution variants
        - Corruption-level appropriate overlays
        """
        # Extract actual RegionData
        region_data = region_result.specific_data
        region_name = self._to_rust_module_name(region_data.name)
        
        # Use actual analyzed data
        dominant_biome = region_data.dominant_biome.value
        corruption_level = region_data.corruption_level.value / 5.0  # Normalize to 0-1
        
        generated_images = []
        image_dir = output_dir / "assets" / "textures" / "biomes" / region_name
        
        # Generate variants for dominant biome + any secondary biomes from distribution
        biomes_to_generate = [dominant_biome]
        
        # Add secondary biomes from actual distribution
        for biome_name, count in region_data.biome_distribution.items():
            if biome_name != dominant_biome and count > 0:
                biomes_to_generate.append(biome_name)
        
        # Limit to 3 most common biomes
        biomes_to_generate = biomes_to_generate[:3]
        
        for biome_idx, biome in enumerate(biomes_to_generate):
            prompt = self._generate_biome_tile_prompt(biome, corruption_level, f"variant_{biome_idx}")
            image_path = image_dir / f"{region_name}_{biome}.png"
            
            generated_path = self.generate_image(prompt, image_path)
            generated_images.append(generated_path)
        
        return generated_images
    
    def _prepare_hex_tile_context(self, tile: ProcessingResult) -> dict[str, Any]:
        """Prepare hex tile data for template context."""
        return {
            'name': tile.cluster_name,
            'module_name': self._to_rust_module_name(tile.cluster_name),
            'struct_name': self._to_rust_struct_name(tile.cluster_name),
            'coordinates': self._extract_hex_coordinates(tile.specific_data),
            'entities': tile.entity_count,
            'terrain_type': tile.specific_data.get('terrain', 'plains')
        }
    
    def _calculate_region_bounds_from_hex_count(self, total_hexes: int) -> dict[str, Any]:
        """Calculate approximate region bounds from total hex count."""
        # Estimate region size - assume roughly square region
        side_length = int((total_hexes ** 0.5)) + 1
        
        return {
            'min_q': 0,
            'max_q': side_length,
            'min_r': 0,
            'max_r': side_length,
            'center_q': side_length // 2,
            'center_r': side_length // 2,
            'total_hexes': total_hexes
        }
    
    def _generate_region_tilemap_layers_from_data(self, region_data) -> list[dict[str, Any]]:
        """Generate tilemap layer configuration using actual RegionData."""
        region_name = self._to_rust_module_name(region_data.name)
        
        return [{
            'name': f"{region_name}_terrain",
            'z_index': 0,
            'tile_count': region_data.total_hexes,
            'dominant_biome': region_data.dominant_biome.value,
            'corruption_level': region_data.corruption_level.value,
            'settlement_density': region_data.settlement_density,
            'connectivity_score': self._calculate_connectivity_from_features(region_data.geographic_features)
        }]
    
    def _generate_region_physics_from_data(self, region_data) -> dict[str, Any]:
        """Generate physics configuration using actual RegionData."""
        corruption_level = region_data.corruption_level.value / 5.0  # Normalize to 0-1
        
        # Movement modifier based on terrain and corruption
        base_movement = 1.0
        if region_data.dominant_biome.value in ['black_swamp', 'fungal_cathedral']:
            base_movement = 0.7
        elif region_data.dominant_biome.value in ['rust_plains', 'bone_forest']:
            base_movement = 0.8
        elif region_data.dominant_biome.value == 'dragon_scar':
            base_movement = 0.4
        
        # Connectivity improves movement
        connectivity = self._calculate_connectivity_from_features(region_data.geographic_features)
        movement_bonus = connectivity * 0.2
        
        return {
            'movement_modifier': max(0.3, base_movement - corruption_level * 0.4 + movement_bonus),
            'visibility_modifier': max(0.2, 1.0 - corruption_level * 0.6),
            'encounter_rate_modifier': 0.5 + corruption_level * 1.5,
            'ambient_danger': corruption_level > 0.4,
            'infrastructure_bonus': connectivity > 0.5,
            'settlement_safety': region_data.settlement_density > 0.1
        }
    
    def _calculate_connectivity_from_features(self, geographic_features) -> float:
        """Calculate connectivity score from actual geographic features."""
        score = 0.0
        score += geographic_features.rivers * 0.15
        score += geographic_features.trails * 0.25
        score += geographic_features.roads * 0.35
        score += geographic_features.harbors * 0.2
        score += geographic_features.bridges * 0.1
        score -= geographic_features.borders * 0.05  # Borders reduce connectivity
        
        return max(0.0, min(1.0, score / 10.0))  # Normalize
    
    def _generate_biome_tile_prompt(self, biome: str, corruption_level: float, 
                                  variant: str) -> str:
        """Generate prompt for biome tile image generation."""
        base_prompts = {
            'wet_meadow': 'lush green meadow with morning dew, peaceful pastoral setting',
            'ashen_forest': 'gray leafless forest with ash-covered ground, somber atmosphere',
            'flooded_village': 'partially submerged village ruins, reflective water',
            'black_swamp': 'dark murky swampland with twisted trees, ominous shadows',
            'fungal_cathedral': 'massive mushroom formations creating cathedral-like space',
            'rust_plains': 'rust-colored barren plains with metallic debris scattered',
            'famine_fields': 'withered croplands with dried stalks, desolate farming area',
            'bone_forest': 'forest of white bone-like trees, skeletal branches',
            'dragon_scar': 'scorched earth with glass formations, dragon burn marks',
            'abyssal_chasm': 'deep dark chasm with otherworldly emanations',
            'forest': 'standard green forest with tall trees'
        }
        
        base_prompt = base_prompts.get(biome, base_prompts['forest'])
        
        # Add corruption effects
        if corruption_level > 0.7:
            corruption_desc = ', deeply corrupted with unnatural colors and twisted formations'
        elif corruption_level > 0.4:
            corruption_desc = ', showing signs of corruption and decay'
        elif corruption_level > 0.1:
            corruption_desc = ', with subtle signs of wrongness'
        else:
            corruption_desc = ', pristine and natural'
        
        # Add variant details
        variant_details = {
            'variant_0': ', wide angle view',
            'variant_1': ', detailed close-up perspective', 
            'variant_2': ', atmospheric lighting with shadows'
        }
        variant_desc = variant_details.get(variant, '')
        
        return f"Create a hexagonal tile texture showing {base_prompt}{corruption_desc}{variant_desc}. Top-down perspective, seamless edges for tiling, 64x64 pixel art style, no UI elements or text."
