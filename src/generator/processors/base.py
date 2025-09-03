"""
BaseProcessor - Base class for specialized entity processors with ML capabilities.

Provides common functionality for processors that receive pre-classified
EntityCluster objects from the transformer. Includes advanced ML processing
for data extraction and analysis.
"""

from __future__ import annotations

import json
from typing import Any

import json
from pathlib import Path
from jinja2 import Template, Environment, FileSystemLoader

from generator.constants import ENTITIES_OUTPUT_DIR
from generator.entities.processors.ml_utilities import process_entity_batch
from generator.entities.processors.types import ProcessorType
from generator.entities.processors.models import (
    SettlementData, DungeonData, RegionData, FactionData, MetaData,
    ThreatAssessment, StructuralAnalysis, InfrastructureFeatures
)


class BaseProcessor:
    """
    Base class for specialized entity processors.
    
    Processors inherit from this to get:
    - Standard processing workflow
    - Cross-system integration routing
    - World hooks generation pattern
    - Common utility methods
    """
    
    def __init__(self, processor_type: str):
        self.processor_type = processor_type
    
    def process_cluster(self, cluster, logger) -> SettlementData | DungeonData | RegionData | FactionData | MetaData:
        """
        Main entry point for processing pre-classified entity clusters.
        
        Args:
            cluster: EntityCluster with pre-classified entities from transformer
            logger: Logger instance from orchestrator
            
        Returns:
            Proper Pydantic model (SettlementData, DungeonData, RegionData, etc.)
        """
        
        logger.info(f"Processing {self.processor_type} cluster: {cluster.name} ({cluster.get_entity_count()} entities)")
        
        # Process entities with advanced ML
        entity_pairs = []
        for i, entity in enumerate(cluster.entities):
            entity_id = f"{cluster.name}_{i}"
            entity_content = self._serialize_entity(entity)
            entity_pairs.append((entity_id, entity_content))
        
        # Run ML processing batch
        ml_results = process_entity_batch(entity_pairs)
        
        # Extract processor-specific data (implemented by subclasses)
        # This now returns the proper Pydantic model
        result = self._extract_specific_data(cluster, ml_results, logger)
        
        logger.info(f"{self.processor_type.title()} processing complete: {cluster.name}")
        
        return result

    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger) -> SettlementData | DungeonData | RegionData | FactionData | MetaData:
        """
        Extract processor-specific data from cluster entities and ML results.
        MUST be overridden in subclasses to return proper Pydantic model.
        """
        raise NotImplementedError("Subclasses must implement _extract_specific_data")
    
    def _to_rust_identifier(self, name: str) -> str:
        """Convert entity name to valid Rust identifier."""
        return name.lower().replace(" ", "_").replace("'", "").replace("-", "_").replace(".", "")
    
    def _scale_to_rust_enum(self, scale: str) -> str:
        """Convert settlement scale to Rust enum variant."""
        scale_mapping = {
            "village": "Village",
            "town": "Town", 
            "city": "City",
            "metropolis": "Metropolis"
        }
        return scale_mapping.get(scale.lower(), "Village")
    
    def _service_types_to_rust(self, service_types: list[str]) -> str:
        """Convert service types list to Rust enum variants."""
        service_mapping = {
            "commerce": "ServiceType::Commerce",
            "lodging": "ServiceType::Lodging",
            "crafting": "ServiceType::Crafting",
            "medical": "ServiceType::Medical",
            "religious": "ServiceType::Religious",
            "defense": "ServiceType::Defense",
            "government": "ServiceType::Government",
            "learning": "ServiceType::Learning"
        }
        
        rust_services = [service_mapping.get(svc, f"ServiceType::{svc.title()}") for svc in service_types]
        return ", ".join(rust_services)
    
    def _biome_to_rust_enum(self, biome: str) -> str:
        """Convert biome name to Rust enum variant."""
        biome_mapping = {
            "wet_meadow": "WetMeadow",
            "ashen_forest": "AshenForest",
            "flooded_village": "FloodedVillage",
            "black_swamp": "BlackSwamp",
            "fungal_cathedral": "FungalCathedral",
            "rust_plains": "RustPlains",
            "famine_fields": "FamineFields",
            "bone_forest": "BoneForest",
            "dragon_scar": "DragonScar",
            "abyssal_chasm": "AbyssalChasm",
            "forest": "Forest",
            "desert": "Desert",
            "mountain": "Mountain",
            "plains": "Plains",
            "swamp": "Swamp",
            "tundra": "Tundra"
        }
        return biome_mapping.get(biome.lower(), "Forest")

    def _serialize_entity(self, entity: dict[str, Any]) -> str:
        """Serialize entity dict to string for processing."""
        return json.dumps(entity, indent=2)

    def _extract_entity_content(self, entity: dict[str, Any]) -> str:
        """Extract content string from entity for analysis."""
        # Try common content fields
        for field in ["content", "description", "text", "data"]:
            if field in entity and entity[field]:
                return str(entity[field])
        
        # Fallback to full JSON
        return self._serialize_entity(entity)

    def _count_entities_with_field(self, cluster, field_name: str) -> int:
        """Count entities in cluster that have a specific field."""
        count = 0
        for entity in cluster.entities:
            if field_name in entity and entity[field_name]:
                count += 1
        return count

    def _extract_unique_values(self, cluster, field_name: str) -> list[str]:
        """Extract unique values for a field across all entities in cluster."""
        values = set()
        for entity in cluster.entities:
            if field_name in entity and entity[field_name]:
                value = str(entity[field_name]).strip()
                if value:
                    values.add(value)
        return list(values)

    def _calculate_confidence_score(self, processed_entities: int, total_entities: int) -> float:
        """Calculate confidence score based on processing success rate."""
        if total_entities == 0:
            return 0.0
        return min(1.0, processed_entities / total_entities)

    def _calculate_ml_confidence(self, entities: list[dict[str, Any]]) -> float:
        """Calculate average ML confidence across all entities."""
        if not entities:
            return 0.0
        
        confidences = [entity.get("confidence", 0.0) for entity in entities]
        return sum(confidences) / len(confidences)
