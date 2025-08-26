"""
Asset Generation Package - Revolutionary Variant-Based Asset Generation

This package implements the new variant system architecture:
- Universal TOML specifications with combinatorial generation
- Resolution optimization (256x256 UI, 512x512 tokens, 1024x1024 tiles) 
- Automatic sprite sheet generation with Pillow
- Generic archetypes without proper names
- Exponential asset coverage from minimal prompt definitions

Architecture:
- workflow.py: Main coordinator with clean LangGraph
- toml_parser.py: Focused TOML parsing
- combinatorial_generator.py: Variant combination logic
- dalle_generator.py: DALL-E integration (to be created)
- sprite_sheet_processor.py: Pillow sprite sheet processing (to be created)
- bevy_integrator.py: Rust/Bevy integration (to be created)

Usage:
```python
from dragons_labyrinth.workflows.asset_generation import create_asset_generation_workflow

workflow = create_asset_generation_workflow()
result = workflow.generate_assets(
    asset_category="character",
    toml_spec_path=Path("crates/game-engine/prompts/characters/universal-character-variants.toml"),
    output_dir=Path("generated_assets"),
    autonomous_mode=False
)
```

This system replaces 12+ level-banded TOML files with 3 universal variant systems:
- Characters: 12 archetypes × 30 variants = 360+ assets
- Biomes: 10 terrains × 20 variants = 200+ assets  
- Monsters: 13 creatures × 25 variants = 325+ assets

Total: 900+ assets from minimal, maintainable prompt definitions.
"""

from .workflow import AssetGenerationWorkflow, create_asset_generation_workflow

__all__ = [
    "AssetGenerationWorkflow",
    "create_asset_generation_workflow"
]

# Version info for the variant system
__version__ = "2.0.0"
__variant_system__ = True
__architecture__ = "universal_variants_with_sprite_sheets"
