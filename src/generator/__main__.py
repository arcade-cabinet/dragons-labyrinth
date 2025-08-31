"""
Dragon's Labyrinth Generator

Simple orchestration of all subpackages to build game.db.
"""

# DEVELOPMENT SAFETY: Set to True to test imports without running pipeline
HEALTH_CHECK_ONLY = False

import logging
from pathlib import Path

from rich.console import Console
from rich.logging import RichHandler
from rich.table import Table
from sqlmodel import SQLModel, create_engine

from generator.constants import GAME_DB_PATH
from generator.statistics import RunStatistics
from generator.entities import run as run_entities
from generator.seeds import run as run_seeds
from generator.psychology import run as run_psychology
from generator.world import run as run_world
from generator.maps import run as run_maps
from generator.encounters import run as run_encounters
from generator.sprites import run as run_sprites
from generator.assets import run as run_assets


def setup_rich_logging() -> tuple[logging.Logger, Console]:
    """Set up Rich logging with proper suppression."""
    
    # Suppress framework noise (typer extends click so this catches both)
    logging.getLogger("typer").setLevel(logging.WARNING)
    
    # Set up Rich handler
    console = Console()
    rich_handler = RichHandler(
        console=console,
        rich_tracebacks=True,
        tracebacks_suppress=["typer"]
    )
    
    # Configure logging
    logging.basicConfig(
        level="INFO",
        format="%(message)s",
        datefmt="[%X]",
        handlers=[rich_handler]
    )
    
    logger = logging.getLogger("generator")
    return logger, console


def display_final_summary(console: Console, results: dict[str, RunStatistics]):
    """Display final summary table with Rich."""
    
    table = Table(title="🐉 Dragon's Labyrinth Generation Summary")
    table.add_column("Subpackage", style="cyan")
    table.add_column("Items", style="green")
    table.add_column("Duration", style="yellow")
    table.add_column("Success Rate", style="magenta")
    table.add_column("Status", style="bold")
    
    for name, stats in results.items():
        status = "✅ Success" if stats.success_rate > 0.8 else "⚠️ Issues"
        table.add_row(
            name.title(),
            f"{stats.items_stored}/{stats.items_processed}",
            f"{stats.duration:.1f}s",
            f"{stats.success_rate:.1%}",
            status
        )
    
    console.print(table)


def main() -> dict[str, RunStatistics]:
    """
    Build complete game.db by running all subpackages in sequence.
    
    Returns:
        Dictionary with RunStatistics from each subpackage
    """
    # Set up logging and console
    logger, console = setup_rich_logging()
    
    # DEVELOPMENT SAFETY: Health check mode for testing imports
    if HEALTH_CHECK_ONLY:
        logger.info("🔍 Health Check Mode - Testing imports only")
        logger.info("✅ Healthy - All imports successful")
        logger.info("💡 Set HEALTH_CHECK_ONLY = False to run full pipeline")
        return {}
    
    # Ensure metadata directory exists
    GAME_DB_PATH.parent.mkdir(parents=True, exist_ok=True)
    
    logger.info(f"Database generation starting: {GAME_DB_PATH}")
    logger.info("🚀 Starting complete database generation pipeline")
    
    # Create single database engine for all subpackages - EFFICIENT!
    engine = create_engine(f"sqlite:///{GAME_DB_PATH}")
    SQLModel.metadata.create_all(engine, checkfirst=True)
    logger.info("📊 Database engine created and tables initialized")
    
    results = {}
    
    # Phase 1: Foundation - Entities and Seeds
    logger.info("📊 Phase 1: Foundation data (entities + seeds)")
    
    logger.info("Extracting entities from HBF...")
    results['entities'] = run_entities(
        engine=engine,
        logger=logger,
        console=console
    )
    
    logger.info("Loading narrative seeds...")
    results['seeds'] = run_seeds(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 2: Psychology - Needs entities and seeds
    logger.info("🧠 Phase 2: Companion psychology")
    results['psychology'] = run_psychology(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 3: World - Needs all foundation data
    logger.info("🌍 Phase 3: World regions")
    results['world'] = run_world(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 4: Maps - Needs world and entities
    logger.info("🗺️ Phase 4: Hex maps")
    results['maps'] = run_maps(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 5: Encounters - Needs maps, entities, psychology
    logger.info("⚔️ Phase 5: Encounter scenarios")
    results['encounters'] = run_encounters(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 6: Sprites - Needs entities, psychology, world
    logger.info("👥 Phase 6: Character sprites")
    results['sprites'] = run_sprites(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Phase 7: Assets - Needs all previous data for context
    logger.info("🎨 Phase 7: Visual assets")
    results['assets'] = run_assets(
        engine=engine,
        logger=logger,
        console=console
    )
    
    # Display final summary
    display_final_summary(console, results)
    
    logger.info(f"✅ Database complete: {GAME_DB_PATH}")
    return results


if __name__ == "__main__":
    main()
