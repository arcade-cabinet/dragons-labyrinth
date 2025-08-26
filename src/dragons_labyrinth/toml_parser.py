"""
TOML Parser Module - Focused variant TOML specification parsing.
Handles universal variant TOML files with combinatorial generation rules.
"""

from pathlib import Path
from importlib.resources import files as pkg_files
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
        self.resolution_tiers = self._load_resolution_tiers_from_global_style_guide()
    
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
        
        # Extract optional code-generation extras (category-specific)
        codegen_extras = raw_toml.get('codegen', {})
        prompt_category = Path(state.toml_spec_path).parent.name
        
        return {
            "variant_config": variant_config,
            "resolution_tiers": self.resolution_tiers,
            "codegen_extras": codegen_extras,
            "prompt_category": prompt_category,
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
        """Determine resolution tier from GLOBAL_STYLE_GUIDE based on asset category."""
        
        # Get asset category from batch info
        batch_info = raw_toml.get('batch', {})
        category = batch_info.get('category', 'unknown')
        
        # Map categories to resolution tiers based on GLOBAL_STYLE_GUIDE
        category_to_tier = {
            'biome': 'hex_standard',      # 128x128 hex tiles
            'character': 'hex_standard',  # 128x128 character tokens 
            'monster': 'hex_standard',    # 128x128 character tokens
            'feature': 'hex_standard',    # 128x128 buildings and overlays
            'path': 'hex_standard',       # 128x128 path overlays
            'bridge': 'hex_standard',     # 128x128 bridge overlays
            'effect': 'hex_standard',     # 128x128 effect overlays
            'ui': 'ui_buttons',           # 64x64 UI buttons
            'item': 'hex_standard'        # 128x128 item tokens
        }
        
        return category_to_tier.get(category, 'hex_standard')
    
    def _load_resolution_tiers_from_global_style_guide(self) -> Dict[str, ResolutionTier]:
        """Load resolution tiers from GLOBAL_STYLE_GUIDE.toml (REQUIRED)."""
        
        # Resolve packaged resource via importlib.resources
        try:
            resource = pkg_files("dragons_labyrinth.workflows.asset_generation.prompts").joinpath("GLOBAL_STYLE_GUIDE.toml")
            with resource.open("r") as f:
                global_style = toml.load(f)
        except Exception as e:
            raise FileNotFoundError("GLOBAL_STYLE_GUIDE.toml not found in packaged resources") from e
        
        # Extract resolution mappings (REQUIRED)
        resolution_usage = global_style.get('resolution_usage')
        if not resolution_usage:
            raise ValueError("resolution_usage section missing from GLOBAL_STYLE_GUIDE.toml")
        
        return {
            "hex_standard": ResolutionTier(
                tier_name="hex_standard",
                resolution=resolution_usage['hex_tiles'],
                use_case="Hex tiles, characters, features",
                batch_size_multiplier=2.0,
                sprite_sheet_compatible=True
            ),
            "ui_buttons": ResolutionTier(
                tier_name="ui_buttons", 
                resolution=resolution_usage['ui_buttons'],
                use_case="UI buttons and interface elements",
                batch_size_multiplier=3.0,
                sprite_sheet_compatible=True
            ),
            "ui_icons": ResolutionTier(
                tier_name="ui_icons",
                resolution=resolution_usage['ui_icons'],
                use_case="Small UI icons",
                batch_size_multiplier=4.0,
                sprite_sheet_compatible=True
            ),
            "multi_hex": ResolutionTier(
                tier_name="multi_hex",
                resolution=resolution_usage['large_features'],
                use_case="Large multi-hex features",
                batch_size_multiplier=1.0,
                sprite_sheet_compatible=True
            )
        }
