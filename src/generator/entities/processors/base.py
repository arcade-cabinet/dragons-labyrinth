"""
BaseProcessor - Base class for specialized entity processors with ML capabilities.

Provides common functionality for processors that receive pre-classified
EntityCluster objects from the transformer. Includes advanced ML processing
for data extraction and analysis.
"""

from __future__ import annotations

import json
from typing import Any

from sqlmodel import create_engine, Session
from generator.constants import GAME_DB_PATH
from generator.entities.processors.ml_utilities import process_entity_batch


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
    
    def process_cluster(self, cluster, logger, console) -> dict[str, Any]:
        """
        Main entry point for processing pre-classified entity clusters.
        
        Args:
            cluster: EntityCluster with pre-classified entities from transformer
            logger: Logger instance from orchestrator
            console: Rich console from orchestrator
            
        Returns:
            Processed cluster data with world_hooks for Godot integration
        """
        
        logger.info(f"🎯 Processing {self.processor_type} cluster: {cluster.name} ({cluster.get_entity_count()} entities)")
        console.print(f"🎯 Processing {self.processor_type} cluster: [bold cyan]{cluster.name}[/bold cyan] ({cluster.get_entity_count()} entities)")
        
        # Process entities with advanced ML
        entity_pairs = []
        for i, entity in enumerate(cluster.entities):
            entity_id = f"{cluster.name}_{i}"
            entity_content = self._serialize_entity(entity)
            entity_pairs.append((entity_id, entity_content))
        
        # Run ML processing batch
        ml_results = process_entity_batch(entity_pairs)
        
        # Extract processor-specific data (implemented by subclasses)
        specific_data = self._extract_specific_data(cluster, ml_results, logger, console)
        
        # Generate world_hooks for Godot integration (implemented by subclasses)
        world_hooks = self._generate_world_hooks(cluster, specific_data)
        
        result = {
            "cluster_name": cluster.name,
            "cluster_category": cluster.category,
            "entity_count": cluster.get_entity_count(),
            "specific_data": specific_data,
            "world_hooks": world_hooks,
            "ml_results": ml_results,
            "processor_type": self.processor_type
        }
        
        # Route to integration modules for database population
        self._route_to_integrations(result, logger, console)
        
        logger.info(f"✅ {self.processor_type.title()} processing complete: {cluster.name}")
        console.print(f"✅ {self.processor_type.title()} processing complete: [bold green]{cluster.name}[/bold green]")
        
        return result

    def _extract_specific_data(self, cluster, ml_results: dict[str, Any], logger, console) -> dict[str, Any]:
        """
        Extract processor-specific data from cluster entities and ML results.
        Override in subclasses for specialized extraction.
        """
        entities = ml_results.get("entities", [])
        
        return {
            "name": cluster.name,
            "entity_count": cluster.get_entity_count(),
            "entities_processed": len(entities),
            "ml_confidence": self._calculate_ml_confidence(entities),
            "anomaly_count": ml_results.get("anomaly_count", 0),
            "relationship_count": len(ml_results.get("relationships", []))
        }

    def _generate_world_hooks(self, cluster, specific_data: dict[str, Any]) -> dict[str, Any]:
        """
        Generate world_hooks for Godot integration.
        Override in subclasses for specialized world hooks.
        """
        return {
            "entity_name": cluster.name,
            "processor_type": self.processor_type,
            "godot_integration": {
                "base_sprite_path": f"res://art/{self.processor_type}/{cluster.name.lower().replace(' ', '_')}.png"
            }
        }

    def _route_to_integrations(self, result: dict[str, Any], logger, console) -> None:
        """
        Route processing results to world integration as MASTER COORDINATOR.
        
        ARCHITECTURAL CHANGE: Entity processors now route ONLY to world integration,
        which acts as the master coordinator for all world hooks and Godot integration.
        World integration then coordinates with maps/sprites/encounters as data providers.
        """
        
        engine = create_engine(f"sqlite:///{GAME_DB_PATH}")
        with Session(engine) as session:
            from generator.world.integration import integrate_from_entities_processors as world_integrate
            
            logger.info(f"🌍 Routing {self.processor_type} to world integration (master coordinator)")
            console.print(f"🌍 Routing {self.processor_type} to world integration (master coordinator)")
            
            # Route ONLY to world integration as master coordinator
            world_stats = world_integrate(session, {self.processor_type: result})
            result["world_master_coordination"] = world_stats.to_dict()
            
            logger.info(f"✅ World master coordination complete for {self.processor_type}")
            console.print(f"✅ World master coordination complete for {self.processor_type}")

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
