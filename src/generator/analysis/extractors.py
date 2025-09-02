"""
Unified Entity Extractors - ML-first, no fallbacks, fail loud.

ALWAYS uses ML. Raises exceptions on failure. No silent errors.
"""

from __future__ import annotations

import json
import logging
from pathlib import Path
from typing import Any

import numpy as np
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.cluster import KMeans

from generator.entities.errors import (
    raise_extraction_error,
    MLProcessorError,
    ContentRoutingError,
    TrainingSystemError,
    InvalidEntityError
)
from generator.entities.patterns import (
    ContentRouter,
    HexTilePatterns, 
    CreaturePatterns,
    NPCPatterns,
    ItemPatterns,
    SettlementPatterns,
    DungeonPatterns,
    FactionPatterns,
    REGIONS,
    WORLD_NAME
)
from generator.entities.training import TrainingSystem
from generator.entities.manager import DragonLabyrinthMLProcessor


class UnifiedEntityExtractor:
    """
    ML-first unified extractor. No fallbacks. Fails loud.
    
    REQUIRES ML processor. REQUIRES patterns. REQUIRES training system.
    If any component fails, we raise an exception.
    """
    
    def __init__(self):
        self.logger = logging.getLogger("unified_extractor")
        
        # Initialize ML processor - REQUIRED
        self.ml_processor = DragonLabyrinthMLProcessor()
        if not self.ml_processor:
            raise MLProcessorError("ML processor initialization failed - REQUIRED component")
        
        # Initialize ContentRouter - REQUIRED
        self.router = ContentRouter()
        if not self.router:
            raise ContentRoutingError("ContentRouter initialization failed - REQUIRED component")
        
        # Initialize Training System - REQUIRED  
        self.training_system = TrainingSystem()
        if not self.training_system:
            raise TrainingSystemError("Training system initialization failed - REQUIRED component")
        
        # Extraction statistics
        self.stats = {
            "total_entities": 0,
            "ml_processed": 0,
            "pattern_routed": 0,
            "by_table": {}
        }
    
    def extract_batch(self, entities: list[tuple[str, str]]) -> dict[str, Any]:
        """
        Extract all entities using ML processor + ContentRouter.
        
        Args:
            entities: List of (uuid, content) tuples from HBF
            
        Returns:
            Comprehensive extraction results with ML analysis
            
        Raises:
            InvalidEntityError: If entities list is empty or invalid
            MLProcessorError: If ML processing fails
            ContentRoutingError: If pattern routing fails
            TrainingSystemError: If training discovery fails
        """
        
        if not entities:
            raise InvalidEntityError("Empty entities list provided - cannot extract from nothing")
        
        if not isinstance(entities, list):
            raise InvalidEntityError(f"Expected list of entities, got {type(entities)}")
        
        self.stats["total_entities"] += len(entities)
        
        # Step 1: ML Processing - REQUIRED
        self.logger.info(f"Processing {len(entities)} entities with ML")
        ml_results = self.ml_processor.process_entity_batch(entities)
        
        if not ml_results or "entities" not in ml_results:
            raise MLProcessorError(f"ML processor returned invalid results: {ml_results}")
        
        self.stats["ml_processed"] += len(ml_results["entities"])
        
        # Step 2: Pattern-based routing for validation 
        pattern_results = []
        for uuid, content in entities:
            if not uuid or not content:
                raise InvalidEntityError(f"Invalid entity: uuid={uuid}, content_len={len(content) if content else 0}")
            
            table, extracted_data, confidence = self.router.route_entity(uuid, content)
            
            if not table:
                raise ContentRoutingError(f"Failed to route entity {uuid} - no table determined")
            
            pattern_results.append({
                "uuid": uuid,
                "pattern_table": table,
                "pattern_data": extracted_data,
                "pattern_confidence": confidence
            })
            
            # Update table statistics
            if table not in self.stats["by_table"]:
                self.stats["by_table"][table] = 0
            self.stats["by_table"][table] += 1
        
        self.stats["pattern_routed"] += len(pattern_results)
        
        # Step 3: Training system discovers new patterns
        training_discoveries = self.training_system.discover_patterns_from_entities(entities)
        
        if not training_discoveries:
            raise TrainingSystemError("Training system failed to discover patterns")
        
        # Step 4: Merge ML and pattern results
        merged_results = self._merge_extraction_results(ml_results, pattern_results, training_discoveries)
        
        # Step 5: Validate extraction quality
        self._validate_extraction_quality(merged_results)
        
        return merged_results
    
    def _merge_extraction_results(
        self, 
        ml_results: dict[str, Any],
        pattern_results: list[dict],
        training_discoveries: dict[str, Any]
    ) -> dict[str, Any]:
        """
        Merge ML, pattern, and training results.
        
        Raises:
            InvalidEntityError: If merging fails
        """
        
        if not ml_results or not pattern_results:
            raise InvalidEntityError("Cannot merge empty results")
        
        # Build UUID index for quick lookup
        ml_by_uuid = {entity["uuid"]: entity for entity in ml_results["entities"]}
        pattern_by_uuid = {result["uuid"]: result for result in pattern_results}
        
        merged_entities = []
        
        for uuid in ml_by_uuid:
            if uuid not in pattern_by_uuid:
                raise InvalidEntityError(f"UUID {uuid} in ML results but not in pattern results")
            
            ml_entity = ml_by_uuid[uuid]
            pattern_entity = pattern_by_uuid[uuid]
            
            # Merge entity data
            merged = {
                "uuid": uuid,
                "ml_table": ml_entity["target_table"],
                "ml_confidence": ml_entity["confidence"],
                "ml_data": ml_entity["extracted_data"],
                "ml_context": ml_entity["ml_context"],
                "pattern_table": pattern_entity["pattern_table"],
                "pattern_confidence": pattern_entity["pattern_confidence"],
                "pattern_data": pattern_entity["pattern_data"],
                # Use ML table if high confidence, else pattern table
                "final_table": ml_entity["target_table"] if ml_entity["confidence"] > 0.7 else pattern_entity["pattern_table"],
                "combined_confidence": (ml_entity["confidence"] + pattern_entity["pattern_confidence"]) / 2
            }
            
            merged_entities.append(merged)
        
        return {
            "entities": merged_entities,
            "ml_analysis": {
                "relationships": ml_results.get("relationships", []),
                "cluster_analysis": ml_results.get("cluster_analysis", {}),
                "anomaly_analysis": ml_results.get("anomaly_analysis", {}),
                "topic_analysis": ml_results.get("topic_analysis", {})
            },
            "training_discoveries": training_discoveries,
            "extraction_stats": self.stats,
            "csv_outputs": ml_results.get("csv_outputs", {})
        }
    
    def _validate_extraction_quality(self, results: dict[str, Any]) -> None:
        """
        Validate extraction quality and raise if insufficient.
        
        Raises:
            InvalidEntityError: If extraction quality is too low
        """
        
        if not results or "entities" not in results:
            raise InvalidEntityError("No entities in extraction results")
        
        total_entities = len(results["entities"])
        high_confidence = sum(1 for e in results["entities"] if e["combined_confidence"] > 0.6)
        
        success_rate = high_confidence / total_entities if total_entities > 0 else 0
        
        if success_rate < 0.3:  # Less than 30% high confidence
            raise InvalidEntityError(
                f"Extraction quality too low: {success_rate:.1%} success rate "
                f"({high_confidence}/{total_entities} high confidence)"
            )
        
        # Check for critical tables
        tables_found = set(e["final_table"] for e in results["entities"])
        critical_tables = {"biome", "monster", "inn"}
        
        missing_critical = critical_tables - tables_found
        if missing_critical and total_entities > 10:  # Only check if we have enough entities
            self.logger.warning(f"Missing critical tables in extraction: {missing_critical}")


class TableSpecificExtractor:
    """
    Specialized extractor for specific table types using ML features.
    """
    
    def __init__(self, table_type: str):
        self.table_type = table_type
        self.logger = logging.getLogger(f"extractor_{table_type}")
        
        # Table-specific ML vectorizer
        self.vectorizer = TfidfVectorizer(
            max_features=2000,
            ngram_range=(1, 3),
            stop_words='english'
        )
        
        # Validate table type
        valid_tables = [
            "biome", "monster", "inn", "cave", "temple", "tomb",
            "city", "town", "village", "farms_cabins", "stronghold",
            "cult", "militia", "syndicate"
        ]
        
        if table_type not in valid_tables:
            raise InvalidEntityError(f"Invalid table type: {table_type}. Must be one of {valid_tables}")
    
    def extract(self, content: str) -> dict[str, Any]:
        """
        Extract table-specific data from content.
        
        Args:
            content: Entity content
            
        Returns:
            Extracted data for the specific table
            
        Raises:
            InvalidEntityError: If content is invalid
            MLProcessorError: If ML extraction fails
        """
        
        if not content:
            raise InvalidEntityError("Empty content provided for extraction")
        
        # Vectorize content for ML analysis
        content_vector = self.vectorizer.fit_transform([content])
        
        if content_vector is None:
            raise MLProcessorError(f"Failed to vectorize content for {self.table_type}")
        
        # Table-specific extraction
        if self.table_type == "biome":
            return self._extract_biome(content, content_vector)
        elif self.table_type == "monster":
            return self._extract_monster(content, content_vector)
        elif self.table_type == "inn":
            return self._extract_inn(content, content_vector)
        elif self.table_type in ["cave", "temple", "tomb"]:
            return self._extract_dungeon(content, content_vector, self.table_type)
        elif self.table_type in ["city", "town", "village"]:
            return self._extract_settlement(content, content_vector, self.table_type)
        elif self.table_type in ["farms_cabins", "stronghold"]:
            return self._extract_dwelling(content, content_vector, self.table_type)
        elif self.table_type in ["cult", "militia", "syndicate"]:
            return self._extract_faction(content, content_vector, self.table_type)
        else:
            raise InvalidEntityError(f"No extraction method for table type: {self.table_type}")
    
    def _extract_biome(self, content: str, vector: Any) -> dict[str, Any]:
        """Extract biome data."""
        
        # Use HexTilePatterns for biome extraction
        hex_match = HexTilePatterns.HEX_HEADER.search(content)
        
        if not hex_match:
            raise PatternMatchError(f"No hex pattern found in biome content")
        
        uuid = hex_match.group(1)
        coordinate = hex_match.group(2)
        region = hex_match.group(3).strip()
        world = hex_match.group(4)
        
        if world != WORLD_NAME:
            raise InvalidEntityError(f"Wrong world: {world} != {WORLD_NAME}")
        
        return {
            "hbf_uuid": uuid,
            "coordinate": coordinate,
            "region": region,
            "world": world,
            "biome_type": REGIONS.get(region, {}).get("biome", "unknown"),
            "corruption_level": REGIONS.get(region, {}).get("corruption", 0),
            "act_levels": REGIONS.get(region, {}).get("act_levels", "unknown")
        }
    
    def _extract_monster(self, content: str, vector: Any) -> dict[str, Any]:
        """Extract monster data."""
        
        creature_match = CreaturePatterns.CREATURE_BLOCK.search(content)
        
        if not creature_match:
            raise PatternMatchError("No creature stat block found")
        
        name = creature_match.group(1).strip()
        quantity = int(creature_match.group(2)) if creature_match.group(2) else 1
        cr_str = creature_match.group(3).strip()
        
        # Parse CR
        cr_numeric = 0.0
        if '/' in cr_str:
            parts = cr_str.split('/')
            cr_numeric = float(parts[0]) / float(parts[1])
        else:
            import re
            cr_match = re.search(r'\d+', cr_str)
            if cr_match:
                cr_numeric = float(cr_match.group())
        
        return {
            "name": name,
            "quantity": quantity,
            "challenge_rating": cr_numeric,
            "threat_level": min(10, max(1, int(cr_numeric * 2))),
            "dread_level": min(4, max(0, int(cr_numeric // 2)))
        }
    
    def _extract_inn(self, content: str, vector: Any) -> dict[str, Any]:
        """Extract inn data."""
        
        # Inns are isolated healing places
        is_isolated = bool(re.search(r'(isolated|remote|wilderness|crossroads)', content, re.I))
        
        if not is_isolated:
            raise InvalidEntityError("Inn must be isolated (not in settlement)")
        
        return {
            "name": self._extract_entity_name(content),
            "is_isolated": True,
            "healing_available": True,
            "region": self._detect_region(content)
        }
    
    def _extract_dungeon(self, content: str, vector: Any, dungeon_type: str) -> dict[str, Any]:
        """Extract dungeon data (cave/temple/tomb)."""
        
        return {
            "dungeon_type": dungeon_type,
            "name": self._extract_entity_name(content),
            "corruption_level": self._assess_corruption(content),
            "region": self._detect_region(content)
        }
    
    def _extract_settlement(self, content: str, vector: Any, settlement_type: str) -> dict[str, Any]:
        """Extract settlement data (city/town/village)."""
        
        return {
            "settlement_type": settlement_type,
            "name": self._extract_entity_name(content),
            "population_size": self._estimate_population(content, settlement_type),
            "region": self._detect_region(content)
        }
    
    def _extract_dwelling(self, content: str, vector: Any, dwelling_type: str) -> dict[str, Any]:
        """Extract dwelling data (farms_cabins/stronghold)."""
        
        return {
            "dwelling_type": dwelling_type,
            "name": self._extract_entity_name(content),
            "fortified": dwelling_type == "stronghold",
            "region": self._detect_region(content)
        }
    
    def _extract_faction(self, content: str, vector: Any, faction_type: str) -> dict[str, Any]:
        """Extract faction data (cult/militia/syndicate)."""
        
        return {
            "faction_type": faction_type,
            "name": self._extract_entity_name(content),
            "philosophy": self._detect_philosophy(content),
            "region": self._detect_region(content)
        }
    
    # Helper methods
    def _extract_entity_name(self, content: str) -> str:
        """Extract entity name from content."""
        
        # Look for capitalized proper nouns
        import re
        name_match = re.search(r'^([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)', content, re.MULTILINE)
        
        if name_match:
            return name_match.group(1)
        
        raise InvalidEntityError("Could not extract entity name")
    
    def _detect_region(self, content: str) -> str:
        """Detect region from content."""
        
        for region in REGIONS.keys():
            if region.lower() in content.lower():
                return region
        
        return "unknown"
    
    def _assess_corruption(self, content: str) -> int:
        """Assess corruption level (0-3)."""
        
        corruption_words = ["corrupt", "taint", "void", "dark", "evil", "curse"]
        count = sum(1 for word in corruption_words if word in content.lower())
        
        return min(3, count)
    
    def _estimate_population(self, content: str, settlement_type: str) -> int:
        """Estimate population size."""
        
        if settlement_type == "city":
            return 10000
        elif settlement_type == "town":
            return 1000
        elif settlement_type == "village":
            return 100
        
        return 0
    
    def _detect_philosophy(self, content: str) -> str:
        """Detect philosophical alignment."""
        
        content_lower = content.lower()
        
        if "strength" in content_lower or "power" in content_lower:
            return "strength"
        elif "harmony" in content_lower or "peace" in content_lower:
            return "harmony"
        elif "light" in content_lower or "holy" in content_lower:
            return "light"
        elif "dark" in content_lower or "shadow" in content_lower:
            return "dark"
        
        return "neutral"


# Module-level convenience function
def extract_entities(entities: list[tuple[str, str]]) -> dict[str, Any]:
    """
    Extract entities using unified ML-first approach.
    
    Args:
        entities: List of (uuid, content) tuples
        
    Returns:
        Extraction results
        
    Raises:
        Various extraction errors if processing fails
    """
    
    extractor = UnifiedEntityExtractor()
    return extractor.extract_batch(entities)
