"""Orchestration models for entity processing and generation.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

import logging
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field

from generator.analysis.constants import REGIONS, SETTLEMENTS, FACTIONS, DUNGEONS
from generator.analysis.models.raw import RawEntity
from generator.analysis.models.results import GenerationResults
from generator.analysis.models.regions import RawRegionEntities
from generator.analysis.models.settlements import RawSettlementEntities
from generator.analysis.models.factions import RawFactionEntities
from generator.analysis.models.dungeons import RawDungeonEntities
from generator.analysis.models.clusters import BaseEntitiesCluster
from generator.utils import generate_with_openai


class RawEntities(BaseModel):
    """Collection of clustered entities with orchestration capabilities."""
    regions: dict[str, BaseEntitiesCluster] = Field(default_factory=dict)
    settlements: dict[str, BaseEntitiesCluster] = Field(default_factory=dict)
    factions: dict[str, BaseEntitiesCluster] = Field(default_factory=dict)
    dungeons: dict[str, BaseEntitiesCluster] = Field(default_factory=dict)
    uncategorized: list[RawEntity] = Field(default_factory=list)
    total_entities: int = 0
    
    def model_post_init(self, __context: Any) -> None:
        """Initialize clusters for known entities."""
        # Initialize region clusters
        for region in REGIONS:
            self.regions[region] = RawRegionEntities(category="regions", cluster_name=region)
        # Initialize settlement clusters
        for settlement in SETTLEMENTS:
            self.settlements[settlement] = RawSettlementEntities(category="settlements", cluster_name=settlement)
        # Initialize faction clusters
        for faction in FACTIONS:
            self.factions[faction] = RawFactionEntities(category="factions", cluster_name=faction)
        # Initialize dungeon clusters
        for dungeon in DUNGEONS:
            self.dungeons[dungeon] = RawDungeonEntities(category="dungeons", cluster_name=dungeon)
    
    def add_entity(self, uuid: str, value: str) -> None:
        """Add entity and route to appropriate cluster using factory method."""
        entity = RawEntity.create(uuid, value)  # Use factory method
        
        # Try to add to appropriate cluster
        if entity.category == "regions" and entity.entity_name in self.regions:
            self.regions[entity.entity_name].add_entity(entity)
        elif entity.category == "settlements" and entity.entity_name in self.settlements:
            self.settlements[entity.entity_name].add_entity(entity)
        elif entity.category == "factions" and entity.entity_name in self.factions:
            self.factions[entity.entity_name].add_entity(entity)
        elif entity.category == "dungeons" and entity.entity_name in self.dungeons:
            self.dungeons[entity.entity_name].add_entity(entity)
        else:
            self.uncategorized.append(entity)
        
        self.total_entities += 1
    
    def write_all_entities(self, analysis_dir: Path, logger: logging.Logger) -> None:
        """Write all entities to disk in their cluster directories."""
        logger.info("Writing clustered entities to disk...")
        
        for category_name, clusters in [
            ("regions", self.regions),
            ("settlements", self.settlements),
            ("factions", self.factions),
            ("dungeons", self.dungeons)
        ]:
            for cluster_name, cluster in clusters.items():
                if cluster.entities:
                    logger.info(f"  Writing {cluster_name}: {len(cluster.entities)} entities")
                    cluster.write_entities_to_disk(analysis_dir)
    
    def generate_all_individual_models(
        self,
        models_dir: Path,
        templates_dir: Path,
        logger: logging.Logger
    ) -> dict[str, GenerationResults]:
        """Generate AI models for all clusters - Phase 1."""
        logger.info("PHASE 1: Generating individual category models...")
        results = {}

        for category_name, clusters in [
            ("regions", self.regions),
            ("settlements", self.settlements),
            ("factions", self.factions),
            ("dungeons", self.dungeons)
        ]:
            logger.info(f"Processing {category_name}...")
            # Use specialized cluster class for each category
            category_map = {
                "regions": RawRegionEntities,
                "settlements": RawSettlementEntities,
                "factions": RawFactionEntities,
                "dungeons": RawDungeonEntities,
            }
            ClusterCls = category_map[category_name]
            combined_cluster = ClusterCls(category=category_name, cluster_name="combined")
            for cluster in clusters.values():
                if cluster.entities:
                    combined_cluster.entities.extend(cluster.entities)
                    combined_cluster.html_files.extend(cluster.html_files)
                    combined_cluster.json_files.extend(cluster.json_files)

            if combined_cluster.can_generate_models():
                result = combined_cluster.generate_models(models_dir, templates_dir, logger)
                results[category_name] = result
                if result.success:
                    logger.info(f"✓ Generated models for {category_name}")
                else:
                    logger.warning(f"✗ Failed to generate models for {category_name}")
            else:
                logger.warning(f"No samples collected for {category_name}")

        return results
    
    def generate_container_models(
        self,
        models_dir: Path,
        templates_dir: Path,
        phase1_results: dict[str, GenerationResults],
        logger: logging.Logger
    ) -> dict[str, GenerationResults]:
        """Generate container models - Phase 2 & 3."""
        results = {}
        
        # Phase 2: Dungeon containers
        logger.info("PHASE 2: Generating dungeon container models...")
        if "dungeons" in phase1_results and phase1_results["dungeons"].success:
            dungeons_connections = phase1_results["dungeons"].connections
            
            template_context = {
                "category": "dungeon_container",
                "html_count": 0,
                "json_count": 0,
                "dungeons_connections": dungeons_connections
            }
            
            try:
                template_path = templates_dir / "dungeon_container.j2"
                models_content = generate_with_openai(
                    template_path=template_path,
                    template_context=template_context,
                    logger=logger
                )
                
                models_file = models_dir / "dungeon_container.py"
                with open(models_file, 'w', encoding='utf-8') as f:
                    f.write(models_content)
                
                results["dungeon_container"] = GenerationResults(
                    models_generated=[str(models_file)],
                    analysis_notes=["Generated from dungeon connections"],
                    success=True
                )
                logger.info("✓ Generated dungeon_container model")
                
            except Exception as e:
                logger.error(f"Failed to generate dungeon_container: {e}")
                raise
        
        # Phase 3: Region containers
        logger.info("PHASE 3: Generating region container models...")
        individual_models = {}
        for category in ["regions", "settlements", "factions", "dungeons"]:
            if category in phase1_results and phase1_results[category].success:
                individual_models[category] = phase1_results[category].connections
        
        if individual_models:
            template_context = {
                "category": "region_container",
                "html_count": 0,
                "json_count": 0,
                "individual_models": individual_models
            }
            
            try:
                template_path = templates_dir / "region_container.j2"
                models_content = generate_with_openai(
                    template_path=template_path,
                    template_context=template_context,
                    logger=logger
                )
                
                models_file = models_dir / "region_container.py"
                with open(models_file, 'w', encoding='utf-8') as f:
                    f.write(models_content)
                
                results["region_container"] = GenerationResults(
                    models_generated=[str(models_file)],
                    analysis_notes=["Generated from all individual models"],
                    success=True
                )
                logger.info("✓ Generated region_container model")
                
            except Exception as e:
                logger.error(f"Failed to generate region_container: {e}")
                raise
        
        return results
    
    def get_summary(self) -> dict[str, dict[str, int] | int]:
        """Get summary of entity counts by category."""
        return {
            "regions": {name: len(cluster.entities) for name, cluster in self.regions.items() if cluster.entities},
            "settlements": {name: len(cluster.entities) for name, cluster in self.settlements.items() if cluster.entities},
            "factions": {name: len(cluster.entities) for name, cluster in self.factions.items() if cluster.entities},
            "dungeons": {name: len(cluster.entities) for name, cluster in self.dungeons.items() if cluster.entities},
            "uncategorized": len(self.uncategorized)
        }
