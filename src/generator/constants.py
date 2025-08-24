"""
Dragon's Labyrinth Generator Constants

Configuration for AI-powered asset generation with BPY + Bevy
"""

from pathlib import Path

# Game Identity
GAME_NAME = "Dragon's Labyrinth"
GAME_VERSION = "1.0.0"

# Project Structure
PROJECT_ROOT = Path(__file__).parent.parent.parent
GENERATOR_DIR = PROJECT_ROOT / "src" / "generator"
ASSETS_DIR = PROJECT_ROOT / "assets"
LIBRARY_DIR = ASSETS_DIR / "library"
GENERATED_DIR = ASSETS_DIR / "generated"

# Database
ASSETS_DB_PATH = ASSETS_DIR / "assets.db"

# Horror Progression (Dread Levels 0-4)
DREAD_LEVELS = {
    0: {"name": "Peace", "description": "Beautiful morning, birds singing, warm sunlight"},
    1: {"name": "Unease", "description": "Something feels off, shadows too long, colors muted"},
    2: {"name": "Dread", "description": "Visible decay, darkness spreading, hope fading"},
    3: {"name": "Terror", "description": "Active malevolence, reality distorting, companions breaking"},
    4: {"name": "Horror", "description": "Complete nightmare, first-person stalking, reality shattered"}
}

# AI Configuration
DEFAULT_AI_MODEL = "gpt-4o"
DEFAULT_TEMPERATURE = 0.7
MAX_TOKENS = 4096

# Image Generation (DALL-E)
DEFAULT_IMAGE_MODEL = "dall-e-3"
DEFAULT_IMAGE_SIZE = "1024x1024"
DEFAULT_IMAGE_QUALITY = "hd"
DEFAULT_IMAGE_COUNT = 1

# Asset Categories
ASSET_CATEGORIES = {
    'models': {'.glb', '.gltf'},  # Bevy 3D models
    'textures': {'.png', '.jpg', '.jpeg', '.tga', '.bmp'},
    'audio': {'.ogg', '.mp3', '.wav', '.flac'},
    'fonts': {'.ttf', '.otf'},
}

# Tile System
TILE_TYPES = [
    "grassland", "forest", "mountain", "swamp", 
    "village", "dungeon", "ruins", "corruption"
]

DEFAULT_HEX_RADIUS = 1.0
DEFAULT_TILE_HEIGHT = 0.3

# BPY Export Settings
BPY_EXPORT_FORMAT = 'GLB'
BPY_EXPORT_SETTINGS = {
    'export_format': 'GLB',
    'use_selection': True,
    'export_apply': True,
    'export_texcoords': True,
    'export_normals': True,
    'export_materials': 'EXPORT',
    'export_cameras': False,
    'export_lights': False,
    'export_animations': False,
    'export_yup': True,
    'export_extras': False,
    'export_image_format': 'AUTO'
}

# Performance Targets
MAX_VERTEX_COUNT = 100000  # Per asset
MAX_TEXTURE_SIZE = 2048    # Pixels
TARGET_FPS = 60
MOBILE_TARGET_FPS = 30

# Horror-Relevant Keywords for CC0 Filtering
RELEVANT_KEYWORDS = {
    # Horror/Dark themes
    'zombie', 'monster', 'skeleton', 'ghost', 'demon', 'evil', 'dark', 
    'horror', 'creepy', 'nightmare', 'dead', 'undead', 'cursed',
    
    # Medieval/Fantasy
    'medieval', 'castle', 'dungeon', 'knight', 'sword', 'shield', 'armor',
    'dragon', 'goblin', 'orc', 'troll', 'wizard', 'magic', 'spell',
    'village', 'tavern', 'blacksmith', 'church', 'ruins', 'tower',
    
    # RPG Elements
    'character', 'hero', 'warrior', 'rogue', 'mage', 'priest',
    'weapon', 'potion', 'chest', 'treasure', 'coin', 'key',
    
    # Nature/Environment
    'forest', 'swamp', 'mountain', 'cave', 'river', 'bridge',
    'tree', 'rock', 'grass', 'path', 'road',
    
    # Building/Architecture
    'wall', 'door', 'window', 'roof', 'floor', 'stairs',
    'furniture', 'table', 'chair', 'bed', 'barrel', 'crate'
}

# Exclude modern/sci-fi assets
EXCLUDE_KEYWORDS = {
    'sci-fi', 'scifi', 'space', 'spaceship', 'robot', 'cyber', 'futuristic',
    'car', 'vehicle', 'gun', 'rifle', 'pistol', 'modern', 'city',
    'computer', 'tech', 'electronic', 'neon', 'laser'
}

# Horror Progression
HORROR_TRANSITIONS = {
    "peace": "You open your front door to a beautiful morning",
    "unease": "The birds have stopped singing", 
    "dread": "NPCs lock their doors when you approach",
    "terror": "Your companions beg to turn back",
    "horror": "You stand before the Dragon's Labyrinth"
}

# Companions
COMPANIONS = {
    "einar": {"role": "loyal_friend", "trauma_progression": True},
    "mira": {"role": "optimist", "departure_stage": 2},
    "sorin": {"role": "scholar", "betrayal_potential": True},
    "tamara": {"role": "baker_apprentice", "innocence_loss": True}
}

# Logging
LOG_FORMAT = "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
LOGGER_NAME = "dragons_labyrinth_generator"
