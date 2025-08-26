"""
Bevy Integrator Module - Focused Rust/Bevy integration code generation.
Handles automatic Bevy plugin and resource generation for variant assets.
"""

import json
from pathlib import Path
from typing import Any, Dict
from datetime import datetime

from dragons_labyrinth.models import VariantAssetGenerationState
from xdg_base_dirs import xdg_state_home, xdg_data_home
from jinja2 import Environment, select_autoescape
from importlib.resources import files as pkg_files


class BevyIntegrator:
    """
    Focused Bevy integration code generator.
    Creates Rust code for game-engine crate integration.
    """
    
    def __init__(self) -> None:
        self.env = Environment(autoescape=select_autoescape(enabled_extensions=(".j2",)), trim_blocks=True, lstrip_blocks=True)
    
    def finalize_generation(self, state: VariantAssetGenerationState) -> Dict[str, Any]:
        """Finalize variant generation with all integration files."""
        
        print("  🦀 Generating Bevy integration files")
        
        # Persist atlas metadata to XDG state
        atlas_files = self._write_atlas_metadata(state)
        
        # Write generation metadata to XDG state
        metadata_file = self._write_generation_metadata(state)
        
        # Write Bevy integration code via templates to game-engine crate
        bevy_file = self._write_bevy_integration_code(state)
        
        # Write summary report alongside code (XDG state)
        summary_file = self._write_summary_report(state)
        
        print(f"  ✅ Integration files complete")
        print(f"  📊 {len(state.generated_variants)} variants ready for game engine")
        
        return {
            "atlas_files": atlas_files,
            "metadata_file": str(metadata_file),
            "bevy_file": str(bevy_file),
            "summary_file": str(summary_file),
            "completed_at": datetime.now(),
            "success": len(state.generated_variants) > 0,
            "step_count": state.step_count + 1
        }
    
    def _atlas_state_dir(self, state: VariantAssetGenerationState) -> Path:
        return xdg_state_home() / "dragons_labyrinth" / state.asset_category / "atlases"
    
    def _write_atlas_metadata(self, state: VariantAssetGenerationState) -> list[str]:
        """Write JSON atlas metadata files for sprite sheets to XDG state."""
        
        atlas_files = []
        out_dir = self._atlas_state_dir(state)
        out_dir.mkdir(parents=True, exist_ok=True)
        
        for group_name, atlas_data in state.atlas_metadata.items():
            atlas_file = out_dir / f"{group_name}.json"
            with open(atlas_file, 'w') as f:
                json.dump(atlas_data, f, indent=2, default=str)
            atlas_files.append(str(atlas_file))
            print(f"    📝 Atlas: {atlas_file.name}")
        
        return atlas_files
    
    def _write_generation_metadata(self, state: VariantAssetGenerationState) -> Path:
        """Write comprehensive generation metadata to XDG state."""
        
        out_dir = xdg_state_home() / "dragons_labyrinth" / state.asset_category
        out_dir.mkdir(parents=True, exist_ok=True)
        metadata_file = out_dir / f"{state.asset_category}_variant_metadata.json"
        
        comprehensive_metadata = {
            "workflow_info": {
                "workflow_id": state.workflow_id,
                "asset_category": state.asset_category,
                "generated_at": datetime.now().isoformat(),
                "generator_version": "2.0.0"
            },
            "variant_config": state.variant_config.model_dump() if state.variant_config else {},
            "generation_summary": {
                "total_variants_planned": state.total_variants_planned,
                "variants_generated": len(state.generated_variants),
                "variants_failed": len(state.failed_generations),
                "sprite_sheets_created": len(state.sprite_sheets_generated),
                "total_cost_usd": state.total_cost_usd
            },
            "per_variant_metadata": state.generation_metadata,
            "combinatorial_results": {
                name: result.model_dump() for name, result in state.combinatorial_results.items()
            }
        }
        
        with open(metadata_file, 'w') as f:
            json.dump(comprehensive_metadata, f, indent=2, default=str)
        
        print(f"    📝 Metadata: {metadata_file.name}")
        return metadata_file
    
    def _write_bevy_integration_code(self, state: VariantAssetGenerationState) -> Path:
        """Write Rust/Bevy integration code for game-engine crate using templates."""
        
        # Prepare template
        # Load packaged template
        template_resource = pkg_files("dragons_labyrinth.workflows.asset_generation.templates").joinpath("variants_plugin.rs.j2")
        template_source = template_resource.read_text(encoding="utf-8")
        template = self.env.from_string(template_source)
        
        # Build context
        # Only individual variants are referenced; sprite sheets stay an internal optimization.
        context = {
            "asset_category": state.asset_category,
            "generated_variants": sorted(state.generated_variants.keys()),
            "generation_metadata": state.generation_metadata,
            "atlas_groups": sorted(state.sprite_sheets_generated.keys()),
            "timestamp": datetime.now().isoformat(),
        }
        rendered = template.render(**context)
        
        # Save into game-engine crate under integration/
        game_engine_src = Path("crates/game-engine/src/integration")
        game_engine_src.mkdir(parents=True, exist_ok=True)
        bevy_file = game_engine_src / f"{state.asset_category}_variants.rs"
        bevy_file.write_text(rendered, encoding="utf-8")
        
        print(f"    📝 Bevy: {bevy_file.name}")
        return bevy_file
    
    def _write_summary_report(self, state: VariantAssetGenerationState) -> Path:
        """Write comprehensive summary report to XDG state."""
        
        out_dir = xdg_state_home() / "dragons_labyrinth" / state.asset_category
        out_dir.mkdir(parents=True, exist_ok=True)
        summary_file = out_dir / f"{state.asset_category}_generation_summary.md"
        
        # Calculate statistics
        total_archetypes = len(state.combinatorial_results)
        total_variants = len(state.generated_variants)
        total_failed = len(state.failed_generations)
        total_sprites = len(state.sprite_sheets_generated)
        
        success_rate = (total_variants / (total_variants + total_failed)) * 100 if (total_variants + total_failed) > 0 else 0
        
        summary_content = f"""# {state.asset_category.title()} Variant Generation Summary

## Workflow: {state.workflow_id}
**Generated:** {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

## Results Overview
- **Total Archetypes Processed:** {total_archetypes}
- **Variants Generated:** {total_variants}
- **Variants Failed:** {total_failed} 
- **Success Rate:** {success_rate:.1f}%
- **Sprite Sheets Created:** {total_sprites}
- **Total Cost:** ${state.total_cost_usd:.2f}

## Archetype Breakdown
{chr(10).join([f"- **{archetype}:** {len(result.generated_specs)} variants planned, {result.total_sprite_sheets} sprite sheets" for archetype, result in state.combinatorial_results.items()])}

## Performance Metrics
- **Resolution Tier:** {state.variant_config.resolution_tier if state.variant_config else 'unknown'}
- **API Calls Made:** {state.api_calls_made}
- **Processing Time:** {(state.completed_at - state.started_at).total_seconds() if state.completed_at else 0:.1f} seconds

## File Outputs
- **Individual Variants:** {len(state.generated_variants)} files in XDG data variants cache
- **Sprite Sheets:** {len(state.sprite_sheets_generated)} files in XDG data atlases
- **Atlas Metadata:** {len(state.atlas_metadata)} JSON files in XDG state
- **Bevy Integration:** `{state.asset_category}_variants.rs`

## Variant System Features
- ✅ Combinatorial variant generation
- ✅ Resolution optimization
- ✅ Sprite sheet automation
- ✅ Memory-efficient processing
- ✅ Game engine integration ready
"""
        
        summary_file.write_text(summary_content, encoding="utf-8")
        print(f"    📝 Summary: {summary_file.name}")
        
        return summary_file
