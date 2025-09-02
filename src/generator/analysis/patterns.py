"""
Comprehensive HBF Patterns - All regex patterns for Dragon's Labyrinth extraction.

Built from hbf_analysis/hex_tiles_full.csv and "The Lands of Vo'il" first principles.
Separated from entity logic for maintainability and iterative improvement.
"""

from __future__ import annotations

import json
import re
from enum import Enum, auto
from typing import Any


# ============================================================================
# WORLD CONSTANTS - Dragon's Labyrinth Core Facts
# ============================================================================

WORLD_NAME = "The Lands of Vo'il"

# 4 Regions from HBF analysis mapping to Dragon's Labyrinth acts
REGIONS = {
    "Fearless Wilds": {"act_levels": "1-20", "corruption": 0, "biome": "jungle"},
    "Vicious Crags": {"act_levels": "21-40", "corruption": 1, "biome": "mountains"}, 
    "Ragthorn Woods": {"act_levels": "41-60", "corruption": 2, "biome": "forest"},
    "Heartseeker Forest": {"act_levels": "61-180", "corruption": 3, "biome": "deep_forest"}
}

# Horror corruption variants for creatures
CORRUPTION_VARIANTS = ["tainted", "corrupted", "nightmare", "unspeakable"]

# Philosophy paths for Dragon's Labyrinth
PHILOSOPHY_PATHS = ["strength", "harmony", "light", "dark"]


# ============================================================================
# HEX TILE PATTERNS - From hbf_analysis/hex_tiles_full.csv
# ============================================================================

class HexTilePatterns:
    """Patterns for extracting hex tile data from HBF entities."""
    
    # Main hex header pattern - captures coordinate and region
    HEX_HEADER = re.compile(
        r'^([a-zA-Z0-9]{8})\s+Hex\s+([A-Z0-9]+)\s+in\s+([^(]+)\s*\(([^)]+)\)\s*([^>]*?)\s*>',
        re.MULTILINE
    )
    
    # Coordinate types from analysis
    COORDINATE_BASE = re.compile(r'^BASE$')
    COORDINATE_SIMPLE = re.compile(r'^[NSEW]\d+$')
    COORDINATE_COMPLEX = re.compile(r'^[NSEW]\d+[NSEW]\d+$')
    
    # Regional validation
    REGION_FEARLESS_WILDS = re.compile(r'Fearless Wilds', re.IGNORECASE)
    REGION_VICIOUS_CRAGS = re.compile(r'Vicious Crags', re.IGNORECASE)
    REGION_RAGTHORN_WOODS = re.compile(r'Ragthorn Woods', re.IGNORECASE)  
    REGION_HEARTSEEKER_FOREST = re.compile(r'Heartseeker Forest', re.IGNORECASE)
    
    # Content flags from analysis
    HAS_ENCOUNTERS = re.compile(r'1d4\s+encounter', re.IGNORECASE)
    HAS_WEATHER = re.compile(r'2d6.*weather', re.IGNORECASE)
    HAS_NPCS = re.compile(r'level\s+\d+\s+(wizard|fighter|cleric|rogue)', re.IGNORECASE)
    HAS_TREASURE = re.compile(r'(gp in coins|lair hoard)', re.IGNORECASE)
    HAS_SHOPS = re.compile(r'(for sale|offering)', re.IGNORECASE)
    HAS_RUMORS = re.compile(r'rumor', re.IGNORECASE)
    HAS_BOSS = re.compile(r'(tarrasque|guardian naga|stone golem|shambling mound|gorgon)', re.IGNORECASE)
    HAS_FACTION = re.compile(r'(fists of justice|swords of justice)', re.IGNORECASE)


# ============================================================================
# CREATURE PATTERNS - D&D 5e stat blocks for horror progression
# ============================================================================

class CreaturePatterns:
    """Patterns for extracting creature stat blocks from HBF entities."""
    
    # Main creature stat block pattern
    CREATURE_BLOCK = re.compile(
        r'([A-Za-z\s\'-]+?)\s*(?:\((\d+)\))?\s*CR:\s*([^\n]+?).*?'
        r'AC:\s*(\d+).*?HP:\s*\(([^)]+)\).*?Speed:\s*([^\n]+?).*?'
        r'STR\s+DEX\s+CON\s+INT\s+WIS\s+CHA\s+([\d\s+-]+)',
        re.DOTALL
    )
    
    # Challenge rating patterns
    CR_FRACTION = re.compile(r'(\d+)/(\d+)')
    CR_WITH_XP = re.compile(r'(\d+(?:\.\d+)?)\s*\((\d+)\s*XP\)')
    CR_SIMPLE = re.compile(r'\b(\d+(?:\.\d+)?)\b')
    
    # Ability scores pattern
    ABILITY_SCORES = re.compile(r'(\d+)\s*[+-]\d+')
    
    # Speed patterns  
    SPEED_WALK = re.compile(r'Walk\s*(\d+)\s*ft', re.IGNORECASE)
    SPEED_FLY = re.compile(r'Fly\s*(\d+)\s*ft', re.IGNORECASE)
    SPEED_SWIM = re.compile(r'Swim\s*(\d+)\s*ft', re.IGNORECASE)
    SPEED_CLIMB = re.compile(r'Climb\s*(\d+)\s*ft', re.IGNORECASE)
    
    # Horror classification patterns for Dragon's Labyrinth
    CORRUPTION_INDICATORS = {
        "tainted": re.compile(r'(diseased|sick|wounded|corrupted)', re.IGNORECASE),
        "scorched": re.compile(r'(burned|scorched|charred|fire)', re.IGNORECASE),
        "nightmare": re.compile(r'(nightmare|terror|horror|void)', re.IGNORECASE),
        "eldritch": re.compile(r'(eldritch|cosmic|otherworldly|ancient)', re.IGNORECASE)
    }
    
    # Dragon's Labyrinth creature categories (not D&D monster types)
    COMPANION_TRAUMA_TRIGGERS = {
        "violence": re.compile(r'(blood|gore|brutal|savage|torture)', re.IGNORECASE),
        "corruption": re.compile(r'(corruption|taint|void|darkness)', re.IGNORECASE),
        "death": re.compile(r'(death|corpse|undead|necromancy)', re.IGNORECASE),
        "madness": re.compile(r'(insanity|madness|mind|charm)', re.IGNORECASE)
    }


# ============================================================================
# NPC PATTERNS - Psychology-focused for companion system
# ============================================================================

class NPCPatterns:
    """Patterns for extracting NPCs with Dragon's Labyrinth psychology focus."""
    
    # Character with level and class
    NPC_WITH_CLASS = re.compile(
        r'([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*),?\s+level\s+(\d+)\s+(wizard|fighter|cleric|rogue|paladin|ranger|bard)',
        re.IGNORECASE
    )
    
    # Named NPCs in settlements
    SETTLEMENT_NPC = re.compile(
        r'([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*).{0,50}(tavern|shop|temple|inn|guard)',
        re.IGNORECASE
    )
    
    # Dragon's Labyrinth psychology indicators
    TRAUMA_INDICATORS = {
        "baseline_trauma": re.compile(r'(scared|nervous|worried|anxious)', re.IGNORECASE),
        "moderate_trauma": re.compile(r'(terrified|desperate|broken|haunted)', re.IGNORECASE),
        "severe_trauma": re.compile(r'(mad|insane|catatonic|void-touched)', re.IGNORECASE)
    }
    
    # Companion potential indicators
    COMPANION_INDICATORS = {
        "warrior": re.compile(r'(fighter|soldier|guard|veteran)', re.IGNORECASE),
        "healer": re.compile(r'(cleric|priest|healer|medic)', re.IGNORECASE),
        "scholar": re.compile(r'(wizard|sage|scholar|loremaster)', re.IGNORECASE),
        "rogue": re.compile(r'(rogue|thief|scout|ranger)', re.IGNORECASE)
    }
    
    # Sentimental item sources (for forge system)
    SENTIMENTAL_SOURCES = re.compile(
        r'(family|heirloom|memory|precious|dear|beloved|treasured)',
        re.IGNORECASE
    )


# ============================================================================
# ITEM PATTERNS - Sentimental value and forge reagent focus
# ============================================================================

class ItemPatterns:
    """Patterns for extracting items with Dragon's Labyrinth sentimental focus."""
    
    # Treasure hoards
    TREASURE_HOARD = re.compile(
        r'(\d+)\s*gp in coins.*?(?:plus\s+(.+?)(?:\n|$))?',
        re.IGNORECASE | re.DOTALL
    )
    
    # Equipment with material progression
    EQUIPMENT_WITH_MATERIAL = re.compile(
        r'(wooden|stone|iron|steel|silver|gold|diamond|mythic)?\s*(sword|axe|bow|shield|armor|weapon)',
        re.IGNORECASE
    )
    
    # Sentimental value indicators for forge system
    SENTIMENTAL_INDICATORS = {
        "family_heirloom": re.compile(r'(family|ancestor|grandmother|grandfather)', re.IGNORECASE),
        "love_token": re.compile(r'(love|heart|wedding|romance|beloved)', re.IGNORECASE),
        "memory_item": re.compile(r'(memory|remember|childhood|past)', re.IGNORECASE),
        "sacrifice_item": re.compile(r'(sacrifice|offered|given|dedicated)', re.IGNORECASE)
    }
    
    # Forge path alignment
    FORGE_PATH_INDICATORS = {
        "light": re.compile(r'(blessed|holy|sacred|divine|pure)', re.IGNORECASE),
        "dark": re.compile(r'(cursed|dark|shadow|evil|corrupt)', re.IGNORECASE)
    }


# ============================================================================
# SETTLEMENT PATTERNS - Scale and psychology state
# ============================================================================

class SettlementPatterns:
    """Patterns for extracting settlements with Dragon's Labyrinth scale focus."""
    
    # Settlement scale indicators
    VILLAGE_INDICATORS = re.compile(r'(village|hamlet|farm|cabin)', re.IGNORECASE)
    TOWN_INDICATORS = re.compile(r'(town|market|trading post)', re.IGNORECASE)  
    CITY_INDICATORS = re.compile(r'(city|fortress|stronghold|capital)', re.IGNORECASE)
    
    # Settlement features
    TAVERN = re.compile(r'tavern|inn|pub|alehouse', re.IGNORECASE)
    SHOP = re.compile(r'shop|store|merchant|trader|goods', re.IGNORECASE)
    TEMPLE = re.compile(r'temple|shrine|church|altar', re.IGNORECASE)
    
    # Population psychological state (for Dragon's Labyrinth)
    POPULATION_STATE = {
        "peaceful": re.compile(r'(peaceful|calm|prosperous|thriving)', re.IGNORECASE),
        "defensive": re.compile(r'(guard|wall|fortified|defensive)', re.IGNORECASE),
        "militarized": re.compile(r'(military|army|soldier|garrison)', re.IGNORECASE),
        "ruined": re.compile(r'(ruin|abandoned|destroyed|empty)', re.IGNORECASE)
    }


# ============================================================================
# DUNGEON PATTERNS - Cave/Temple/Tomb classification
# ============================================================================

class DungeonPatterns:
    """Patterns for extracting dungeon types for Dragon's Labyrinth."""
    
    CAVE_INDICATORS = re.compile(r'(cave|cavern|grotto|underground)', re.IGNORECASE)
    TEMPLE_INDICATORS = re.compile(r'(temple|shrine|sanctuary|altar)', re.IGNORECASE)
    TOMB_INDICATORS = re.compile(r'(tomb|crypt|grave|burial|mausoleum)', re.IGNORECASE)
    
    # Dungeon complexity for progression
    SIMPLE_DUNGEON = re.compile(r'(single room|chamber|small)', re.IGNORECASE)
    COMPLEX_DUNGEON = re.compile(r'(multiple|maze|labyrinth|complex)', re.IGNORECASE)
    
    # Horror elements for dread progression
    HORROR_ELEMENTS = {
        "corruption": re.compile(r'(corrupted|tainted|void|darkness)', re.IGNORECASE),
        "undead": re.compile(r'(undead|ghost|skeleton|zombie)', re.IGNORECASE),
        "eldritch": re.compile(r'(eldritch|cosmic|otherworldly)', re.IGNORECASE)
    }


# ============================================================================
# FACTION PATTERNS - Cults/Militias/Syndicates for Dragon's Labyrinth
# ============================================================================

class FactionPatterns:
    """Patterns for extracting factions aligned with Dragon's Labyrinth themes."""
    
    # Faction types for Dragon's Labyrinth progression
    CULT_INDICATORS = re.compile(r'(cult|worship|ritual|sacrifice|dark)', re.IGNORECASE)
    MILITIA_INDICATORS = re.compile(r'(militia|guard|soldier|army|defense)', re.IGNORECASE)
    SYNDICATE_INDICATORS = re.compile(r'(syndicate|gang|crime|thief|underground)', re.IGNORECASE)
    
    # Known factions from HBF analysis
    FISTS_OF_JUSTICE = re.compile(r'Fists of Justice', re.IGNORECASE)
    SWORDS_OF_JUSTICE = re.compile(r'Swords of Justice', re.IGNORECASE)
    
    # Faction psychological state for horror progression
    FACTION_STATE = {
        "helpful": re.compile(r'(help|aid|assist|protect)', re.IGNORECASE),
        "neutral": re.compile(r'(neutral|watch|observe)', re.IGNORECASE),
        "hostile": re.compile(r'(hostile|enemy|attack|raid)', re.IGNORECASE),
        "corrupted": re.compile(r'(corrupted|tainted|void|dark)', re.IGNORECASE)
    }


# ============================================================================
# MASTER PATTERN MATCHER - Routes content to specialized tables
# ============================================================================

class ContentRouter:
    """
    Master content router using all patterns to determine entity destination.
    
    Routes to: hex_tiles, creatures, npcs, treasures, settlements, factions
    Falls back to: html_entities or json_entities if no match
    """
    
    def __init__(self):
        self.routing_stats = {
            "hex_tiles": 0, "creatures": 0, "npcs": 0, "treasures": 0,
            "settlements": 0, "factions": 0, "html_fallback": 0, "json_fallback": 0
        }
    
    def route_entity(self, hbf_uuid: str, content: str) -> tuple[str, dict[str, Any], float]:
        """
        Analyze content and route to appropriate table.
        
        Args:
            hbf_uuid: 8-character HBF UUID
            content: Raw entity content
            
        Returns:
            (target_table, extracted_data, confidence_score)
        """
        
        # Try JSON first
        try:
            json_data = json.loads(content)
            self.routing_stats["json_fallback"] += 1
            return ("json_entities", {"json_data": json_data}, 1.0)
        except json.JSONDecodeError:
            pass
        
        # Check hex tile pattern first (highest priority)
        hex_match = HexTilePatterns.HEX_HEADER.search(content)
        if hex_match and WORLD_NAME in content:
            extracted = self._extract_hex_tile_data(hex_match, content)
            self.routing_stats["hex_tiles"] += 1
            return ("hex_tiles", extracted, 0.95)
        
        # Check for creature stat blocks
        creature_match = CreaturePatterns.CREATURE_BLOCK.search(content)
        if creature_match:
            extracted = self._extract_creature_data(creature_match, content)
            self.routing_stats["creatures"] += 1  
            return ("creatures", extracted, 0.90)
        
        # Check for NPCs with levels/classes
        npc_match = NPCPatterns.NPC_WITH_CLASS.search(content)
        if npc_match:
            extracted = self._extract_npc_data(npc_match, content)
            self.routing_stats["npcs"] += 1
            return ("npcs", extracted, 0.85)
        
        # Check for treasure hoards
        treasure_match = ItemPatterns.TREASURE_HOARD.search(content)
        if treasure_match:
            extracted = self._extract_treasure_data(treasure_match, content)  
            self.routing_stats["treasures"] += 1
            return ("treasures", extracted, 0.80)
        
        # Check for settlement descriptions
        if any(pattern.search(content) for pattern in [
            SettlementPatterns.VILLAGE_INDICATORS,
            SettlementPatterns.TOWN_INDICATORS, 
            SettlementPatterns.CITY_INDICATORS
        ]):
            extracted = self._extract_settlement_data(content)
            self.routing_stats["settlements"] += 1
            return ("settlements", extracted, 0.75)
        
        # Check for faction information
        if any(pattern.search(content) for pattern in [
            FactionPatterns.CULT_INDICATORS,
            FactionPatterns.MILITIA_INDICATORS,
            FactionPatterns.SYNDICATE_INDICATORS
        ]):
            extracted = self._extract_faction_data(content)
            self.routing_stats["factions"] += 1
            return ("factions", extracted, 0.70)
        
        # Fallback to HTML entities for ML analysis
        self.routing_stats["html_fallback"] += 1
        return ("html_entities", {"html_content": content}, 0.50)
    
    def _extract_hex_tile_data(self, match: re.Match, content: str) -> dict[str, Any]:
        """Extract hex tile data for Dragon's Labyrinth."""
        
        uuid = match.group(1)
        coordinate = match.group(2) 
        region = match.group(3).strip()
        world = match.group(4)
        feature = match.group(5).strip() if match.group(5) else None
        
        # Determine corruption level from region
        corruption_level = REGIONS.get(region, {}).get("corruption", 0)
        biome_type = REGIONS.get(region, {}).get("biome", "unknown")
        
        # Content analysis
        content_flags = {
            "has_encounters": bool(HexTilePatterns.HAS_ENCOUNTERS.search(content)),
            "has_weather": bool(HexTilePatterns.HAS_WEATHER.search(content)),
            "has_npcs": bool(HexTilePatterns.HAS_NPCS.search(content)),
            "has_treasure": bool(HexTilePatterns.HAS_TREASURE.search(content)),
            "has_shops": bool(HexTilePatterns.HAS_SHOPS.search(content)),
            "has_rumors": bool(HexTilePatterns.HAS_RUMORS.search(content)),
            "has_boss": bool(HexTilePatterns.HAS_BOSS.search(content)),
            "has_faction": bool(HexTilePatterns.HAS_FACTION.search(content))
        }
        
        return {
            "hbf_uuid": uuid,
            "coordinate": coordinate,
            "region": region,
            "world": world,
            "biome_type": biome_type,
            "corruption_level": corruption_level,
            "feature_name": feature,
            **content_flags,
            "environmental_description": self._extract_description(content)
        }
    
    def _extract_creature_data(self, match: re.Match, content: str) -> dict[str, Any]:
        """Extract creature data focused on Dragon's Labyrinth horror progression."""
        
        name = match.group(1).strip()
        quantity = int(match.group(2)) if match.group(2) else 1
        cr_str = match.group(3).strip()
        ac = int(match.group(4))
        hp_dice = match.group(5)
        speed_str = match.group(6)
        abilities_str = match.group(7)
        
        # Parse abilities 
        abilities = self._parse_abilities(abilities_str)
        speeds = self._parse_speeds(speed_str)
        cr_numeric = self._parse_cr(cr_str)
        
        # Dragon's Labyrinth classification
        corruption_variant = self._detect_corruption_variant(name, content)
        trauma_triggers = self._detect_trauma_triggers(content)
        horror_impact = self._calculate_horror_impact(cr_numeric, corruption_variant)
        
        return {
            "base_name": name,
            "corruption_variant": corruption_variant,
            "quantity": quantity,
            "threat_level": min(10, max(1, int(cr_numeric * 2))),  # Convert CR to 1-10 scale
            "health_points": self._estimate_hp_from_dice(hp_dice),
            "attack_damage": hp_dice,  # Use as placeholder
            "armor_class": ac,
            "movement_speed": speeds.get("walk", 30),
            "dread_level": min(4, max(0, int(cr_numeric // 2))),
            "horror_impact": horror_impact,
            "trauma_triggers": trauma_triggers,
            "abilities": abilities
        }
    
    def _extract_npc_data(self, match: re.Match, content: str) -> dict[str, Any]:
        """Extract NPC data focused on Dragon's Labyrinth psychology."""
        
        name = match.group(1).strip()
        level = int(match.group(2))
        class_name = match.group(3).lower()
        
        # Psychology analysis
        trauma_level = self._assess_trauma_level(content)
        companion_type = self._map_class_to_companion(class_name)
        sentimental_items = bool(NPCPatterns.SENTIMENTAL_SOURCES.search(content))
        
        # Settlement context
        settlement_type = self._detect_settlement_context(content)
        region = self._detect_region_from_content(content)
        
        return {
            "name": name,
            "level": level,
            "class_name": class_name,
            "companion_type": companion_type,
            "can_be_companion": level >= 3 and companion_type != "unknown",
            "baseline_trauma": trauma_level,
            "current_stress": trauma_level // 2,
            "settlement_type": settlement_type,
            "region": region,
            "has_sentimental_items": sentimental_items,
            "philosophy_lean": self._detect_philosophy_lean(content)
        }
    
    def _extract_treasure_data(self, match: re.Match, content: str) -> dict[str, Any]:
        """Extract treasure data focused on Dragon's Labyrinth sentimental system."""
        
        gp_amount = int(match.group(1))
        additional_items = match.group(2) if match.group(2) else ""
        
        # Sentimental value assessment
        sentimental_score = self._assess_sentimental_value(content)
        forge_potential = sentimental_score > 0.5
        material_tier = self._assess_material_tier(gp_amount, additional_items)
        
        return {
            "treasure_value": gp_amount,
            "additional_items": additional_items.strip(),
            "is_sentimental": forge_potential,
            "emotional_weight": sentimental_score,
            "material_tier": material_tier,
            "forge_reagent_type": self._determine_forge_type(content, sentimental_score)
        }
    
    def _extract_settlement_data(self, content: str) -> dict[str, Any]:
        """Extract settlement data for Dragon's Labyrinth scale system."""
        
        # Determine settlement type
        if SettlementPatterns.VILLAGE_INDICATORS.search(content):
            settlement_type = "village"
        elif SettlementPatterns.TOWN_INDICATORS.search(content):
            settlement_type = "town"
        elif SettlementPatterns.CITY_INDICATORS.search(content):
            settlement_type = "city"
        else:
            settlement_type = "unknown"
        
        # Population state for horror progression
        pop_state = "unknown"
        for state, pattern in SettlementPatterns.POPULATION_STATE.items():
            if pattern.search(content):
                pop_state = state
                break
        
        # Features available
        features = []
        if SettlementPatterns.TAVERN.search(content):
            features.append("tavern")
        if SettlementPatterns.SHOP.search(content):
            features.append("shop")
        if SettlementPatterns.TEMPLE.search(content):
            features.append("temple")
        
        return {
            "settlement_type": settlement_type,
            "population_state": pop_state,
            "features": features,
            "region": self._detect_region_from_content(content)
        }
    
    def _extract_faction_data(self, content: str) -> dict[str, Any]:
        """Extract faction data for Dragon's Labyrinth progression."""
        
        # Determine faction type
        if FactionPatterns.CULT_INDICATORS.search(content):
            faction_type = "cult"
        elif FactionPatterns.MILITIA_INDICATORS.search(content):
            faction_type = "militia"
        elif FactionPatterns.SYNDICATE_INDICATORS.search(content):
            faction_type = "syndicate"
        else:
            faction_type = "unknown"
        
        # Known faction detection
        faction_name = "unknown"
        if FactionPatterns.FISTS_OF_JUSTICE.search(content):
            faction_name = "Fists of Justice"
        elif FactionPatterns.SWORDS_OF_JUSTICE.search(content):
            faction_name = "Swords of Justice"
        
        # Faction state for horror progression
        state = "unknown"
        for faction_state, pattern in FactionPatterns.FACTION_STATE.items():
            if pattern.search(content):
                state = faction_state
                break
        
        return {
            "faction_type": faction_type,
            "faction_name": faction_name,
            "faction_state": state,
            "philosophy_alignment": self._detect_philosophy_lean(content)
        }
    
    # ========================================================================
    # HELPER METHODS - Pattern analysis utilities
    # ========================================================================
    
    def _detect_corruption_variant(self, name: str, content: str) -> str:
        """Detect corruption variant for horror progression."""
        
        for variant, pattern in CreaturePatterns.CORRUPTION_INDICATORS.items():
            if pattern.search(content):
                return variant
        return "normal"
    
    def _detect_trauma_triggers(self, content: str) -> list[str]:
        """Detect what trauma this creature might trigger."""
        
        triggers = []
        for trigger, pattern in CreaturePatterns.COMPANION_TRAUMA_TRIGGERS.items():
            if pattern.search(content):
                triggers.append(trigger)
        return triggers
    
    def _calculate_horror_impact(self, cr: float, corruption_variant: str) -> float:
        """Calculate horror impact on companions."""
        
        base_impact = min(1.0, cr / 10.0)  # CR contributes to impact
        
        # Corruption amplifies horror
        corruption_multiplier = {
            "normal": 1.0, "tainted": 1.2, "corrupted": 1.5, 
            "nightmare": 2.0, "unspeakable": 3.0
        }
        
        return min(1.0, base_impact * corruption_multiplier.get(corruption_variant, 1.0))
    
    def _assess_trauma_level(self, content: str) -> int:
        """Assess NPC baseline trauma from content."""
        
        for trauma_type, pattern in NPCPatterns.TRAUMA_INDICATORS.items():
            if pattern.search(content):
                if trauma_type == "severe_trauma":
                    return 8
                elif trauma_type == "moderate_trauma":
                    return 5
                elif trauma_type == "baseline_trauma":
                    return 2
        return 0
    
    def _map_class_to_companion(self, class_name: str) -> str:
        """Map D&D class to Dragon's Labyrinth companion type."""
        
        class_mapping = {
            "fighter": "warrior", "paladin": "warrior", "ranger": "warrior",
            "cleric": "healer", "druid": "healer",
            "wizard": "scholar", "sorcerer": "scholar", "warlock": "scholar",
            "rogue": "rogue", "bard": "rogue"
        }
        
        return class_mapping.get(class_name, "unknown")
    
    def _assess_sentimental_value(self, content: str) -> float:
        """Assess sentimental value for forge system."""
        
        score = 0.0
        for indicator_type, pattern in ItemPatterns.SENTIMENTAL_INDICATORS.items():
            if pattern.search(content):
                score += 0.25
        
        return min(1.0, score)
    
    def _detect_region_from_content(self, content: str) -> str:
        """Detect region from content."""
        
        for region in REGIONS.keys():
            if region.lower() in content.lower():
                return region
        return "unknown"
    
    def _detect_philosophy_lean(self, content: str) -> str:
        """Detect philosophical alignment from content."""
        
        content_lower = content.lower()
        
        if any(word in content_lower for word in ["strength", "power", "force", "violence", "might"]):
            return "strength"
        elif any(word in content_lower for word in ["harmony", "peace", "cooperation", "together", "unity"]):
            return "harmony"
        elif any(word in content_lower for word in ["light", "holy", "blessed", "sacred", "divine"]):
            return "light"
        elif any(word in content_lower for word in ["dark", "cursed", "shadow", "void", "corrupt"]):
            return "dark"
        
        return "neutral"
    
    def _extract_description(self, content: str) -> str:
        """Extract environmental description from hex content."""
        
        lines = content.split('\n')
        description_lines = []
        
        in_description = False
        for line in lines:
            line = line.strip()
            
            # Start after hex header
            if '>' in line and WORLD_NAME in line:
                in_description = True
                continue
            
            # Stop at first major section
            if in_description and any(keyword in line for keyword in ['Random encounter', 'Lair hoard', 'CR:', 'Level:']):
                break
                
            # Collect description lines
            if in_description and line and not line.startswith('>') and not line.startswith('<'):
                description_lines.append(line)
        
        return ' '.join(description_lines[:3])  # First 3 sentences
    
    def _parse_abilities(self, abilities_str: str) -> dict[str, int]:
        """Parse STR DEX CON INT WIS CHA ability scores."""
        
        scores = re.findall(r'(\d+)\s*[+-]\d+', abilities_str)
        ability_names = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"]
        
        abilities = {}
        for i, score in enumerate(scores[:6]):  # Limit to 6 abilities
            if i < len(ability_names):
                abilities[ability_names[i]] = int(score)
        
        # Fill missing abilities with default
        for name in ability_names:
            if name not in abilities:
                abilities[name] = 10
        
        return abilities
    
    def _parse_speeds(self, speed_str: str) -> dict[str, int]:
        """Parse movement speeds from speed string."""
        
        speeds = {}
        
        # Walk speed (always present)
        walk_match = CreaturePatterns.SPEED_WALK.search(speed_str)
        speeds["walk"] = int(walk_match.group(1)) if walk_match else 30
        
        # Optional speeds
        fly_match = CreaturePatterns.SPEED_FLY.search(speed_str)
        if fly_match:
            speeds["fly"] = int(fly_match.group(1))
        
        swim_match = CreaturePatterns.SPEED_SWIM.search(speed_str)
        if swim_match:
            speeds["swim"] = int(swim_match.group(1))
        
        climb_match = CreaturePatterns.SPEED_CLIMB.search(speed_str)
        if climb_match:
            speeds["climb"] = int(climb_match.group(1))
        
        return speeds
    
    def _parse_cr(self, cr_str: str) -> float:
        """Parse challenge rating to numeric value."""
        
        # Check for XP format first
        xp_match = CreaturePatterns.CR_WITH_XP.search(cr_str)
        if xp_match:
            return float(xp_match.group(1))
        
        # Check for fraction
        fraction_match = CreaturePatterns.CR_FRACTION.search(cr_str)
        if fraction_match:
            return float(fraction_match.group(1)) / float(fraction_match.group(2))
        
        # Simple number
        simple_match = CreaturePatterns.CR_SIMPLE.search(cr_str)
        if simple_match:
            return float(simple_match.group(1))
        
        return 0.0
    
    def _estimate_hp_from_dice(self, hp_dice: str) -> int:
        """Estimate HP from dice notation."""
        
        # Simple estimation: extract first number
        numbers = re.findall(r'\d+', hp_dice)
        if numbers:
            return int(numbers[0]) * 4  # Rough HP estimation
        return 4
    
    def _detect_settlement_context(self, content: str) -> str:
        """Detect what type of settlement context this NPC is in."""
        
        if SettlementPatterns.VILLAGE_INDICATORS.search(content):
            return "village"
        elif SettlementPatterns.TOWN_INDICATORS.search(content):
            return "town"
        elif SettlementPatterns.CITY_INDICATORS.search(content):
            return "city"
        return "wilderness"
    
    def _assess_material_tier(self, gp_amount: int, items: str) -> str:
        """Assess material tier for Dragon's Labyrinth progression."""
        
        if gp_amount > 1000 or "diamond" in items.lower():
            return "diamond"
        elif gp_amount > 500 or "gold" in items.lower():
            return "gold"
        elif gp_amount > 100 or "silver" in items.lower():
            return "stone"
        else:
            return "wood"
    
    def _determine_forge_type(self, content: str, sentimental_score: float) -> str | None:
        """Determine forge reagent type if applicable."""
        
        if sentimental_score < 0.5:
            return None
        
        # Check forge path alignment
        if ItemPatterns.FORGE_PATH_INDICATORS["light"].search(content):
            return "light_reagent"
        elif ItemPatterns.FORGE_PATH_INDICATORS["dark"].search(content):
            return "dark_reagent"
        else:
            return "neutral_reagent"
    
    def get_routing_stats(self) -> dict[str, int]:
        """Get routing statistics for analysis."""
        return self.routing_stats.copy()


# ============================================================================
# CSV GENERATION FOR ANALYSIS - Generate analysis CSVs like hex_tiles_full.csv
# ============================================================================

def generate_analysis_csv(entities: list[dict[str, Any]], target_table: str) -> str:
    """
    Generate analysis CSV for specific table to assess ML accuracy.
    
    Args:
        entities: List of extracted entities for the table
        target_table: Table name (hex_tiles, creatures, npcs, etc.)
        
    Returns:
        CSV content as string
    """
    
    if not entities:
        return f"# No {target_table} entities extracted\n"
    
    # Get column headers from first entity
    headers = list(entities[0].keys())
    
    # Generate CSV
    csv_lines = [",".join(headers)]
    
    for entity in entities:
        row = []
        for header in headers:
            value = entity.get(header, "")
            # Escape commas and quotes
            if isinstance(value, (list, dict)):
                value = json.dumps(value).replace('"', '""')
            else:
                value = str(value).replace('"', '""')
            
            if ',' in value or '"' in value:
                value = f'"{value}"'
            row.append(value)
        
        csv_lines.append(",".join(row))
    
    return "\n".join(csv_lines)
