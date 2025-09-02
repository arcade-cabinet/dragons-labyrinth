"""
Constants for processors - analysis patterns only, no hardcoded classifications.
"""

from pathlib import Path

# Analysis patterns for processors to use (NOT for entity splitting)
ESTABLISHMENT_KEYWORDS = ["tavern", "inn", "shop", "market", "temple", "smithy", "guild", "hall"]

SERVICE_TYPES = {
    "lodging": ["tavern", "inn"],
    "commerce": ["shop", "market"],
    "crafting": ["smith", "forge"],
    "religious": ["temple", "shrine"],
    "defense": ["guard", "militia"],
    "healing": ["healer", "medicine"]
}

# Analysis patterns for processors (NOT splitting rules)
FACTION_TYPE_PATTERNS = {
    "cult": ["defiled", "corrupted", "dark", "shadow", "unholy"],
    "militia": ["justice", "fists", "swords", "order", "guard"],
    "syndicate": ["snakes", "wolves", "wyverns", "gang", "raiders"],
    "guild": ["guild", "company", "merchants", "trade"]
}

HOSTILITY_PATTERNS = {
    "hostile": ["defiled", "corrupted", "dark", "evil", "violence"],
    "lawful": ["justice", "protection", "guard", "order"],
    "aggressive": ["snakes", "wolves", "raiders", "eliminate"]
}

ALIGNMENT_PATTERNS = {
    "lawful": ["justice", "order", "protection", "guard", "law"],
    "chaotic": ["defiled", "corrupted", "chaos", "shadow", "dark"],
    "neutral": ["neutral", "balance", "trade", "commerce"]
}

DUNGEON_TYPE_PATTERNS = {
    "crypt": ["crypt", "tomb", "mausoleum", "burial"],
    "cave": ["cave", "cavern", "grotto", "underground"],
    "temple": ["temple", "shrine", "sanctuary", "altar"],
    "lair": ["lair", "den", "nest", "hideout"],
    "pit": ["bowel", "pit", "abyss", "chasm"]
}

# Processing thresholds
THREAT_THRESHOLDS = {
    "enemy_high": 10,
    "enemy_moderate": 5,
    "trap_high": 5,
    "max_threat": 5
}

TREASURE_THRESHOLDS = {
    "mentions_high": 5,
    "mentions_moderate": 2,
    "currency_high": 10,
    "currency_moderate": 5
}

COMPLEXITY_THRESHOLDS = {
    "very_high": 10,
    "high": 5,
    "moderate": 2
}

ECONOMIC_THRESHOLDS = {
    "very_high": 15,
    "high": 10,
    "moderate": 5,
    "low": 1,
    "none": 0
}

# Analysis output directory
PROCESSED_OUTPUT_DIR = Path("src/generator/processed")
