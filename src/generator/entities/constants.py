"""
Constants for entities processing system.
"""

from pathlib import Path

# Database paths
GAME_DB_PATH = Path("metadata/game.db")
HBF_RAW_PATH = Path("inputs/raw_game.hbf")

# Processor settings
DEFAULT_MODEL = "gpt-4o"
MAX_ENTITIES_PER_BATCH = 100
CONFIDENCE_THRESHOLD = 0.5

# Template settings
TEMPLATE_DIR = Path(__file__).parent / "prompt_templates"

# Output settings
DEFAULT_OUTPUT_DIR = Path("art")
DEFAULT_CLUSTERS_DIR = Path("clusters")
DEFAULT_PANDORA_DIR = Path("pandora")
