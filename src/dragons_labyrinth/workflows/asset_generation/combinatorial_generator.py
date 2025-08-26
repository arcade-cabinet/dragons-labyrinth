"""
Combinatorial Generator Module - Focused variant combination generation.
Handles exponential asset creation from variant dimensions.
"""

import itertools
import math
from typing import Any, Dict, List
import toml

from dragons_labyrinth.models import (
    VariantAssetGenerationState, VariantAssetSpec, CombinatorialGeneration,
    SpriteSheetMetadata
)


class CombinatorialGenerator:
    """
    Focused combinatorial variant generator.
    Creates all valid combinations of variants for each archetype.
    """
    
    def generate_combinations(self, state: VariantAssetGenerationState) -> Dict[str, Any]:
        """Generate all combinatorial variant specifications for all archetypes."""
        
        print("  🎨 Loading asset definitions from TOML")
        
        # Load TOML again to get asset definitions
        with open(state.toml_spec_path, 'r') as f:
            raw_toml = toml.load(f)
        
        assets_section = raw_toml.get('assets', {})
        combinatorial_results = {}
        total_variants_planned = 0
        
        for asset_name, asset_data in assets_section.items():
            base_archetype = asset_data.get('archetype', asset_name)
            base_prompt = asset_data.get('base_prompt', asset_data.get('prompt', ''))
            variants_to_use = asset_data.get('variants', [])
            
            print(f"    🎭 Processing archetype: {base_archetype}")
            
            # Generate all combinations for this archetype
            combinations = self._generate_variant_combinations(
                state.variant_config, 
                variants_to_use
            )
            
            # Create variant specs
            generated_specs = []
            for i, combination in enumerate(combinations):
                if i >= state.variant_config.max_variants_per_archetype:
                    break
                    
                # Apply substitutions to prompt
                final_prompt = self._apply_variant_substitutions(
                    base_prompt, 
                    combination, 
                    state.variant_config.dimension_descriptors
                )
                
                # Generate variant name
                variant_name = self._generate_variant_name(
                    base_archetype,
                    combination,
                    state.variant_config.naming_convention
                )
                
                # Create variant spec
                variant_spec = VariantAssetSpec(
                    asset_name=variant_name,
                    base_archetype=base_archetype,
                    variant_combination=combination,
                    final_prompt=final_prompt,
                    resolution=state.resolution_tiers[state.variant_config.resolution_tier].resolution,
                    asset_category=state.asset_category,
                    asset_type=asset_data.get('type', 'unknown'),
                    layer_type=asset_data.get('layer_type', 'base'),
                    priority=asset_data.get('priority', 5),
                    sprite_sheet_group=f"{state.asset_category}_{base_archetype}"
                )
                
                generated_specs.append(variant_spec)
            
            # Plan sprite sheets for this archetype
            sprite_sheets = self._plan_sprite_sheets(
                generated_specs,
                state.variant_config.resolution_tier,
                state.resolution_tiers
            )
            
            # Create combinatorial generation result
            combinatorial_result = CombinatorialGeneration(
                base_archetype=base_archetype,
                variant_config=state.variant_config,
                generated_specs=generated_specs,
                total_combinations=len(combinations),
                generated_combinations=len(generated_specs),
                excluded_combinations=len(combinations) - len(generated_specs),
                sprite_sheets=sprite_sheets,
                total_sprite_sheets=len(sprite_sheets),
                estimated_generation_time=len(generated_specs) * 3.0,  # 3 seconds per variant
                estimated_cost_usd=len(generated_specs) * 0.04,  # $0.04 per DALL-E generation
                estimated_file_size_mb=len(generated_specs) * 0.5  # ~0.5MB per 512x512 image
            )
            
            combinatorial_results[base_archetype] = combinatorial_result
            total_variants_planned += len(generated_specs)
            
            print(f"      ✅ {len(generated_specs)} variants planned ({len(sprite_sheets)} sprite sheets)")
        
        total_cost = sum(r.estimated_cost_usd for r in combinatorial_results.values())
        print(f"  🎯 Total variants planned: {total_variants_planned}")
        print(f"  💰 Estimated cost: ${total_cost:.2f}")
        
        return {
            "combinatorial_results": combinatorial_results,
            "total_variants_planned": total_variants_planned,
            "step_count": state.step_count + 1
        }
    
    def _generate_variant_combinations(self, config, variants_to_use: List[str]) -> List[Dict[str, str]]:
        """Generate all valid variant combinations for specified dimensions."""
        
        # Get dimension values for specified variants
        dimension_values = []
        dimension_names = []
        
        for variant_name in variants_to_use:
            if variant_name in config.dimensions:
                dimension = config.dimensions[variant_name]
                dimension_values.append(dimension.possible_values)
                dimension_names.append(variant_name)
        
        # Generate all combinations
        all_combinations = []
        for values in itertools.product(*dimension_values):
            combination = dict(zip(dimension_names, values))
            
            # Check exclusion rules
            if not self._is_combination_excluded(combination, config.exclude_combinations):
                all_combinations.append(combination)
        
        # Prioritize combinations with priority dimensions first
        if config.priority_dimensions:
            all_combinations.sort(key=lambda c: sum(
                1 for dim in config.priority_dimensions if dim in c
            ), reverse=True)
        
        return all_combinations
    
    def _is_combination_excluded(self, combination: Dict[str, str], exclusion_rules: List[List[str]]) -> bool:
        """Check if a combination matches any exclusion rule."""
        for rule in exclusion_rules:
            if len(rule) >= 2:
                dim_name, excluded_value = rule[0], rule[1]
                if combination.get(dim_name) == excluded_value:
                    return True
        return False
    
    def _apply_variant_substitutions(
        self, 
        base_prompt: str, 
        combination: Dict[str, str],
        descriptors: Dict[str, Dict[str, str]]
    ) -> str:
        """Apply variant substitutions to prompt template."""
        
        final_prompt = base_prompt
        
        for dim_name, value in combination.items():
            placeholder = f"{{{dim_name}}}"
            
            # Get descriptor for this value if available
            if dim_name in descriptors and value in descriptors[dim_name]:
                replacement = descriptors[dim_name][value]
            else:
                replacement = value
            
            final_prompt = final_prompt.replace(placeholder, replacement)
        
        return final_prompt
    
    def _generate_variant_name(
        self,
        base_archetype: str,
        combination: Dict[str, str], 
        naming_convention: str
    ) -> str:
        """Generate variant name using naming convention."""
        
        # Replace archetype placeholder
        name = naming_convention.replace("{archetype}", base_archetype)
        
        # Replace other placeholders with combination values
        for dim_name, value in combination.items():
            placeholder = f"{{{dim_name}}}"
            name = name.replace(placeholder, value)
        
        # Clean up any remaining placeholders or extra underscores
        name = name.replace("{", "").replace("}", "")
        while "__" in name:
            name = name.replace("__", "_")
        
        return name.strip("_")
    
    def _plan_sprite_sheets(self, specs: List[VariantAssetSpec], resolution_tier: str, resolution_tiers) -> List[SpriteSheetMetadata]:
        """Plan sprite sheet layout for variants."""
        
        tier = resolution_tiers[resolution_tier]
        if not tier.sprite_sheet_compatible:
            return []
        
        # Parse resolution
        width, height = map(int, tier.resolution.split('x'))
        
        # Plan grid layout (simple square grid for now)
        variants_per_sheet = 16  # 4x4 grid
        total_sheets = math.ceil(len(specs) / variants_per_sheet)
        
        sprite_sheets = []
        for sheet_idx in range(total_sheets):
            sheet_specs = specs[sheet_idx * variants_per_sheet:(sheet_idx + 1) * variants_per_sheet]
            
            # Calculate grid size
            grid_cols = min(4, len(sheet_specs))
            grid_rows = math.ceil(len(sheet_specs) / grid_cols)
            
            sheet_metadata = SpriteSheetMetadata(
                sheet_name=f"{specs[0].sprite_sheet_group}_sheet_{sheet_idx}",
                sheet_category=specs[0].asset_category,
                base_archetype=specs[0].base_archetype,
                sheet_size=(width * grid_cols, height * grid_rows),
                cell_size=(width, height),
                grid_size=(grid_cols, grid_rows),
                total_variants=len(sheet_specs),
                resolution_tier=resolution_tier
            )
            
            sprite_sheets.append(sheet_metadata)
        
        return sprite_sheets
