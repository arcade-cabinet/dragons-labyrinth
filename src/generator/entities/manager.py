"""
Entities Manager - Dual-mode orchestrator for entity extraction and generation.

Coordinates transformer → specialized processors → output generation.
Supports both library usage and CLI commands via Typer.
"""

from __future__ import annotations

import logging
import typer
from pathlib import Path
from typing import Any

from rich.console import Console
from rich.logging import RichHandler

from sqlmodel import create_engine, Session

from generator.constants import GAME_DB_PATH, HBF_RAW_PATH
from generator.entities.transformer import EntityTransformer, transform_hbf_to_clusters, extract_world_hooks_for_pandora, route_to_specialized_processor
from generator.entities.models import create_all_tables, get_table_stats
from generator.entities.integration import get_simple_statistics


class EntitiesManager:
    """Manager for entity extraction using new transformer/processor architecture."""
    
    def __init__(self, hbf_db_path: str | None = None):
        """Initialize entities manager.
        
        Args:
            hbf_db_path: Path to HBF SQLite database
        """
        self.hbf_db_path = hbf_db_path or str(HBF_RAW_PATH)
        self.transformer = EntityTransformer(self.hbf_db_path)
    
    def run(self) -> dict[str, Any]:
        """
        Main entry point - extract and cluster entities, then process with specialized processors.
        
        Returns:
            Complete processing results
        """
        
        print("🚀 Starting entity extraction and processing pipeline")
        
        # Set up logger and console for processors
        console = Console()
        logging.basicConfig(level=logging.INFO, format="%(message)s", handlers=[RichHandler(console=console)])
        logger = logging.getLogger("entities")
        
        # Step 1: Transform and cluster entities
        clusters = self.transformer.extract_and_cluster_entities()
        
        # Step 2: Process each cluster with specialized processors
        processing_results = {}
        for cluster_key, cluster in clusters.items():
            if cluster.get_entity_count() > 0:
                print(f"🔍 Processing {cluster.name} ({cluster.get_entity_count()} entities)")
                
                result = route_to_specialized_processor(cluster, logger, console)
                processing_results[cluster_key] = result
        
        # Step 3: Extract world hooks for Pandora
        world_hooks = extract_world_hooks_for_pandora(clusters)
        
        return {
            "clusters": {k: {"name": v.name, "category": v.category, "count": v.get_entity_count()} 
                        for k, v in clusters.items()},
            "processing_results": processing_results,
            "world_hooks": world_hooks,
            "pipeline_stats": self._calculate_pipeline_stats(clusters, processing_results)
        }
    
    def _calculate_pipeline_stats(self, clusters: dict[str, Any], results: dict[str, Any]) -> dict[str, Any]:
        """Calculate pipeline statistics."""
        
        total_entities = sum(cluster.get_entity_count() for cluster in clusters.values())
        successful_processes = len([r for r in results.values() if "error" not in r])
        
        return {
            "total_entities_clustered": total_entities,
            "total_clusters": len(clusters),
            "non_empty_clusters": len([c for c in clusters.values() if c.get_entity_count() > 0]),
            "successful_processes": successful_processes,
            "failed_processes": len(results) - successful_processes
        }
    
    def export_clusters(self, output_dir: str = "clusters") -> dict[str, str]:
        """Export clusters to JSON files."""
        return self.transformer.export_clusters_to_json(output_dir)
    
    def get_entities_for_processor(self, processor_type: str) -> list[dict[str, Any]]:
        """Get entities for specific processor type."""
        return self.transformer.get_entities_for_processor(processor_type)


# Typer CLI Application
app = typer.Typer(
    name="entities",
    help="Dragon's Labyrinth Entities - Extract, cluster, and process game entities",
    no_args_is_help=True
)


@app.command("extract")
def cli_extract(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database"),
    output: str = typer.Option("clusters", help="Output directory for clusters")
) -> None:
    """Extract and cluster entities from HBF database."""
    
    manager = EntitiesManager(hbf)
    results = manager.run()
    
    # Export clusters
    exported_files = manager.export_clusters(output)
    
    print("\n✅ Entity extraction complete!")
    print(f"📊 Pipeline Stats: {results['pipeline_stats']}")
    print(f"📁 Exported {len(exported_files)} cluster files to {output}/")


@app.command("transform")
def cli_transform(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database"),
    output: str = typer.Option("clusters", help="Output directory for clusters")
) -> None:
    """Transform HBF entities into categorized clusters."""
    
    results = transform_hbf_to_clusters(hbf, output)
    
    print("✅ Transformation complete!")
    print(f"📊 Category counts: {results['category_counts']}")
    print(f"🔄 Processor routing: {results['processor_routing']}")


@app.command("export-hooks")
def cli_export_hooks(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database"),
    output: str = typer.Option("pandora/world_hooks", help="Output directory for world hooks")
) -> None:
    """Export world_hooks JSON for Pandora addon."""
    
    # Create transformer and extract clusters
    transformer = EntityTransformer(hbf)
    clusters = transformer.extract_and_cluster_entities()
    
    # Extract world hooks
    world_hooks = extract_world_hooks_for_pandora(clusters)
    
    # Write world hooks to files
    output_path = Path(output)
    output_path.mkdir(parents=True, exist_ok=True)
    
    import json
    for category, data in world_hooks.items():
        if data:  # Only write non-empty categories
            file_path = output_path / f"{category}.json"
            with open(file_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
    
    # Write summary
    summary_path = output_path / "summary.json"
    summary = {
        "categories": list(world_hooks.keys()),
        "counts": {cat: len(data) for cat, data in world_hooks.items()},
        "total_items": sum(len(data) for data in world_hooks.values())
    }
    
    with open(summary_path, "w", encoding="utf-8") as f:
        json.dump(summary, f, indent=2, ensure_ascii=False)
    
    print(f"✅ Exported world hooks to {output}/")
    print(f"📊 Summary: {summary}")


@app.command("gen-images")
def cli_gen_images(
    kind: str = typer.Argument(..., help="Image type: biomes|tokens|body-bases|data-driven"),
    output: str = typer.Option("art", help="Output directory for images"),
    size: str = typer.Option("1024x1024", help="Image size (e.g. 1024x1024)"),
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database (for data-driven)")
) -> None:
    """Generate sprite sheets using image generator."""
    
    try:
        from generator.entities.image_generator import (
            generate_biome_spritesheet, generate_token_sprites, generate_body_bases,
            generate_all_data_driven_sprites
        )
    except ImportError:
        print("❌ Image generator not available. Install OpenAI and jinja2 dependencies")
        raise typer.Exit(1)
    
    output_dir = Path(output)
    output_dir.mkdir(parents=True, exist_ok=True)
    
    if kind == "biomes":
        result_path = generate_biome_spritesheet(output_dir, size=size)
        print(f"✅ Generated biome spritesheet: {result_path}")
    elif kind == "tokens":
        result_path = generate_token_sprites(output_dir, size=size)
        print(f"✅ Generated token sprites: {result_path}")
    elif kind == "body-bases":
        result_path = generate_body_bases(output_dir, size=size)
        print(f"✅ Generated body bases: {result_path}")
    elif kind == "data-driven":
        print("🎨 Running complete data-driven sprite generation...")
        
        # Run transformer and processors
        manager = EntitiesManager(hbf)
        results = manager.run()
        
        # Extract processed results by category
        processing_results = results.get("processing_results", {})
        
        region_results = [r for r in processing_results.values() if r.get("processor_type") == "regions"]
        settlement_results = [r for r in processing_results.values() if r.get("processor_type") == "settlements"]
        faction_results = [r for r in processing_results.values() if r.get("processor_type") == "factions"]
        dungeon_results = [r for r in processing_results.values() if r.get("processor_type") == "dungeons"]
        
        # Generate all data-driven sprites
        sprite_results = generate_all_data_driven_sprites(
            region_results, settlement_results, faction_results, dungeon_results, output_dir
        )
        
        print(f"✅ Generated data-driven sprites:")
        for sprite_type, files in sprite_results.items():
            print(f"   {sprite_type}: {len(files)} files")
        
    else:
        print(f"❌ Unknown image type: {kind}")
        print("Available types: biomes, tokens, body-bases, data-driven")
        raise typer.Exit(1)


@app.command("godot-build")
def cli_godot_build(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database"),
    output: str = typer.Option("pandora", help="Output directory for Godot data")
) -> None:
    """Build complete Godot data package (world hooks + Pandora collections)."""
    
    # Extract clusters and world hooks
    transformer = EntityTransformer(hbf)
    clusters = transformer.extract_and_cluster_entities()
    world_hooks = extract_world_hooks_for_pandora(clusters)
    
    # Write Pandora collections
    output_path = Path(output)
    collections_path = output_path / "collections"
    collections_path.mkdir(parents=True, exist_ok=True)
    
    import json
    
    # Write collections for Pandora addon
    for category, items in world_hooks.items():
        if items:
            collection_data = {
                "collection": category.title(),
                "items": items,
                "metadata": {
                    "source": "Dragon's Labyrinth HBF Analysis",
                    "world": "The Lands of Vo'il",
                    "generated_by": "entities.manager"
                }
            }
            
            file_path = collections_path / f"{category.title()}.json"
            with open(file_path, "w", encoding="utf-8") as f:
                json.dump(collection_data, f, indent=2, ensure_ascii=False)
    
    # Write world hooks for direct Godot access
    hooks_path = output_path / "world_hooks"
    hooks_path.mkdir(parents=True, exist_ok=True)
    
    for category, data in world_hooks.items():
        if data:
            file_path = hooks_path / f"{category}.json"
            with open(file_path, "w", encoding="utf-8") as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
    
    # Write manifest
    manifest = {
        "name": "Dragon's Labyrinth World Data",
        "collections": list(world_hooks.keys()),
        "addons_required": [
            "hexagon_tilemaplayer",
            "godot-sqlite", 
            "pandora",
            "dialogic"
        ],
        "total_items": sum(len(data) for data in world_hooks.values())
    }
    
    manifest_path = output_path / "manifest.json"
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2, ensure_ascii=False)
    
    print(f"✅ Built complete Godot data package: {output}/")
    print(f"📦 Pandora collections: {collections_path}/")
    print(f"🎮 World hooks: {hooks_path}/")
    print(f"📋 Manifest: {manifest}")


@app.command("test-pipeline")
def cli_test_pipeline(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database")
) -> None:
    """Test the complete extraction and processing pipeline."""
    
    print("🧪 Testing complete entities pipeline...")
    
    manager = EntitiesManager(hbf)
    results = manager.run()
    
    print("\n✅ Pipeline test successful!")
    print(f"📊 Stats: {results['pipeline_stats']}")
    
    # Test clustering
    non_empty_clusters = len([c for c in results['clusters'].values() if c['count'] > 0])
    print(f"📦 Clusters: {non_empty_clusters} non-empty clusters")
    
    # Test world hooks
    hooks_categories = len([cat for cat, data in results['world_hooks'].items() if data])
    print(f"🌍 World hooks: {hooks_categories} categories with data")


@app.command("init-db")
def cli_init_db() -> None:
    """Initialize the simple 5-table database schema."""
    
    print("🗄️ Initializing simple 5-table database schema...")
    
    engine = create_engine(f"sqlite:///{GAME_DB_PATH}")
    create_all_tables(engine)
    
    print("✅ Database initialized with 5 simple tables:")
    print("   - hex_tiles (spatial data for hexagon_tilemaplayer)")
    print("   - entities (all entity data from ML processing)")
    print("   - companions (companion psychology data)")
    print("   - encounters (encounter data)")
    print("   - assets (asset references)")
    print(f"📍 Database: {GAME_DB_PATH}")


@app.command("db-stats")
def cli_db_stats() -> None:
    """Show statistics for the simple 5-table database."""
    
    engine = create_engine(f"sqlite:///{GAME_DB_PATH}")
    
    try:
        with Session(engine) as session:
            stats = get_simple_statistics(session)
            
            print("📊 Simple 5-Table Database Statistics:")
            print(f"   Hex Tiles: {stats['hex_tiles']}")
            print(f"   Entities: {stats['entities']}")
            print(f"   Companions: {stats['companions']}")
            print(f"   Encounters: {stats['encounters']}")
            print(f"   Assets: {stats['assets']}")
            print(f"   Entities with coordinates: {stats['entities_with_coordinates']}")
            print(f"   Database schema: {stats['database_schema']}")
            print(f"   Godot integration ready: {stats['godot_integration_ready']}")
            print(f"   Hexagon tilemap compatible: {stats['hexagon_tilemaplayer_compatible']}")
            print(f"   SQLite addon compatible: {stats['godot_sqlite_compatible']}")
    
    except Exception as e:
        print(f"❌ Error reading database: {str(e)}")
        print("💡 Try running 'init-db' first to create the database schema")


@app.command("consolidate")
def cli_consolidate(
    hbf: str = typer.Option(str(HBF_RAW_PATH), help="Path to HBF SQLite database")
) -> None:
    """Run complete architectural consolidation: extract → process → populate 5 tables."""
    
    print("🔄 Running complete architectural consolidation...")
    print("   Phase 1: Initialize simple 5-table database")
    
    # Initialize database
    engine = create_engine(f"sqlite:///{GAME_DB_PATH}")
    create_all_tables(engine)
    
    print("   Phase 2: Extract and process entities")
    
    # Run entity processing
    manager = EntitiesManager(hbf)
    results = manager.run()
    
    print("   Phase 3: Database population complete")
    
    # Show results
    with Session(engine) as session:
        stats = get_simple_statistics(session)
        
        print("\n✅ Architectural consolidation complete!")
        print(f"📊 Pipeline Stats: {results['pipeline_stats']}")
        print(f"📊 Database Stats:")
        print(f"   Hex Tiles: {stats['hex_tiles']}")
        print(f"   Entities: {stats['entities']}")
        print(f"   Companions: {stats['companions']}")
        print(f"   Encounters: {stats['encounters']}")
        print(f"   Assets: {stats['assets']}")
        print(f"🎮 Ready for Godot integration via godot-sqlite addon")


def create_entities_manager(hbf_db_path: str | None = None) -> EntitiesManager:
    """Factory function for backward compatibility."""
    return EntitiesManager(hbf_db_path)


if __name__ == "__main__":
    app()
