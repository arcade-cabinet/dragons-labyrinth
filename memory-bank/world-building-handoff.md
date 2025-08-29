## Dragon's Labyrinth – Handoff (Godot Pivot + Regions-First Plan)

This document captures the current direction, why we pivoted, and how to proceed. It is intentionally human-authored and stable; do NOT auto‑generate it. Use it to brief any agent or contributor before running generators.

### Core Direction
- Godot 4 front‑end. Python generator/orchestrator drives assets, code, and manifests.
- 2.5D hex overworld (tilted hex view), with 3D first‑person “Transitions” for major beats.
- Horror‑first: dread progression (0–4), dual Light/Dark philosophical paths, companion psychology.

### World Structure: Regions First (Bands 1–180)
- Abandon fully open, unstructured overworld for now; favor named Regions grouped by bands:
  - 1–20: Peace → Unease (gentle, pastoral onboarding)
  - 21–40: Unease → Dread (transition zones)
  - 41–60: Dread → Terror (approach the Labyrinth)
  - 61–120: Terror → Despair → Madness (return journey, social apocalypse)
  - 121–180: Madness → Void (cosmic horror)
- Target: ~3 regions per band. Each region defines:
  - Name, summary, biome mix (layer‑cake tiles), adjacency targets to neighboring regions
  - Travel pacing: typical days to traverse, day/night expectations, rest/camp pressure
  - Core quests and NPCs; sample monsters/encounters; transition hooks (3D scenes)

### Tiles: Layer‑Cake Design
- Base biome + overlays (paths, features) with deterministic adjacency rules.
- PNGs 128×128 with transparent backgrounds; packed into Godot TileSets.
- Movement pacing binds to day/night cycle: walking 1 tile; optional 2–3 tile “run” with risk.
- Rest/shelter mechanics required in harsher climates and at night (later bands).

### Transitions: 3D First‑Person Scenes
- Major emotional beats are 3D scenes (e.g., Bandit Camp, Cleansing Ritual, Blood Pact, Last Sanctuary, etc.).
- Godot `TransitionLoader` swaps from overworld to `res://scenes/transitions/<name>.tscn` (or from manifest).

### Orchestrator: What’s Automated vs Manual
- Manual (critical): First pass of Region meta‑prompts. A human/agent crafts high‑level prompts per region reflecting Themes and Architecture. These become the source of truth for generation.
- Automated (structured outputs via GPT‑5):
  - Deriving per‑region prompt TOMLs (assets) and YAML specs (code rules) from the manual meta‑prompts + compact guides
  - Generating biomes/adjacency/movement/hazard rules and Godot manifests
  - Emitting transitions manifest and Godot scene stubs (when ready)

### Stable Inputs for AI
- Do not feed the full, mutable memory‑bank into models. Instead, use distilled guides:
  - `source/guides/Architecture.guide.md`
  - `source/guides/Themes.guide.md`
- Orchestrator also grounds itself on curated prompt libraries:
  - `prompts/orchestrator/**` (systems, transitions, project setup)

### Project Layout (key paths)
- Godot project: `godot/`
  - Assets: `godot/assets/`
  - Metadata (manifests): `godot/metadata/`
  - Scripts: `godot/scripts/`
  - TileSets: `godot/tilesets/`
- Orchestrator source: `src/dragons_labyrinth/`
  - Constants: `constants.py` (single source of path truth)
  - Orchestrator: `orchestrator.py`
  - CLI: `cli.py` (entry `dl_cli`)
  - Guides: `source/guides/`
  - Orchestrator prompts: `prompts/orchestrator/**`
  - Regions outputs: `prompts/regions/<id>/prompt.toml`, `specs/regions/<id>/spec.yaml`

### Commands (internal use)
- Distill stable guides (copies for now; can later summarize with GPT‑5):
  - `hatch run dl_cli distill_guides`
- Bootstrap regions from Themes/Architecture using structured outputs (after manual meta‑prompt direction is set):
  - `hatch run dl_cli bootstrap_regions`
- Bootstrap project scaffolding (as needed):
  - `hatch run dl_cli bootstrap_project`
- Generate GDScript biome rules from specs:
  - `hatch run dl_cli codegen-biomes-gd`
- Asset generation (per TOML):
  - `hatch run dl_cli generate-assets <path/to/prompt.toml> -n 3 --autonomous`

### Asset Generation Rules of Thumb
- Transparent PNGs enforced; alpha anomaly detection regenerates outliers.
- Upscale generate (>=256×256) then downsample to target (e.g., 128×128) for quality.
- Sprite sheets are interim; final variants live in `godot/assets/<category>/<archetype>/`.
- Interim sheets/manifests in XDG paths; final manifests in `godot/metadata/`.

### Manual Region Meta‑Prompts (What to Write)
For each region:
- Title, band, emotional goal, tone
- Biome mix (with counts/ratios), adjacency to two plausible neighbors
- Travel days target and rest pressure; day/night constraints; weather notes
- 3–5 core quests and 3–5 side encounters
- Named NPCs (roles, Light/Dark/Companion implications)
- Transition hook(s) with scene names to load via `TransitionLoader`
- Asset cues (what must exist visually/audio)

### Immediate Next Steps
1. Author initial Region meta‑prompts (Band 1–20: three gentle, pastoral regions).
2. Run `distill_guides` → `bootstrap_regions` to materialize region prompts/specs.
3. Generate minimal biomes and build TileSets; paint sample hex grid in Godot.
4. Implement first Transition 3D scene and verify swaps from `World.gd`.

### Non‑Goals (for now)
- Fully open, unbounded procedural overworld. We’ll revisit once Regions are solid.
- Over‑reliance on raw memory‑bank files in prompts; prefer distilled guides.

This plan preserves the directoral vision while maximizing AI’s strength with structured, themed scaffolding. Manual meta‑prompts set intent; orchestrator scales it consistently.


