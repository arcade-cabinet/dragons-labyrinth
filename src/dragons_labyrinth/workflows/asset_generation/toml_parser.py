"""
TOML Parser Module - Focused variant TOML specification parsing.
Handles universal variant TOML files with combinatorial generation rules.
"""

from pathlib import Path
from typing import Any, Dict
import toml

from dragons_labyrinth.models import (
    VariantAssetGenerationState, VariantConfiguration,
    VariantDimension, ResolutionTier
)


class VariantTOMLParser:
    """
    Focused TOML parser for variant asset specifications.
    Handles parsing of universal variant TOML files into structured configuration.
    """
    
    def __init__(self):
        # HEX-BASED RESOLUTION STRATEGY (Following game design principles)
        # Standard hex tile: 128x128 - Large enough for detail, not ridiculous for movement
        # UI elements follow proper design ratios relative to hex tiles
        self.resolution_tiers = {
            "hex_standard": ResolutionTier(
                tier_name="hex_standard",
                resolution="128x128",
                use_case="Standard hex tiles, characters, features - the main game scale",
                batch_size_multiplier=2.0,
                sprite_sheet_compatible=True
            ),
            "ui_buttons": ResolutionTier(
                tier_name="ui_buttons", 
                resolution="64x64",
                use_case="UI buttons and interface elements (1/2 hex ratio)",
                batch_size_multiplier=3.0,
                sprite_sheet_compatible=True
            ),
            "ui_icons": ResolutionTier(
                tier_name="ui_icons",
                resolution="32x32", 
                use_case="Small UI icons and indicators (1/4 hex ratio)",
                batch_size_multiplier=4.0,
                sprite_sheet_compatible=True
            ),
            "multi_hex": ResolutionTier(
                tier_name="multi_hex",
                resolution="256x256",
                use_case="Large features spanning 2x2 hexes (dragon lairs, cities)",
                batch_size_multiplier=1.0,
                sprite_sheet_compatible=True
            )
        }
    
    def parse_variant_toml(self, state: VariantAssetGenerationState) -> Dict[str, Any]:
        """Parse universal variant TOML into structured configuration."""
        
        print(f"  📋 Loading TOML from {state.toml_spec_path}")
        
        # Load raw TOML
        with open(state.toml_spec_path, 'r') as f:
            raw_toml = toml.load(f)
        
        # Parse variant dimensions
        variant_dimensions = self._parse_variant_dimensions(raw_toml)
        
        # Parse dimension descriptors for prompt substitution
        dimension_descriptors = self._parse_dimension_descriptors(raw_toml)
        
        # Parse generation rules
        generation_rules = raw_toml.get('generation_rules', {})
        max_variants = generation_rules.get('max_variants_per_archetype', 30)
        priority_dims = generation_rules.get('priority_variants', [])
        naming_convention = generation_rules.get('naming_convention', "{archetype}_{variant_1}_{variant_2}")
        sprite_sheet_grouping = generation_rules.get('sprite_sheet_grouping', 'archetype')
        
        # Determine resolution tier
        resolution_tier = self._determine_resolution_tier(raw_toml)
        
        # Create variant configuration
        variant_config = VariantConfiguration(
            dimensions=variant_dimensions,
            dimension_descriptors=dimension_descriptors,
            max_variants_per_archetype=max_variants,
            priority_dimensions=priority_dims,
            sprite_sheet_grouping=sprite_sheet_grouping,
            naming_convention=naming_convention,
            resolution_tier=resolution_tier
        )
        
        print(f"  ✅ Parsed {len(variant_dimensions)} variant dimensions")
        print(f"  🎯 Resolution tier: {resolution_tier}")
        
        return {
            "variant_config": variant_config,
            "resolution_tiers": self.resolution_tiers,
            "step_count": state.step_count + 1
        }
    
    def _parse_variant_dimensions(self, raw_toml: Dict[str, Any]) -> Dict[str, VariantDimension]:
        """Parse variant dimensions from TOML variants section."""
        
        variants_section = raw_toml.get('variants', {})
        variant_dimensions = {}
        
        for dim_name, values in variants_section.items():
            if isinstance(values, list):
                variant_dimensions[dim_name] = VariantDimension(
                    dimension_name=dim_name,
                    possible_values=values,
                    description=f"Variant dimension: {dim_name}"
                )
        
        return variant_dimensions
    
    def _parse_dimension_descriptors(self, raw_toml: Dict[str, Any]) -> Dict[str, Dict[str, str]]:
        """Parse dimension descriptors for prompt substitution."""
        
        dimension_descriptors = {}
        
        for section_name, section_data in raw_toml.items():
            if section_name.endswith('_descriptors') and isinstance(section_data, dict):
                dim_name = section_name.replace('_descriptors', '')
                dimension_descriptors[dim_name] = section_data
        
        return dimension_descriptors
    
    def _determine_resolution_tier(self, raw_toml: Dict[str, Any]) -> str:
        """Determine the appropriate resolution tier for this asset type."""
        
        style_constraints = raw_toml.get('style_constraints', {})
        resolution_from_toml = style_constraints.get('resolution', '1024x1024')
        
        # Find matching resolution tier
        for tier_name, tier in self.resolution_tiers.items():
            if tier.resolution == resolution_from_toml:
                return tier_name
        
        # Default fallback based on common patterns
        if '256' in resolution_from_toml:
            return 'ui_elements'
        elif '512' in resolution_from_toml:
            return 'character_tokens'
        elif '768' in resolution_from_toml:
            return 'feature_overlays'
        else:
            return 'biome_tiles'  # Default to biome tiles for 1024x1024+
