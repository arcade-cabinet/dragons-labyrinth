"""
Asset Generation Workflow - GPT-5 + GPT Image 1 Native Integration
Direct OpenAI integration for sprite sheet and variant generation.
"""

from pathlib import Path
from typing import Literal
from datetime import datetime

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
import openai
from openai import OpenAI

from dragons_labyrinth.models import VariantAssetGenerationState
from . import (
    toml_parser,
    combinatorial_generator,
    sprite_sheet_processor,
    bevy_integrator
)


class AssetGenerationWorkflow:
    """
    GPT-5 + GPT Image 1 native asset generation workflow.
    
    Each step handled by focused modules:
    - TOML parsing → toml_parser
    - Combinatorial generation → combinatorial_generator
    - GPT-5/GPT Image 1 generation → inline (direct OpenAI)
    - Sprite sheet processing → sprite_sheet_processor
    - Bevy integration → bevy_integrator
    """
    
    def __init__(self):
        self.toml_parser = toml_parser.VariantTOMLParser()
        self.combinatorial = combinatorial_generator.CombinatorialGenerator()
        self.sprite_processor = sprite_sheet_processor.SpriteSheetProcessor()
        self.bevy_integrator = bevy_integrator.BevyIntegrator()
        
        # Direct OpenAI client
        self.openai = OpenAI()
        self.model = "gpt-5"
        self.image_model = "gpt-image-1"
        
        # Track conversation states for multi-turn editing
        self.conversation_states = {}
        
        # Image token costs for tracking
        self.image_token_costs = {
            "low": {"1024x1024": 272, "1024x1536": 408, "1536x1024": 400},
            "medium": {"1024x1024": 1056, "1024x1536": 1584, "1536x1024": 1568},
            "high": {"1024x1024": 4160, "1024x1536": 6240, "1536x1024": 6208}
        }
    
    def parse_toml_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Parse variant TOML specifications."""
        print("🔍 Parsing variant TOML specifications")
        return self.toml_parser.parse_variant_toml(state)
    
    def generate_combinations_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Generate combinatorial variant specifications."""
        print("🧮 Generating combinatorial variants")
        return self.combinatorial.generate_combinations(state)
    
    def generate_assets_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Generate assets with GPT-5 + GPT Image 1."""
        print("🎨 Generating assets with GPT-5 + GPT Image 1")
        
        generated_variants = {}
        generation_metadata = {}
        failed_generations = []
        total_cost = 0.0
        
        # Group variants by sprite sheet for efficient generation
        variant_groups = self._group_variants_for_sprite_generation(state)
        
        for group_name, group_specs in variant_groups.items():
            print(f"  📦 Generating sprite sheet: {group_name}")
            
            # Generate entire sprite sheet in one call
            sprite_sheet_data = self._generate_sprite_sheet_with_gpt(
                group_name,
                group_specs,
                state
            )
            
            if sprite_sheet_data:
                generated_variants.update(sprite_sheet_data['variants'])
                generation_metadata.update(sprite_sheet_data['metadata'])
                total_cost += sprite_sheet_data['cost']
            else:
                failed_generations.extend([spec.asset_name for spec in group_specs])
        
        print(f"  🎯 Generation complete: {len(generated_variants)} variants")
        print(f"  💰 Total cost: ${total_cost:.2f}")
        
        return {
            "generated_variants": generated_variants,
            "generation_metadata": generation_metadata,
            "failed_generations": failed_generations,
            "api_calls_made": state.api_calls_made + len(variant_groups),
            "total_cost_usd": state.total_cost_usd + total_cost,
            "step_count": state.step_count + 1
        }
    
    def _group_variants_for_sprite_generation(self, state: VariantAssetGenerationState) -> dict:
        """Group variants for efficient sprite sheet generation."""
        groups = {}
        
        for result in state.combinatorial_results.values():
            for spec in result.generated_specs:
                group = spec.sprite_sheet_group
                if group not in groups:
                    groups[group] = []
                groups[group].append(spec)
        
        return groups
    
    def _generate_sprite_sheet_with_gpt(self, group_name: str, specs: list, state: VariantAssetGenerationState) -> dict:
        """Generate complete sprite sheet using GPT-5 + GPT Image 1."""
        
        # Build comprehensive prompt with GPT-5
        enhanced_prompt = self._build_sprite_sheet_prompt(group_name, specs, state)
        
        # Generate sprite sheet with GPT Image 1
        response = self.openai.responses.create(
            model=self.model,
            input=enhanced_prompt,
            tools=[{
                "type": "image_generation",
                "background": "transparent",
                "quality": "high",
                "size": "1024x1024"
            }]
        )
        
        # Extract image data
        image_data = [
            output.result
            for output in response.output
            if output.type == "image_generation_call"
        ]
        
        if not image_data:
            return None
        
        # Save sprite sheet
        import base64
        output_dir = state.output_dir / "sprite_sheets"
        output_dir.mkdir(parents=True, exist_ok=True)
        
        sprite_path = output_dir / f"{group_name}.png"
        with open(sprite_path, 'wb') as f:
            f.write(base64.b64decode(image_data[0]))
        
        # Build return data
        variants = {spec.asset_name: str(sprite_path) for spec in specs}
        metadata = {
            spec.asset_name: {
                "base_archetype": spec.base_archetype,
                "variant_combination": spec.variant_combination,
                "resolution": spec.resolution,
                "sprite_sheet_group": spec.sprite_sheet_group,
                "timestamp": datetime.now().isoformat()
            }
            for spec in specs
        }
        
        # Calculate cost (high quality 1024x1024)
        cost = 4160 * 0.00001  # image tokens * price per token
        
        return {
            "variants": variants,
            "metadata": metadata,
            "cost": cost
        }
    
    def _build_sprite_sheet_prompt(self, group_name: str, specs: list, state: VariantAssetGenerationState) -> str:
        """Build comprehensive sprite sheet prompt."""
        
        base_archetype = specs[0].base_archetype if specs else "unknown"
        
        # Extract unique variant dimensions
        variant_dimensions = set()
        for spec in specs:
            variant_dimensions.update(spec.variant_combination.keys())
        
        prompt = f"""Generate a 4x4 sprite sheet for {base_archetype} game assets.
        
Horror game aesthetic with pixel art style.
Transparent background required.
Each sprite should be a distinct variant based on:
{', '.join(variant_dimensions)}

Specific variants needed:
"""
        
        for i, spec in enumerate(specs[:16]):  # Max 16 for 4x4 grid
            variant_desc = ', '.join([f"{k}={v}" for k, v in spec.variant_combination.items()])
            prompt += f"\n{i+1}. {spec.base_archetype} - {variant_desc}"
        
        prompt += "\n\nMaintain consistent character design across all variants."
        prompt += "\nUse horror elements appropriate for each variant."
        prompt += "\nEnsure clear visual distinction between variants."
        
        return prompt
    
    def process_sprite_sheets_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Process sprite sheets with Pillow."""
        print("📄 Processing sprite sheets")
        return self.sprite_processor.create_sprite_sheets(state)
    
    def human_review_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Human review checkpoint."""
        if state.autonomous_mode:
            print("🤖 Autonomous mode: Skipping human review")
            return {"human_approval": "approved", "step_count": state.step_count + 1}
        
        review_data = {
            "workflow_id": state.workflow_id,
            "asset_category": state.asset_category,
            "variants_generated": len(state.generated_variants),
            "sprites_created": len(state.sprite_sheets_generated),
            "total_cost": state.total_cost_usd
        }
        
        human_response = interrupt({
            "type": "variant_asset_review",
            "message": f"Review {len(state.generated_variants)} variants in {len(state.sprite_sheets_generated)} sprite sheets",
            "data": review_data,
            "actions": ["approve", "regenerate_failed", "adjust_sprites", "reject"]
        })
        
        approval = human_response.get("action", "approved") if isinstance(human_response, dict) else str(human_response)
        
        print(f"👤 Human review: {approval}")
        return {
            "human_approval": approval,
            "human_feedback": human_response.get("feedback", {}) if isinstance(human_response, dict) else {},
            "step_count": state.step_count + 1
        }
    
    def finalize_node(self, state: VariantAssetGenerationState) -> dict:
        """Node: Finalize generation with integration files."""
        print("🎯 Finalizing asset generation")
        return self.bevy_integrator.finalize_generation(state)
    
    def should_review(self, state: VariantAssetGenerationState) -> Literal["review", "finalize"]:
        """Determine if human review is needed."""
        return "review" if not state.autonomous_mode and state.generated_variants else "finalize"
    
    def after_review(self, state: VariantAssetGenerationState) -> Literal["finalize", "regenerate", "end"]:
        """Handle human review decision."""
        approval = state.human_approval
        if approval == "approved":
            return "finalize"
        elif approval in ["regenerate_failed", "adjust_sprites"]:
            return "regenerate"
        else:
            return "end"
    
    def build_workflow(self) -> StateGraph:
        """Build the GPT-5 + GPT Image 1 workflow."""
        workflow = StateGraph(VariantAssetGenerationState)
        
        # Add nodes
        workflow.add_node("parse_toml", self.parse_toml_node)
        workflow.add_node("generate_combinations", self.generate_combinations_node)
        workflow.add_node("generate_assets", self.generate_assets_node)
        workflow.add_node("process_sprites", self.process_sprite_sheets_node)
        workflow.add_node("human_review", self.human_review_node)
        workflow.add_node("finalize", self.finalize_node)
        
        # Build workflow graph
        workflow.add_edge(START, "parse_toml")
        workflow.add_edge("parse_toml", "generate_combinations")
        workflow.add_edge("generate_combinations", "generate_assets")
        workflow.add_edge("generate_assets", "process_sprites")
        
        workflow.add_conditional_edges(
            "process_sprites",
            self.should_review,
            {"review": "human_review", "finalize": "finalize"}
        )
        
        workflow.add_conditional_edges(
            "human_review",
            self.after_review,
            {"finalize": "finalize", "regenerate": "generate_assets", "end": "finalize"}
        )
        
        workflow.add_edge("finalize", END)
        
        return workflow
    
    def compile_workflow(self, checkpointer=None) -> StateGraph:
        """Compile the workflow for execution."""
        workflow = self.build_workflow()
        
        if checkpointer:
            compiled = workflow.compile(checkpointer=checkpointer)
        else:
            compiled = workflow.compile()
        
        print("🔧 GPT-5 + GPT Image 1 workflow compiled")
        return compiled
    
    def generate_assets(
        self,
        asset_category: str,
        toml_spec_path: Path,
        output_dir: Path,
        batch_size: int = 16,
        autonomous_mode: bool = False,
        level_range: str = None,
        skip_existing: bool = True
    ):
        """Main entry point for GPT-5 + GPT Image 1 asset generation."""
        
        import uuid
        
        # Create workflow
        workflow = self.compile_workflow()
        
        # Initial state
        workflow_id = f"gpt5_{asset_category}_{uuid.uuid4().hex[:8]}"
        initial_state = {
            "asset_category": asset_category,
            "toml_spec_path": toml_spec_path,
            "output_dir": output_dir,
            "batch_size": batch_size,
            "autonomous_mode": autonomous_mode,
            "workflow_id": workflow_id,
            "started_at": datetime.now(),
            "step_count": 0,
            "variant_config": None,
            "combinatorial_results": {},
            "total_variants_planned": 0,
            "generated_variants": {},
            "generation_metadata": {},
            "failed_generations": [],
            "sprite_sheets_generated": {},
            "atlas_metadata": {},
            "human_approval": None,
            "human_feedback": {},
            "api_calls_made": 0,
            "total_cost_usd": 0.0,
            "completed_at": None
        }
        
        print(f"🚀 Starting GPT-5 + GPT Image 1 generation: {workflow_id}")
        
        # Execute workflow
        final_state = workflow.invoke(initial_state)
        
        print(f"✨ Asset generation complete!")
        print(f"📊 Generated: {len(final_state.get('generated_variants', {}))} variants")
        print(f"💰 Total cost: ${final_state.get('total_cost_usd', 0):.2f}")
        
        return final_state


def create_asset_generation_workflow() -> AssetGenerationWorkflow:
    """Factory function to create GPT-5 + GPT Image 1 workflow."""
    return AssetGenerationWorkflow()
