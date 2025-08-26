"""
Sprite Sheet Processor Module - Focused Pillow-based sprite sheet automation.
Handles automatic sprite sheet generation, validation, and atlas creation.
"""

import math
import json
from pathlib import Path
from typing import Any, Dict, List, Tuple
from datetime import datetime

from PIL import Image, ImageDraw, ImageFont
import psutil

from dragons_labyrinth.models import VariantAssetGenerationState


class SpriteSheetProcessor:
    """
    Focused sprite sheet processor using Pillow.
    Handles sprite sheet creation, validation, and atlas generation.
    """
    
    def __init__(self):
        self.max_memory_mb = 2048  # Default memory limit
        self.default_grid_size = 4  # 4x4 grids by default
    
    def create_sprite_sheets(self, state: VariantAssetGenerationState) -> Dict[str, Any]:
        """Create sprite sheets from individual variants using Pillow."""
        
        # Check if sprite sheets are compatible with resolution tier
        tier = state.resolution_tiers[state.variant_config.resolution_tier]
        if not tier.sprite_sheet_compatible:
            print("  🚫 Skipping sprite sheets - resolution tier not compatible")
            return {"step_count": state.step_count + 1}
        
        print(f"  🎨 Creating sprite sheets with Pillow")
        
        sprite_sheets_generated = {}
        atlas_metadata = {}
        
        # Group variants by sprite sheet group
        variant_groups = self._group_variants_for_sheets(state)
        
        for group_name, variants in variant_groups.items():
            print(f"    📄 Processing group: {group_name} ({len(variants)} variants)")
            
            try:
                # Create sprite sheet with validation
                sheet_path, atlas_data = self._create_and_validate_sprite_sheet(
                    group_name,
                    variants,
                    state.output_dir,
                    tier
                )
                
                if sheet_path:
                    sprite_sheets_generated[group_name] = sheet_path
                    atlas_metadata[group_name] = atlas_data
                    
                    # Calculate file size
                    file_size_mb = Path(sheet_path).stat().st_size / (1024 * 1024)
                    print(f"      ✅ Created {Path(sheet_path).name} ({file_size_mb:.1f}MB)")
                else:
                    print(f"      ❌ Failed to create sprite sheet for {group_name}")
                    
            except Exception as e:
                print(f"      ❌ Error creating sprite sheet {group_name}: {e}")
        
        # Memory usage check
        memory_usage = psutil.Process().memory_info().rss / (1024 * 1024)
        print(f"  📊 Memory usage: {memory_usage:.1f}MB")
        print(f"  🎯 Generated {len(sprite_sheets_generated)} sprite sheets")
        
        return {
            "sprite_sheets_generated": sprite_sheets_generated,
            "atlas_metadata": atlas_metadata,
            "peak_memory_usage_mb": memory_usage,
            "step_count": state.step_count + 1
        }
    
    def _group_variants_for_sheets(self, state: VariantAssetGenerationState) -> Dict[str, List[Tuple[str, str, Dict]]]:
        """Group variants by sprite sheet group for processing."""
        
        variant_groups = {}
        
        for variant_name, variant_path in state.generated_variants.items():
            metadata = state.generation_metadata.get(variant_name, {})
            group = metadata.get("sprite_sheet_group", "default")
            
            if group not in variant_groups:
                variant_groups[group] = []
            variant_groups[group].append((variant_name, variant_path, metadata))
        
        # Sort groups by size for memory efficiency (smaller first)
        sorted_groups = sorted(variant_groups.items(), key=lambda x: len(x[1]))
        return dict(sorted_groups)
    
    def _create_and_validate_sprite_sheet(
        self,
        group_name: str,
        variants: List[Tuple[str, str, Dict]],
        output_dir: Path,
        tier
    ) -> Tuple[str | None, Dict]:
        """Create sprite sheet with validation and error handling."""
        
        try:
            # Parse cell dimensions from resolution tier
            cell_width, cell_height = map(int, tier.resolution.split('x'))
            
            # Calculate optimal grid layout
            grid_cols, grid_rows = self._calculate_optimal_grid(len(variants), self.default_grid_size)
            
            # Memory check before creation
            estimated_memory = self._estimate_memory_usage(
                len(variants), cell_width, cell_height, grid_cols, grid_rows
            )
            
            if estimated_memory > self.max_memory_mb:
                print(f"      ⚠️ Memory estimate {estimated_memory:.1f}MB exceeds limit, splitting into smaller sheets")
                return self._create_split_sprite_sheets(group_name, variants, output_dir, tier)
            
            # Create single sprite sheet
            sheet_path, atlas_data = self._create_single_sprite_sheet(
                group_name, variants, output_dir, cell_width, cell_height, grid_cols, grid_rows
            )
            
            # Validate sprite sheet
            if self._validate_sprite_sheet(sheet_path, atlas_data, len(variants)):
                return sheet_path, atlas_data
            else:
                print(f"      ❌ Sprite sheet validation failed for {group_name}")
                return None, {}
                
        except Exception as e:
            print(f"      ❌ Exception creating sprite sheet {group_name}: {e}")
            return None, {}
    
    def _calculate_optimal_grid(self, variant_count: int, preferred_size: int = 4) -> Tuple[int, int]:
        """Calculate optimal grid dimensions for sprite sheet."""
        
        # Try square-ish grids first
        cols = min(preferred_size, variant_count)
        rows = math.ceil(variant_count / cols)
        
        # Optimize for better aspect ratio if needed
        if rows > cols * 2:  # Too tall
            cols = math.ceil(math.sqrt(variant_count))
            rows = math.ceil(variant_count / cols)
        
        return cols, rows
    
    def _estimate_memory_usage(self, variant_count: int, cell_width: int, cell_height: int, cols: int, rows: int) -> float:
        """Estimate memory usage for sprite sheet creation."""
        
        # Each variant image in memory
        variant_memory = variant_count * cell_width * cell_height * 4  # RGBA
        
        # Final sprite sheet
        sheet_memory = (cell_width * cols) * (cell_height * rows) * 4
        
        # Working memory overhead
        overhead = max(variant_memory, sheet_memory) * 0.5
        
        total_bytes = variant_memory + sheet_memory + overhead
        return total_bytes / (1024 * 1024)  # Convert to MB
    
    def _create_single_sprite_sheet(
        self,
        group_name: str,
        variants: List[Tuple[str, str, Dict]],
        output_dir: Path,
        cell_width: int,
        cell_height: int,
        grid_cols: int,
        grid_rows: int
    ) -> Tuple[str, Dict]:
        """Create a single sprite sheet with all variants."""
        
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
            "variant_count": len(variants),
            "frames": {},
            "generated_at": datetime.now().isoformat()
        }
        
        # Place each variant in grid
        successful_placements = 0
        for idx, (variant_name, variant_path, metadata) in enumerate(variants):
            if idx >= grid_cols * grid_rows:
                break  # Grid full
                
            try:
                # Load and validate variant image
                variant_image = self._load_and_validate_variant(variant_path, cell_width, cell_height)
                
                if variant_image:
                    # Calculate grid position
                    col = idx % grid_cols
                    row = idx // grid_cols
                    x = col * cell_width
                    y = row * cell_height
                    
                    # Paste into sprite sheet
                    sprite_sheet.paste(variant_image, (x, y))
                    
                    # Add to atlas metadata
                    atlas_data["frames"][variant_name] = {
                        "x": x, "y": y, "w": cell_width, "h": cell_height,
                        "variant_combination": metadata.get("variant_combination", {}),
                        "base_archetype": metadata.get("base_archetype", ""),
                        "cell_index": idx
                    }
                    
                    successful_placements += 1
                    
            except Exception as e:
                print(f"        ⚠️ Failed to place {variant_name}: {e}")
        
        # Save sprite sheet
        sheet_path = output_dir / "sprite_sheets" / f"{group_name}.png"
        sheet_path.parent.mkdir(parents=True, exist_ok=True)
        sprite_sheet.save(sheet_path, "PNG", optimize=True)
        
        atlas_data["successful_placements"] = successful_placements
        return str(sheet_path), atlas_data
    
    def _load_and_validate_variant(self, variant_path: str, expected_width: int, expected_height: int) -> Image.Image | None:
        """Load and validate individual variant image."""
        
        try:
            variant_image = Image.open(variant_path)
            
            # Convert to RGBA if needed
            if variant_image.mode != 'RGBA':
                variant_image = variant_image.convert('RGBA')
            
            # Resize if needed
            if variant_image.size != (expected_width, expected_height):
                variant_image = variant_image.resize(
                    (expected_width, expected_height), 
                    Image.Resampling.LANCZOS
                )
            
            return variant_image
            
        except Exception as e:
            print(f"          ⚠️ Failed to load {variant_path}: {e}")
            return None
    
    def _create_split_sprite_sheets(self, group_name: str, variants: List, output_dir: Path, tier) -> Tuple[str | None, Dict]:
        """Create multiple smaller sprite sheets for memory efficiency."""
        
        # Split into smaller chunks
        max_variants_per_sheet = 9  # 3x3 grid for memory efficiency
        
        sheet_paths = []
        combined_atlas = {"split_sheets": [], "total_variants": len(variants)}
        
        for chunk_idx in range(0, len(variants), max_variants_per_sheet):
            chunk = variants[chunk_idx:chunk_idx + max_variants_per_sheet]
            chunk_name = f"{group_name}_part_{chunk_idx // max_variants_per_sheet}"
            
            cell_width, cell_height = map(int, tier.resolution.split('x'))
            cols, rows = self._calculate_optimal_grid(len(chunk), 3)
            
            try:
                sheet_path, atlas_data = self._create_single_sprite_sheet(
                    chunk_name, chunk, output_dir, cell_width, cell_height, cols, rows
                )
                
                sheet_paths.append(sheet_path)
                combined_atlas["split_sheets"].append(atlas_data)
                
            except Exception as e:
                print(f"        ❌ Failed to create split sheet {chunk_name}: {e}")
        
        # Return info about split sheets
        if sheet_paths:
            return sheet_paths[0], combined_atlas  # Return first sheet path and combined atlas
        else:
            return None, {}
    
    def _validate_sprite_sheet(self, sheet_path: str, atlas_data: Dict, expected_variants: int) -> bool:
        """Validate created sprite sheet for correctness."""
        
        try:
            # Check file exists and has reasonable size
            sheet_file = Path(sheet_path)
            if not sheet_file.exists():
                return False
            
            file_size = sheet_file.stat().st_size
            if file_size < 1024:  # Less than 1KB is suspicious
                return False
            
            # Validate image can be opened
            with Image.open(sheet_path) as img:
                # Check dimensions match atlas
                expected_size = tuple(atlas_data["sheet_size"])
                if img.size != expected_size:
                    print(f"        ⚠️ Size mismatch: expected {expected_size}, got {img.size}")
                    return False
            
            # Check atlas has expected number of frames
            frames_count = len(atlas_data.get("frames", {}))
            if frames_count < min(expected_variants, atlas_data.get("successful_placements", 0)):
                print(f"        ⚠️ Frame count mismatch: expected ~{expected_variants}, got {frames_count}")
                return False
            
            return True
            
        except Exception as e:
            print(f"        ❌ Validation error: {e}")
            return False
