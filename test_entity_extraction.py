#!/usr/bin/env python
"""
Test script for the unified entity extraction system.

Tests the ML-first extraction pipeline with real HBF data.
"""

import json
import logging
from pathlib import Path

from sqlmodel import create_engine, SQLModel, Session

from src.generator.db.entities.extractors import UnifiedEntityExtractor
from src.generator.db.entities.orm import (
    Biome, Monster, Inn, NPC, Treasure,
    Dungeon, Cave, Temple, Tomb,
    Settlement, City, Town, Village,
    FarmsCabins, Stronghold,
    Cult, Militia, Syndicate,
    HTMLEntity, JSONEntity
)
from src.generator.db.entities.errors import EntityExtractionError

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


def load_hbf_entities(hbf_path: Path) -> list[tuple[str, str]]:
    """Load entities from HBF file."""
    
    if not hbf_path.exists():
        raise FileNotFoundError(f"HBF file not found: {hbf_path}")
    
    entities = []
    
    # Simple HBF parsing - each entity starts with 8-char UUID
    with open(hbf_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Split by entity boundary (8 hex chars at start of line)
    import re
    entity_pattern = re.compile(r'^([a-f0-9]{8})\s', re.MULTILINE)
    
    matches = list(entity_pattern.finditer(content))
    
    for i, match in enumerate(matches):
        uuid = match.group(1)
        start = match.start()
        
        # Get content until next entity or end
        if i < len(matches) - 1:
            end = matches[i + 1].start()
        else:
            end = len(content)
        
        entity_content = content[start:end].strip()
        entities.append((uuid, entity_content))
    
    logger.info(f"Loaded {len(entities)} entities from {hbf_path}")
    return entities


def test_extraction():
    """Test the extraction pipeline."""
    
    # Load HBF entities
    hbf_path = Path("hbf_analysis/nTR8nJOW_clean.hbf")
    
    if not hbf_path.exists():
        logger.warning(f"HBF file not found at {hbf_path}, using sample data")
        # Sample entities for testing
        entities = [
            ("a1b2c3d4", "a1b2c3d4 Hex N1 in Fearless Wilds (The Lands of Vo'il) Watchtower >"),
            ("e5f6g7h8", "e5f6g7h8 Monster CR: 5 AC: 15 HP: (10d8+20) Speed: Walk 30 ft STR 18 +4 DEX 12 +1"),
            ("i9j0k1l2", "i9j0k1l2 Village of Harad in Ragthorn Woods with tavern and shops")
        ]
    else:
        entities = load_hbf_entities(hbf_path)[:10]  # Test with first 10
    
    # Initialize extractor
    logger.info("Initializing UnifiedEntityExtractor...")
    extractor = UnifiedEntityExtractor()
    
    # Extract entities
    logger.info(f"Extracting {len(entities)} entities...")
    results = extractor.extract_batch(entities)
    
    # Analyze results
    logger.info("\n=== EXTRACTION RESULTS ===")
    logger.info(f"Total processed: {results['extraction_stats']['total_entities']}")
    logger.info(f"ML processed: {results['extraction_stats']['ml_processed']}")
    logger.info(f"Pattern routed: {results['extraction_stats']['pattern_routed']}")
    
    logger.info("\nEntities by table:")
    for table, count in results['extraction_stats']['by_table'].items():
        logger.info(f"  {table}: {count}")
    
    # Show sample extractions
    logger.info("\n=== SAMPLE EXTRACTIONS ===")
    for entity in results['entities'][:5]:
        logger.info(f"\nUUID: {entity['uuid']}")
        logger.info(f"  Final table: {entity['final_table']}")
        logger.info(f"  Confidence: {entity['combined_confidence']:.2f}")
        logger.info(f"  ML table: {entity['ml_table']} ({entity['ml_confidence']:.2f})")
        logger.info(f"  Pattern table: {entity['pattern_table']} ({entity['pattern_confidence']:.2f})")
    
    # Show ML analysis
    if 'ml_analysis' in results:
        logger.info("\n=== ML ANALYSIS ===")
        logger.info(f"Relationships found: {len(results['ml_analysis']['relationships'])}")
        
        if results['ml_analysis']['cluster_analysis']:
            logger.info(f"Clusters: {results['ml_analysis']['cluster_analysis']}")
    
    return results


def test_database_creation():
    """Test creating the database with extracted entities."""
    
    logger.info("\n=== DATABASE CREATION TEST ===")
    
    # Create in-memory database for testing
    engine = create_engine("sqlite:///:memory:")
    SQLModel.metadata.create_all(engine)
    
    logger.info("Created all tables successfully")
    
    # Test inserting sample data
    with Session(engine) as session:
        # Create a biome
        biome = Biome(
            hbf_uuid="test1234",
            coordinate="N1",
            region="Fearless Wilds",
            biome_type="jungle",
            corruption_level=0,
            dread_level=0,
            extraction_confidence=0.95
        )
        session.add(biome)
        
        # Create a monster
        monster = Monster(
            hbf_uuid="test5678",
            name="Corrupted Bear",
            base_name="Bear",
            corruption_variant="corrupted",
            challenge_rating=5.0,
            threat_level=5,
            health_points=50,
            armor_class=15,
            dread_level=2,
            extraction_confidence=0.88,
            biome=biome
        )
        session.add(monster)
        
        # Create a settlement
        settlement = Settlement(
            hbf_uuid="test9012",
            name="Village of Harad",
            settlement_type="village",
            population_size=100,
            region="Ragthorn Woods",
            extraction_confidence=0.92,
            biome=biome
        )
        session.add(settlement)
        
        session.commit()
        
        # Query test
        biomes = session.query(Biome).all()
        monsters = session.query(Monster).all()
        settlements = session.query(Settlement).all()
        
        logger.info(f"Inserted: {len(biomes)} biomes, {len(monsters)} monsters, {len(settlements)} settlements")
    
    logger.info("Database operations successful!")


def main():
    """Run all tests."""
    
    logger.info("Starting entity extraction tests...")
    
    # Test extraction
    results = test_extraction()
    
    # Test database
    test_database_creation()
    
    # Generate game.db if extraction was successful
    if results and results['entities']:
        logger.info("\n=== GENERATING game.db ===")
        
        db_path = Path("game.db")
        engine = create_engine(f"sqlite:///{db_path}")
        SQLModel.metadata.create_all(engine)
        
        logger.info(f"Created {db_path} with all tables")
        logger.info("Ready for godot-sqlite testing!")
    
    logger.info("\n✅ All tests completed successfully!")


if __name__ == "__main__":
    main()
