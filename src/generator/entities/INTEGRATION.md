#!/usr/bin/env bash
# Repo-wide audit for Dragons’ Labyrinth
# - Verifies structure, coding standards (Py3.13 generics, absolute imports)
# - Summarizes entities/training coverage and CLI wiring
# - Writes results into audit/_report.txt

set -euo pipefail

ROOT_DIR=$(git rev-parse --show-toplevel 2>/dev/null || pwd)
OUT_DIR="$ROOT_DIR/audit"
REPORT="$OUT_DIR/_report.txt"

mkdir -p "$OUT_DIR"
: > "$REPORT"

say() { echo -e "$*" | tee -a "$REPORT"; }

say "# Dragons' Labyrinth — Repo Audit"
 say "generated: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
 say "root: $ROOT_DIR"

say "\n## 1) Tree (top level)"
if command -v tree >/dev/null 2>&1; then
  (cd "$ROOT_DIR" && tree -L 2 | sed 's/\x1b\[[0-9;]*m//g') | tee -a "$REPORT" || true
else
  (cd "$ROOT_DIR" && find . -maxdepth 2 -type d -print | sed 's/^\.\///') | tee -a "$REPORT"
fi

say "\n## 2) Python version & env"
python3 --version 2>&1 | tee -a "$REPORT" || true

say "\n## 3) Coding standards checks"
# No Optional/List/Dict
say "- Disallowed generics (Optional, List, Dict):"
rg -n "\b(Optional|List|Dict)\[" --glob '!**/.venv/**' --glob '!**/__pycache__/**' || true | tee -a "$REPORT"

# Absolute imports only (flag common relative patterns)
say "\n- Relative imports (should be avoided):"
rg -n "^from \." src --glob '!**/__pycache__/**' || true | tee -a "$REPORT"

# Typing Any where avoidable (summary only)
say "\n- Heavy Any usage (heads-up review):"
rg -n "\bAny\b" src --glob '!**/__pycache__/**' || true | tee -a "$REPORT"

say "\n## 4) Entities subpackage wiring"
# Check manager CLI commands present
say "- manager.py commands:"
rg -n "@app\.command\(" src/generator/entities/manager.py | tee -a "$REPORT" || true

# Ensure direct imports (no try/except fallbacks)
say "\n- manager.py generation imports:"
rg -n "from generator\.image_generator|from generator\.godot_generator" src/generator/entities/manager.py | tee -a "$REPORT" || true

say "\n- image generator functions present:"
rg -n "def generate_(biome|token|body_bases)" src/generator/image_generator.py | tee -a "$REPORT" || true

say "\n- godot exporter present:"
rg -n "def export_world_hooks_json" src/generator/godot_generator.py | tee -a "$REPORT" || true

say "\n## 5) Training coverage (world_hooks expected keys)"
EXPECTED_KEYS='dominant_biome|has_rivers|has_trails|scale_hint'
for mod in regions settlements dungeons factions; do
  say "- training/$mod.py hooks coverage:"
  rg -n "(world_hooks|$EXPECTED_KEYS)" "src/generator/entities/training/$mod.py" || true | tee -a "$REPORT"
  echo >> "$REPORT"
done

say "\n## 6) SQLite/HBF presence"
if ls memory-bank/world-output/*.hbf >/dev/null 2>&1; then
  say "- Found HBF file(s):"
  ls -lh memory-bank/world-output/*.hbf | tee -a "$REPORT"
else
  say "- No HBF files found under memory-bank/world-output/"
fi

say "\n## 7) Art & data target folders"
for p in game/art game/data/world_hooks; do
  mkdir -p "$ROOT_DIR/$p"
  say "- ensured: $p"
done

say "\n## 8) Suggested next commands"
cat <<'EON' | tee -a "$REPORT"
# help
hatch run python -m generator.entities.manager --help

# extract (edit the path if different)
hatch run python -m generator.entities.manager extract --hbf inputs/raw_game.hbf

# export hooks
hatch run python -m generator.entities.manager export-hooks --out game/data/world_hooks

# generate images
hatch run python -m generator.entities.manager gen-images biomes --out game/art --size 1024x1024
hatch run python -m generator.entities.manager gen-images tokens --out game/art --size 1024x1024
hatch run python -m generator.entities.manager gen-images body-bases --out game/art --size 1024x1024
EON

say "\n✅ Audit complete. See $REPORT"
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://dragons-labyrinth.io/schemas/world_hooks.schema.json",
  "title": "World Hooks",
  "type": "object",
  "required": ["name", "kind", "hooks"],
  "properties": {
    "name": {"type": "string", "minLength": 1},
    "kind": {"type": "string", "enum": ["regions", "settlements", "dungeons", "factions"]},
    "confidence": {"type": "number", "minimum": 0, "maximum": 1},
    "biomes": {"type": "array", "items": {"type": "string"}},
    "hooks": {
      "type": "object",
      "required": ["dominant_biome", "scale_hint"],
      "properties": {
        "dominant_biome": {"type": "string"},
        "has_rivers": {"type": "boolean"},
        "has_trails": {"type": "boolean"},
        "scale_hint": {"type": "string", "enum": ["wilderness", "hamlet", "village", "town", "city", "dungeon"]}
      },
      "additionalProperties": true
    }
  },
  "additionalProperties": false
}
# Integration Guide for `entities` Subpackage

## 1. Overview
The `entities` subpackage is the data spine of Dragons’ Labyrinth. It parses **HBF/SQLite** world data (The Lands of Vo’il), enriches it into normalized entities, exports **Godot‑ready world hooks**, and automates **sprite sheets** via gpt-image-1. Everything is CI/headless friendly.

## 2. Worldbuilding Data Flow
```
HBF (SQLite) → entities.processor (regex/ML) → training/* (domain extractors)
            → SQLModel rows + world_hooks dict → godot_generator (JSON)
                                           → image_generator (PNGs)
```
- **Source:** `memory-bank/world-output/*.hbf` (or your path)
- **Processor:** `src/generator/entities/processor.py`
- **Training:** `src/generator/entities/training/{regions,settlements,dungeons,factions}.py`
- **Hooks contract:** `src/generator/entities/schemas/world_hooks.schema.json`

## 3. Godot Integration (4.4 + Zehir hex map)
- Export location: `game/data/world_hooks/{regions,settlements,dungeons}/...` + `index.json`
- Each file contains `{ name, kind, confidence?, biomes?, hooks }` where `hooks` includes `dominant_biome`, `has_rivers?`, `has_trails?`, `scale_hint`.
- Example Godot loader (GDScript sketch):
```gdscript
var hooks_dir := "res://data/world_hooks/regions"
for file_name in DirAccess.get_files_at(hooks_dir):
    var json := JSON.parse_string(FileAccess.get_file_as_string(hooks_dir + "/" + file_name))
    if json and json.has("hooks"):
        var biome := json["hooks"]["dominant_biome"]
        # Select TileSet region / autotile based on biome
```
- Works with **Zehir/godot-hexagon-tile-map-layer**: use `TileMapLayer` for axial coordinates; stamp tiles according to `biome` and any overlays (rivers/trails).

## 4. Sprite Sheets (Transparent PNG)
Generated to `game/art/` by `image_generator.py`:
- `generate_biome_spritesheet()` → 3×3 hex biomes (Desert, Forest, Jungle, Mountains, Ocean, Plains, Swamps, Tundra, Hills)
- `generate_token_sprites()` → 4×2 villager tokens
- `generate_body_bases()` → 2 base figurines (masc/fem)
All use `background=transparent`, configurable `--size` (default `1024x1024`).

## 5. Automation (Dual‑mode Manager)
Typer CLI in `entities/manager.py`:
```bash
hatch run python -m generator.entities.manager extract --hbf inputs/raw_game.hbf
hatch run python -m generator.entities.manager export-hooks --out game/data/world_hooks
hatch run python -m generator.entities.manager gen-images biomes --out game/art --size 1024x1024
```
Library mode:
```py
from pathlib import Path
from generator.entities.manager import EntitiesManager
from generator.godot_generator import export_world_hooks_json

mgr = EntitiesManager(db_path="inputs/raw_game.hbf")
mgr.run()
export_world_hooks_json(Path("game/data/world_hooks"))
```

## 6. Extensibility
- Add hooks → update the **training** extractor first and the JSON **schema**.
- New entity kinds → add `training/<kind>.py` and register in processor/exporter.
- New art sheets → add a function in `image_generator.py` and wire into `manager.py`.

## 7. Validation
Use the provided schema to validate exports (example with `jsonschema`):
```bash
python - <<'PY'
import json, sys
from jsonschema import validate
from pathlib import Path
schema = json.loads(Path('src/generator/entities/schemas/world_hooks.schema.json').read_text())
for p in Path('game/data/world_hooks').rglob('*.json'):
    data = json.loads(p.read_text())
    validate(instance=data, schema=schema)
print('ok')
PY
```

## 8. CI‑Friendly Audit
Run the audit to confirm structure and standards:
```bash
bash tools/repo_audit.sh
cat audit/_report.txt
```
