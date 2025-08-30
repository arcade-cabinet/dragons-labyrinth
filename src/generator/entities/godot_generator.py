"""
Godot Generator - Exports world data for Godot engine integration.

Exports world hooks and Pandora collections for seamless integration with:
- Pandora addon (RPG data management)
- Hexagon TileMapLayer addon (hex grid system)
- Godot-SQLite addon (database integration)
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from generator.entities.transformer import EntityTransformer, extract_world_hooks_for_pandora


def export_world_hooks_json(output_dir: Path, hbf_db_path: str | None = None) -> dict[str, str]:
    """
    Export world hooks JSON for Godot integration.
    
    Args:
        output_dir: Directory to save world hooks JSON files
        hbf_db_path: Path to HBF database (uses default if None)
        
    Returns:
        Dictionary mapping category names to exported file paths
    """
    
    from generator.constants import HBF_RAW_PATH
    
    db_path = hbf_db_path or str(HBF_RAW_PATH)
    
    print("🌍 Exporting world hooks for Godot integration...")
    
    # Create transformer and extract data
    transformer = EntityTransformer(db_path)
    clusters = transformer.extract_and_cluster_entities()
    world_hooks = extract_world_hooks_for_pandora(clusters)
    
    # Ensure output directory exists
    output_dir.mkdir(parents=True, exist_ok=True)
    
    exported_files = {}
    
    # Export each category
    for category, data in world_hooks.items():
        if data:  # Only export non-empty categories
            file_path = output_dir / f"{category}.json"
            
            with open(file_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            
            exported_files[category] = str(file_path)
            print(f"  ✅ Exported {len(data)} {category} items")
    
    # Export index for easy iteration
    index_data = {
        "categories": list(exported_files.keys()),
        "files": exported_files,
        "counts": {cat: len(world_hooks[cat]) for cat in exported_files.keys()},
        "total_items": sum(len(world_hooks[cat]) for cat in exported_files.keys())
    }
    
    index_path = output_dir / "index.json"
    with open(index_path, "w", encoding="utf-8") as f:
        json.dump(index_data, f, indent=2, ensure_ascii=False)
    
    exported_files["index"] = str(index_path)
    
    print(f"📋 Exported index: {index_path}")
    print(f"🎮 Total exported: {index_data['total_items']} world hook items")
    
    return exported_files


def export_pandora_collections(output_dir: Path, hbf_db_path: str | None = None) -> dict[str, str]:
    """
    Export Pandora collections for RPG data management.
    
    Args:
        output_dir: Directory to save Pandora collection files
        hbf_db_path: Path to HBF database (uses default if None)
        
    Returns:
        Dictionary mapping collection names to exported file paths
    """
    
    from generator.constants import HBF_RAW_PATH
    
    db_path = hbf_db_path or str(HBF_RAW_PATH)
    
    print("📦 Exporting Pandora collections...")
    
    # Create transformer and extract data
    transformer = EntityTransformer(db_path)
    clusters = transformer.extract_and_cluster_entities()
    world_hooks = extract_world_hooks_for_pandora(clusters)
    
    # Ensure collections directory exists
    collections_dir = output_dir / "collections"
    collections_dir.mkdir(parents=True, exist_ok=True)
    
    exported_files = {}
    
    # Export each collection in Pandora format
    for category, items in world_hooks.items():
        if items:
            # Convert to Pandora collection format
            collection_data = {
                "collection": category.title(),
                "items": _convert_to_pandora_items(items, category),
                "metadata": {
                    "source": "Dragon's Labyrinth HBF Analysis",
                    "world_name": "The Lands of Vo'il",
                    "category": category,
                    "generated_by": "entities.godot_generator",
                    "item_count": len(items)
                }
            }
            
