"""
Variant Asset Generation Workflow - Revolutionary AI-Generated Game Assets using DALL-E.

This workflow generates game assets using the new variant system:
- Universal variant TOML specification parsing
- Combinatorial variant generation (exponential asset creation)
- LangChain DALL-E tool integration for image generation
- Automatic sprite sheet generation and validation with Pillow
- Resolution optimization based on asset type
- Human review checkpoints for quality control
- Layer cake system integration with proper priorities

Following Professor Pixel workflow patterns with durable execution.
"""

import uuid
import itertools
import math
import json
from datetime import datetime
from pathlib import Path
from typing import Literal, Any, Dict, List, Tuple
import logging

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
from langchain_community.tools.dalle_image_generator import OpenAIDALLEImageGenerationTool
from pydantic import BaseModel, Field
import toml
from PIL import Image, ImageDraw, ImageFont
import requests
from io import BytesIO

from dragons_labyrinth.models import (
    VariantAssetGenerationState, VariantAssetSpec, CombinatorialGeneration,
    SpriteSheetMetadata, SpriteSheetCell, VariantConfiguration,
    VariantDimension, ResolutionTier, VariantAssetGenerationResult
)


class VariantAssetGenerationWorkflow:
    """
    Revolutionary workflow for generating variant-based game assets.
    
    Key features:
    1. Combinatorial variant generation from TOML specs
    2. Resolution optimization (256x256 UI, 512x512 tokens, 1024x1024 tiles)
    3. Automatic sprite sheet generation with Pillow
    4. Sprite sheet validation and splitting
    5. Atlas generation with JSON metadata
    6. Batch processing with human review checkpoints
    """
    
    def __init__(self):
        self.dalle_tool = OpenAIDALLEImageGenerationTool()
        
        # Resolution optimization tiers
        self.resolution_tiers = {
            "ui_elements": ResolutionTier(
                tier_name="ui_elements",
                resolution="256x256",
                use_case="Small UI components and icons",
                batch_size_multiplier=2.0,
                sprite_sheet_compatible=True
            ),
            "character_tokens": ResolutionTier(
                tier_name="character_tokens", 
                resolution="512x512",
                use_case="Character and monster tokens",
                batch_size_multiplier=1.5,
                sprite_sheet_compatible=True
            ),
            "biome_tiles": ResolutionTier(
                tier_name="biome_tiles",
                resolution="1024x1024", 
                use_case="Biome tiles that need seamless tiling",
                batch_size_multiplier=0.5,
                sprite_sheet_compatible=False
            ),
            "feature_overlays": ResolutionTier(
                tier_name="feature_overlays",
                resolution="768x768",
                use_case="Buildings and dungeon features",
                batch_size_multiplier=1.0,
                sprite_sheet_compatible=True
            ),
            "effect_overlays": ResolutionTier(
                tier_name="effect_overlays",
                resolution="512x512",
                use_case="Particle effects and atmospheric overlays",
                batch_size_multiplier=1.5,
                sprite_sheet_compatible=True
            )
        }
    
    def parse_variant_toml_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Parse universal variant TOML specifications into combinatorial generation plan."""
        
        print(f"🔍 Parsing variant TOML from {state.toml_spec_path}")
        
        # Load TOML file
        with open(state.toml_spec_path, 'r') as f:
            raw_toml = toml.load(f)
        
        # Parse variant dimensions
        variants_section = raw_toml.get('variants', {})
        variant_dimensions = {}
        
        for dim_name, values in variants_section.items():
            if isinstance(values, list):
                variant_dimensions[dim_name] = VariantDimension(
                    dimension_name=dim_name,
                    possible_values=values,
                    description=f"Variant dimension: {dim_name}"
                )
        
        # Parse dimension descriptors for prompt substitution
        dimension_descriptors = {}
        for section_name, section_data in raw_toml.items():
            if section_name.endswith('_descriptors'):
                dim_name = section_name.replace('_descriptors', '')
                dimension_descriptors[dim_name] = section_data
        
        # Parse generation rules
        generation_rules = raw_toml.get('generation_rules', {})
        max_variants = generation_rules.get('max_variants_per_archetype', 30)
        priority_dims = generation_rules.get('priority_variants', [])
        naming_convention = generation_rules.get('naming_convention', 
                                                "{archetype}_{variant_1}_{variant_2}")
        sprite_sheet_grouping = generation_rules.get('sprite_sheet_grouping', 'archetype')
        
        # Determine resolution tier
        style_constraints = raw_toml.get('style_constraints', {})
        resolution_from_toml = style_constraints.get('resolution', '1024x1024')
        
        resolution_tier = 'character_tokens'  # Default
        for tier_name, tier in self.resolution_tiers.items():
            if tier.resolution == resolution_from_toml:
                resolution_tier = tier_name
                break
        
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
        
        print(f"✅ Parsed {len(variant_dimensions)} variant dimensions")
        print(f"🎯 Resolution tier: {resolution_tier} ({self.resolution_tiers[resolution_tier].resolution})")
        
        return {
            "variant_config": variant_config,
            "resolution_tiers": self.resolution_tiers,
            "step_count": state.step_count + 1
        }
    
    def generate_combinatorial_specs_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Generate all combinatorial variant specifications."""
        
        print(f"🧮 Generating combinatorial variant specifications")
        
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
            
            print(f"  🎨 Processing archetype: {base_archetype}")
            
            # Generate all combinations for this archetype
            combinations = self._generate_variant_combinations(
                state.variant_config, 
                variants_to_use
            )
            
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
                    resolution=self.resolution_tiers[state.variant_config.resolution_tier].resolution,
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
                state.variant_config.resolution_tier
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
            
            print(f"    ✅ {len(generated_specs)} variants planned ({len(sprite_sheets)} sprite sheets)")
        
        print(f"🎯 Total variants planned: {total_variants_planned}")
        print(f"💰 Estimated cost: ${sum(r.estimated_cost_usd for r in combinatorial_results.values()):.2f}")
        
        return {
            "combinatorial_results": combinatorial_results,
            "total_variants_planned": total_variants_planned,
            "step_count": state.step_count + 1
        }
    
    def generate_variant_batch_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Generate variant assets using DALL-E in optimized batches."""
        
        print(f"🎨 Generating {state.total_variants_planned} variants using DALL-E")
        
        generated_variants = {}
        failed_generations = []
        generation_metadata = {}
        
        # Collect all variant specs across archetypes
        all_specs = []
        for result in state.combinatorial_results.values():
            all_specs.extend(result.generated_specs)
        
        # Adjust batch size based on resolution tier
        tier = self.resolution_tiers[state.variant_config.resolution_tier]
        adjusted_batch_size = max(1, int(state.batch_size * tier.batch_size_multiplier))
        
        print(f"📦 Using batch size: {adjusted_batch_size} (adjusted for {tier.tier_name})")
        
        # Process in batches
        for i in range(0, len(all_specs), adjusted_batch_size):
            batch = all_specs[i:i + adjusted_batch_size]
            batch_num = (i // adjusted_batch_size) + 1
            total_batches = (len(all_specs) + adjusted_batch_size - 1) // adjusted_batch_size
            
            print(f"📦 Processing batch {batch_num}/{total_batches} ({len(batch)} variants)")
            
            for spec in batch:
                try:
                    # Generate image using DALL-E
                    dalle_params = {
                        "query": spec.final_prompt,
                        "size": spec.resolution,
                        "quality": tier.quality_override or spec.quality,
                        "style": tier.style_override or spec.style
                    }
                    
                    result = self.dalle_tool.run(dalle_params)
                    
                    # Download and save image
                    if isinstance(result, str) and result.startswith('http'):
                        response = requests.get(result)
                        if response.status_code == 200:
                            # Save individual variant
                            output_path = Path(state.output_dir) / "variants" / f"{spec.asset_name}.png"
                            output_path.parent.mkdir(parents=True, exist_ok=True)
                            
                            with open(output_path, 'wb') as f:
                                f.write(response.content)
                            
                            generated_variants[spec.asset_name] = str(output_path)
                            
                            # Store metadata
                            generation_metadata[spec.asset_name] = {
                                "base_archetype": spec.base_archetype,
                                "variant_combination": spec.variant_combination,
                                "resolution": spec.resolution,
                                "sprite_sheet_group": spec.sprite_sheet_group,
                                "dalle_params": dalle_params,
                                "timestamp": datetime.now().isoformat()
                            }
                            
                            print(f"    ✅ {spec.asset_name}")
                        else:
                            raise Exception(f"Failed to download image: {response.status_code}")
                    else:
                        raise Exception(f"Unexpected DALL-E result: {result}")
                        
                except Exception as e:
                    print(f"    ❌ Failed {spec.asset_name}: {e}")
                    failed_generations.append(spec.asset_name)
        
        success_count = len(generated_variants)
        fail_count = len(failed_generations)
        
        print(f"🎯 Variant generation complete: {success_count} success, {fail_count} failed")
        
        return {
            "generated_variants": generated_variants,
            "generation_metadata": generation_metadata,
            "failed_generations": failed_generations,
            "api_calls_made": state.api_calls_made + success_count + fail_count,
            "total_cost_usd": state.total_cost_usd + (success_count * 0.04),  # Rough cost estimate
            "step_count": state.step_count + 1
        }
    
    def generate_sprite_sheets_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Generate sprite sheets from individual variants using Pillow."""
        
        if not self.resolution_tiers[state.variant_config.resolution_tier].sprite_sheet_compatible:
            print(f"🚫 Skipping sprite sheets - {state.variant_config.resolution_tier} not compatible")
            return {"step_count": state.step_count + 1}
        
        print(f"🎨 Generating sprite sheets using Pillow")
        
        sprite_sheets_generated = {}
        atlas_metadata = {}
        
        # Group variants by sprite sheet group
        variant_groups = {}
        for variant_name, variant_path in state.generated_variants.items():
            metadata = state.generation_metadata.get(variant_name, {})
            group = metadata.get("sprite_sheet_group", "default")
            
            if group not in variant_groups:
                variant_groups[group] = []
            variant_groups[group].append((variant_name, variant_path, metadata))
        
        for group_name, variants in variant_groups.items():
            print(f"  📄 Creating sprite sheet: {group_name}")
            
            try:
                sheet_path, atlas_data = self._create_sprite_sheet(
                    group_name,
                    variants,
                    state.output_dir,
                    self.resolution_tiers[state.variant_config.resolution_tier]
                )
                
                sprite_sheets_generated[group_name] = sheet_path
                atlas_metadata[group_name] = atlas_data
                
                print(f"    ✅ Generated: {Path(sheet_path).name} ({len(variants)} variants)")
                
            except Exception as e:
                print(f"    ❌ Failed to create sprite sheet {group_name}: {e}")
        
        print(f"🎯 Sprite sheet generation complete: {len(sprite_sheets_generated)} sheets")
        
        return {
            "sprite_sheets_generated": sprite_sheets_generated,
            "atlas_metadata": atlas_metadata,
            "step_count": state.step_count + 1
        }
    
    def human_review_variants_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Human review of generated variants and sprite sheets."""
        
        if state.autonomous_mode:
            print("🤖 Autonomous mode: Skipping human review")
            return {
                "human_approval": "approved",
                "step_count": state.step_count + 1
            }
        
        # Prepare review data
        review_data = {
            "workflow_id": state.workflow_id,
            "asset_category": state.asset_category,
            "total_variants_generated": len(state.generated_variants),
            "total_variants_failed": len(state.failed_generations),
            "sprite_sheets_generated": len(state.sprite_sheets_generated),
            "estimated_cost": state.total_cost_usd,
            "variant_sample": dict(list(state.generated_variants.items())[:5]),
            "sprite_sheet_sample": dict(list(state.sprite_sheets_generated.items())[:3])
        }
        
        # Human review interrupt
        human_response = interrupt({
            "type": "variant_generation_review",
            "message": f"Review {len(state.generated_variants)} generated variants in {len(state.sprite_sheets_generated)} sprite sheets",
            "data": review_data,
            "actions": [
                "approve - Accept all generated variants and sprite sheets",
                "regenerate_failed - Retry only failed variants",
                "adjust_sprite_sheets - Regenerate sprite sheets with different layout",
                "reject - Start over with modified configuration"
            ]
        })
        
        # Process response
        if isinstance(human_response, dict):
            approval = human_response.get("action", "approved")
            feedback = human_response.get("feedback", {})
        else:
            approval = str(human_response) if human_response else "approved"
            feedback = {}
        
        print(f"👤 Human review: {approval}")
        
        return {
            "human_approval": approval,
            "human_feedback": feedback,
            "step_count": state.step_count + 1
        }
    
    def finalize_variant_generation_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Finalize variant generation with metadata and integration files."""
        
        print(f"🎯 Finalizing variant asset generation")
        
        output_dir = Path(state.output_dir)
        
        # Write atlas metadata JSON files
        for group_name, atlas_data in state.atlas_metadata.items():
            atlas_file = output_dir / "atlases" / f"{group_name}.json"
            atlas_file.parent.mkdir(parents=True, exist_ok=True)
            
            with open(atlas_file, 'w') as f:
                json.dump(atlas_data, f, indent=2, default=str)
            
            print(f"📝 Wrote atlas metadata: {atlas_file}")
        
        # Write generation metadata
        metadata_file = output_dir / f"{state.asset_category}_variant_metadata.json"
        with open(metadata_file, 'w') as f:
            json.dump(state.generation_metadata, f, indent=2, default=str)
        
        print(f"📝 Wrote generation metadata: {metadata_file}")
        
        # Write Bevy integration code
        bevy_code = self._generate_bevy_integration_code(state)
        bevy_file = output_dir / f"{state.asset_category}_variants.rs"
        bevy_file.write_text(bevy_code, encoding="utf-8")
        
        print(f"📝 Wrote Bevy integration: {bevy_file}")
        
        print(f"✨ Variant generation complete!")
        print(f"📊 Generated: {len(state.generated_variants)} variants")
        print(f"📄 Sprite sheets: {len(state.sprite_sheets_generated)}")
        print(f"💰 Total cost: ${state.total_cost_usd:.2f}")
        
        return {
            "completed_at": datetime.now(),
            "success": len(state.generated_variants) > 0,
            "step_count": state.step_count + 1
        }
    
    # Helper methods
    
    def _generate_variant_combinations(self, config: VariantConfiguration, variants_to_use: list[str]) -> list[dict[str, str]]:
        """Generate all valid variant combinations."""
        
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
    
    def _is_combination_excluded(self, combination: dict[str, str], exclusion_rules: list[list[str]]) -> bool:
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
        combination: dict[str, str],
        descriptors: dict[str, dict[str, str]]
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
        combination: dict[str, str], 
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
    
    def _plan_sprite_sheets(self, specs: list[VariantAssetSpec], resolution_tier: str) -> list[SpriteSheetMetadata]:
        """Plan sprite sheet layout for variants."""
        
        tier = self.resolution_tiers[resolution_tier]
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
    
    def _create_sprite_sheet(
        self,
        group_name: str,
        variants: list[tuple[str, str, dict]],
        output_dir: Path,
        tier: ResolutionTier
    ) -> tuple[str, dict]:
        """Create sprite sheet from individual variant images using Pillow."""
        
        # Parse cell dimensions
        cell_width, cell_height = map(int, tier.resolution.split('x'))
        
        # Calculate grid layout
        grid_cols = min(4, len(variants))
        grid_rows = math.ceil(len(variants) / grid_cols)
        
        # Create sprite sheet canvas
        sheet_width = cell_width * grid_cols
        sheet_height = cell_height * grid_rows
        sprite_sheet = Image.new('RGBA', (sheet_width, sheet_height), (0, 0, 0, 0))
        
        # Atlas metadata
        atlas_data = {
            "sheet_name": f"{group_name}.png",
            "sheet_size": [sheet_width, sheet_height],
            "cell_size": [cell_width, cell_height],
            "grid_size": [grid_cols, grid_rows],
            "frames": {}
        }
        
        # Place each variant in grid
        for idx, (variant_name, variant_path, metadata) in enumerate(variants):
            try:
                # Load variant image
                variant_image = Image.open(variant_path)
                
                # Ensure correct size
                if variant_image.size != (cell_width, cell_height):
                    variant_image = variant_image.resize((cell_width, cell_height), Image.Resampling.LANCZOS)
                
                # Calculate grid position
                col = idx % grid_cols
                row = idx // grid_cols
                x = col * cell_width
                y = row * cell_height
                
                # Paste into sprite sheet
                sprite_sheet.paste(variant_image, (x, y))
                
                # Add to atlas metadata
                atlas_data["frames"][variant_name] = {
                    "x": x,
                    "y": y,
                    "w": cell_width,
                    "h": cell_height,
                    "variant_combination": metadata.get("variant_combination", {}),
                    "base_archetype": metadata.get("base_archetype", "")
                }
                
            except Exception as e:
                print(f"      ⚠️ Failed to add {variant_name} to sprite sheet: {e}")
        
        # Save sprite sheet
        sheet_path = output_dir / "sprite_sheets" / f"{group_name}.png"
        sheet_path.parent.mkdir(parents=True, exist_ok=True)
        sprite_sheet.save(sheet_path, "PNG")
        
        return str(sheet_path), atlas_data
    
    def _generate_bevy_integration_code(self, state: VariantAssetGenerationState) -> str:
        """Generate Bevy integration code for variant system."""
        
        return f"""
// Auto-generated Bevy integration for {state.asset_category} variants
// Generated: {datetime.now().isoformat()}
// Total variants: {len(state.generated_variants)}
// Sprite sheets: {len(state.sprite_sheets_generated)}

use bevy::prelude::*;
use serde::{{Serialize, Deserialize}};
use std::collections::HashMap;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct {state.asset_category.title()}VariantAtlas {{
    pub sprite_sheets: HashMap<String, Handle<TextureAtlas>>,
    pub individual_variants: HashMap<String, Handle<Image>>,
    pub variant_metadata: HashMap<String, VariantMetadata>,
}}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantMetadata {{
    pub base_archetype: String,
    pub variant_combination: HashMap<String, String>,
    pub sprite_sheet_group: String,
    pub atlas_index: Option<usize>,
}}

pub struct {state.asset_category.title()}VariantPlugin;

impl Plugin for {state.asset_category.title()}VariantPlugin {{
    fn build(&self, app: &mut App) {{
        app
            .add_systems(Startup, load_{state.asset_category}_variants)
            .init_resource::<{state.asset_category.title()}VariantAtlas>();
    }}
}}

fn load_{state.asset_category}_variants(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {{
    let mut variant_atlas = {state.asset_category.title()}VariantAtlas {{
        sprite_sheets: HashMap::new(),
        individual_variants: HashMap::new(),
        variant_metadata: HashMap::new(),
    }};

    // Load sprite sheets
{chr(10).join([f'    // Load sprite sheet: {group}' for group in state.sprite_sheets_generated.keys()])}

    // Load individual variants  
{chr(10).join([f'    // variant_atlas.individual_variants.insert("{name}".to_string(), asset_server.load("variants/{name}.png"));' for name in list(state.generated_variants.keys())[:10]])}

    commands.insert_resource(variant_atlas);
}}

// Helper functions for variant selection
impl {state.asset_category.title()}VariantAtlas {{
    pub fn get_variant_by_combination(
        &self,
