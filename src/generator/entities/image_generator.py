"""
Image Generator - Creates sprite sheets using OpenAI gpt-image-1.

Generates transparent PNG sprite sheets for Dragon's Labyrinth using real data:
- Data-driven biome tiles from transformer region results
- Character tokens based on actual NPC/faction data
- Settlement sprites from processed settlement data
- Dungeon entrance sprites from processed dungeon data

Uses Jinja2 templates for consistent, data-driven prompt generation.
"""

from __future__ import annotations

import base64
import json
import time
from pathlib import Path
from typing import Any

from jinja2 import Environment, FileSystemLoader
from openai import OpenAI


# Template environment
def _get_template_env() -> Environment:
    """Get Jinja2 template environment."""
    template_dir = Path(__file__).parent / "prompt_templates"
    return Environment(loader=FileSystemLoader(template_dir))


def _render_template(template_name: str, context: dict[str, Any]) -> str:
    """Render Jinja2 template with context data."""
    env = _get_template_env()
    template = env.get_template(template_name)
    return template.render(**context)


# Prompt templates for consistent generation
BIOME_SPRITESHEET_PROMPT = """A 1024x1024 transparent sprite sheet for Dragon's Labyrinth 2.5D hex RPG.
Grid: 3x3 layout (nine 256x256 cells) with biomes from The Lands of Vo'il.

Biomes (row-major order):
Row 1: Desert, Forest, Jungle
Row 2: Mountains, Plains, Swamps  
Row 3: Tundra, Ocean, Void (end-game corruption)

Style: 2.5D top-down hex tiles, seamless edges, medieval dark fantasy progressing to cosmic horror.
Clean albedo textures, soft lighting from top-left, no borders, no text, transparent background.
Each tile 256x256, centered in cell, designed for hexagon tile map layer in Godot."""

TOKEN_SPRITES_PROMPT = """A 1024x1024 transparent sprite sheet for Dragon's Labyrinth character tokens.
Grid: 4x3 layout (twelve 256x256 cells) with chess-piece style villager tokens.

Characters by presentation and skin tone:
Row 1 (Masculine): Light, Medium-Light, Medium, Medium-Dark skin
Row 2 (Feminine): Light, Medium-Light, Medium, Medium-Dark skin  
Row 3 (Androgynous): Light, Medium-Light, Medium, Medium-Dark skin

Style: Chess-piece tokens with small pedestal base, neutral gray ring for team coloring.
2.5D top-down readable at 64-96px, medieval fantasy, clean silhouettes, no weapons/class icons.
Transparent background, uniform scale and alignment."""

BODY_BASES_PROMPT = """A 1024x1024 transparent sprite sheet for Dragon's Labyrinth character creation.
Layout: 1 row with 2 equal cells (512x1024 each), centered and scaled identically.

Content: Two neutral pawn-like figurine bodies with no clothing, no platform.
Left: Masculine base body (broad chest, thicker limbs)
Right: Feminine base body (stylized breast shape, narrower waist - non-anatomical)

Style: Clean carved-pawn aesthetic, soft shading, no outlines, designed for recoloring 
and overlaying clothes/hair/accessories. Medieval fantasy base for character creation."""


def _get_openai_client() -> OpenAI:
    """Get OpenAI client instance."""
    return OpenAI()


def _generate_image_with_retry(prompt: str, size: str = "1024x1024", retries: int = 3) -> bytes:
    """Generate image with retry logic for reliability."""
    
    client = _get_openai_client()
    
    for attempt in range(retries):
        try:
            response = client.images.generate(
                model="dall-e-3",
                prompt=prompt,
                size=size,
                quality="hd",
                response_format="b64_json",
                n=1
            )
            
            # Extract base64 image data
            image_b64 = response.data[0].b64_json
            return base64.b64decode(image_b64)
            
        except Exception as e:
            print(f"🔄 Generation attempt {attempt + 1} failed: {e}")
            if attempt < retries - 1:
                time.sleep(2.0)  # Wait before retry
            else:
                raise e


def generate_biome_spritesheet(output_dir: Path, size: str = "1024x1024") -> Path:
    """
    Generate biome spritesheet with all 7 core biomes from HBF analysis.
    
    Args:
        output_dir: Directory to save the generated image
        size: Image size (default: 1024x1024)
        
    Returns:
        Path to generated image file
    """
    
    print("🎨 Generating biome spritesheet...")
    
    output_path = output_dir / "biomes_3x3.png"
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate image
    image_data = _generate_image_with_retry(BIOME_SPRITESHEET_PROMPT, size)
    
    # Save to file
    with open(output_path, "wb") as f:
        f.write(image_data)
    
    # Create layout JSON for Godot slicing
    layout_data = {
        "grid": [3, 3],
        "cell_size": [256, 256],
        "biomes": {
            "desert": [0, 0], "forest": [1, 0], "jungle": [2, 0],
            "mountains": [0, 1], "plains": [1, 1], "swamps": [2, 1], 
            "tundra": [0, 2], "ocean": [1, 2], "void": [2, 2]
        }
    }
    
    layout_path = output_dir / "biomes_3x3_layout.json"
    import json
    with open(layout_path, "w", encoding="utf-8") as f:
        json.dump(layout_data, f, indent=2)
    
    print(f"✅ Generated biome spritesheet: {output_path}")
    print(f"📋 Generated layout file: {layout_path}")
    
    return output_path


def generate_token_sprites(output_dir: Path, size: str = "1024x1024") -> Path:
    """
    Generate character token spritesheet for chess-piece style tokens.
    
    Args:
        output_dir: Directory to save the generated image  
        size: Image size (default: 1024x1024)
        
    Returns:
        Path to generated image file
    """
    
    print("🎨 Generating character token sprites...")
    
    output_path = output_dir / "tokens_4x3.png"
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate image
    image_data = _generate_image_with_retry(TOKEN_SPRITES_PROMPT, size)
    
    # Save to file
    with open(output_path, "wb") as f:
        f.write(image_data)
    
    # Create layout JSON for Godot slicing
    layout_data = {
        "grid": [4, 3],
        "cell_size": [256, 256],
        "tokens": {
            "masc_light": [0, 0], "masc_medium_light": [1, 0], 
            "masc_medium": [2, 0], "masc_medium_dark": [3, 0],
            "fem_light": [0, 1], "fem_medium_light": [1, 1],
            "fem_medium": [2, 1], "fem_medium_dark": [3, 1],
            "nb_light": [0, 2], "nb_medium_light": [1, 2],
            "nb_medium": [2, 2], "nb_medium_dark": [3, 2]
        }
    }
    
    layout_path = output_dir / "tokens_4x3_layout.json" 
    import json
    with open(layout_path, "w", encoding="utf-8") as f:
        json.dump(layout_data, f, indent=2)
    
    print(f"✅ Generated token sprites: {output_path}")
    print(f"📋 Generated layout file: {layout_path}")
    
    return output_path


def generate_body_bases(output_dir: Path, size: str = "1024x1024") -> Path:
    """
    Generate body base mannequins for character creation system.
    
    Args:
        output_dir: Directory to save the generated image
        size: Image size (default: 1024x1024)
        
    Returns:
        Path to generated image file
    """
    
    print("🎨 Generating body base mannequins...")
    
    output_path = output_dir / "body_bases_2x1.png"
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # Generate image
    image_data = _generate_image_with_retry(BODY_BASES_PROMPT, size)
    
    # Save to file
    with open(output_path, "wb") as f:
        f.write(image_data)
    
    # Create layout JSON
    layout_data = {
        "grid": [2, 1],
        "cell_size": [512, 1024],
        "bodies": {
            "masculine": [0, 0],
            "feminine": [1, 0]
        }
    }
    
    layout_path = output_dir / "body_bases_2x1_layout.json"
    import json
    with open(layout_path, "w", encoding="utf-8") as f:
        json.dump(layout_data, f, indent=2)
    
    print(f"✅ Generated body bases: {output_path}")
    print(f"📋 Generated layout file: {layout_path}")
    
    return output_path


def generate_custom_spritesheet(
    prompt: str, 
    output_path: Path,
    size: str = "1024x1024",
    layout_data: dict[str, Any] | None = None
) -> Path:
    """
    Generate custom spritesheet with arbitrary prompt.
    
    Args:
        prompt: Custom generation prompt
        output_path: Full path for output file
        size: Image size
        layout_data: Optional layout data to save alongside image
        
    Returns:
        Path to generated image file
    """
    
    print(f"🎨 Generating custom spritesheet: {output_path.name}")
    
    output_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Generate image
    image_data = _generate_image_with_retry(prompt, size)
    
    # Save to file
    with open(output_path, "wb") as f:
        f.write(image_data)
    
    # Save layout data if provided
    if layout_data:
        layout_path = output_path.with_suffix(".json")
        import json
        with open(layout_path, "w", encoding="utf-8") as f:
            json.dump(layout_data, f, indent=2)
        print(f"📋 Generated layout file: {layout_path}")
    
    print(f"✅ Generated custom spritesheet: {output_path}")
    
    return output_path


def generate_region_biome_sprites(region_results: list[dict[str, Any]], output_dir: Path) -> list[Path]:
    """
    Generate biome sprites based on actual processed region data.
    
    Args:
        region_results: List of processed region results from regions processor
        output_dir: Directory to save generated sprites
        
    Returns:
        List of paths to generated sprite files
    """
    
    print(f"🏔️ Generating region-specific biome sprites from {len(region_results)} regions...")
    
    generated_files = []
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for region_result in region_results:
        region_data = region_result.get("region_data", {})
        world_hooks = region_result.get("world_hooks", {})
        
        region_name = region_data.get("name", "Unknown Region")
        dominant_biome = region_data.get("dominant_biome", "unknown")
        corruption_level = world_hooks.get("godot_integration", {}).get("corruption_base_level", 0)
        
        # Generate region-specific prompt
        prompt = _build_region_biome_prompt(region_name, dominant_biome, region_data, corruption_level)
        
        # Generate sprite
        safe_name = region_name.lower().replace(" ", "_").replace("'", "")
        output_path = output_dir / f"region_{safe_name}_{dominant_biome}.png"
        
        try:
            image_data = _generate_image_with_retry(prompt, "512x512")
            
            with open(output_path, "wb") as f:
                f.write(image_data)
            
            # Save world_hooks data
            hooks_path = output_path.with_suffix(".json")
            with open(hooks_path, "w", encoding="utf-8") as f:
                json.dump({
                    "region_name": region_name,
                    "biome_type": dominant_biome,
                    "world_hooks": world_hooks,
                    "generation_prompt": prompt
                }, f, indent=2)
            
            generated_files.append(output_path)
            print(f"✅ Generated region sprite: {output_path}")
            
        except Exception as e:
            print(f"❌ Failed to generate sprite for {region_name}: {e}")
    
    print(f"🎯 Generated {len(generated_files)} region biome sprites")
    return generated_files


def generate_settlement_sprites(settlement_results: list[dict[str, Any]], output_dir: Path) -> list[Path]:
    """
    Generate settlement sprites based on actual processed settlement data.
    
    Args:
        settlement_results: List of processed settlement results from settlements processor
        output_dir: Directory to save generated sprites
        
    Returns:
        List of paths to generated sprite files
    """
    
    print(f"🏘️ Generating settlement sprites from {len(settlement_results)} settlements...")
    
    generated_files = []
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for settlement_result in settlement_results:
        settlement_data = settlement_result.get("settlement_data", {})
        world_hooks = settlement_result.get("world_hooks", {})
        
        settlement_name = settlement_data.get("name", "Unknown Settlement")
        scale_hint = settlement_data.get("scale_hint", "village")
        service_types = settlement_data.get("service_types", [])
        
        # Generate settlement-specific prompt
        prompt = _build_settlement_sprite_prompt(settlement_name, scale_hint, service_types, world_hooks)
        
        # Generate sprite
        safe_name = settlement_name.lower().replace(" ", "_").replace("'", "")
        output_path = output_dir / f"settlement_{safe_name}_{scale_hint}.png"
        
        try:
            image_data = _generate_image_with_retry(prompt, "512x512")
            
            with open(output_path, "wb") as f:
                f.write(image_data)
            
            # Save world_hooks data
            hooks_path = output_path.with_suffix(".json")
            with open(hooks_path, "w", encoding="utf-8") as f:
                json.dump({
                    "settlement_name": settlement_name,
                    "scale_hint": scale_hint,
                    "world_hooks": world_hooks,
                    "generation_prompt": prompt
                }, f, indent=2)
            
            generated_files.append(output_path)
            print(f"✅ Generated settlement sprite: {output_path}")
            
        except Exception as e:
            print(f"❌ Failed to generate sprite for {settlement_name}: {e}")
    
    print(f"🎯 Generated {len(generated_files)} settlement sprites")
    return generated_files


def generate_faction_banners(faction_results: list[dict[str, Any]], output_dir: Path) -> list[Path]:
    """
    Generate faction banner sprites based on actual processed faction data.
    
    Args:
        faction_results: List of processed faction results from factions processor
        output_dir: Directory to save generated sprites
        
    Returns:
        List of paths to generated sprite files
    """
    
    print(f"⚔️ Generating faction banners from {len(faction_results)} factions...")
    
    generated_files = []
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for faction_result in faction_results:
        faction_data = faction_result.get("faction_data", {})
        world_hooks = faction_result.get("world_hooks", {})
        
        faction_name = faction_data.get("name", "Unknown Faction")
        political_alignment = faction_data.get("political_alignment", "neutral")
        hostility_level = world_hooks.get("hostility_level", "neutral")
        
        # Generate faction-specific prompt
        prompt = _build_faction_banner_prompt(faction_name, political_alignment, hostility_level, world_hooks)
        
        # Generate sprite
        safe_name = faction_name.lower().replace(" ", "_").replace("'", "")
        output_path = output_dir / f"faction_{safe_name}_banner.png"
        
        try:
            image_data = _generate_image_with_retry(prompt, "256x256")
            
            with open(output_path, "wb") as f:
                f.write(image_data)
            
            # Save world_hooks data
            hooks_path = output_path.with_suffix(".json")
            with open(hooks_path, "w", encoding="utf-8") as f:
                json.dump({
                    "faction_name": faction_name,
                    "political_alignment": political_alignment,
                    "world_hooks": world_hooks,
                    "generation_prompt": prompt
                }, f, indent=2)
            
            generated_files.append(output_path)
            print(f"✅ Generated faction banner: {output_path}")
            
        except Exception as e:
            print(f"❌ Failed to generate banner for {faction_name}: {e}")
    
    print(f"🎯 Generated {len(generated_files)} faction banners")
    return generated_files


def generate_dungeon_entrance_sprites(dungeon_results: list[dict[str, Any]], output_dir: Path) -> list[Path]:
    """
    Generate dungeon entrance sprites based on actual processed dungeon data.
    
    Args:
        dungeon_results: List of processed dungeon results from dungeons processor
        output_dir: Directory to save generated sprites
        
    Returns:
        List of paths to generated sprite files
    """
    
    print(f"🏰 Generating dungeon entrance sprites from {len(dungeon_results)} dungeons...")
    
    generated_files = []
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for dungeon_result in dungeon_results:
        dungeon_data = dungeon_result.get("dungeon_data", {})
        world_hooks = dungeon_result.get("world_hooks", {})
        
        dungeon_name = dungeon_data.get("name", "Unknown Dungeon")
        dungeon_type = dungeon_data.get("dungeon_type", "crypt")
        horror_intensity = dungeon_data.get("horror_intensity", "none")
        entrance_type = world_hooks.get("entrance_type", "unknown-entrance")
        
        # Generate dungeon-specific prompt
        prompt = _build_dungeon_entrance_prompt(dungeon_name, dungeon_type, horror_intensity, world_hooks)
        
        # Generate sprite
        safe_name = dungeon_name.lower().replace(" ", "_").replace("'", "")
        output_path = output_dir / f"dungeon_{safe_name}_{dungeon_type}_entrance.png"
        
        try:
            image_data = _generate_image_with_retry(prompt, "512x512")
            
            with open(output_path, "wb") as f:
                f.write(image_data)
            
            # Save world_hooks data
            hooks_path = output_path.with_suffix(".json")
            with open(hooks_path, "w", encoding="utf-8") as f:
                json.dump({
                    "dungeon_name": dungeon_name,
                    "dungeon_type": dungeon_type,
                    "horror_intensity": horror_intensity,
                    "world_hooks": world_hooks,
                    "generation_prompt": prompt
                }, f, indent=2)
            
            generated_files.append(output_path)
            print(f"✅ Generated dungeon entrance: {output_path}")
            
        except Exception as e:
            print(f"❌ Failed to generate entrance for {dungeon_name}: {e}")
    
    print(f"🎯 Generated {len(generated_files)} dungeon entrance sprites")
    return generated_files


def _build_region_biome_prompt(region_name: str, dominant_biome: str, region_data: dict[str, Any], corruption_level: int) -> str:
    """Build region-specific biome sprite prompt using Jinja2 template."""
    
    # Base biome characteristics
    biome_descriptions = {
        "forest": "dense woodland with towering trees and dappled sunlight",
        "mountain": "rocky peaks with snow-capped summits and stone outcroppings", 
        "desert": "sandy dunes with sparse vegetation and ancient ruins",
        "swamp": "murky wetlands with twisted trees and stagnant water",
        "plains": "rolling grasslands with wildflowers and gentle hills",
        "tundra": "frozen expanse with ice formations and hardy shrubs",
        "jungle": "thick canopy with vibrant foliage and tangled vines"
    }
    
    # Corruption effects
    corruption_effects = {
        0: "",
        1: "Subtle wrongness - shadows slightly too long, colors slightly muted",
        2: "Visible decay - withered plants, darkened earth, unnatural shadows", 
        3: "Heavy corruption - twisted vegetation, scorched ground, void-touched elements"
    }
    
    # Build template context
    context = {
        "size": "512x512",
        "region_name": region_name,
        "biome_description": biome_descriptions.get(dominant_biome.lower(), "varied terrain"),
        "settlement_locations": region_data.get("settlement_locations", []),
        "corruption_level": corruption_level,
        "corruption_effects": corruption_effects,
        "world_hooks": {
            "has_rivers": region_data.get("has_rivers", False),
            "has_trails": region_data.get("has_trails", False),
            "political_control": region_data.get("political_control", [])
        }
    }
    
    return _render_template("region_biome.j2", context)


def _build_settlement_sprite_prompt(settlement_name: str, scale_hint: str, service_types: list[str], world_hooks: dict[str, Any]) -> str:
    """Build settlement-specific sprite prompt using Jinja2 template."""
    
    # Scale-based descriptions
    scale_descriptions = {
        "village": "small rural village with thatched cottages and dirt paths",
        "town": "modest town with stone buildings, market square, and cobbled streets",
        "city": "large city with multi-story buildings, defensive walls, and major thoroughfares"
    }
    
    # Service-based additions
    service_additions = []
    if "lodging" in service_types:
        service_additions.append("prominent tavern with hanging sign")
    if "commerce" in service_types:
        service_additions.append("market stalls and merchant buildings")
    if "crafting" in service_types:
        service_additions.append("forge with smoking chimney")
    if "religious" in service_types:
        service_additions.append("temple or shrine with distinctive architecture")
    
    # Corruption resistance effects
    corruption_resistance = world_hooks.get("godot_integration", {}).get("corruption_resistance", 0)
    protection_desc = ""
    if corruption_resistance >= 3:
        protection_desc = "Blessed aura, holy symbols, protective barriers against corruption."
    elif corruption_resistance >= 1:
        protection_desc = "Some protective elements, well-maintained appearance."
    
    # Build template context
    context = {
        "size": "512x512",
        "settlement_name": settlement_name,
        "scale_description": scale_descriptions.get(scale_hint, "settlement"),
        "services_description": ", ".join(service_additions) if service_additions else "typical settlement buildings",
        "protection_description": protection_desc,
        "world_hooks": world_hooks
    }
    
    return _render_template("settlement_sprite.j2", context)


def _build_faction_banner_prompt(faction_name: str, political_alignment: str, hostility_level: str, world_hooks: dict[str, Any]) -> str:
    """Build faction-specific banner prompt using Jinja2 template."""
    
    # Alignment-based color schemes
    alignment_colors = {
        "lawful": "blue and white with gold accents, representing justice and order",
        "chaotic": "black and red with dark purple, representing corruption and chaos",
        "neutral": "earth tones with green accents, representing balance and nature"
    }
    
    # Hostility-based design elements
    hostility_elements = {
        "hostile": "aggressive imagery, sharp geometric patterns, intimidating symbols",
        "aggressive": "bold designs, crossed weapons, predatory animal motifs",
        "lawful": "orderly patterns, scales of justice, protective shields",
        "neutral": "balanced composition, natural elements, diplomatic symbols"
    }
    
    # Build template context
    context = {
        "size": "256x256",
        "faction_name": faction_name,
        "color_scheme": alignment_colors.get(political_alignment, "muted colors"),
        "design_elements": hostility_elements.get(hostility_level, "simple geometric patterns"),
        "symbolic_elements": _extract_faction_symbolism(faction_name),
        "political_alignment": political_alignment,
        "world_hooks": world_hooks
    }
    
    return _render_template("faction_banner.j2", context)


def _build_dungeon_entrance_prompt(dungeon_name: str, dungeon_type: str, horror_intensity: str, world_hooks: dict[str, Any]) -> str:
    """Build dungeon-specific entrance prompt using Jinja2 template."""
    
    # Type-based entrance descriptions
    entrance_descriptions = {
        "crypt": "ancient stone crypt entrance with weathered carvings and iron gates",
        "lair": "natural cave entrance with claw marks and bone scattered around",
        "temple": "ornate temple entrance with pillars and religious symbols",
        "tomb": "imposing tomb entrance with hieroglyphs and sealed doorway",
        "hideout": "concealed hideout entrance camouflaged in natural terrain",
        "cavern": "yawning cave mouth with stalactites and dark depths",
        "bowel": "terrifying pit entrance descending into absolute darkness"
    }
    
    # Horror intensity effects
    horror_effects = {
        "none": "",
        "low": "Subtle wrongness - shadows seem deeper than they should",
        "moderate": "Visible corruption - blackened stone, withered plants nearby",
        "high": "Obvious horror - unnatural darkness, reality distortion effects",
        "extreme": "Absolute terror - void-touched elements, reality breakdown"
    }
    
    # Extract horror themes from name
    horror_themes = world_hooks.get("horror_themes", [])
    theme_desc = ""
    if horror_themes:
        theme_desc = f"Incorporates {', '.join(horror_themes[:2])} horror themes."
    
    # Build template context
    context = {
        "size": "512x512",
        "dungeon_name": dungeon_name,
        "entrance_description": entrance_descriptions.get(dungeon_type, "mysterious entrance"),
        "horror_description": horror_effects.get(horror_intensity, ""),
        "theme_description": theme_desc,
        "dungeon_type": dungeon_type,
        "horror_intensity": horror_intensity,
        "world_hooks": world_hooks
    }
    
    return _render_template("dungeon_entrance.j2", context)


def _extract_faction_symbolism(faction_name: str) -> str:
    """Extract symbolic elements from faction name."""
    
    name_lower = faction_name.lower()
    
    if "wolves" in name_lower:
        return "wolf head silhouettes and pack symbols"
    elif "snakes" in name_lower:
        return "serpentine motifs and coiled snake designs"
    elif "wyverns" in name_lower:
        return "dragon wing patterns and scaled elements"
    elif "justice" in name_lower:
        return "balanced scales and crossed swords"
    elif "fists" in name_lower:
        return "clenched fist symbols and strength imagery"
    elif "defiled" in name_lower:
        return "corrupted symbols and tainted imagery"
    else:
        return "abstract symbolic elements"


def generate_all_data_driven_sprites(
    region_results: list[dict[str, Any]], 
    settlement_results: list[dict[str, Any]],
    faction_results: list[dict[str, Any]], 
    dungeon_results: list[dict[str, Any]],
    output_dir: Path
) -> dict[str, list[Path]]:
    """
    Generate all sprite types using real processed data.
    
    Args:
        region_results: Processed region data from regions processor
        settlement_results: Processed settlement data from settlements processor
        faction_results: Processed faction data from factions processor
        dungeon_results: Processed dungeon data from dungeons processor
        output_dir: Base directory for generated sprites
        
    Returns:
        Dictionary mapping sprite types to lists of generated file paths
    """
    
    print("🎨 Generating all data-driven sprites...")
    
    results = {
        "regions": generate_region_biome_sprites(region_results, output_dir / "regions"),
        "settlements": generate_settlement_sprites(settlement_results, output_dir / "settlements"),
        "factions": generate_faction_banners(faction_results, output_dir / "factions"),
        "dungeons": generate_dungeon_entrance_sprites(dungeon_results, output_dir / "dungeons")
    }
    
    total_generated = sum(len(files) for files in results.values())
    print(f"🎯 Generated {total_generated} total data-driven sprites")
    
    return results
