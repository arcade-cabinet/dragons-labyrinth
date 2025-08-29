"""
JSON Entities Table - Auto-decodable JSON entities from HBF.

Handles HBF entities that are pure JSON objects that can be automatically
decoded without ML analysis. Provides quick insights and routing.
"""

from __future__ import annotations

import json
from datetime import datetime
from enum import Enum, auto
from typing import Any

from sqlmodel import SQLModel, Field, Column, JSON, Text
from sqlalchemy import Index


class JSONEntityType(str, Enum):
    """Types of JSON entities auto-detected from HBF."""
    REFERENCE = auto()      # Simple reference objects
    METADATA = auto()       # Metadata objects  
    CONFIGURATION = auto()  # Configuration data
    LOOKUP_TABLE = auto()   # Lookup/mapping tables
    UNKNOWN = auto()        # Unclassified JSON
    
    @classmethod
    def classify_json(cls, json_data: dict[str, Any]) -> "JSONEntityType":
        """Auto-classify JSON entity type from structure."""
        
        # Simple heuristics for classification
        if "ref" in json_data or "reference" in json_data:
            return cls.REFERENCE
        elif "config" in json_data or "settings" in json_data:
            return cls.CONFIGURATION
        elif len(json_data) < 5 and all(isinstance(v, (str, int, float)) for v in json_data.values()):
            return cls.LOOKUP_TABLE
        elif "title" in json_data or "description" in json_data:
            return cls.METADATA
        else:
            return cls.UNKNOWN


class JSONEntityRecord(SQLModel, table=True):
    """
    JSON entities that can be auto-decoded without ML analysis.
    
    These are HBF entities that are pure JSON objects, allowing
    immediate insight extraction and routing without pattern analysis.
    """
    __tablename__ = "json_entities"
    
    # Primary key using HBF UUID
    hbf_uuid: str = Field(primary_key=True, regex=r'^[a-zA-Z0-9]{8}$')
    
    # JSON data (decoded)
    json_data: dict[str, Any] = Field(..., sa_column=Column(JSON))
    
    # Classification
    entity_type: JSONEntityType = Field(...)
    detected_structure: str = Field(..., description="Auto-detected JSON structure type")
    
    # Processing status
    processed: bool = Field(default=False)
    routing_target: str | None = Field(default=None, description="Where this should be routed for processing")
    
    # Dragon's Labyrinth integration  
    horror_relevance_score: float = Field(default=0.0, ge=0.0, le=1.0)
    philosophy_alignment: str | None = Field(default=None)  # strength, harmony, light, dark
    companion_relevance: bool = Field(default=False)
    forge_relevance: bool = Field(default=False)
    
    # Raw entity tracking (required columns)
    source_entity_id: str = Field(...)
    extraction_timestamp: datetime = Field(default_factory=datetime.now)
    raw_content: str = Field(..., sa_column=Column(Text))
    confidence_score: float = Field(default=1.0, description="JSON parsing is always confident")
    
    # Indexes for efficient queries
    __table_args__ = (
        Index('idx_json_type_processed', 'entity_type', 'processed'),
        Index('idx_json_horror_relevance', 'horror_relevance_score'),
        Index('idx_json_routing', 'routing_target'),
    )


class JSONEntityExtractor:
    """
    Auto-decoder for JSON entities from HBF import.
    
    Provides immediate insights without ML analysis and routes
    entities to appropriate processing pipelines.
    """
    
    def __init__(self, logger=None):
        import logging
        self.logger = logger or logging.getLogger("json_entity_extractor")
        self.stats = {"extracted": 0, "classified": 0, "routed": 0, "errors": 0}
    
    def extract_and_store(self, hbf_uuid: str, raw_content: str, session) -> JSONEntityRecord | None:
        """
        Extract JSON entity and store with auto-classification.
        
        Args:
            hbf_uuid: 8-character HBF UUID
            raw_content: Raw entity content (should be JSON)
            session: SQLModel session for storage
            
        Returns:
            JSONEntityRecord if successful, None if failed
        """
        try:
            # Parse JSON
            json_data = json.loads(raw_content)
            
            # Auto-classify
            entity_type = JSONEntityType.classify_json(json_data)
            structure = self._analyze_structure(json_data)
            
            # Assess Dragon's Labyrinth relevance
            horror_score = self._calculate_horror_relevance(json_data)
            philosophy = self._detect_philosophy_alignment(json_data)
            companion_rel = self._detect_companion_relevance(json_data)
            forge_rel = self._detect_forge_relevance(json_data)
            
            # Determine routing
            routing_target = self._determine_routing(entity_type, json_data)
            
            # Create record
            json_entity = JSONEntityRecord(
                hbf_uuid=hbf_uuid,
                json_data=json_data,
                entity_type=entity_type,
                detected_structure=structure,
                routing_target=routing_target,
                horror_relevance_score=horror_score,
                philosophy_alignment=philosophy,
                companion_relevance=companion_rel,
                forge_relevance=forge_rel,
                source_entity_id=hbf_uuid,
                raw_content=raw_content
            )
            
            session.add(json_entity)
            self.stats["extracted"] += 1
            self.stats["classified"] += 1
            if routing_target:
                self.stats["routed"] += 1
            
            return json_entity
            
        except json.JSONDecodeError:
            # Not actually JSON
            return None
        except Exception as e:
            self.stats["errors"] += 1
            self.logger.error(f"Failed to extract JSON entity {hbf_uuid}: {e}")
            return None
    
    def _analyze_structure(self, json_data: dict[str, Any]) -> str:
        """Analyze JSON structure for insights."""
        
        keys = list(json_data.keys())
        value_types = [type(v).__name__ for v in json_data.values()]
        
        return f"keys={len(keys)}, types={set(value_types)}, depth={self._calculate_depth(json_data)}"
    
    def _calculate_depth(self, obj: Any, current_depth: int = 0) -> int:
        """Calculate nested depth of JSON object."""
        
        if isinstance(obj, dict):
            if not obj:
                return current_depth
            return max(self._calculate_depth(v, current_depth + 1) for v in obj.values())
        elif isinstance(obj, list):
            if not obj:
                return current_depth
            return max(self._calculate_depth(item, current_depth + 1) for item in obj)
        else:
            return current_depth
    
    def _calculate_horror_relevance(self, json_data: dict[str, Any]) -> float:
        """Calculate relevance to Dragon's Labyrinth horror themes."""
        
        content_str = json.dumps(json_data).lower()
        horror_keywords = [
            "horror", "fear", "terror", "dread", "nightmare", "void", "corruption",
            "trauma", "madness", "sacrifice", "dragon", "labyrinth", "companion",
            "forge", "dark", "light", "philosophy", "sentimental"
        ]
        
        matches = sum(1 for keyword in horror_keywords if keyword in content_str)
        return min(1.0, matches / len(horror_keywords))
    
    def _detect_philosophy_alignment(self, json_data: dict[str, Any]) -> str | None:
        """Detect philosophical alignment hints in JSON."""
        
        content_str = json.dumps(json_data).lower()
        
        if any(word in content_str for word in ["strength", "power", "force", "violence"]):
            return "strength"
        elif any(word in content_str for word in ["harmony", "peace", "cooperation", "together"]):
            return "harmony" 
        elif any(word in content_str for word in ["light", "holy", "blessed", "sacred"]):
            return "light"
        elif any(word in content_str for word in ["dark", "cursed", "shadow", "void"]):
            return "dark"
        
        return None
    
    def _detect_companion_relevance(self, json_data: dict[str, Any]) -> bool:
        """Detect if JSON relates to companion psychology."""
        
        content_str = json.dumps(json_data).lower()
        companion_keywords = ["companion", "friend", "ally", "trauma", "stress", "therapy", "bond"]
        
        return any(keyword in content_str for keyword in companion_keywords)
    
    def _detect_forge_relevance(self, json_data: dict[str, Any]) -> bool:
        """Detect if JSON relates to forge/sentimental items."""
        
        content_str = json.dumps(json_data).lower()
        forge_keywords = ["forge", "sentimental", "memory", "precious", "heirloom", "reagent"]
        
        return any(keyword in content_str for keyword in forge_keywords)
    
    def _determine_routing(self, entity_type: JSONEntityType, json_data: dict[str, Any]) -> str | None:
        """Determine where this JSON entity should be routed for processing."""
        
        # Route based on Dragon's Labyrinth needs
        if self._detect_companion_relevance(json_data):
            return "npcs"
        elif self._detect_forge_relevance(json_data):
            return "treasures"
        elif entity_type == JSONEntityType.CONFIGURATION:
            return "grammar"
        elif entity_type == JSONEntityType.LOOKUP_TABLE:
            return "grammar"
        else:
            return None
    
    def get_extraction_stats(self) -> dict[str, Any]:
        """Get extraction statistics."""
        return {
            "extractor_type": "JSONEntityExtractor",
            "stats": self.stats.copy()
        }
