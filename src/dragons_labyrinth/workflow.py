"""
Asset Generation Workflow - GPT-5 + GPT Image 1 Native Integration
Direct OpenAI integration for sprite sheet and variant generation.
"""

from pathlib import Path
from typing import Literal
from datetime import datetime
from concurrent.futures import ThreadPoolExecutor, as_completed
import time
from PIL import Image

from langgraph.graph import StateGraph, START, END
from langgraph.types import interrupt
from openai import OpenAI

from dragons_labyrinth.models import VariantAssetGenerationState
from dragons_labyrinth import (
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
        """Node: Generate assets with GPT-5 + GPT Image 1 (per-variant images)."""
        print("🎨 Generating assets with GPT-5 + GPT Image 1")

        generated_variants = dict(state.generated_variants)
        generation_metadata = dict(state.generation_metadata)
        failed_generations = []
        api_calls = 0
        total_cost = state.total_cost_usd

        # Flatten all variant specs across archetypes
        all_specs = []
        for result in state.combinatorial_results.values():
            all_specs.extend(result.generated_specs)

        # Respect batch size: generate up to N new variants per invocation
        variants_to_generate = [s for s in all_specs if s.asset_name not in generated_variants]
        if state.batch_size > 0:
            variants_to_generate = variants_to_generate[: state.batch_size]

        # Determine final assets directory structure based on prompts path and archetype
        # Desired layout: <assets_root>/<prompt_category>/<archetype>/<file>.png
        toml_category = Path(state.toml_spec_path).parent.name  # e.g., "biomes", "items"

        def _assets_root_from(dir_path: Path) -> Path | None:
            if dir_path.name == "assets":
                return dir_path
            for parent in dir_path.parents:
                if parent.name == "assets":
                    return parent
            return None

        base_output_dir = state.output_dir
        assets_root = _assets_root_from(base_output_dir)
        # Compute category root strictly under the detected assets root
        if assets_root is None:
            # Fallback: treat provided output_dir as assets root
            assets_root = base_output_dir
        category_root = assets_root / toml_category
        category_root.mkdir(parents=True, exist_ok=True)

        # Internal helper: map logical resolution to API-supported sizes
        def _map_size(resolution: str) -> str:
            # Always upscale small tiles to 256 for quality, then downscale later
            size_map = {
                "32x32": "256x256",
                "64x64": "256x256",
                "128x128": "256x256",
                "256x256": "256x256",
                "512x512": "512x512",
                "1024x1024": "1024x1024",
            }
            return size_map.get(resolution, "1024x1024")

        def _dest_path_for_spec(spec) -> Path:
            archetype_dir = category_root / spec.base_archetype
            archetype_dir.mkdir(parents=True, exist_ok=True)
            return archetype_dir / f"{spec.asset_name}.png"

        def _asset_server_path(file_path: Path) -> str | None:
            root = assets_root
            try:
                if root and file_path.is_absolute():
                    return str(file_path.relative_to(root)).replace("\\", "/")
                elif root:
                    return str((root / file_path).relative_to(root)).replace("\\", "/")
            except Exception:
                return None
            return None

        def _generate_one(spec) -> tuple[str, str | None, dict | None]:
            file_path = _dest_path_for_spec(spec)
            if file_path.exists():
                return spec.asset_name, str(file_path), {
                    "base_archetype": spec.base_archetype,
                    "variant_combination": spec.variant_combination,
                    "resolution": spec.resolution,
                    "sprite_sheet_group": spec.sprite_sheet_group,
                    "asset_server_path": _asset_server_path(file_path),
                    "timestamp": datetime.now().isoformat(),
                }

            # Prompt sanitization to avoid background instructions in text
            def _sanitize_prompt(p: str) -> str:
                banned_terms = [
                    "on a white background", "on a black background", "studio background",
                    "background:", "solid background", "bokeh background", "tabletop background",
                    "photographic background", "gradient background"
                ]
                for t in banned_terms:
                    p = p.replace(t, "")
                # Affirm transparent background intent
                p = p.strip() + "; subject centered; no background; transparent PNG"
                return p

            # Retry policy
            max_attempts = 2
            delay_seconds = 2
            last_error = None

            for attempt in range(1, max_attempts + 1):
                try:
                    requested_size = getattr(spec, "resolution", "1024x1024")
                    size = _map_size(requested_size)

                    requested_quality = getattr(spec, "quality", None)
                    supported_qualities = {"low", "medium", "high", "auto"}
                    quality = requested_quality if requested_quality in supported_qualities else "high"

                    # Optional style passthrough if present on spec
                    style = getattr(spec, "style", None)

                    resp = None

                    # Mask/edit flow
                    base_image = getattr(spec, "base_image_path", None)
                    mask_image = getattr(spec, "mask_image_path", None)
                    if base_image or mask_image:
                        # Use image edit endpoint when either base or mask provided
                        sanitized = _sanitize_prompt(spec.final_prompt)
                        if base_image and mask_image:
                            with open(base_image, "rb") as _img, open(mask_image, "rb") as _mask:
                                resp = self.openai.images.edits(
                                    model=self.image_model,
                                    prompt=sanitized,
                                    image=_img,
                                    mask=_mask,
                                    size=size,
                                    quality=quality,
                                    background="transparent",
                                    timeout=90,
                                )
                        elif base_image and not mask_image:
                            with open(base_image, "rb") as _img:
                                resp = self.openai.images.edits(
                                    model=self.image_model,
                                    prompt=sanitized,
                                    image=_img,
                                    size=size,
                                    quality=quality,
                                    background="transparent",
                                    timeout=90,
                                )
                        else:  # mask only (rare)
                            with open(mask_image, "rb") as _mask:
                                resp = self.openai.images.edits(
                                    model=self.image_model,
                                    prompt=sanitized,
                                    mask=_mask,
                                    size=size,
                                    quality=quality,
                                    background="transparent",
                                    timeout=90,
                                )
                    else:
                        # Standard generation
                        sanitized = _sanitize_prompt(spec.final_prompt)
                        gen_kwargs = {
                            "model": self.image_model,
                            "prompt": sanitized,
                            "size": size,
                            "quality": quality,
                            "background": "transparent",
                            "timeout": 90,
                        }
                        if style in {"vivid", "natural"}:
                            gen_kwargs["style"] = style
                        resp = self.openai.images.generate(**gen_kwargs)

                    # Success path
                    image_b64 = resp.data[0].b64_json if resp and getattr(resp, "data", None) else None
                    if not image_b64:
                        raise RuntimeError("Empty image data returned")

                    import base64
                    with open(file_path, "wb") as f:
                        f.write(base64.b64decode(image_b64))

                    # Downsample to intended resolution (e.g., 128x128)
                    try:
                        target_w, target_h = map(int, str(spec.resolution).split('x'))
                        with Image.open(file_path) as _img:
                            if _img.size != (target_w, target_h):
                                _img = _img.convert('RGBA').resize((target_w, target_h), Image.Resampling.LANCZOS)
                                _img.save(file_path, format='PNG', optimize=True)
                    except Exception as _e:
                        print(f"    ⚠️ Downsample failed for {spec.asset_name}: {_e}")

                    return spec.asset_name, str(file_path), {
                        "base_archetype": spec.base_archetype,
                        "variant_combination": spec.variant_combination,
                        "resolution": spec.resolution,
                        "sprite_sheet_group": spec.sprite_sheet_group,
                        "asset_server_path": _asset_server_path(file_path),
                        "timestamp": datetime.now().isoformat(),
                    }

                except Exception as e:
                    last_error = e
                    if attempt < max_attempts:
                        time.sleep(delay_seconds)
                    else:
                        print(f"  ❌ Failed to generate {spec.asset_name}: {e}")
                        return spec.asset_name, None, None

        # Concurrency: thread pool tuned for IO-bound API calls
        max_workers = min(16, max(4, state.batch_size))
        futures = []
        with ThreadPoolExecutor(max_workers=max_workers) as executor:
            for spec in variants_to_generate:
                futures.append(executor.submit(_generate_one, spec))

            for fut in as_completed(futures):
                name, path, meta = fut.result()
                api_calls += 1
                if path and meta:
                    generated_variants[name] = path
                    generation_metadata[name] = meta
                    # Rough cost estimate (assume 1024 == $0.04, smaller proportionally)
                    res = meta.get("resolution", "1024x1024")
                    if res == "1024x1024":
                        total_cost += 0.04
                    elif res == "512x512":
                        total_cost += 0.02
                    else:
                        total_cost += 0.01
                else:
                    failed_generations.append(name)

        print(f"  🎯 Generation complete: {len(generated_variants)} total, +{len(variants_to_generate) - len(failed_generations)} new")
        print(f"  💰 Estimated cost: ${total_cost:.2f}")

        return {
            "generated_variants": generated_variants,
            "generation_metadata": generation_metadata,
            "failed_generations": failed_generations,
            "api_calls_made": state.api_calls_made + api_calls,
            "total_cost_usd": total_cost,
            "step_count": state.step_count + 1,
        }
    
    def _group_variants_for_sprite_generation(self, state: VariantAssetGenerationState) -> dict:
        """Group variant specs for planning (used by prompt building helper)."""
        groups = {}
        for result in state.combinatorial_results.values():
            for spec in result.generated_specs:
                groups.setdefault(spec.sprite_sheet_group, []).append(spec)
        return groups
    
    def _generate_sprite_sheet_with_gpt(self, group_name: str, specs: list, state: VariantAssetGenerationState) -> dict:
        """Deprecated: Previously attempted one-shot sprite sheet generation. Kept for reference."""
        return {}
    
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
        result = self.sprite_processor.create_sprite_sheets(state)
        rejected = result.get("rejected_variants", []) or []
        if rejected:
            print(f"  🔁 Regenerating {len(rejected)} rejected variants with anomalous transparency")

            # Map variant name -> spec for prompt and resolution
            name_to_spec = {}
            for comb in state.combinatorial_results.values():
                for spec in comb.generated_specs:
                    name_to_spec[spec.asset_name] = spec

            # Generate each rejected variant afresh
            for name in rejected:
                spec = name_to_spec.get(name)
                if not spec:
                    continue
                file_path = state.output_dir / "variants" / f"{name}.png"
                # Remove bad file if present
                try:
                    if file_path.exists():
                        file_path.unlink()
                except Exception:
                    pass

                try:
                    # Reuse the same generation logic (transparent, sanitized)
                    requested_size = getattr(spec, "resolution", "1024x1024")
                    def _map_size(resolution: str) -> str:
                        size_map = {
                            "32x32": "256x256",
                            "64x64": "256x256",
                            "128x128": "256x256",
                            "256x256": "256x256",
                            "512x512": "512x512",
                            "1024x1024": "1024x1024",
                        }
                        return size_map.get(resolution, "1024x1024")
                    size = _map_size(requested_size)

                    def _sanitize_prompt(p: str) -> str:
                        banned_terms = [
                            "on a white background", "on a black background", "studio background",
                            "background:", "solid background", "bokeh background", "tabletop background",
                            "photographic background", "gradient background"
                        ]
                        for t in banned_terms:
                            p = p.replace(t, "")
                        return p.strip() + "; subject centered; no background; transparent PNG"

                    sanitized = _sanitize_prompt(spec.final_prompt)
                    resp = self.openai.images.generate(
                        model=self.image_model,
                        prompt=sanitized,
                        size=size,
                        quality="high",
                        background="transparent",
                        timeout=90,
                    )
                    import base64
                    image_b64 = resp.data[0].b64_json if resp and getattr(resp, "data", None) else None
                    if not image_b64:
                        continue
                    with open(file_path, "wb") as f:
                        f.write(base64.b64decode(image_b64))
                    # Downsample to intended resolution
                    try:
                        target_w, target_h = map(int, str(spec.resolution).split('x'))
                        with Image.open(file_path) as _img:
                            if _img.size != (target_w, target_h):
                                _img = _img.convert('RGBA').resize((target_w, target_h), Image.Resampling.LANCZOS)
                                _img.save(file_path, format='PNG', optimize=True)
                    except Exception as _e:
                        print(f"    ⚠️ Downsample failed (regen) for {name}: {_e}")
                    # Update state
                    state.generated_variants[name] = str(file_path)
                except Exception as e:
                    print(f"    ❌ Failed to regenerate {name}: {e}")

            # Rebuild sheets after regeneration
            result = self.sprite_processor.create_sprite_sheets(state)
        return result
    
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
