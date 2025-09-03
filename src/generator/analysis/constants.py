"""
Constants for entities processing system.
"""

from pathlib import Path


HBF_RAW_FILE = Path("raw/game.hbf")
ANALYSIS_OUTPUT_DIR = Path("analysis")

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

# Processor settings
# Use a broadly available latest model; override via code when gpt-5.* is available
DEFAULT_MODEL = "gpt-4.1"
MAX_ENTITIES_PER_BATCH = 100
CONFIDENCE_THRESHOLD = 0.5

# OpenAI model generation settings
HTML_ENTITIES_SAMPLE_THRESHOLD = 5
JSON_ENTITIES_SAMPLE_THRESHOLD = 5

# Output directories for processor models
PROCESSOR_MODELS_DIR = "src/generator/processors/models"
