"""
Essential generator constants.

Minimal constants following .clinerules patterns.
"""

from pathlib import Path

# Essential database path
GAME_DB_PATH = Path("metadata/game.db")

# Essential HBF path  
HBF_RAW_PATH = Path("inputs/raw_game.hbf")

# Entity classification lists (used by transformer and processors)
REGIONS = [
    "Aurora Bushes", "Black Shield Timberlands", "Blood Blade Fields",
    "Bonecrusher Plains", "Darkfall Dunes", "Darkfall Plains", "Fallen Star Steppe",
    "Fearless Wilds", "Firefly Cliffs", "Goblinchaser Jungle", "Goblinchaser Wilderness",
    "Goldenswan Timberlands", "Goldseeker's Cliffs", "Grey Mist Snowlands",
    "Heartseeker Forest", "Heartseeker Moors", "Hell's Gate Desert",
    "Holloweye Wilderness", "Iceborn Wilderness", "Javelin Plains", "Javelin Wetlands",
    "Moonwatcher Wetlands", "Nightmare Desert", "Ragthorn Meadows", "Ragthorn Woods",
    "Thunderwave Woodlands", "Vicious Crags"
]

SETTLEMENTS = [
    "Village of Ashamar", "Village of Balaal", "Town of Devilville",
    "Village of Dokar", "Village of Dorith", "Village of Harad",
    "Village of Headbone", "City of Headsmen", "Village of Kothian",
    "City of Palemoon"
]

FACTIONS = [
    "The Defiled Wolves", "The Fists Of Justice", "The Red Snakes",
    "The Swords Of Justice", "The White Wyverns"
]

DUNGEONS = [
    "Bowel of the Raging Pits", "Caverns of the Burning Souls",
    "Caverns of the Infernal Lich", "Crypt of the Corrupted Order",
    "Crypt of the Infernal Blades", "Crypt of the Mourning Goblin",
    "Crypt of the Unholy Goblin", "Crypt of the Violent Ogre",
    "Hideout of the Corrupted Order", "Hideout of the Unspoken Desire",
    "Lair of the Foresaken Desire", "Lair of the Mourning Hopes",
    "Shrine of the Infernal Blades", "Shrine of the Infernal Desire",
    "Temple of the Violent Ogre", "Tomb of the Cursed Pits",
    "Tomb of the Grey Ogre", "Tomb of the Unspoken Skeletons"
]

BIOMES = ["Desert", "Forest", "Jungle", "Mountains", "Plains", "Swamps", "Tundra"]

# Processing configuration data
BIOME_KEYWORDS = {
    "Forest": ["forest", "tree", "wood", "grove", "timber"],
    "Desert": ["desert", "sand", "dune", "arid", "wasteland"],
    "Mountain": ["mountain", "hill", "peak", "crag", "cliff"],
    "Plains": ["plain", "field", "grass", "meadow", "prairie"],
    "Swamp": ["swamp", "bog", "marsh", "wetland", "mire"],
    "Tundra": ["tundra", "frozen", "ice", "snow", "arctic"],
    "Jungle": ["jungle", "wild", "vine", "dense", "tropical"]
}

CORRUPTION_THEMES = {
    4: ["dark", "blood", "bone", "nightmare", "hell", "vicious", "infernal"],
    3: ["fallen", "grey", "shadow", "fear", "skull", "cursed"],
    2: ["black", "death", "haunted", "violent"],
    1: ["heart", "moon", "gold", "thunder"],
    0: ["golden", "silver", "bright", "light", "aurora"]
}

ESTABLISHMENT_KEYWORDS = ["tavern", "inn", "shop", "market", "temple", "smithy", "guild", "hall"]

SERVICE_TYPES = {
    "lodging": ["tavern", "inn"],
    "commerce": ["shop", "market"],
    "crafting": ["smith", "forge"],
    "religious": ["temple", "shrine"],
    "defense": ["guard", "militia"],
    "healing": ["healer", "medicine"]
}

# Faction type classification patterns
FACTION_TYPE_PATTERNS = {
    "cult": ["defiled", "corrupted", "dark", "shadow", "unholy"],
    "militia": ["justice", "fists", "swords", "order", "guard"],
    "syndicate": ["snakes", "wolves", "wyverns", "gang", "raiders"],
    "guild": ["guild", "company", "merchants", "trade"]
}

# Faction hostility patterns
HOSTILITY_PATTERNS = {
    "hostile": ["defiled", "corrupted", "dark", "evil", "violence"],
    "lawful": ["justice", "protection", "guard", "order"],
    "aggressive": ["snakes", "wolves", "raiders", "eliminate"]
}

# Political alignment patterns
ALIGNMENT_PATTERNS = {
    "lawful": ["justice", "order", "protection", "guard", "law"],
    "chaotic": ["defiled", "corrupted", "chaos", "shadow", "dark"],
    "neutral": ["neutral", "balance", "trade", "commerce"]
}

# Dungeon type patterns
DUNGEON_TYPE_PATTERNS = {
    "crypt": ["crypt", "tomb", "mausoleum", "burial"],
    "cave": ["cave", "cavern", "grotto", "underground"],
    "temple": ["temple", "shrine", "sanctuary", "altar"],
    "lair": ["lair", "den", "nest", "hideout"],
    "pit": ["bowel", "pit", "abyss", "chasm"]
}

# Dungeon entrance mappings
ENTRANCE_MAPPINGS = {
    "crypt": "tomb_entrance",
    "cave": "cave_mouth",
    "temple": "temple_entrance", 
    "lair": "lair_entrance",
    "pit": "pit_entrance"
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

# Resistance factors for settlements
RESISTANCE_FACTORS = {
    "religious_bonus": 2,
    "economic_high": 3,
    "economic_moderate": 2,
    "economic_low": 1,
    "city_bonus": 3,
    "town_bonus": 2,
    "village_bonus": 1,
    "defense_bonus": 1
}

# Connectivity weights for regions
CONNECTIVITY_WEIGHTS = {
    "rivers": 0.3,
    "trails": 0.4,
    "harbors": 0.2,
    "borders": -0.1
}

# Threat calculation weights for factions
THREAT_WEIGHTS = {
    "hostile": 0.4,
    "aggressive": 0.3,
    "lawful": -0.1,
    "member_factor": 0.003,
    "territorial_widespread": 0.2,
    "territorial_regional": 0.1
}
