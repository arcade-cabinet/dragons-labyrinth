"""
Entities System - New transformer-based architecture for Dragon's Labyrinth.

Coordinates transformer → specialized processors → world_hooks generation.
Completely self-contained system independent of old comprehensive database.
"""

# Export key components for the new architecture
from generator.entities.manager import EntitiesManager, create_entities_manager
from generator.entities.transformer import EntityTransformer, EntityCluster


__all__ = [
    "EntitiesManager",
    "create_entities_manager", 
    "EntityTransformer",
    "EntityCluster"
]
