# Progress Log

## 2025-01-26: GPT-5 + GPT Image 1 Integration

### Completed
- **Asset Generation Overhaul**: Replaced LangChain DALL-E with direct OpenAI API
  - Integrated GPT-5 for prompt enhancement
  - Integrated GPT Image 1 (gpt-image-1) for sprite sheet generation
  - Removed `dalle_generator.py` completely
  - Updated workflow to generate complete sprite sheets in single API calls
  - Added native transparent background support
  - Implemented conversation state tracking for multi-turn editing
  - Fixed organization verification issue
  - Enforced always-transparent images; sanitized prompts to remove background hints
  - Added downsampling to intended resolution (e.g., 128x128) after generation
  - Introduced concurrency, retries, and mask/reference edit support

### Technical Implementation
- Direct `OpenAI()` client in `workflow.py`
- `responses.create()` API with image_generation tool
- 4x4 sprite sheet generation (16 variants per call)
- Cost tracking: ~$0.04 per high-quality 1024x1024 sheet
- Version bumped to 3.0.0
 - Interim artifacts moved to **XDG**: sprite sheets -> `~/.local/share/dragons_labyrinth/<category>/sprite_sheets`; metadata/atlases -> `~/.local/state/dragons_labyrinth/<category>/...`
 - Final variants saved under `crates/game-engine/assets/<prompt_category>/<archetype>/...`
 - Bevy integration generated via Jinja templates to `crates/game-engine/src/integration/<category>_variants.rs`, using `asset_server_path`
 - Alpha anomaly detection for transparency; auto-regeneration of bad variants
 - Transparent-background-aware reprocessing of sheets after regeneration
 - Prompts and style guide packaged as resources and loaded via `importlib.resources`
 - Templates packaged similarly
 - AssetServer configured in `main.rs` with env override `DL_ASSETS_DIR`, dev fallback to `crates/game-engine/assets`, packaged fallback `assets`, and live reload in debug

### Current Issue: Performance Hang
- **STATUS**: Generation hangs when calling GPT-5/GPT Image 1
- **Location**: `_generate_sprite_sheet_with_gpt()` in workflow.py
- **Symptom**: Hangs at "Generating sprite sheet: character_knight"
- **Suspected Causes**:
  1. Wrong API endpoint (`responses.create()` may not be correct)
  2. Invalid model names (GPT-5, gpt-image-1)
  3. No timeout or async handling
  4. Batch size too large (30 variants)

### Architecture Decision
- **No backwards compatibility** - single approach only
- Main workflows still use LangChain/LangGraph
- Asset generation uses native OpenAI exclusively
- Batch optimization for efficiency

### Files Changed
- Modified: `src/dragons_labyrinth/workflows/asset_generation/workflow.py`
- Modified: `src/dragons_labyrinth/workflows/asset_generation/__init__.py`
- Deleted: `dalle_generator.py`, `MIGRATION_PLAN.md`, test scripts
- Cleaned: All old asset files in `crates/game-engine/assets/` (except videos)

## Previous Progress

### World Generation System
- HBF format integration complete
- Hexroll transformer operational
- 180-level progression mapped

### Audio System
- Dread-level responsive audio
- 5 emotional stages (0-4)
- Dynamic atmosphere system

### Dialogue System
- Companion narratives implemented
- Elena and Marcus characters
- Yarn-based dialogue trees

### Asset Generation Foundation
- Universal variant TOML system
- Combinatorial generation logic
- Sprite sheet processor with Pillow
- Bevy integration code generation

## Next Task
Fix the performance hang in GPT-5/GPT Image 1 generation:
1. Investigate correct OpenAI API for image generation
2. Verify model names are valid
3. Add timeout and error handling
4. Reduce initial batch size for testing
5. Add verbose logging to identify exact hang point

## Commands
```bash
# Test command that currently hangs:
hatch run dl_cli generate-assets \
    crates/game-engine/prompts/characters/universal-character-variants.toml \
    --base-dir crates/game-engine

# Alternative commands to test:
hatch run dl_cli list-asset-specs --specs-dir crates/game-engine/prompts  # Works
```

## Known Working Systems
- Rust/Bevy game engine compiles
- Python workflows operational
- TOML parsing functional
- Sprite sheet processing ready
- OpenAI client initialized (org verified)
- CLI commands structure working

## Pending Work
- Fix API call structure for GPT-5/GPT Image 1
- Test actual image generation
- Verify transparent backgrounds
- Validate sprite sheet layouts
- Check cost tracking accuracy
- Generate all asset categories (characters, biomes, monsters)

## Investigation Notes
- User confirmed GPT-5 and gpt-image-1 models exist
- Organization verification was required and fixed
- The hang suggests the API call never returns
- May need different API endpoint than `responses.create()`
