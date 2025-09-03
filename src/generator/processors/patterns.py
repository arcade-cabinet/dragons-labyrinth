"""
Core Pattern Extraction Utilities - Content extraction from organized HBF data.

Focuses on extracting WHAT content FROM entities, not detecting entity types.
Uses organized HBF examples to train content extraction patterns.
"""

from __future__ import annotations

import re
import logging
from typing import Any
from dataclasses import dataclass


@dataclass
class ContentExtractionResult:
    """Result of content extraction with confidence scoring."""
    extracted_content: dict[str, Any]
    confidence_score: float
    extraction_method: str
    source_patterns: list[str]


def extract_npc_data(content: str) -> dict[str, Any]:
    """
    Extract NPC data from HBF content using organized patterns.
    
    Args:
        content: Raw HBF entity content
        
    Returns:
        Structured NPC data with stats, possessions, emotions
    """
    
    npc_data = {
        "name": None,
        "class_level": None,
        "stats": {},
        "possessions": [],
        "emotional_state": None,
        "faction_membership": None,
        "location": None
    }
    
    # Extract name (first strong tag typically)
    name_match = re.search(r'<strong>([^<]+)</strong>', content)
    if name_match:
        npc_data["name"] = name_match.group(1)
    
    # Extract class and level
    class_level_match = re.search(r'level (\d+) (\w+) (\w+)', content, re.IGNORECASE)
    if class_level_match:
        npc_data["class_level"] = {
            "level": int(class_level_match.group(1)),
            "race": class_level_match.group(2),
            "class": class_level_match.group(3)
        }
    
    # Extract stats from stat tables
    npc_data["stats"] = _extract_stat_block(content)
    
    # Extract possessions from "In the pocket" sections
    npc_data["possessions"] = _extract_possessions(content)
    
    # Extract emotional state
    emotion_match = re.search(r'\(<em>([^<]+)</em>\)', content)
    if emotion_match:
        npc_data["emotional_state"] = emotion_match.group(1)
    
    # Extract faction membership
    faction_match = re.search(r'Member of the <a[^>]*><strong>([^<]+)</strong>', content)
    if faction_match:
        npc_data["faction_membership"] = faction_match.group(1)
    
    return npc_data


def extract_settlement_data(content: str) -> dict[str, Any]:
    """
    Extract settlement data from HBF content.
    
    Args:
        content: Raw settlement content
        
    Returns:
        Structured settlement data with establishments, services, politics
    """
    
    settlement_data = {
        "name": None,
        "scale": "unknown",
        "establishments": [],
        "services": {},
        "npcs": [],
        "districts": [],
        "economic_activity": {}
    }
    
    # Extract settlement name and scale
    settlement_data["name"], settlement_data["scale"] = _extract_settlement_identity(content)
    
    # Extract establishments
    settlement_data["establishments"] = _extract_establishments(content)
    
    # Extract services and prices
    settlement_data["services"] = _extract_services(content)
    
    # Extract NPCs
    settlement_data["npcs"] = _extract_settlement_npcs(content)
    
    # Extract districts/areas
    settlement_data["districts"] = _extract_districts(content)
    
    # Extract economic data
    settlement_data["economic_activity"] = _extract_economic_data(content)
    
    return settlement_data


def extract_faction_data(content: str) -> dict[str, Any]:
    """
    Extract faction data from HBF content.
    
    Args:
        content: Raw faction content
        
    Returns:
        Structured faction data with leadership, membership, politics
    """
    
    faction_data = {
        "name": None,
        "type": None,
        "leader": None,
        "members": [],
        "collaborators": [],
        "base_location": None,
        "activities": [],
        "political_stance": None
    }
    
    # Extract faction name and type
    faction_data["name"], faction_data["type"] = _extract_faction_identity(content)
    
    # Extract leader information
    faction_data["leader"] = _extract_faction_leader(content)
    
    # Extract membership
    faction_data["members"] = _extract_faction_members(content, "Members")
    faction_data["collaborators"] = _extract_faction_members(content, "Collaborators")
    
    # Extract base/headquarters
    faction_data["base_location"] = _extract_faction_base(content)
    
    # Extract activities
    faction_data["activities"] = _extract_faction_activities(content)
    
    return faction_data


def extract_dungeon_data(content: str) -> dict[str, Any]:
    """
    Extract dungeon data from HBF content.
    
    Args:
        content: Raw dungeon content
        
    Returns:
        Structured dungeon data with encounters, treasure, themes
    """
    
    dungeon_data = {
        "name": None,
        "type": None,
        "encounters": [],
        "treasure_hoards": [],
        "challenge_rating": None,
        "horror_themes": [],
        "location": None
    }
    
    # Extract dungeon name and type
    dungeon_data["name"], dungeon_data["type"] = _extract_dungeon_identity(content)
    
    # Extract encounters
    dungeon_data["encounters"] = _extract_encounters(content)
    
    # Extract treasure hoards
    dungeon_data["treasure_hoards"] = _extract_treasure_hoards(content)
    
    # Extract challenge rating
    dungeon_data["challenge_rating"] = _extract_challenge_rating(content)
    
    # Extract horror themes
    dungeon_data["horror_themes"] = _extract_horror_themes(content)
    
    return dungeon_data


def extract_biome_features(hex_data: dict[str, Any]) -> dict[str, Any]:
    """
    Extract biome features from hex tile data.
    
    Args:
        hex_data: Hex tile JSON data
        
    Returns:
        Structured biome data with environmental characteristics
    """
    
    return {
        "type": hex_data.get("type", "Unknown"),
        "coordinate": (hex_data.get("x", 0), hex_data.get("y", 0)),
        "features": {
            "rivers": hex_data.get("rivers", []),
            "trails": hex_data.get("trails", []), 
            "harbor": hex_data.get("harbor"),
            "borderline": hex_data.get("borderline", False)
        },
        "political_control": {
            "region": hex_data.get("region"),
            "realm": hex_data.get("realm")
        },
        "special_features": hex_data.get("feature"),
        "special_label": hex_data.get("label")
    }


# Core extraction helper functions
def _extract_stat_block(content: str) -> dict[str, Any]:
    """Extract D&D stat block from content."""
    
    stats = {}
    
    # Extract basic stats
    stat_pattern = r'<td>(\d+)\s*<small>\s*([+-]?\d+)\s*</small>'
    stat_matches = re.findall(stat_pattern, content)
    
    if len(stat_matches) >= 6:  # STR, DEX, CON, INT, WIS, CHA
        stat_names = ["STR", "DEX", "CON", "INT", "WIS", "CHA"]
        for i, (value, modifier) in enumerate(stat_matches[:6]):
            stats[stat_names[i]] = {"value": int(value), "modifier": int(modifier)}
    
    # Extract AC, HP, Speed
    ac_match = re.search(r'AC:</span>\s*(\d+)', content)
    if ac_match:
        stats["AC"] = int(ac_match.group(1))
    
    hp_match = re.search(r'HP:</span>\s*(\d+)', content)
    if hp_match:
        stats["HP"] = int(hp_match.group(1))
    
    level_match = re.search(r'Level:</span>\s*(\d+)', content)
    if level_match:
        stats["Level"] = int(level_match.group(1))
    
    return stats


def _extract_possessions(content: str) -> list[str]:
    """Extract possessions from 'In the pocket' sections."""
    
    possessions = []
    
    # Find possession sections
    possession_pattern = r'In the pocket:[^<]*<strong>([^<]+)</strong>'
    matches = re.findall(possession_pattern, content)
    
    for match in matches:
        # Clean up possession text
        possession = match.strip()
        if possession and possession != "and":
            possessions.append(possession)
    
    return possessions


def _extract_settlement_identity(content: str) -> tuple[str | None, str]:
    """Extract settlement name and scale."""
    
    name = None
    scale = "unknown"
    
    # Look for settlement titles
    name_match = re.search(r'(?:City|Town|Village) of ([^<"]+)', content)
    if name_match:
        name = name_match.group(1)
        
        if "City of" in content:
            scale = "city"
        elif "Town of" in content:
            scale = "town" 
        elif "Village of" in content:
            scale = "village"
    
    return name, scale


def _extract_establishments(content: str) -> list[dict[str, Any]]:
    """Extract establishments (taverns, shops, etc.) from content."""
    
    establishments = []
    
    # Look for establishment patterns
    establishment_patterns = [
        r'<strong>"([^"]+Tavern[^"]*)"</strong>',
        r'<strong>"([^"]+Inn[^"]*)"</strong>', 
        r'<strong>([^<]+Shop[^<]*)</strong>',
        r'<strong>([^<]+Market[^<]*)</strong>'
    ]
    
    for pattern in establishment_patterns:
        matches = re.findall(pattern, content)
        for match in matches:
            est_type = "tavern" if "Tavern" in match or "Inn" in match else "shop"
            establishments.append({
                "name": match,
                "type": est_type
            })
    
    return establishments


def _extract_services(content: str) -> dict[str, Any]:
    """Extract services and pricing from content."""
    
    services = {}
    
    # Extract drink prices
    if "Drinks" in content:
        services["drinks"] = _extract_price_table(content, "Drinks")
    
    # Extract food prices  
    if "Food" in content:
        services["food"] = _extract_price_table(content, "Food")
    
    # Extract lodging prices
    if "Lodging" in content:
        services["lodging"] = _extract_price_table(content, "Lodging")
    
    return services


def _extract_price_table(content: str, section: str) -> list[dict[str, str]]:
    """Extract price table from a specific section."""
    
    prices = []
    
    # Find section and extract table rows
    section_start = content.find(f"<h5>{section}</h5>")
    if section_start == -1:
        return prices
    
    # Look for table rows after section
    table_section = content[section_start:section_start + 2000]  # Limit search
    
    price_pattern = r'<tr><td>([^<]+)</td><td>([^<]+)</td></tr>'
    matches = re.findall(price_pattern, table_section)
    
    for item, price in matches:
        prices.append({
            "item": item.strip(),
            "price": price.strip()
        })
    
    return prices


def _extract_settlement_npcs(content: str) -> list[str]:
    """Extract NPC names from settlement content."""
    
    npcs = []
    
    # Find NPC anchors
    npc_pattern = r'<a class="npc-anchor"[^>]*></a><strong>([^<]+)</strong>'
    matches = re.findall(npc_pattern, content)
    
    for match in matches:
        npcs.append(match.strip())
    
    return npcs


def _extract_districts(content: str) -> list[str]:
    """Extract district/area names from settlement."""
    
    districts = []
    
    # Look for href patterns to districts
    district_pattern = r'<a href="[^"]*location/[^"]*">([^<]+)</a>'
    matches = re.findall(district_pattern, content)
    
    for match in matches:
        if "Walk" not in match and "Arch" in match:  # District-like names
            districts.append(match.strip())
    
    return districts


def _extract_economic_data(content: str) -> dict[str, Any]:
    """Extract economic activity data."""
    
    return {
        "currency_circulation": _count_currency_mentions(content),
        "trade_goods": _extract_trade_goods(content),
        "service_variety": _count_service_types(content)
    }


def _extract_faction_identity(content: str) -> tuple[str | None, str | None]:
    """Extract faction name and type."""
    
    name = None
    faction_type = None
    
    # Extract faction name from title
    name_match = re.search(r'<span[^>]*>([^<]+)</span>', content)
    if name_match:
        name = name_match.group(1)
    
    # Determine type from content
    if "cult" in content.lower():
        faction_type = "cult"
    elif "militia" in content.lower():
        faction_type = "militia"
    else:
        faction_type = "unknown"
    
    return name, faction_type


def _extract_faction_leader(content: str) -> dict[str, Any] | None:
    """Extract faction leader information."""
    
    # Look for leader section
    if "leader is a" in content.lower():
        leader_pattern = r'leader is a <strong>([^<]+)</strong>, a level (\d+) (\w+) (\w+)'
        match = re.search(leader_pattern, content)
        
        if match:
            return {
                "name": match.group(1),
                "level": int(match.group(2)),
                "race": match.group(3),
                "class": match.group(4)
            }
    
    return None


def _extract_faction_members(content: str, section: str) -> list[dict[str, Any]]:
    """Extract faction members from specific section."""
    
    members = []
    
    # Find the section
    section_start = content.find(f"<h4> {section} </h4>")
    if section_start == -1:
        return members
    
    # Extract member links
    section_content = content[section_start:section_start + 5000]
    member_pattern = r'<a href="[^"]*"><strong>([^<]+)</strong></a>\s*([^&#]*)'
    
    matches = re.findall(member_pattern, section_content)
    
    for name, description in matches:
        member_data = {
            "name": name.strip(),
            "description": description.strip(),
            "role": _extract_role_from_description(description),
            "location": _extract_location_from_description(description)
        }
        members.append(member_data)
    
    return members


def _extract_faction_base(content: str) -> str | None:
    """Extract faction base/headquarters location."""
    
    # Look for hideout or venue patterns
    base_patterns = [
        r'hideout is the ([^.]+)',
        r"gathering venue is ([^<]+)",
        r"meeting place[^<]*([A-Z][^<]*)"
    ]
    
    for pattern in base_patterns:
        match = re.search(pattern, content, re.IGNORECASE)
        if match:
            return match.group(1).strip()
    
    return None


def _extract_faction_activities(content: str) -> list[str]:
    """Extract what the faction does."""
    
    activities = []
    
    activity_patterns = [
        (r'conducting ([^.]+)', "conducting"),
        (r'plotting to ([^.]+)', "plotting"),
        (r'are ([^.]+ing [^.]+)', "general_activity")
    ]
    
    for pattern, activity_type in activity_patterns:
        matches = re.findall(pattern, content, re.IGNORECASE)
        for match in matches:
            activities.append({
                "type": activity_type,
                "description": match.strip()
            })
    
    return activities


def _extract_dungeon_identity(content: str) -> tuple[str | None, str | None]:
    """Extract dungeon name and type."""
    
    # Extract from title
    name = None
    dungeon_type = None
    
    title_match = re.search(r'<span[^>]*>([^<]+)</span>', content)
    if title_match:
        name = title_match.group(1)
        
        # Classify type from name
        name_lower = name.lower()
        if "crypt" in name_lower:
            dungeon_type = "crypt"
        elif "lair" in name_lower:
            dungeon_type = "lair"
        elif "temple" in name_lower:
            dungeon_type = "temple" 
        elif "shrine" in name_lower:
            dungeon_type = "shrine"
        elif "tomb" in name_lower:
            dungeon_type = "tomb"
        else:
            dungeon_type = "unknown"
    
    return name, dungeon_type


def _extract_encounters(content: str) -> list[dict[str, Any]]:
    """Extract creature encounters from dungeon content."""
    
    encounters = []
    
    # Look for monster blocks
    monster_pattern = r'<div id="block-[^"]*" class="monster-block">(.*?)</div>\s*</div>\s*</div>\s*</div>'
    matches = re.findall(monster_pattern, content, re.DOTALL)
    
    for match in matches:
        encounter = _parse_monster_block(match)
        if encounter:
            encounters.append(encounter)
    
    return encounters


def _extract_treasure_hoards(content: str) -> list[dict[str, Any]]:
    """Extract treasure hoard information."""
    
    hoards = []
    
    # Look for hoard patterns
    hoard_pattern = r'<strong>([^<]*hoard[^<]*)</strong>[^<]*<ul>(.*?)</ul>'
    matches = re.findall(hoard_pattern, content, re.DOTALL | re.IGNORECASE)
    
    for hoard_name, hoard_content in matches:
        hoard = {
            "type": hoard_name.strip(),
            "contents": _parse_hoard_contents(hoard_content)
        }
        hoards.append(hoard)
    
    return hoards


def _extract_challenge_rating(content: str) -> str | None:
    """Extract overall challenge rating assessment."""
    
    # Count high-CR monsters
    cr_mentions = re.findall(r'CR:\s*(\d+)', content)
    
    if not cr_mentions:
        return None
    
    max_cr = max(int(cr) for cr in cr_mentions)
    
    if max_cr >= 10:
        return "deadly"
    elif max_cr >= 5:
        return "hard"
    elif max_cr >= 2:
        return "medium"
    else:
        return "easy"


def _extract_horror_themes(content: str) -> list[str]:
    """Extract horror themes from dungeon content."""
    
    themes = []
    
    horror_keywords = [
        "corrupted", "infernal", "cursed", "mourning", "violent",
        "unholy", "burning", "bleeding", "foresaken", "unspoken",
        "raging", "grey", "defiled"
    ]
    
    content_lower = content.lower()
    for keyword in horror_keywords:
        if keyword in content_lower:
            themes.append(keyword)
    
    return list(set(themes))  # Remove duplicates


# Utility helper functions
def _count_currency_mentions(content: str) -> dict[str, int]:
    """Count currency mentions by type."""
    return {
        "gold": content.count(" gp"),
        "silver": content.count(" sp"), 
        "copper": content.count(" cp")
    }


def _extract_trade_goods(content: str) -> list[str]:
    """Extract trade goods from economic content."""
    
    goods = []
    
    # Look in inventory tables
    if "inventory" in content.lower():
        item_pattern = r'<tr>\s*<td>([^<]+)</td>'
        matches = re.findall(item_pattern, content)
        goods.extend([match.strip() for match in matches if match.strip()])
    
    return goods


def _count_service_types(content: str) -> int:
    """Count different types of services offered."""
    
    service_sections = ["Drinks", "Food", "Lodging", "Inventory", "Services"]
    count = 0
    
    for section in service_sections:
        if f"<h5>{section}</h5>" in content:
            count += 1
    
    return count


def _extract_role_from_description(description: str) -> str | None:
    """Extract professional role from member description."""
    
    role_patterns = ["Fighter", "Cleric", "Wizard", "Rogue", "Druid"]
    
    for role in role_patterns:
        if role in description:
            return role.lower()
    
    return None


def _extract_location_from_description(description: str) -> str | None:
    """Extract location from member description."""
    
    location_match = re.search(r'from ([A-Z][a-z]+)', description)
    if location_match:
        return location_match.group(1)
    
    return None


def _parse_monster_block(block_content: str) -> dict[str, Any] | None:
    """Parse individual monster block content."""
    
    encounter = {}
    
    # Extract CR
    cr_match = re.search(r'CR:</span>\s*([^<]+)', block_content)
    if cr_match:
        encounter["challenge_rating"] = cr_match.group(1).strip()
    
    # Extract name (usually in nearby content)
    # This is simplified - in practice would need more context
    
    return encounter if encounter else None


def _parse_hoard_contents(hoard_content: str) -> dict[str, Any]:
    """Parse treasure hoard contents."""
    
    contents = {
        "coins": 0,
        "gemstones": [],
        "magic_items": [],
        "artifacts": []
    }
    
    # Extract coin amounts
    coin_pattern = r'<strong>([0-9,]+)\s*gp</strong>'
    coin_match = re.search(coin_pattern, hoard_content)
    if coin_match:
        contents["coins"] = int(coin_match.group(1).replace(",", ""))
    
    # Extract gemstones
    if "gemstones" in hoard_content:
        gem_pattern = r'<span[^>]*>([^<]+)</span>'
        gems = re.findall(gem_pattern, hoard_content)
        contents["gemstones"] = gems
    
    # Extract magic items
    if "Magic Items" in hoard_content:
        magic_pattern = r'Magic Items:</strong>([^<]+)'
        magic_match = re.search(magic_pattern, hoard_content)
        if magic_match:
            items = [item.strip() for item in magic_match.group(1).split(",")]
            contents["magic_items"] = items
    
    return contents


def validate_extraction_quality(extracted_data: dict[str, Any]) -> float:
    """
    Validate quality of extraction results.
    
    Returns:
        Confidence score 0.0-1.0
    """
    
    quality_factors = []
    
    # Check for required fields
    if extracted_data.get("name"):
        quality_factors.append(0.3)  # Has name
    
    if extracted_data.get("category"):
        quality_factors.append(0.2)  # Has category
    
    # Check for content richness
    content_fields = ["stats", "possessions", "establishments", "members", "encounters"]
    populated_fields = sum(1 for field in content_fields if extracted_data.get(field))
    
    if populated_fields > 0:
        quality_factors.append(0.3 * (populated_fields / len(content_fields)))
    
    # Check for data consistency
    if extracted_data.get("training_signals"):
        quality_factors.append(0.2)
    
    return sum(quality_factors)


def create_ml_training_vector(extracted_data: dict[str, Any], category: str) -> dict[str, Any]:
    """
    Convert extracted data into ML training vector.
    
    Args:
        extracted_data: Result of content extraction
        category: Category type for feature selection
        
    Returns:
        ML-ready feature vector
    """
    
    if category == "regions":
        return _create_region_vector(extracted_data)
    elif category == "settlements":
        return _create_settlement_vector(extracted_data)
    elif category == "factions":
        return _create_faction_vector(extracted_data) 
    elif category == "dungeons":
        return _create_dungeon_vector(extracted_data)
    else:
        return {}


def _create_region_vector(data: dict[str, Any]) -> dict[str, Any]:
    """Create ML vector for region data."""
    
    training_signals = data.get("training_signals", {})
    
    return {
        "hex_count": training_signals.get("hex_count", 0),
        "biome_variety": training_signals.get("biome_variety", 0),
        "settlement_density": training_signals.get("political_entities", 0),
        "geographic_complexity": training_signals.get("geographic_features", 0),
        "content_richness": data.get("content_features", {}).get("npc_density", 0)
    }


def _create_settlement_vector(data: dict[str, Any]) -> dict[str, Any]:
    """Create ML vector for settlement data."""
    
    training_signals = data.get("training_signals", {})
    
    return {
        "scale_numeric": _scale_to_numeric(training_signals.get("scale_indicators", "unknown")),
        "establishment_count": training_signals.get("establishment_count", 0),
        "npc_density": training_signals.get("npc_density", 0),
        "economic_complexity": training_signals.get("economic_complexity", 0),
        "service_variety": len(data.get("services", {}))
    }


def _create_faction_vector(data: dict[str, Any]) -> dict[str, Any]:
    """Create ML vector for faction data."""
    
    training_signals = data.get("training_signals", {})
    
    return {
        "member_count": training_signals.get("member_count", 0),
        "has_leader": 1 if training_signals.get("leadership_structure", {}).get("has_leader") else 0,
        "territorial_control": len(training_signals.get("territorial_control", {})),
        "activity_complexity": len(data.get("activities", [])),
        "organizational_sophistication": _assess_org_sophistication(data)
    }


def _create_dungeon_vector(data: dict[str, Any]) -> dict[str, Any]:
    """Create ML vector for dungeon data."""
    
    training_signals = data.get("training_signals", {})
    
    return {
        "encounter_density": len(data.get("encounters", [])),
        "treasure_complexity": _treasure_to_numeric(training_signals.get("treasure_density", "none")),
        "challenge_rating": _cr_to_numeric(training_signals.get("challenge_rating", "unknown")),
        "horror_theme_count": len(training_signals.get("horror_themes", [])),
        "content_richness": len(data.get("content_features", {}))
    }


# Helper conversion functions
def _scale_to_numeric(scale: str) -> int:
    """Convert settlement scale to numeric value."""
    scale_mapping = {"village": 1, "town": 2, "city": 3}
    return scale_mapping.get(scale.lower(), 0)


def _treasure_to_numeric(treasure_level: str) -> int:
    """Convert treasure level to numeric value."""
    treasure_mapping = {"none": 0, "basic": 1, "moderate": 2, "rich": 3}
    return treasure_mapping.get(treasure_level.lower(), 0)


def _cr_to_numeric(cr_level: str) -> int:
    """Convert challenge rating to numeric value."""
    cr_mapping = {"easy": 1, "medium": 2, "hard": 3, "deadly": 4}
    return cr_mapping.get(cr_level.lower(), 0)


def _assess_org_sophistication(faction_data: dict[str, Any]) -> int:
    """Assess organizational sophistication level."""
    
    sophistication = 0
    
    if faction_data.get("leader"):
        sophistication += 1
    
    if len(faction_data.get("members", [])) > 10:
        sophistication += 1
    
    if faction_data.get("collaborators"):
        sophistication += 1
    
    if faction_data.get("base_location"):
        sophistication += 1
    
    return sophistication


def _assess_territorial_reach(content: str) -> dict[str, int]:
    """Assess territorial reach of faction."""
    
    # Count location mentions
    import re
    locations = re.findall(r'from ([A-Z][a-z]+)', content)
    
    location_count = {}
    for location in locations:
        location_count[location] = location_count.get(location, 0) + 1
    
    return location_count


def _detect_alignment_patterns(content: str) -> str | None:
    """Detect political alignment from faction activities."""
    
    content_lower = content.lower()
    
    if "justice" in content_lower:
        return "lawful"
    elif "cult" in content_lower or "defiled" in content_lower:
        return "chaotic"
    elif "eliminate" in content_lower or "plotting" in content_lower:
        return "aggressive"
    else:
        return "neutral"
