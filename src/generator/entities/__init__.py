"""
Entity extraction subpackage for Dragon's Labyrinth database.

Simple run() function replaces complex manager classes.
Modern Python standards: no Optional, Dict, List - use built-in generics.
"""

import shutil
import sqlite3
import time
from logging import Logger
from pathlib import Path

import pandas as pd
from rich.console import Console
from sqlmodel import Session, SQLModel, create_engine, select

from generator.constants import GAME_DB_PATH, HBF_RAW_PATH
from generator.statistics import RunStatistics
from generator.entities.orm import (
    Biome as BiomeRecord,
    Monster as MonsterRecord,
    Inn as InnRecord,
    Cave as CaveRecord,
    Temple as TempleRecord,
    Tomb as TombRecord,
    City as CityRecord,
    Town as TownRecord,
    Village as VillageRecord,
    FarmsCabins as FarmsCabinsRecord,
    Stronghold as StrongholdRecord,
    Cult as CultRecord,
    Militia as MilitiaRecord,
    Syndicate as SyndicateRecord
)
from generator.entities.processor import DragonLabyrinthMLProcessor


def run(
    db_path: str,
    logger: Logger,
    console: Console
) -> RunStatistics:
    """
    Extract entities from HBF database - simple function replaces EntitiesManager class.
    
    Args:
        db_path: Path to game database 
        logger: Rich logger instance
        console: Rich console for output
        
    Returns:
        RunStatistics with extraction results
    """
    start_time = time.time()
    items_processed = 0
    items_stored = 0
    
    logger.info("🔍 Extracting entities from HBF database...")
    
    # Create database engine
    engine = create_engine(f"sqlite:///{db_path}")
    SQLModel.metadata.create_all(engine, checkfirst=True)
    
    with Session(engine) as session:
        # Step 1: Setup raw HBF database
        hbf_path = Path("inputs/raw_game.hbf")
        raw_db_path = Path(HBF_RAW_PATH)
        
        if not hbf_path.exists():
            logger.error(f"❌ HBF file not found at {hbf_path}")
            return RunStatistics(
                subpackage="entities",
                items_processed=0,
                items_stored=0,
                duration=time.time() - start_time,
                success_rate=0.0,
                errors=["HBF file not found"]
            )
        
        # Copy HBF to raw database location if needed
        if not raw_db_path.exists() or raw_db_path.stat().st_size != hbf_path.stat().st_size:
            logger.info(f"📦 Setting up raw HBF database from {hbf_path}")
            raw_db_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(hbf_path, raw_db_path)
            logger.info(f"✅ Raw HBF database ready at {raw_db_path}")
        
        # Step 2: Extract entities from HBF using ML processor
        processor = DragonLabyrinthMLProcessor()
        
        # Connect to raw HBF database
        conn = sqlite3.connect(HBF_RAW_PATH)
        cursor = conn.cursor()
        
        entity_records = []
        
        # Try JSON entities first
        try:
            cursor.execute("SELECT uuid, json_data FROM EntityJSON LIMIT 1000")
            json_entities = cursor.fetchall()
            for uuid, json_data in json_entities:
                if json_data:
                    entity_records.append((uuid, json_data))
            logger.info(f"  Found {len(json_entities)} JSON entities")
        except sqlite3.Error:
            logger.info("  No EntityJSON table found")
        
        # Try HTML entities
        try:
            cursor.execute("SELECT uuid, html_content FROM EntityHTML LIMIT 1000")
            html_entities = cursor.fetchall()
            for uuid, html_content in html_entities:
                if html_content:
                    entity_records.append((uuid, html_content))
            logger.info(f"  Found {len(html_entities)} HTML entities")
        except sqlite3.Error:
            logger.info("  No EntityHTML table found")
        
        conn.close()
        items_processed = len(entity_records)
        
        if not entity_records:
            logger.warning("⚠️ No entities found in HBF database")
            
        # Step 3: Process entities with ML processor
        if entity_records:
            logger.info(f"🤖 Processing {len(entity_records)} entities with ML...")
            results = processor.process_entity_batch(entity_records)
            
            # Store processed entities
            items_stored = _store_processed_entities(session, results, logger)
        
        # Step 4: Load CSV supplements (hex tiles)
        csv_stored = _load_csv_supplements(session, logger)
        items_stored += csv_stored
    
    duration = time.time() - start_time
    success_rate = items_stored / items_processed if items_processed > 0 else 1.0
    
    logger.info(f"✅ Entities extraction complete: {items_stored}/{items_processed} stored in {duration:.1f}s")
    
    return RunStatistics(
        subpackage="entities", 
        items_processed=items_processed,
        items_stored=items_stored,
        duration=duration,
        success_rate=success_rate,
        errors=[]
    )


def _store_processed_entities(session: Session, results: dict[str, any], logger: Logger) -> int:
    """Store processed entities in appropriate tables."""
    
    # Map table names to ORM classes
    table_orm_map = {
        "biome": BiomeRecord,
        "monster": MonsterRecord,
        "inn": InnRecord,
        "cave": CaveRecord,
        "temple": TempleRecord,
        "tomb": TombRecord,
        "city": CityRecord,
        "town": TownRecord,
        "village": VillageRecord,
        "farms_cabins": FarmsCabinsRecord,
        "stronghold": StrongholdRecord,
        "cult": CultRecord,
        "militia": MilitiaRecord,
        "syndicate": SyndicateRecord
    }
    
    counts = {}
    
    for entity in results.get("entities", []):
        table = entity.get("target_table")
        if table in table_orm_map and entity.get("confidence", 0) > 0.5:
            # Create ORM record
            orm_class = table_orm_map[table]
            record_data = {
                "hbf_uuid": entity["uuid"],
                "extraction_confidence": entity["confidence"],
                **entity.get("extracted_data", {})
            }
            
            try:
                record = orm_class(**record_data)
                session.add(record)
                counts[table] = counts.get(table, 0) + 1
            except Exception as e:
                logger.warning(f"  ⚠️ Failed to store {table} entity: {e}")
    
    # Commit all entities
    try:
        session.commit()
        total_stored = sum(counts.values())
        for table, count in counts.items():
            logger.info(f"  ✓ Stored {count} {table} entities")
        return total_stored
    except Exception as e:
        session.rollback()
        logger.error(f"  ❌ Failed to commit entities: {e}")
        return 0


def _load_csv_supplements(session: Session, logger: Logger) -> int:
    """Load supplementary data from CSV files in hbf_analysis."""
    
    analysis_dir = Path("hbf_analysis")
    hex_tiles_path = analysis_dir / "hex_tiles_full.csv"
    
    if not hex_tiles_path.exists():
        logger.info("  No hex_tiles_full.csv found for supplement")
        return 0
    
    logger.info("📊 Loading hex tiles from CSV...")
    df = pd.read_csv(hex_tiles_path)
    
    count = 0
    for _, row in df.iterrows():
        try:
            biome = BiomeRecord(
                hbf_uuid=row.get("uuid", f"hex_{row.get('coordinate', 'unknown')}"),
                coordinate=row.get("coordinate", ""),
                biome_type=row.get("biome", "unknown"),
                region=row.get("region", ""),
                environmental_description=row.get("features", ""),
                extraction_confidence=0.95  # High confidence for CSV data
            )
            session.add(biome)
            count += 1
        except Exception as e:
            logger.warning(f"  ⚠️ Failed to load hex tile: {e}")
    
    try:
        session.commit()
        logger.info(f"  ✓ Loaded {count} biome records from CSV")
        return count
    except Exception as e:
        session.rollback()
        logger.error(f"  ❌ Failed to commit CSV data: {e}")
        return 0


# Export models for other subpackages
from generator.entities.orm import (
    Biome, Monster, Inn, Dungeon, Cave, Temple, Tomb,
    Settlement, City, Town, Village, FarmsCabins, Stronghold,
    Cult, Militia, Syndicate, NPC, Treasure, HTMLEntity, JSONEntity
)

__all__ = [
    "run",
    "Biome", "Monster", "Inn", "Dungeon", "Cave", "Temple", "Tomb",
    "Settlement", "City", "Town", "Village", "FarmsCabins", "Stronghold", 
    "Cult", "Militia", "Syndicate", "NPC", "Treasure", "HTMLEntity", "JSONEntity"
]
