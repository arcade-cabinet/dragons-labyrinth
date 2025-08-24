# Style Guide Crate

## Purpose

This crate provides **programmatic style enforcement** for Dragon's Labyrinth. It's not just documentation - it's actual code that returns values for all visual and aesthetic decisions.

## Core Principle

**Every visual decision queries this crate**. This ensures consistency across:
- Blender model exports
- UI element sizing
- Color palettes
- Animation timings
- Audio curves
- Material properties

## Usage

```rust
use style_guide::{StyleGuide, DreadLevel};

// Create guide for current dread level
let guide = StyleGuide::new(DreadLevel(2));

// Query any visual parameter
let character_scale = guide.character_scale();      // 1.7
let ui_color = guide.colors().primary();            // Shifts from blue to red
let margin = guide.ui_layout().margin(MarginSize::Medium);
let shake = guide.camera().shake_intensity();       // Increases with dread
```

## Critical: Blender Integration

All Blender exports MUST use these settings:
```rust
let settings = ScaleSystem::blender_export_settings();
// unit_scale: 1.0
// forward_axis: "-Z"
// up_axis: "Y"
```

## Dread Adaptation

Everything adapts to dread level (0-4):
- **0 (Peace)**: Clear, regular, calm
- **1 (Unease)**: Slight distortion begins
- **2 (Anxiety)**: Noticeable warping
- **3 (Horror)**: Heavy corruption
- **4 (Madness)**: Complete chaos

## Modules

- `scale` - Model and UI element sizing
- `color` - Adaptive color palettes
- `typography` - Font and text settings
- `animation` - Timing and easing curves
- `ui_layout` - Margins, padding, positioning
- `audio` - Volume curves and parameters
- `materials` - PBR material properties
- `camera` - FOV, shake, distance
- `particles` - Emission rates and lifetimes
- `lighting` - Ambient and shadow values

## Integration Points

This crate should be used by:
- `game-engine` - Runtime visual queries
- `build-tools` - Asset generation parameters
- `blender-bridge` - Export scale consistency
- AI agents - When generating visual content

## No Placeholders

This crate returns REAL values, not placeholders. Every function returns meaningful, tested values that create the intended horror progression.
