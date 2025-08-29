"""
Entities Manager - Coordinates all entity extraction tasks from HBF database.

This manager:
1. Sets up the raw HBF database from inputs/raw_game.hbf
2. Creates and manages the processor for ML extraction
3. Coordinates entity extraction and database population
"""

from __future__ import annotations

import shutil
import sqlite3
from pathlib import Path
from typing import Any, Dict, List, Optional

from sqlmodel import Session, SQLModel, create_engine

from generator.constants import GAME_DB_PATH, HBF_RAW_PATH
from generator.entities.processor import DragonLabyrinthMLProcessor
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
from sqlmodel import select


class EntitiesManager:
    """Manager for entity extraction from HBF database."""
    
    def __init__(self, db_path: Optional[Path] = None):
        """Initialize entities manager.
        
        Args:
            db_path: Optional custom database path (defaults to GAME_DB_PATH)
        """
        self.db_path = Path(db_path) if db_path else GAME_DB_PATH
        self.hbf_path = Path("inputs/raw_game.hbf")
        self.processor = DragonLabyrinthMLProcessor()
        
        # Ensure database directory exists
        self.db_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Initialize database engine
        self.engine = create_engine(f"sqlite:///{self.db_path}")
    
    def initialize_database_connections(self, session: Session) -> None:
        """Initialize database connections and ensure tables exist.
        
        Args:
            session: SQLModel database session
        """
        self.create_tables(session)
    
    def setup_raw_hbf_database(self) -> bool:
        """Set up the raw HBF database from inputs directory.
        
        Returns:
            True if setup successful, False otherwise
        """
        if not self.hbf_path.exists():
            print(f"❌ HBF file not found at {self.hbf_path}")
            return False
        
        # Copy HBF to raw database location if needed
        raw_db_path = Path(HBF_RAW_PATH)
        if not raw_db_path.exists() or raw_db_path.stat().st_size != self.hbf_path.stat().st_size:
            print(f"📦 Setting up raw HBF database from {self.hbf_path}")
            raw_db_path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(self.hbf_path, raw_db_path)
            print(f"✅ Raw HBF database ready at {raw_db_path}")
        else:
            print(f"✓ Raw HBF database already exists at {raw_db_path}")
        
        return True
    
    def create_tables(self, session: Session) -> None:
        """Create all entity tables in the database.
        
        Args:
            session: SQLModel database session
        """
        # Create all entity tables
        SQLModel.metadata.create_all(self.engine, checkfirst=True)
        print("✅ Entity tables created/verified")
    
    def extract_entities_from_hbf(self, session: Session) -> Dict[str, Any]:
        """Extract entities from HBF database using ML processor.
        
        Args:
            session: SQLModel database session
            
        Returns:
            Dictionary containing extraction results and statistics
        """
        if not Path(HBF_RAW_PATH).exists():
            if not self.setup_raw_hbf_database():
                return {"error": "Failed to setup HBF database"}
        
        print("🔍 Extracting entities from HBF database...")
        
        # Connect to raw HBF database
        conn = sqlite3.connect(HBF_RAW_PATH)
        cursor = conn.cursor()
        
        # Get entity records from appropriate tables
        # Assuming HBF has tables like EntityJSON and EntityHTML
        entity_records = []
        
        try:
            # Try JSON entities first
            cursor.execute("SELECT uuid, json_data FROM EntityJSON LIMIT 1000")
            json_entities = cursor.fetchall()
            for uuid, json_data in json_entities:
                if json_data:
                    entity_records.append((uuid, json_data))
            print(f"  Found {len(json_entities)} JSON entities")
        except sqlite3.Error:
            print("  No EntityJSON table found")
        
        try:
            # Try HTML entities
            cursor.execute("SELECT uuid, html_content FROM EntityHTML LIMIT 1000")
            html_entities = cursor.fetchall()
            for uuid, html_content in html_entities:
                if html_content:
                    entity_records.append((uuid, html_content))
            print(f"  Found {len(html_entities)} HTML entities")
        except sqlite3.Error:
            print("  No EntityHTML table found")
        
        conn.close()
        
        if not entity_records:
            print("⚠️ No entities found in HBF database")
            return {"warning": "No entities found"}
        
        # Process entities with ML processor
        print(f"🤖 Processing {len(entity_records)} entities with ML...")
        results = self.processor.process_entity_batch(entity_records)
        
        # Store processed entities in game database
        stored_counts = self._store_processed_entities(session, results)
        
        print(f"✅ Entity extraction complete!")
        print(f"   Stats: {results.get('processing_stats', {})}")
        
        return {
            "total_processed": len(entity_records),
            "results": results,
            "stored_counts": stored_counts
        }
    
    def _store_processed_entities(self, session: Session, results: Dict[str, Any]) -> Dict[str, int]:
        """Store processed entities in appropriate tables.
        
        Args:
            session: SQLModel database session
            results: Processing results from ML processor
            
        Returns:
            Dictionary with counts of entities stored per table
        """
        counts = {}
        
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
        
        for entity in results.get("entities", []):
            table = entity.get("target_table")
            if table in table_orm_map and entity.get("confidence", 0) > 0.5:
                # Create ORM record
                orm_class = table_orm_map[table]
                record_data = {
                    "hbf_uuid": entity["uuid"],
                    "confidence_score": entity["confidence"],
                    **entity.get("extracted_data", {})
                }
                
                try:
                    record = orm_class(**record_data)
                    session.add(record)
                    counts[table] = counts.get(table, 0) + 1
                except Exception as e:
                    print(f"  ⚠️ Failed to store {table} entity: {e}")
        
        # Commit all entities
        try:
            session.commit()
            for table, count in counts.items():
                print(f"  ✓ Stored {count} {table} entities")
        except Exception as e:
            session.rollback()
            print(f"  ❌ Failed to commit entities: {e}")
        
        return counts
    
    def load_csv_supplements(self, session: Session) -> Dict[str, int]:
        """Load supplementary data from CSV files in hbf_analysis.
        
        Args:
            session: SQLModel database session
            
        Returns:
            Dictionary with counts of loaded records
        """
        import pandas as pd
        
        counts = {}
        analysis_dir = Path("hbf_analysis")
        
        # Load hex tiles for biome data
        hex_tiles_path = analysis_dir / "hex_tiles_full.csv"
        if hex_tiles_path.exists():
            print("📊 Loading hex tiles from CSV...")
            df = pd.read_csv(hex_tiles_path)
            
            for _, row in df.iterrows():
                try:
                    biome = BiomeRecord(
                        hbf_uuid=row.get("uuid", f"hex_{row.get('coordinate', 'unknown')}"),
                        coordinate=row.get("coordinate", ""),
                        biome_type=row.get("biome", "unknown"),
                        region=row.get("region", ""),
                        features=row.get("features", ""),
                        confidence_score=0.95  # High confidence for CSV data
                    )
                    session.add(biome)
                    counts["biome"] = counts.get("biome", 0) + 1
                except Exception as e:
                    print(f"  ⚠️ Failed to load hex tile: {e}")
            
            try:
                session.commit()
                print(f"  ✓ Loaded {counts.get('biome', 0)} biome records from CSV")
            except Exception as e:
                session.rollback()
                print(f"  ❌ Failed to commit CSV data: {e}")
        
        return counts
    
    def extract_entities_with_hbf_integration(self, session: Session, hbf_path: str) -> Dict[str, Any]:
        """Extract entities with HBF integration.
        
        Args:
            session: SQLModel database session
            hbf_path: Path to HBF file
            
        Returns:
            Extraction results
        """
        # Update HBF path if provided
        if hbf_path:
            self.hbf_path = Path(hbf_path)
        
        return self.run_complete_extraction(session)
    
    def extract_entities_from_database(self, session: Session) -> Dict[str, Any]:
        """Extract entities from existing database.
        
        Args:
            session: SQLModel database session
            
        Returns:
            Extraction results from database
        """
        # Just extract from HBF without loading CSV supplements
        extraction_results = self.extract_entities_from_hbf(session)
        return {
            "total_entities": extraction_results.get("total_processed", 0),
            "extraction_results": extraction_results
        }
    
    def get_all_npcs(self, session: Session) -> List[Dict[str, Any]]:
        """Get all NPCs from database."""
        # Simplified - would query actual NPC table
        return []
    
    def get_all_monsters(self, session: Session) -> List[Dict[str, Any]]:
        """Get all monsters from database."""
        monsters = session.exec(select(MonsterRecord)).all()
        return [{"name": m.name, "cr": m.challenge_rating} for m in monsters]
    
    def get_all_biomes(self, session: Session) -> List[Dict[str, Any]]:
        """Get all biomes from database."""
        biomes = session.exec(select(BiomeRecord)).all()
        return [{"type": b.biome_type, "coordinate": b.coordinate} for b in biomes]
    
    def get_all_locations(self, session: Session) -> List[Dict[str, Any]]:
        """Get all locations from database."""
        # Would combine cities, towns, villages, etc.
        return []
    
    def run_complete_extraction(self, session: Session) -> Dict[str, Any]:
        """Run complete entity extraction pipeline.
        
        Args:
            session: SQLModel database session
            
        Returns:
            Complete extraction results
        """
        print("\n" + "="*60)
        print("🚀 ENTITIES EXTRACTION PIPELINE")
        print("="*60)
        
        # Step 1: Setup HBF database
        if not self.setup_raw_hbf_database():
            return {"error": "Failed to setup HBF database"}
        
        # Step 2: Create tables
        self.create_tables(session)
        
        # Step 3: Extract entities from HBF
        extraction_results = self.extract_entities_from_hbf(session)
        
        # Step 4: Load CSV supplements
        csv_counts = self.load_csv_supplements(session)
        
        # Combine results
        total_stored = sum(extraction_results.get("stored_counts", {}).values())
        total_csv = sum(csv_counts.values())
        
        print("\n" + "="*60)
        print(f"✅ ENTITIES EXTRACTION COMPLETE")
        print(f"   Extracted from HBF: {extraction_results.get('total_processed', 0)} entities")
        print(f"   Stored in database: {total_stored} entities")
        print(f"   Loaded from CSV: {total_csv} records")
        print("="*60 + "\n")
        
        return {
            "extraction_results": extraction_results,
            "csv_counts": csv_counts,
            "total_stored": total_stored + total_csv,
            "total_entities": total_stored + total_csv
        }


    def run(self) -> Dict[str, Any]:
        """Main entry point for entities extraction.
        
        Handles session management internally.
        
        Returns:
            Complete extraction results
        """
        # Create tables
        SQLModel.metadata.create_all(self.engine, checkfirst=True)
        
        # Run extraction with managed session
        from sqlmodel import Session
        with Session(self.engine) as session:
            return self.run_complete_extraction(session)


# For backwards compatibility
def create_entities_manager(db_path: Optional[Path] = None) -> EntitiesManager:
    """Factory function to create entities manager.
    
    Args:
        db_path: Optional custom database path
        
    Returns:
        Configured EntitiesManager instance
    """
    return EntitiesManager(db_path)
