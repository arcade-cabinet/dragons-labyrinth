# Worldbuilder Pipeline Handoff Document

This document captures the current state and roadmap for the **Worldbuilder** in Dragon's Labyrinth. It covers both the **Python prep side** and the **Godot addon side**, how they integrate, how to use them, and concrete next steps. It is written so another AI agent (or dev) can pick it up and continue seamlessly.

---

## 1. Python Worldbuilder Side (Unified Generator)

**Repo path:** `src/dragons_labyrinth/worldbuilder/`

### Implemented (now)
- **Single orchestrator:** `worldbuilder/generator.py`
  - Live linguistic datasets via pandas URLs: `DataSourceManager` (OMW/Cleasby) → `godot/addons/worldbuilder/data/lingua`
  - **LLM-driven biomes/specs:** `biome_variants_generator.py` (Structured Outputs)
    - Writes `prompts/assets/universal-biome-variants.toml`
    - Writes `specs/biomes.yaml` (validated by Pydantic models in `biomes_config.py`)
    - Writes `godot/metadata/regions.json` (three regions per act: starting/intermediary/transitional)
  - **Books motifs:** `books/pipeline.py` (Open Library subjects) → `godot/metadata/books/corpus.json`
  - **Region maps:** `maps/map_pipeline.py` (deterministic synthetic atlas JSON)
  - **Region quests:** `quests/quest_pipeline.py` (Structured Outputs; uses motifs + map feature hints)

### How to Use
1. Call from CLI (see §3): the orchestrator runs all worldbuilder steps idempotently.
2. Image generation scans `prompts/assets/*.toml` and produces assets independently of worldbuilder logic.

### Integration with Godot
- The Godot addon (`DataRepo.gd`) loads all JSON under `godot/addons/worldbuilder/data/lingua/` and exposes pools to `NameForge.gd`.
- Determinism can be introduced by seeding name generation with stable tuples (e.g., `(seed_en, region_key)`), see §6.

### What’s Next (prep side)
- Validation schemas for CSV/JSON inputs; clearer errors.
- Richer seeds (500–1000) and better normalization.
- Deterministic mode flags across scripts to keep output ordering stable.

### Notes for Future Expansion
- Add language-specific phonotactic tables during prep (vowel harmony, stress heuristics) to guide runtime name shaping.
- Provide multiple curated packs (e.g., “Wyrd Celtic”, “Grim Norse”, “Semitic Void”) as separate JSONs.

---

## 2. Godot Addon Side

**Repo path:** `godot/addons/worldbuilder/`

### What’s Implemented (now)
- **Core addon files**:
  - `plugin.cfg`, `plugin.gd` — registers autoload `Worldbuilder.gd`.
  - `Worldbuilder.gd` — exposes `make_name(seed_en: String, key: String)` and holds blend presets.
  - `services/DataRepo.gd` — loads JSON pools under `data/lingua/`.
  - `services/NameForge.gd` — composes 1–2 lemmas using blend weights and mild euphony.
- **New services**:
  - `services/QuestRepo.gd` — loads quests per region, exposes queries.
  - `services/QuestEngine.gd` — minimal quest state machine with signals.
  - `services/MapRepo.gd` — loads region atlas JSON for features/rivers/roads.
- **Editor**: `editor/WorldbuilderDock.gd` — preview dock to audition blends, seeds.
- **Data**: `data/lingua/omw_sample.json`, `data/lingua/old_norse_themes.json` (working samples).
- **Example**: `examples/ExampleNames.gd` prints names for a few seeds and blends.
- **Docs**: `README.md` + `LICENSE` (MIT).

### How to Use
1. Copy `godot/addons/` to your project root.
2. Open the project; `Worldbuilder` autoloads.
3. Call: `Worldbuilder.make_name("forest", "meadows_act1")`.

### Integration with Python Generator
- The Python generator writes/updates `data/lingua/`, `metadata/maps/`, `metadata/quests/`, `metadata/books/`; Godot services pick them up at runtime.

### What’s Next (addon side)
- Deterministic RNG path in `NameForge.gd` and phonotactic polish.
- External JSON for blends instead of hardcoding in `Worldbuilder.gd`.
- Optional **Editor preview dock** to audition blends and seeds.

### Notes for Future Expansion
- Add `MapRepo.gd` and `QuestRepo.gd` when features in §7 and §8 are implemented.
- Provide a minimal HUD debug overlay to show chosen blends and seeds per region.

---

## 3. Integration Overview
- **World flow:** Live data (OMW/Cleasby) + LLM specs → prompts/specs/metadata → Godot loaders
- **Idempotency:** `IdempotencyStore` ensures stable writes for JSON/TOML.
- **Determinism:** Use `(world_seed | region_id | seed_en)` tuples for name RNG; map seeds via region_id; quest IDs via region/title hashes.

---

## 4. Summary: What Remains
- **Worldbuilder:** expand linguistic sources; widen acts/regions coverage; richer motif taxonomy.
- **Addon:** polish NameForge RNG phonotactics; externalize blends; add HUD debug overlay.
- **Both:** tests, profiling, and documentation.

---

## 5. Notes for Future AI Agents
- Keep Python output schemas stable; bump a `version` field if you change them.
- Treat runtime name generation as pure functions of seed + context for reproducibility.
- Document any new JSON files in `README.md` and add them to `build_manifest.py`.

---

## 6. Per-File Patch Explanations (Planned Optimizations)

### Python prep side (`src/dragons_labyrinth/worldbuilder/`)
1) **`requirements.txt` / `pyproject.toml`**  
   - Add `unidecode>=1.3.8`. Optional: `python-slugify>=8` for safe ASCII identifiers.  
   - Rationale: robust transliteration + reproducible normalization.

2) **`prep_omw.py`**  
   - Make headers case-insensitive; validate required columns.  
   - Deduplicate lemmas; strip whitespace; skip empties.  
   - Exit with clear error if columns missing.  
   - Optional flag `--deterministic` → sort keys for stable JSON.

3) **`prep_cleasby_norse.py`**  
   - Validate `theme` + `lemma` presence.  
   - Emit `{ magic_key: "norse_themes", themes: {theme:[lemma]} }`.  
   - Optional: normalize diacritics and provide ASCII fallbacks.

4) **`prep_transliterate.py`**  
   - Upgrade AR/HE mappings, fall back to `unidecode` for unknown chars.  
   - Preserve originals in `_orig`.  
   - `--deterministic` → sort arrays and object keys.

5) **`build_manifest.py`**  
   - Add `--source` param; embed per-file `sha256`, `source`, and timestamp.  
   - Extend to traverse new folders later (maps/quests).  
   - Deterministic traversal (sorted filenames).

6) **`inputs/*`**  
   - Add richer OMW seeds (500–1000 rows).  
   - Add more Norse themes (sea/storm/kinship/battle/omens/shore/ice).  
   - Provide a `README_inputs.md` describing schemas.

### Godot addon side (`godot/addons/worldbuilder/`)
7) **`services/NameForge.gd`**  
   - Deterministic RNG: seed from `(seed_en, region_key)`; use Godot `RandomNumberGenerator`.  
   - Phonotactics polish: collapse duplicate boundary consonants; compress double vowels; optional hyphen joiner.  
   - Latinize or keep UTF-8 depending on config.

8) **`services/DataRepo.gd`**  
   - Support per-file `manifest.json`.  
   - Merge multiple pools; retain a `pools_by_file` index for debugging.  
   - Expose `has_seed(seed)` and `langs_for(seed)` helpers.

9) **`Worldbuilder.gd`**  
   - Load `blend_presets.json` from `data/` (fallback to defaults).  
   - API surface: `make_name(seed_en, blend_key, seed_override=null)`.

10) **`examples/ExampleNames.gd`**  
   - Add loops over multiple seeds and all blend keys.  
   - Print with a fixed RNG seed to verify determinism.

11) **`plugin.gd`**  
   - Optional Editor Dock: preview seeds and blends in-editor.  
   - Add menu entry: “Worldbuilder → Preview Names…”.

12) **`README.md`**  
   - Update usage for external `blend_presets.json`.  
   - Document determinism contract and sample outputs.

---

## 7. Region Map Generation (Implemented v1)

### Implementation
- `maps/map_pipeline.py` generates deterministic atlas JSON per region (size, cells with biome/band/elev/moist; features, rivers, roads). Stored under `godot/metadata/maps/<region>/` with manifest.

### Determinism Contract
- `seed = blake3(world_seed || region_id || "maps" || version)` used everywhere for AOI pick, warps, placement, quest casting.

### Storage & Provenance
- Every generated folder ships a `manifest.json` with:
  - `sources[]` (datasets, versions)
  - `parameters` (projections, warp scales, thresholds)
  - `sha256` per file
  - `acts` mapping for corruption overlays

### Notes for Future Expansion
- Multiple atlas resolutions; richer biome adjacency shaping; optional OSM overlays with clean licensing; unit tests for plausibility.

---

## 8. Quest Generation (Implemented v1)

### Generation Pipeline
- **Motifs**: Extracted from books corpus (Open Library subjects)
- **Regional prompt**: LLM Structured Outputs with motifs + map features + style guide
- **Output**: Quest bundles balanced across acts; deterministic IDs; stored under `godot/metadata/quests/<region>/`

### New Python Utilities (prep side)
- `quest_seed_pipeline.py` – ingest PD stories → export normalized quest templates (JSON).
- `quest_casting.py` – assign NPC roles from region `features.json` and NPC pools.
- `quest_manifest.py` – provenance for templates (story titles, years, URLs), SHA256 of JSON.

### Outputs
- `godot/addons/worldbuilder/data/quests/<region_id>/quests.json`
- `godot/addons/worldbuilder/data/quests/<region_id>/manifest.json`

### Godot Integration (runtime side)
- `QuestRepo.gd` – load quests per region, expose graph API.
- `QuestEngine.gd` – minimal state machine:
  - `start(quest_id)`
  - `advance(event)`
  - `is_available(quest_id, world_ctx)` (checks act/biome/flags)
  - Emits signals for UI & feature markers.
- `Worldbuilder.gd` gains:
  - `available_quests(region_id, world_ctx)`
  - `quest_markers(region_id)` → feature coordinates for the HUD.

### Example: Template → Region Instance
- Template: “The Watcher at the Mill” (investigation)
- Tavern rumor → Old Mill (swamp) → Hidden reliquary → Night confrontation.
- Region instance (Meadows Act 1):
  - feature: tavern at (12,8) spawns rumor_monger NPC
  - feature: old_mill auto-placed near river in swamp hex
  - beat: confront becomes a small labyrinth entrance with Act-appropriate boss

### Companion & Sentimental Items
- Quests can tag therapy hooks: after confrontations, companions gain/lose stress.
- Certain quests output Sentimental Items (clean inputs for the Forge System).

### QA & Balancing
- Unit tests for graph validity (acyclic unless designed loops), act constraints, and geographical plausibility.
- Runtime asserts to ensure all quest beats map to actual features on the active atlas.

### Notes for Future Expansion
- Expand quest templates with more diverse narrative patterns.
- Integrate AI-driven quest generation for dynamic content.
- Enhance runtime quest state machine with richer event handling.
- Add tooling for quest debugging and visualization in Godot editor.

---

## 9. Additional Thoughts

- **Atlas Resolution Tradeoff**: Choosing between 64×64 and 96×96 atlas resolutions impacts detail versus performance; balance is needed per region.
- **OSM Licensing Safety**: Consider whether to include OSM-derived features or maintain strictly synthetic maps for clean licensing and simplicity.
- **Quest Pacing**: Ideal quest template count per act/region appears to be 6–12 to maintain engaging pacing without overload.
- **Act 3 Traversal Readability**: Decisions on Act 3 void transforms should balance increased traversal difficulty with player readability to avoid frustration.