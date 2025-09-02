from __future__ import annotations
import argparse, json, pathlib, os
from openai import OpenAI
from ai.schemas import GameCanon, ThemeBible, WorldPlan, RegionBible, WorldBook, ImagePlan, BiomeTileset, TileVariant, IconJob, ECSWorld, ECSEntity, ECSResources
from ai.util import write_if_changed, ROOT, call_json_schema, normalize_json_for_model
from ai.prompts import SYSTEM_CREATIVE, SYSTEM_IMAGE
from ai.images import generate_images
from ai.dialogue import expand_npc_dialogue, expand_questlines, write_dialogue_bundle
from ai.atlas import pack_uniform_grid

MODEL_DEFAULT = os.getenv("OPENAI_MODEL", "gpt-5.1")
IMAGE_MODEL_DEFAULT = os.getenv("OPENAI_IMAGE_MODEL", "gpt-image-1")

CONTENT = ROOT / "content"
BUILD = ROOT / "build"
MASTER = BUILD / "master"
WORLD = BUILD / "world"
ASSETS = ROOT / "apps/game/assets"

def canonize(model: str) -> None:
    client = OpenAI()
    arch = (CONTENT/"Architecture.md").read_text(encoding="utf-8")
    themes = (CONTENT/"Themes.md").read_text(encoding="utf-8")
    canon_prompt = f"""{SYSTEM_CREATIVE}
Source (Architecture.md):
{arch}

Task: Convert into GameCanon JSON with:
- title, pillars (<=7), world_rules (<=12), starting_hex (e.g., "0,0")
- progression: five bands with band, statement, goals[], gating_mechanics[]
Return JSON only.
"""
    canon_text = call_json_schema(client, model, canon_prompt, GameCanon)
    canon = GameCanon.model_validate_json(json.dumps(normalize_json_for_model(GameCanon, canon_text)))
    write_if_changed(MASTER/"canon.json", canon.model_dump_json(indent=2).encode())

    theme_prompt = f"""{SYSTEM_CREATIVE}
Source (Themes.md):
{themes}

Task: Convert into ThemeBible JSON with:
- style_directives, content_constraints
- bands[]: tone, biome_palette, enemy_archetypes, quest_archetypes, art_prompts
Return JSON only.
"""
    theme_text = call_json_schema(client, model, theme_prompt, ThemeBible)
    tb = ThemeBible.model_validate_json(json.dumps(normalize_json_for_model(ThemeBible, theme_text)))
    write_if_changed(MASTER/"themes.json", tb.model_dump_json(indent=2).encode())
    print("Wrote master/canon.json and master/themes.json")

def plan(model: str) -> None:
    client = OpenAI()
    canon = json.loads((MASTER/"canon.json").read_text())
    themes = json.loads((MASTER/"themes.json").read_text())
    prompt = f"""{SYSTEM_CREATIVE}
Using this GameCanon and ThemeBible, propose a WorldPlan with five region_bands.
Keep names evocative, tones aligned, and biome palettes consistent with ThemeBible.
Return JSON only.

GameCanon:
{json.dumps(canon, indent=2)}

ThemeBible:
{json.dumps(themes, indent=2)}
"""
    plan_text = call_json_schema(client, model, prompt, WorldPlan)
    plan_obj = WorldPlan.model_validate_json(json.dumps(normalize_json_for_model(WorldPlan, plan_text)))
    write_if_changed(WORLD/"plan.json", plan_obj.model_dump_json(indent=2).encode())
    print("Wrote world/plan.json")

def expand(model: str) -> None:
    client = OpenAI()
    plan = json.loads((WORLD/"plan.json").read_text())
    themes = json.loads((MASTER/"themes.json").read_text())
    regions: list[dict] = []
    for rb in plan["region_bands"]:
        prompt = f"""{SYSTEM_CREATIVE}
Expand the region for band {rb['band']} ("{rb['name']}") into a RegionBible.
Requirements:
- 18–36 hex_points ("q,r"). Mix village, shrine, lair, ruin, dungeon, camp, forge, portal.
- Quests: >=1 main, >=2 side, >=1 companion, >=1 moral/forge hook.
- NPCs/Creatures reflect the band's tone/biomes.
Return JSON only.

WorldPlan:
{json.dumps(plan, indent=2)}

ThemeBible:
{json.dumps(themes, indent=2)}
"""
        region_text = call_json_schema(client, model, prompt, RegionBible)
        region = RegionBible.model_validate_json(json.dumps(normalize_json_for_model(RegionBible, region_text)))
        regions.append(json.loads(region.model_dump_json()))
        write_if_changed(WORLD/f"region_{rb['band'].replace('-','_')}.json", region.model_dump_json(indent=2).encode())
    worldbook = {"plan": plan, "regions": regions}
    write_if_changed(WORLD/"worldbook.json", json.dumps(worldbook, indent=2).encode())
    print("Wrote world/region_*.json and worldbook.json")

def image_plan(model: str) -> None:
    client = OpenAI()
    themes = json.loads((MASTER/"themes.json").read_text())
    prompt = f"""{SYSTEM_IMAGE}
Create an ImagePlan JSON based on ThemeBible bands. For each biome, produce 3 TileVariant prompts.
Also include icons for poi kinds (village, shrine, lair, ruin, camp, dungeon, forge, portal).
Return JSON only.

ThemeBible:
{json.dumps(themes, indent=2)}
"""
    res_text = call_json_schema(client, model, prompt, ImagePlan)
    plan_obj = ImagePlan.model_validate_json(json.dumps(normalize_json_for_model(ImagePlan, res_text)))
    write_if_changed(BUILD/"image_plan.json", plan_obj.model_dump_json(indent=2).encode())
    print("Wrote build/image_plan.json")

def images(model: str, image_model: str) -> None:
    plan = ImagePlan.model_validate_json((BUILD/"image_plan.json").read_text())
    generate_images(plan, ASSETS, image_model)
    # Build a texture atlas for efficient runtime usage
    atlas_dir = BUILD / "atlas"
    atlas_dir.mkdir(parents=True, exist_ok=True)
    pack_uniform_grid(ASSETS, atlas_dir / "atlas.png", atlas_dir / "atlas.json")
    print("Generated images to apps/game/assets and built atlas to build/atlas")

def narrative(model: str) -> None:
    wb = WorldBook.model_validate_json((WORLD/"worldbook.json").read_text())
    dialogues = expand_npc_dialogue(wb, model)
    questlines = expand_questlines(wb, model)
    out = BUILD / "narrative"
    write_dialogue_bundle(dialogues, questlines, out)
    print("Wrote narrative bundles to build/narrative")

def ecs_ir() -> None:
    # Convert existing worldbook.json into ECS IR deterministically
    wb = json.loads((WORLD/"worldbook.json").read_text())
    # Build biome map
    biomes: list[str] = []
    for r in wb["regions"]:
        for b in r.get("biomes", []):
            if b not in biomes:
                biomes.append(b)
    biome_to_index = {b:i for i,b in enumerate(biomes)}
    # POI registry
    poi_types: list[str] = []
    for r in wb["regions"]:
        for p in r.get("hex_points", []):
            k = p.get("kind")
            if k and k not in poi_types:
                poi_types.append(k)
    resources = ECSResources(biome_to_index=biome_to_index, poi_types=poi_types)
    # Entities: hex tiles only for now (others later)
    entities: list[ECSEntity] = []
    # Derive world radius heuristically from plan bands
    radius = 180
    for q in range(-radius, radius+1):
        for r in range(-radius, radius+1):
            s = -q - r
            if abs(q) <= radius and abs(r) <= radius and abs(s) <= radius:
                # map distance to band
                dist = (abs(q)+abs(r)+abs(s))//2
                band = (
                    "1-20" if dist <= 20 else
                    "21-40" if dist <= 40 else
                    "41-60" if dist <= 60 else
                    "61-120" if dist <= 120 else
                    "121-180"
                )
                region = next((rg for rg in wb["regions"] if rg["band"] == band), None)
                if region and region.get("biomes"):
                    biome = region["biomes"][0]
                    entities.append(ECSEntity(
                        name=f"Hex({q},{r})",
                        components={
                            "HexTile": {"q": q, "r": r, "biome": biome, "distance_band": band}
                        }
                    ))
    ecs = ECSWorld(resources=resources, entities=entities)
    out_dir = BUILD/"ecs"
    out_dir.mkdir(parents=True, exist_ok=True)
    write_if_changed(out_dir/"world.json", ecs.model_dump_json(indent=2).encode())
    print("Wrote build/ecs/world.json")

def gen_world_rust() -> None:
    # Generate Rust source that registers a generated world at startup (no runtime AI/JSON)
    wb = json.loads((WORLD/"worldbook.json").read_text())
    # Collect biomes by band, keep order stable
    band_to_biomes: dict[str, list[str]] = {}
    all_biomes: list[str] = []
    for region in wb.get("regions", []):
        biomes = [b for b in region.get("biomes", []) if isinstance(b, str)]
        band_to_biomes[region.get("band", "")] = biomes
        for b in biomes:
            if b not in all_biomes:
                all_biomes.append(b)

    # Build Rust code (algorithmic spawn by distance band)
    def rs_string_list(items: list[str]) -> str:
        return ", ".join([f"\"{i}\".to_string()" for i in items])

    biome_map_entries = ",\n            ".join([f"m.insert(\"{b}\".to_string(), {i});" for i, b in enumerate(all_biomes)])

    band_arrays = []
    for band in ["1-20","21-40","41-60","61-120","121-180"]:
        biomes = band_to_biomes.get(band, [])
        band_arrays.append(f"const BAND_{band.replace('-','_')}_BIOMES: &[&str] = &[{', '.join([f'\"{b}\"' for b in biomes])}];")

    band_pick_logic = "\n".join([
        "        let biomes: &[&str] = if dist <= 20 { BAND_1_20_BIOMES }",
        "        else if dist <= 40 { BAND_21_40_BIOMES }",
        "        else if dist <= 60 { BAND_41_60_BIOMES }",
        "        else if dist <= 120 { BAND_61_120_BIOMES }",
        "        else { BAND_121_180_BIOMES };",
        "        let biome = biomes.first().copied().unwrap_or(\"default\");",
    ])

    code = f"""// AUTO-GENERATED by ai gen-world. Do not edit by hand.
use bevy::prelude::*;
use std::collections::HashMap;
use crate::ecs_bridge::{{HexTile, BiomeTextureMap}};

{chr(10).join(band_arrays)}

pub fn register_generated_world(app: &mut App) {{
    // Insert biome texture mapping
    let mut m: HashMap<String, u32> = HashMap::new();
    {biome_map_entries}
    app.insert_resource(BiomeTextureMap {{ mappings: m }});

    // Spawn hex tiles algorithmically at startup
    app.add_systems(Startup, spawn_generated_hexes);
}}

fn spawn_generated_hexes(mut commands: Commands) {{
    let radius: i32 = 180;
    for q in -radius..=radius {{
        for r in -radius..=radius {{
            let s = -q - r;
            if q.abs() <= radius && r.abs() <= radius && s.abs() <= radius {{
                let dist = (q.abs() + r.abs() + s.abs())/2;
{band_pick_logic}
                commands.spawn((HexTile {{ q, r, biome: biome.to_string(), distance_band: if dist<=20 {{ \"1-20\".to_string() }} else if dist<=40 {{ \"21-40\".to_string() }} else if dist<=60 {{ \"41-60\".to_string() }} else if dist<=120 {{ \"61-120\".to_string() }} else {{ \"121-180\".to_string() }} }}, Name::new(format!("Tile({{q}},{{r}})"))));
            }}
        }}
    }}
}}
"""

    out_rs = ROOT/"crates/world/src/generated_world.rs"
    out_rs.parent.mkdir(parents=True, exist_ok=True)
    write_if_changed(out_rs, code.encode("utf-8"))
    print(f"Wrote {out_rs}")

def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["canonize","plan","expand","image-plan","images","narrative","ecs-ir","gen-world"])
    ap.add_argument("--model", default=MODEL_DEFAULT)
    ap.add_argument("--image-model", default=IMAGE_MODEL_DEFAULT)
    args = ap.parse_args()

    MASTER.mkdir(parents=True, exist_ok=True)
    WORLD.mkdir(parents=True, exist_ok=True)

    if args.cmd == "canonize":
        canonize(args.model)
    elif args.cmd == "plan":
        plan(args.model)
    elif args.cmd == "expand":
        expand(args.model)
    elif args.cmd == "image-plan":
        image_plan(args.model)
    elif args.cmd == "images":
        images(args.model, args.image_model)
    elif args.cmd == "narrative":
        narrative(args.model)
    elif args.cmd == "ecs-ir":
        ecs_ir()
    elif args.cmd == "gen-world":
        gen_world_rust()

if __name__ == "__main__":
    main()
