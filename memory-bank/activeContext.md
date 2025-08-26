# Active Development Context

## Current Focus: GPT-5 + GPT Image 1 Asset Generation - PERFORMANCE ISSUE

### What Just Happened
- **Replaced DALL-E with GPT-5 + GPT Image 1**: The asset_generation subpackage now uses direct OpenAI API
- **Removed LangChain dependency for images**: While main workflows still use LangChain/LangGraph, image generation is now native OpenAI
- **Deleted old files**: Removed `dalle_generator.py` and migration plans - single approach only
- **Initial Test Run**: Organization verification required (FIXED)
- **Performance Issue**: System hangs when generating first character sprite sheet

### CRITICAL ISSUE: Performance Hang (Addressed)
We reworked generation:
- Added timeouts, retries, and concurrent requests
- Sanitized prompts and enforced transparent backgrounds
- Downsampled assets to target resolution immediately
- Added alpha anomaly detection and targeted regen

### Architecture Changes
```
Before:
LangChain DALL-E wrapper → Individual image generation → Manual sprite sheets

Now:
GPT-5 prompt enhancement → GPT Image 1 sprite sheets → Native 4x4 grids
```

### Key Implementation Details
- **Model**: GPT-5 for text, gpt-image-1 for images
- **Sprite Sheets**: Generate complete 4x4 grids in single API calls
- **Transparency**: Native transparent backgrounds for game assets
- **Cost**: ~$0.04 per 1024x1024 high-quality sprite sheet
- **XDG**: Interim artifacts to `~/.local/share` (data) and `~/.local/state` (state); final assets to game-engine assets tree
- **Bevy**: AssetServer configured; integration code references `asset_server_path`

### Files Modified Today
- `workflow.py`: Complete rewrite with native OpenAI client
- `__init__.py`: Updated to version 3.0.0
- Deleted: `dalle_generator.py`, `MIGRATION_PLAN.md`, test files

## Next Steps

### URGENT - Fix Performance Issue
1. **Check API call structure** - Verify `responses.create()` parameters
2. **Add timeout handling** - Prevent indefinite hangs
3. **Add verbose logging** - See exactly where it's hanging
4. **Reduce batch size** - Start with 1 variant, scale up
5. **Verify model names** - Ensure GPT-5 and gpt-image-1 are correct

### After Fix
1. Generate character sprite sheets first
2. Test transparent backgrounds
3. Verify 4x4 grid layout
4. Monitor actual costs vs estimates

### Command That Hangs
```bash
hatch run dl_cli generate-assets \
    crates/game-engine/prompts/characters/universal-character-variants.toml \
    --base-dir crates/game-engine
```

## Technical Context

### OpenAI Integration (NEEDS REVIEW)
- Direct `OpenAI()` client usage
- `responses.create()` with image_generation tool - MAY BE WRONG API
- Base64 decoding for image data
- Conversation state tracking ready for multi-turn

### Sprite Sheet Strategy
- Group variants by archetype
- Generate 16 variants per sheet (4x4) - MAY BE TOO MANY
- Single API call per sheet
- Pillow post-processing for atlas metadata

## Known Issues
- **CRITICAL**: Generation hangs on first sprite sheet
- Organization verification required (RESOLVED)
- API call structure may be incorrect
- No timeout or error handling

## Design Decisions
- **No backwards compatibility**: Single approach only
- **Direct OpenAI**: No abstraction layers
- **Batch optimization**: 16 variants per API call (may need reduction)
- **Horror aesthetic**: Built into prompts

## Investigation Areas
1. Is `openai.responses.create()` the correct API endpoint?
2. Should we use `openai.images.generate()` instead?
3. Are GPT-5 and gpt-image-1 valid model names?
4. Is the tool structure correct for image generation?
