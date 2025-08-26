"""
DALL-E Generator Module - Focused DALL-E integration with batch optimization.
Handles variant asset generation using LangChain DALL-E tool.
"""

from pathlib import Path
from typing import Any, Dict, List
from datetime import datetime
import requests
from io import BytesIO

from langchain_community.utilities.dalle_image_generator import DallEAPIWrapper

from dragons_labyrinth.models import VariantAssetGenerationState


class DalleVariantGenerator:
    """
    Focused DALL-E generator for variant assets.
    Handles batch generation with optimization and error recovery.
    """
    
    def __init__(self):
        self.dalle_tool = DallEAPIWrapper()
        self.cost_per_generation = 0.04  # Approximate cost per DALL-E 3 generation
        
    def generate_variants(self, state: VariantAssetGenerationState) -> Dict[str, Any]:
        """Generate all variant assets using DALL-E with batch optimization."""
        
        print(f"  🎨 Generating {state.total_variants_planned} variants with DALL-E")
        
        generated_variants = {}
        failed_generations = []
        generation_metadata = {}
        
        # Collect all variant specs across archetypes
        all_specs = self._collect_all_variant_specs(state)
        
        # Optimize batch size based on resolution tier
        tier = state.resolution_tiers[state.variant_config.resolution_tier]
        optimized_batch_size = self._calculate_optimized_batch_size(state.batch_size, tier)
        
        print(f"  📦 Using optimized batch size: {optimized_batch_size}")
        
        # Process with fail-fast behavior - stop immediately on API issues
        consecutive_failures = 0
        max_consecutive_failures = 2  # Stop after just 2 failures
        
        print(f"    🔥 FAIL-FAST MODE: Will stop after {max_consecutive_failures} consecutive failures")
        
        for i, spec in enumerate(all_specs):
            if consecutive_failures >= max_consecutive_failures:
                print(f"    🚨 STOPPING ENTIRE GENERATION: {consecutive_failures} consecutive failures - API issue detected")
                break
                
            batch_num = i + 1
            print(f"    📦 Variant {batch_num}/{len(all_specs)}: {spec.asset_name}")
            
            success = self._generate_single_variant(
                spec, state.output_dir, tier, generated_variants, generation_metadata, failed_generations
            )
            
            if success:
                print(f"      ✅ SUCCESS")
                consecutive_failures = 0  # Reset failure counter
            else:
                print(f"      ❌ FAILED")
                consecutive_failures += 1
                
                if consecutive_failures >= max_consecutive_failures:
                    print(f"      🚨 FAIL-FAST TRIGGERED: {consecutive_failures} consecutive failures")
                    break
        
        # Calculate final metrics
        success_count = len(generated_variants)
        fail_count = len(failed_generations)
        total_cost = success_count * self.cost_per_generation
        
        print(f"  🎯 Generation complete: {success_count} success, {fail_count} failed")
        print(f"  💰 Total cost: ${total_cost:.2f}")
        
        return {
            "generated_variants": generated_variants,
            "generation_metadata": generation_metadata,
            "failed_generations": failed_generations,
            "api_calls_made": state.api_calls_made + success_count + fail_count,
            "total_cost_usd": state.total_cost_usd + total_cost,
            "step_count": state.step_count + 1
        }
    
    def _collect_all_variant_specs(self, state: VariantAssetGenerationState) -> List:
        """Collect all variant specs from combinatorial results."""
        
        all_specs = []
        for result in state.combinatorial_results.values():
            all_specs.extend(result.generated_specs)
        
        # Sort by priority (higher priority first)
        all_specs.sort(key=lambda s: s.priority, reverse=True)
        
        return all_specs
    
    def _calculate_optimized_batch_size(self, base_batch_size: int, tier) -> int:
        """Calculate optimized batch size based on resolution tier."""
        
        adjusted_size = max(1, int(base_batch_size * tier.batch_size_multiplier))
        
        # Additional optimization based on resolution
        if tier.resolution == "1024x1024":
            adjusted_size = max(1, adjusted_size // 2)  # Slower for large images
        elif tier.resolution == "256x256":
            adjusted_size = min(10, adjusted_size * 2)  # Faster for small images
        
        return adjusted_size
    
    def _generate_single_variant(
        self, 
        spec, 
        output_dir: Path, 
        tier,
        generated_variants: Dict[str, str],
        generation_metadata: Dict[str, Any],
        failed_generations: List[str]
    ) -> bool:
        """Generate a single variant asset with error handling."""
        
        try:
            # Generate image using DALL-E (just pass the prompt string per documentation)
            result = self.dalle_tool.run(spec.final_prompt)
            
            # Prepare metadata for download (quality/style not supported by DallEAPIWrapper)
            dalle_params = {
                "prompt": spec.final_prompt,
                "resolution": spec.resolution
            }
            
            # Handle result and download
            if isinstance(result, str) and result.startswith('http'):
                return self._download_and_save_variant(
                    result, spec, output_dir, dalle_params, generated_variants, generation_metadata
                )
            else:
                print(f"        ⚠️ Unexpected DALL-E result format: {type(result)}")
                failed_generations.append(spec.asset_name)
                return False
                
        except Exception as e:
            print(f"        ❌ Generation error for {spec.asset_name}: {e}")
            failed_generations.append(spec.asset_name)
            return False
    
    def _download_and_save_variant(
        self,
        image_url: str,
        spec,
        output_dir: Path,
        dalle_params: Dict[str, str],
        generated_variants: Dict[str, str],
        generation_metadata: Dict[str, Any]
    ) -> bool:
        """Download and save generated variant image."""
        
        try:
            # Download image
            response = requests.get(image_url, timeout=30)
            
            if response.status_code == 200:
                # Save individual variant
                output_path = output_dir / "variants" / f"{spec.asset_name}.png"
                output_path.parent.mkdir(parents=True, exist_ok=True)
                
                with open(output_path, 'wb') as f:
                    f.write(response.content)
                
                # Store file path and metadata
                generated_variants[spec.asset_name] = str(output_path)
                generation_metadata[spec.asset_name] = {
                    "base_archetype": spec.base_archetype,
                    "variant_combination": spec.variant_combination,
                    "resolution": spec.resolution,
                    "sprite_sheet_group": spec.sprite_sheet_group,
                    "dalle_params": dalle_params,
                    "file_size_bytes": len(response.content),
                    "timestamp": datetime.now().isoformat()
                }
                
                return True
            else:
                print(f"        ❌ Download failed: HTTP {response.status_code}")
                return False
                
        except Exception as e:
            print(f"        ❌ Download error: {e}")
            return False
