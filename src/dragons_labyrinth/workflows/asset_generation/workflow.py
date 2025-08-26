"""
Asset Generation Workflow - Clean Modular Architecture
Revolutionary variant-based asset generation with proper separation of concerns.
"""

from pathlib import Path
from typing import Literal, Any
from datetime import datetime

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt

from dragons_labyrinth.models import VariantAssetGenerationState
from . import (
    toml_parser,
    combinatorial_generator, 
    dalle_generator,
    sprite_sheet_processor,
    bevy_integrator
)


class AssetGenerationWorkflow:
    """
    Clean, modular asset generation workflow.
    
    Each step is handled by a focused module:
    - TOML parsing → toml_parser
    - Combinatorial generation → combinatorial_generator
    - DALL-E generation → dalle_generator
    - Sprite sheet processing → sprite_sheet_processor
    - Bevy integration → bevy_integrator
    """
    
    def __init__(self):
        self.toml_parser = toml_parser.VariantTOMLParser()
        self.combinatorial = combinatorial_generator.CombinatorialGenerator()
        self.dalle_gen = dalle_generator.DalleVariantGenerator()
        self.sprite_processor = sprite_sheet_processor.SpriteSheetProcessor()
        self.bevy_integrator = bevy_integrator.BevyIntegrator()
    
    def parse_toml_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Parse variant TOML specifications."""
        print("🔍 Parsing variant TOML specifications")
        return self.toml_parser.parse_variant_toml(state)
    
    def generate_combinations_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Generate combinatorial variant specifications."""
        print("🧮 Generating combinatorial variants")
        return self.combinatorial.generate_combinations(state)
    
    def generate_assets_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Generate assets with DALL-E."""
        print("🎨 Generating assets with DALL-E")
        return self.dalle_gen.generate_variants(state)
    
    def process_sprite_sheets_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Process sprite sheets with Pillow."""
        print("📄 Processing sprite sheets")
        return self.sprite_processor.create_sprite_sheets(state)
    
    def human_review_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
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
    
    def finalize_node(self, state: VariantAssetGenerationState) -> dict[str, Any]:
        """Node: Finalize generation with integration files."""
        print("🎯 Finalizing asset generation")
        return self.bevy_integrator.finalize_generation(state)
    
    # Conditional edges
    
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
        """Build the clean modular workflow."""
        workflow = StateGraph(VariantAssetGenerationState)
        
        # Add focused nodes
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
        
        print("🔧 Modular asset generation workflow compiled")
        return compiled
    
    def generate_assets(
        self,
        asset_category: str,
        toml_spec_path: Path,
        output_dir: Path,
        batch_size: int = 5,
        autonomous_mode: bool = False
    ):
        """Main entry point for asset generation."""
        
        import uuid
        
        # Create workflow
        workflow = self.compile_workflow()
        
        # Initial state
        workflow_id = f"variants_{asset_category}_{uuid.uuid4().hex[:8]}"
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
        
        print(f"🚀 Starting modular asset generation: {workflow_id}")
        
        # Execute workflow
        final_state = workflow.invoke(initial_state)
        
        print(f"✨ Asset generation complete!")
        print(f"📊 Generated: {len(final_state.get('generated_variants', {}))} variants")
        print(f"💰 Total cost: ${final_state.get('total_cost_usd', 0):.2f}")
        
        return final_state


def create_asset_generation_workflow() -> AssetGenerationWorkflow:
    """Factory function to create the modular workflow."""
    return AssetGenerationWorkflow()
