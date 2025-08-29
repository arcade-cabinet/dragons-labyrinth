"""
Meta Knowledge Loader - Load organized world knowledge for ML training.

Loads categorized world knowledge from training/meta/anchors.json and 
organized HBF examples from memory-bank/world-building/ directories.
Translates organized knowledge into ML-ready training parameters.
"""

from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Any


def load_training_anchors() -> dict[str, Any]:
    """Load minimal anchor patterns from training/meta/anchors.json."""
    
    anchor_path = Path("training") / "meta" / "anchors.json"
    
    try:
        with open(anchor_path, "r", encoding="utf-8") as f:
            return json.load(f)
    except FileNotFoundError:
        # Default organized world knowledge if file missing
        return {
            "world_name": "The Lands of Vo'il",
            "organized_categories": {
                "regions": {
                    "source_dir": "memory-bank/world-building/regions/",
                    "count": 27,
                    "example_names": [
                        "Javelin Plains", "Fearless Wilds", "Vicious Crags",
                        "Blood Blade Fields", "Nightmare Desert", "Hell's Gate Desert"
                    ],
                    "content_patterns": {
                        "hex_maps": "JSON map data with coordinates, biomes, features",
                        "political_entities": "Cities, towns, villages within region",
                        "npcs": "Named characters with stats, factions, possessions",
                        "encounters": "Random encounter tables per biome",
                        "weather": "Regional weather systems",
                        "rumors": "Regional rumor tables"
                    }
                },
                "settlements": {
                    "source_dir": "memory-bank/world-building/settlements/",
                    "count": 10,
                    "size_categories": ["Village", "Town", "City"],
                    "example_names": [
                        "Headsmen (City)", "Devilville (Town)", "Harad (Village)",
                        "Palemoon (City)", "Kothian (Village)"
                    ],
                    "content_patterns": {
                        "establishments": "Taverns, shops, services with NPCs",
                        "districts": "Named areas within larger settlements", 
                        "npcs": "Detailed residents with professions, stats",
                        "economics": "Trade goods, prices, services",
                        "politics": "Faction memberships, leadership"
                    }
                },
                "factions": {
                    "source_dir": "memory-bank/world-building/factions/",
                    "count": 5,
                    "example_names": [
                        "The Defiled Wolves", "The Fists Of Justice",
                        "The Swords Of Justice", "The Red Snakes", "The White Wyverns"
                    ],
                    "content_patterns": {
                        "leadership": "Faction leaders with detailed stats",
                        "membership": "Member lists with locations and roles",
                        "collaborators": "Non-member allies and contacts",
                        "politics": "Inter-faction relationships and conflicts",
                        "territories": "Areas of influence and operations"
                    }
                },
                "dungeons": {
                    "source_dir": "memory-bank/world-building/dungeons/",
                    "count": 18,
                    "type_categories": ["Crypt", "Lair", "Hideout", "Shrine", "Temple", "Tomb", "Bowel", "Caverns"],
                    "example_names": [
                        "Crypt of Mourning Goblin", "Lair of Foresaken Desire",
                        "Temple of Violent Ogre", "Shrine of Infernal Blades"
                    ],
                    "content_patterns": {
                        "structure": "Room layouts, area descriptions",
                        "encounters": "Boss creatures with detailed stats",
                        "treasure": "Hoards with coins, gems, magic items",
                        "themes": "Horror/corruption themes per location",
                        "cr_progression": "Challenge rating distribution"
                    }
                }
            },
            "ml_extraction_targets": [
                "biome_classification", "settlement_scale_detection", 
                "faction_alignment", "dungeon_theme_extraction",
                "npc_relationship_mapping", "treasure_value_estimation",
                "encounter_difficulty_assessment", "regional_politics"
            ]
        }


def load_organized_examples(category: str) -> list[dict[str, Any]]:
    """
    Load organized training examples for a specific category.
    
    Args:
        category: One of 'regions', 'settlements', 'factions', 'dungeons'
        
    Returns:
        List of training examples with metadata
    """
    
    anchors = load_training_anchors()
    category_info = anchors["organized_categories"].get(category)
    
    if not category_info:
        logging.warning(f"Unknown category: {category}")
        return []
    
    source_dir = Path(category_info["source_dir"])
    
    if not source_dir.exists():
        logging.warning(f"Source directory not found: {source_dir}")
        return []
    
    examples = []
    
    # Load all files in category directory
    for file_path in source_dir.glob("*.txt"):
        try:
            with open(file_path, "r", encoding="utf-8") as f:
                content = f.read()
            
            # Parse content based on category type
            if category == "regions":
                example = _parse_region_content(file_path.stem, content)
            elif category == "settlements":
                example = _parse_settlement_content(file_path.stem, content)
            elif category == "factions":
                example = _parse_faction_content(file_path.stem, content)
            elif category == "dungeons":
                example = _parse_dungeon_content(file_path.stem, content)
            else:
                continue
                
            examples.append(example)
            
        except Exception as e:
            logging.warning(f"Failed to load {file_path}: {e}")
    
    return examples


def _parse_region_content(name: str, content: str) -> dict[str, Any]:
    """Parse region training content into structured data."""
    
    # Try to parse as JSON first (like javelin_plains.txt)
    try:
        data = json.loads(content)
        return {
            "category": "region",
            "name": name.replace("_", " ").title(),
            "data_type": "structured_json",
            "map_data": data.get("map", []),
            "region_mappings": data.get("regions", {}),
            "content_features": _extract_region_features(data),
            "training_signals": {
                "hex_count": len(data.get("map", [])),
                "biome_variety": _count_biome_types(data.get("map", [])),
                "political_entities": _count_political_entities(data.get("map", [])),
                "geographic_features": _count_geographic_features(data.get("map", []))
            }
        }
    except json.JSONDecodeError:
        # Parse as HTML content
        return {
            "category": "region", 
            "name": name.replace("_", " ").title(),
            "data_type": "html_content",
            "raw_content": content,
            "content_features": _extract_html_features(content),
            "training_signals": {
                "content_length": len(content),
                "npc_mentions": content.count("<strong>"),
                "location_references": content.count("href="),
                "table_structures": content.count("<table>")
            }
        }


def _parse_settlement_content(name: str, content: str) -> dict[str, Any]:
    """Parse settlement training content into structured data."""
    
    return {
        "category": "settlement",
        "name": name.replace("_", " ").title(),
        "raw_content": content,
        "content_features": _extract_settlement_features(content),
        "training_signals": {
            "scale_indicators": _detect_settlement_scale(content),
            "establishment_count": _count_establishments(content),
            "npc_density": content.count("npc-anchor"),
            "economic_complexity": _assess_economic_activity(content)
        }
    }


def _parse_faction_content(name: str, content: str) -> dict[str, Any]:
    """Parse faction training content into structured data."""
    
    return {
        "category": "faction",
        "name": name.replace("_", " ").title(),
        "raw_content": content,
        "content_features": _extract_faction_features(content),
        "training_signals": {
            "member_count": _count_faction_members(content),
            "leadership_structure": _analyze_leadership(content),
            "territorial_control": _assess_territorial_reach(content),
            "political_alignment": _detect_alignment_patterns(content)
        }
    }


def _parse_dungeon_content(name: str, content: str) -> dict[str, Any]:
    """Parse dungeon training content into structured data."""
    
    return {
        "category": "dungeon",
        "name": name.replace("_", " ").title(),
        "raw_content": content,
        "content_features": _extract_dungeon_features(content),
        "training_signals": {
            "dungeon_type": _classify_dungeon_type(name),
            "challenge_rating": _estimate_dungeon_cr(content),
            "treasure_density": _assess_treasure_content(content),
            "horror_themes": _extract_horror_elements(content)
        }
    }


def _extract_region_features(data: dict[str, Any]) -> dict[str, Any]:
    """Extract key features from region JSON data."""
    
    map_tiles = data.get("map", [])
    
    return {
        "biome_distribution": _analyze_biome_distribution(map_tiles),
        "settlement_locations": _find_settlement_tiles(map_tiles),
        "dungeon_locations": _find_dungeon_tiles(map_tiles),
        "geographic_connectivity": _analyze_connectivity(map_tiles),
        "political_boundaries": _map_political_control(map_tiles, data.get("regions", {}))
    }


def _extract_html_features(content: str) -> dict[str, Any]:
    """Extract features from HTML content."""
    
    return {
        "npc_density": content.count("npc-anchor"),
        "stat_blocks": content.count("statblock"),
        "tables": content.count("<table>"),
        "spoilers": content.count("spoiler"),
        "rumor_tables": content.count("d6") and "Rumor" in content,
        "weather_systems": "Regional Weather" in content
    }


def _count_biome_types(map_tiles: list[dict]) -> int:
    """Count unique biome types in map."""
    biomes = set()
    for tile in map_tiles:
        biome_type = tile.get("type", "")
        if biome_type:
            biomes.add(biome_type)
    return len(biomes)


def _count_political_entities(map_tiles: list[dict]) -> int:
    """Count political entities (villages, towns, cities) in map."""
    count = 0
    for tile in map_tiles:
        feature = tile.get("feature", "")
        if feature in ["Village", "Town", "City"]:
            count += 1
    return count


def _count_geographic_features(map_tiles: list[dict]) -> int:
    """Count special geographic features."""
    count = 0
    for tile in map_tiles:
        if tile.get("rivers") or tile.get("harbor") or tile.get("borderline"):
            count += 1
    return count


def _analyze_biome_distribution(map_tiles: list[dict]) -> dict[str, int]:
    """Analyze biome type distribution."""
    distribution = {}
    for tile in map_tiles:
        biome = tile.get("type", "Unknown")
        distribution[biome] = distribution.get(biome, 0) + 1
    return distribution


def _find_settlement_tiles(map_tiles: list[dict]) -> list[dict]:
    """Find tiles with settlements."""
    settlements = []
    for tile in map_tiles:
        feature = tile.get("feature")
        if feature in ["Village", "Town", "City"]:
            settlements.append({
                "type": feature,
                "label": tile.get("label", ""),
                "coordinate": (tile.get("x", 0), tile.get("y", 0))
            })
    return settlements


def _find_dungeon_tiles(map_tiles: list[dict]) -> list[dict]:
    """Find tiles with dungeons."""
    dungeons = []
    for tile in map_tiles:
        if tile.get("feature") == "Dungeon":
            dungeons.append({
                "coordinate": (tile.get("x", 0), tile.get("y", 0)),
                "uuid": tile.get("feature_uuid", "")
            })
    return dungeons


def _analyze_connectivity(map_tiles: list[dict]) -> dict[str, Any]:
    """Analyze trail and river connectivity."""
    total_trails = 0
    total_rivers = 0
    
    for tile in map_tiles:
        if tile.get("trails"):
            total_trails += len(tile["trails"])
        if tile.get("rivers"):
            total_rivers += len(tile["rivers"])
    
    return {
        "trail_connections": total_trails,
        "river_connections": total_rivers,
        "connectivity_density": (total_trails + total_rivers) / max(len(map_tiles), 1)
    }


def _map_political_control(map_tiles: list[dict], regions: dict[str, str]) -> dict[str, Any]:
    """Map political control patterns."""
    region_control = {}
    
    for tile in map_tiles:
        region_id = tile.get("region")
        region_name = regions.get(region_id, "Unknown")
        
        if region_name not in region_control:
            region_control[region_name] = {"tiles": 0, "settlements": 0, "dungeons": 0}
        
        region_control[region_name]["tiles"] += 1
        
        feature = tile.get("feature", "")
        if feature in ["Village", "Town", "City"]:
            region_control[region_name]["settlements"] += 1
        elif feature == "Dungeon":
            region_control[region_name]["dungeons"] += 1
    
    return region_control


def _extract_settlement_features(content: str) -> dict[str, Any]:
    """Extract settlement-specific features."""
    
    return {
        "establishment_types": _identify_establishment_types(content),
        "npc_roles": _extract_npc_roles(content),
        "economic_indicators": _find_economic_patterns(content),
        "faction_presence": _detect_faction_activity(content)
    }


def _detect_settlement_scale(content: str) -> str:
    """Detect settlement scale from content."""
    
    # Look for scale indicators
    if "City of" in content or "city" in content.lower():
        return "city"
    elif "Town of" in content or "town" in content.lower(): 
        return "town"
    elif "Village of" in content or "village" in content.lower():
        return "village"
    else:
        return "unknown"


def _count_establishments(content: str) -> int:
    """Count establishments (taverns, shops, etc.)."""
    
    establishment_keywords = [
        "Tavern", "Inn", "Shop", "Market", "Smithy", "Temple", 
        "Barracks", "Guild", "Warehouse", "Dock"
    ]
    
    count = 0
    for keyword in establishment_keywords:
        count += content.count(keyword)
    
    return count


def _assess_economic_activity(content: str) -> int:
    """Assess economic activity level."""
    
    economic_indicators = [
        "gp", "sp", "cp", "price", "cost", "trade", "merchant", 
        "caravan", "goods", "inventory", "sale"
    ]
    
    activity_score = 0
    for indicator in economic_indicators:
        activity_score += content.count(indicator)
    
    return activity_score


def _extract_faction_features(content: str) -> dict[str, Any]:
    """Extract faction-specific features."""
    
    return {
        "organizational_structure": _analyze_org_structure(content),
        "member_distribution": _map_member_locations(content),
        "activity_patterns": _identify_faction_activities(content)
    }


def _count_faction_members(content: str) -> int:
    """Count faction members."""
    return content.count("Member") + content.count("Collaborator")


def _analyze_leadership(content: str) -> dict[str, Any]:
    """Analyze faction leadership structure."""
    
    leadership_info = {
        "has_leader": "leader" in content.lower(),
        "leader_level": 0,
        "leadership_type": "unknown"
    }
    
    # Extract leader level if present
    if "level" in content.lower():
        import re
        level_matches = re.findall(r'level\s+(\d+)', content.lower())
        if level_matches:
            leadership_info["leader_level"] = int(level_matches[0])
    
    return leadership_info


def _extract_dungeon_features(content: str) -> dict[str, Any]:
    """Extract dungeon-specific features."""
    
    return {
        "encounter_density": content.count("monster-block"),
        "treasure_complexity": _assess_treasure_systems(content),
        "horror_elements": _catalog_horror_themes(content)
    }


def _classify_dungeon_type(name: str) -> str:
    """Classify dungeon type from name."""
    
    name_lower = name.lower()
    
    if "crypt" in name_lower:
        return "crypt"
    elif "lair" in name_lower:
        return "lair" 
    elif "temple" in name_lower:
        return "temple"
    elif "shrine" in name_lower:
        return "shrine"
    elif "tomb" in name_lower:
        return "tomb"
    elif "hideout" in name_lower:
        return "hideout"
    elif "bowel" in name_lower:
        return "bowel"
    elif "cavern" in name_lower:
        return "caverns"
    else:
        return "unknown"


def get_training_metadata() -> dict[str, Any]:
    """Get comprehensive training metadata."""
    
    anchors = load_training_anchors()
    
    return {
        "world_context": {
            "name": anchors["world_name"],
            "total_categories": len(anchors["organized_categories"]),
            "total_examples": sum(
                cat_info.get("count", 0) 
                for cat_info in anchors["organized_categories"].values()
            )
        },
        "category_summary": {
            category: {
                "count": info.get("count", 0),
                "source_directory": info.get("source_dir", ""),
                "example_names": info.get("example_names", [])[:3]  # First 3 examples
            }
            for category, info in anchors["organized_categories"].items()
        },
        "ml_targets": anchors.get("ml_extraction_targets", []),
        "data_quality": "organized_breakthrough_data"
    }


# Helper functions for content analysis
def _identify_establishment_types(content: str) -> list[str]:
    """Identify types of establishments in content."""
    establishments = []
    establishment_patterns = ["Tavern", "Inn", "Shop", "Market", "Smithy", "Temple"]
    
    for pattern in establishment_patterns:
        if pattern in content:
            establishments.append(pattern.lower())
    
    return establishments


def _extract_npc_roles(content: str) -> list[str]:
    """Extract NPC professional roles."""
    roles = []
    role_patterns = ["Fighter", "Cleric", "Wizard", "Rogue", "Druid", "Merchant", "Blacksmith"]
    
    for pattern in role_patterns:
        if pattern in content:
            roles.append(pattern.lower())
    
    return list(set(roles))  # Remove duplicates


def _find_economic_patterns(content: str) -> dict[str, int]:
    """Find economic activity patterns."""
    return {
        "currency_mentions": content.count("gp") + content.count("sp") + content.count("cp"),
        "trade_activity": content.count("trade") + content.count("merchant"),
        "inventory_systems": content.count("inventory") + content.count("sale")
    }


def _detect_faction_activity(content: str) -> list[str]:
    """Detect faction presence in content."""
    factions = []
    
    faction_names = [
        "The Defiled Wolves", "The Fists Of Justice", "The Swords Of Justice",
        "The Red Snakes", "The White Wyverns"
    ]
    
    for faction in faction_names:
        if faction in content:
            factions.append(faction)
    
    return factions


def _analyze_org_structure(content: str) -> dict[str, Any]:
    """Analyze organizational structure patterns."""
    return {
        "has_hierarchy": "leader" in content.lower(),
        "member_categories": ["Members", "Collaborators"] if "Collaborators" in content else ["Members"],
        "territorial_base": "hideout" in content.lower() or "venue" in content.lower()
    }


def _map_member_locations(content: str) -> dict[str, int]:
    """Map where faction members are located."""
    locations = {}
    
    # Simple pattern matching for locations
    import re
    location_matches = re.findall(r'from ([A-Z][a-z]+)', content)
    
    for location in location_matches:
        locations[location] = locations.get(location, 0) + 1
    
    return locations


def _identify_faction_activities(content: str) -> list[str]:
    """Identify what faction does."""
    activities = []
    
    if "experiment" in content.lower():
        activities.append("experimentation")
    if "eliminate" in content.lower() or "plotting" in content.lower():
        activities.append("elimination")
    if "gather" in content.lower() or "meeting" in content.lower():
        activities.append("gathering")
    
    return activities


def _estimate_dungeon_cr(content: str) -> str:
    """Estimate dungeon challenge rating from content."""
    
    # Look for CR indicators
    if "CR: 10" in content or "CR: 1" in content and "0" in content:
        return "high"
    elif "CR: 5" in content or "CR: 8" in content:
        return "medium"
    elif "CR: 1" in content or "CR: 2" in content:
        return "low"
    else:
        return "unknown"


def _assess_treasure_content(content: str) -> str:
    """Assess treasure complexity."""
    
    treasure_score = content.count("gp") + content.count("Magic Items") + content.count("gemstones")
    
    if treasure_score > 10:
        return "rich"
    elif treasure_score > 5:
        return "moderate"
    elif treasure_score > 0:
        return "basic"
    else:
        return "none"


def _assess_treasure_systems(content: str) -> int:
    """Assess treasure system complexity."""
    return content.count("hoard") + content.count("Magic Items") + content.count("gemstones")


def _catalog_horror_themes(content: str) -> list[str]:
    """Catalog horror theme elements."""
    themes = []
    
    horror_keywords = [
        "corrupted", "infernal", "cursed", "mourning", "violent", 
        "unholy", "burning", "bleeding", "foresaken", "unspoken"
    ]
    
    for keyword in horror_keywords:
        if keyword.lower() in content.lower():
            themes.append(keyword)
    
    return themes


def _extract_horror_elements(content: str) -> list[str]:
    """Extract horror theme elements from dungeon names/content."""
    return _catalog_horror_themes(content)
