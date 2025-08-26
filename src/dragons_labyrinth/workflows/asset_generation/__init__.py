"""
Asset Generation Package - GPT-5 + GPT Image 1 Native Integration

Direct OpenAI integration for sprite sheet and variant generation:
- Universal TOML specifications with combinatorial generation
- GPT-5 enhanced prompts with GPT Image 1 generation
- Native sprite sheet generation in single API calls
- Transparent backgrounds for game assets
- Multi-turn iterative refinement capabilities

Architecture:
- workflow.py: Main coordinator with LangGraph + native OpenAI
- toml_parser.py: Focused TOML parsing
- combinatorial_generator.py: Variant combination logic
- sprite_sheet_processor.py: Pillow sprite sheet processing
- bevy_integrator.py: Rust/Bevy integration

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

This system generates complete sprite sheets in single API calls:
- Characters: 12 archetypes × 30 variants = 360+ assets
- Biomes: 10 terrains × 20 variants = 200+ assets  
- Monsters: 13 creatures × 25 variants = 325+ assets

Total: 900+ assets from minimal prompt definitions using GPT-5 + GPT Image 1.
"""

from .workflow import AssetGenerationWorkflow, create_asset_generation_workflow
from .toml_parser import VariantTOMLParser
from .combinatorial_generator import CombinatorialGenerator
from .sprite_sheet_processor import SpriteSheetProcessor
from .bevy_integrator import BevyIntegrator

__all__ = [
    "AssetGenerationWorkflow",
    "create_asset_generation_workflow",
    "VariantTOMLParser", 
    "CombinatorialGenerator",
    "SpriteSheetProcessor",
    "BevyIntegrator"
]

# Version info for GPT-5 + GPT Image 1 system
__version__ = "3.0.0"
__generator__ = "gpt-5 + gpt-image-1"
__architecture__ = "native_openai_sprite_sheets"
