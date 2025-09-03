"""
Entity processing models using Pydantic 2.

Refactored architecture with RawEntitiesCluster for AI generation and
RawEntities for container orchestration. Following .clinerules standards.
Uses factory methods for clean entity creation.
"""

from __future__ import annotations

import json
import logging
import re
from datetime import datetime
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field, model_validator

from generator.analysis.constants import (
    REGIONS, SETTLEMENTS, FACTIONS, DUNGEONS,
    HTML_ENTITIES_SAMPLE_THRESHOLD, JSON_ENTITIES_SAMPLE_THRESHOLD, DEFAULT_MODEL
)
from generator.utils import generate_with_openai


class ModelConnections(BaseModel):
    """Connection information surfaced by generated models."""
    uuid_fields: list[str] = Field(default_factory=list)  # Fields that are UUIDs
    connection_fields: list[str] = Field(default_factory=list)  # Fields that connect to other entities
    import_path: str = ""  # Import path for this model
    exported_classes: list[str] = Field(default_factory=list)  # Class names exported by this model


class GenerationResults(BaseModel):
    """Results from AI model generation."""
    models_generated: list[str] = Field(default_factory=list)
    types_generated: list[str] = Field(default_factory=list)
    protocols_generated: list[str] = Field(default_factory=list)
    analysis_notes: list[str] = Field(default_factory=list)
    token_usage: dict[str, int] = Field(default_factory=dict)
    connections: ModelConnections | None = None  # Connection information for containers
    success: bool = False


class RawEntity(BaseModel):
    """Raw entity extracted from HBF database with clustering logic."""
    uuid: str
    raw_value: str
    entity_type: str
    data: dict[str, Any]
    category: str
    entity_name: str
    file_path: Path | None = None
    
    @classmethod
    def create(cls, uuid: str, raw_value: str) -> "RawEntity":
        """Factory method to create RawEntity with computed fields."""
        # Parse JSON/HTML content
        entity_type, data = cls._parse_content(raw_value)
        
        # Determine category and entity name
        category, entity_name = cls._determine_clustering(raw_value)
        
        return cls(
            uuid=uuid,
            raw_value=raw_value,
            entity_type=entity_type,
            data=data,
            category=category,
            entity_name=entity_name
        )
    
    @staticmethod
    def _parse_content(raw_value: str) -> tuple[str, dict[str, Any]]:
        """Parse raw value into entity type and structured data."""
        try:
            if raw_value.strip().startswith('{'):
                return "json", json.loads(raw_value)
            else:
                return "html", {"content": raw_value}
        except (json.JSONDecodeError, Exception):
            return "html", {"content": str(raw_value)}
    
    @staticmethod
    def _determine_clustering(raw_value: str) -> tuple[str, str]:
        """Determine which category and entity this belongs to."""
        content_lower = raw_value.lower()
        
        # Check regions
        for region in REGIONS:
            if region.lower() in content_lower:
                return "regions", region
        
        # Check settlements
        for settlement in SETTLEMENTS:
            if settlement.lower() in content_lower:
                return "settlements", settlement
        
        # Check factions
        for faction in FACTIONS:
            if faction.lower() in content_lower:
                return "factions", faction
        
        # Check dungeons
        for dungeon in DUNGEONS:
            if dungeon.lower() in content_lower:
                return "dungeons", dungeon
        
        # Uncategorized
        return "uncategorized", "unknown"
    
    def get_sanitized_name(self) -> str:
        """Get sanitized name for directory creation."""
        if not self.entity_name:
            return "unknown"
        return self.entity_name.lower().replace(" ", "_").replace("'", "").replace("-", "_").replace(".", "")
    
    def write_to_disk(self, analysis_dir: Path) -> Path:
        """Write entity to disk and return file path."""
        if not self.category or not self.entity_name:
            raise ValueError(f"Entity {self.uuid} not properly categorized")
            
        # Create directory structure
        entity_dir = analysis_dir / self.category / self.get_sanitized_name()
        entity_dir.mkdir(parents=True, exist_ok=True)
        
        # Write file
        ext = "json" if self.entity_type == "json" else "html"
        filename = f"entity_{self.uuid}.{ext}"
        file_path = entity_dir / filename
        
        if self.entity_type == "json":
            with open(file_path, 'w', encoding='utf-8') as f:
                json.dump(self.data, f, indent=2, ensure_ascii=False)
        else:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(self.raw_value)
        
        self.file_path = file_path
        return file_path



# --- New cluster base and specializations ---
from jinja2 import Template
from abc import ABC, abstractmethod

class BaseEntitiesCluster(BaseModel, ABC):
    """Abstract cluster with two-stage generation (inventory -> code).

    Stage A: Use Responses Structured Outputs to infer a strict JSON inventory of
             fields and UUID connections from sample HTML/JSON files.
    Stage B: Render deterministic Pydantic models via an inline Jinja template.
    """
    category: str
    cluster_name: str
    entities: list[RawEntity] = Field(default_factory=list)
    html_files: list[Path] = Field(default_factory=list)
    json_files: list[Path] = Field(default_factory=list)
    generation_results: GenerationResults | None = None

    # ---------------------------- lifecycle ---------------------------- #
    def add_entity(self, entity: RawEntity) -> bool:
        if entity.category == self.category and entity.entity_name == self.cluster_name:
            self.entities.append(entity)
            return True
        return False

    def write_entities_to_disk(self, analysis_dir: Path) -> None:
        for entity in self.entities:
            file_path = entity.write_to_disk(analysis_dir)
            if entity.entity_type == "json":
                self.json_files.append(file_path)
            else:
                self.html_files.append(file_path)

    def can_generate_models(self) -> bool:
        return (len(self.html_files) + len(self.json_files)) > 0

    # ---------------------------- specialization hooks ---------------------------- #
    @abstractmethod
    def inventory_schema(self) -> dict[str, Any]:
        """Strict JSON Schema for the Stage-A inventory."""
        raise NotImplementedError

    @abstractmethod
    def analysis_prompt(self) -> str:
        """Natural language instructions for Stage A (no code, JSON only)."""
        raise NotImplementedError

    @abstractmethod
    def model_template(self) -> str:
        """Inline Jinja template for Stage B code rendering."""
        raise NotImplementedError

    # ---------------------------- generation core ---------------------------- #
    def _gather_samples(self) -> tuple[list[Path], list[Path]]:
        html_samples = self.html_files[:HTML_ENTITIES_SAMPLE_THRESHOLD]
        json_samples = self.json_files[:JSON_ENTITIES_SAMPLE_THRESHOLD]
        return html_samples, json_samples

    def _stage_a_inventory(self, templates_dir: Path, logger: logging.Logger) -> dict[str, Any]:
        """Call OpenAI with strict structured outputs to infer inventory."""
        # Synthesize a tiny ephemeral template using the analysis_prompt()
        tmp_template_text = (
            "SYSTEM RULES (read carefully):\n"
            "- You are analyzing mixed HTML/JSON snippets.\n"
            "- There are NO PDFs. Do not mention PDFs.\n"
            "- Return ONLY a single JSON object that conforms to the provided schema.\n"
            "- Do NOT output code, markdown fences, or commentary. JSON only."\
        ) + "\n\n" + self.analysis_prompt()

        # Render into a temporary Jinja template in-memory
        # We still need a Path to pass through existing utility; write to a temp file beside templates_dir
        tmp_path = templates_dir / f"_{self.category}_inventory_prompt.auto.j2"
        tmp_path.write_text(tmp_template_text, encoding="utf-8")

        schema = self.inventory_schema()
        html_samples, json_samples = self._gather_samples()
        uploaded = html_samples + json_samples

        raw = generate_with_openai(
            template_path=tmp_path,
            template_context={},
            uploaded_files=uploaded,
            model=DEFAULT_MODEL,
            logger=logger,
            response_schema={
                "name": "field_inventory",
                "schema": schema,
            },
        )

        # Clean up the ephemeral file if possible
        try:
            tmp_path.unlink(missing_ok=True)
        except Exception:
            pass

        try:
            inventory = json.loads(raw) if isinstance(raw, str) else raw
        except Exception:
            # If the helper already returned JSON, keep it
            inventory = raw  # type: ignore
        return inventory

    def _stage_b_render_code(self, inventory: dict[str, Any]) -> str:
        """Render deterministic Pydantic code from inventory via inline template."""
        template = Template(self.model_template())
        return template.render(inventory=inventory, category=self.category, cluster_name=self.cluster_name)

    def _connections_from_inventory(self, inventory: dict[str, Any], models_file: Path) -> ModelConnections:
        uuid_fields: list[str] = []
        connection_fields: list[str] = []
        exported_classes: list[str] = []

        for ent in inventory.get("entities", []):
            name = ent.get("name")
            if name and name not in exported_classes:
                exported_classes.append(name)
            for f in ent.get("fields", []):
                fname = f.get("name")
                if not fname:
                    continue
                if f.get("is_uuid") or ("uuid" in fname.lower()):
                    if fname not in uuid_fields:
                        uuid_fields.append(fname)
                if f.get("is_connection"):
                    if fname not in connection_fields:
                        connection_fields.append(fname)

        import_path = f"generator.processors.models.{self.category}"
        return ModelConnections(
            uuid_fields=uuid_fields,
            connection_fields=connection_fields,
            import_path=import_path,
            exported_classes=exported_classes,
        )

    def generate_models(self, models_dir: Path, templates_dir: Path, logger: logging.Logger) -> GenerationResults:
        if not self.can_generate_models():
            return GenerationResults(success=False, analysis_notes=["No sample files for analysis"]) 

        try:
            inventory = self._stage_a_inventory(templates_dir, logger)
            models_content = self._stage_b_render_code(inventory)

            models_file = models_dir / f"{self.category}.py"
            with open(models_file, "w", encoding="utf-8") as f:
                f.write(models_content)

            connections = self._connections_from_inventory(inventory, models_file)

            html_samples, json_samples = self._gather_samples()
            self.generation_results = GenerationResults(
                models_generated=[str(models_file)],
                analysis_notes=[f"Generated from {len(html_samples)} HTML + {len(json_samples)} JSON files"],
                connections=connections,
                success=True,
            )
            logger.info(f"Generated models for {self.category}: {models_file}")
            return self.generation_results
        except Exception as e:
            logger.error(f"Failed to generate models for {self.category}: {e}")
            raise

# ---------------------------- Specialized clusters ---------------------------- #

class RawRegionEntities(BaseEntitiesCluster):
    def inventory_schema(self) -> dict[str, Any]:
        return {
            "type": "object",
            "properties": {
                "entities": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "name": {"type": "string"},
                            "description": {"type": "string"},
                            "fields": {
                                "type": "array",
                                "items": {
                                    "type": "object",
                                    "properties": {
                                        "name": {"type": "string"},
                                        "type": {"type": "string"},
                                        "required": {"type": "boolean"},
                                        "description": {"type": "string"},
                                        "is_uuid": {"type": "boolean"},
                                        "is_connection": {"type": "boolean"},
                                    },
                                    "required": ["name", "type", "required"],
                                    "additionalProperties": False,
                                },
                            },
                        },
                        "required": ["name", "fields"],
                        "additionalProperties": False,
                    },
                },
                "connections": {"type": "object", "additionalProperties": {"type": "string"}},
                "notes": {"type": "array", "items": {"type": "string"}},
            },
            "required": ["entities"],
            "additionalProperties": False,
        }

    def analysis_prompt(self) -> str:
        return (
            "Analyze the supplied HTML/JSON snippets related to *regions*.\n"
            "Return a JSON object with an 'entities' array describing data models.\n"
            "Focus on names, descriptions, field names/types, and which fields are UUIDs or connections.\n"
            "If uncertain, omit rather than invent."
        )

    def model_template(self) -> str:
        return (
            "# Generated Pydantic models for regions\n"
            "from __future__ import annotations\n"
            "from typing import Any, Optional, List, Dict\n"
            "from pydantic import BaseModel, Field\n\n"
            "{% for e in inventory.entities %}\n"
            "class {{ e.name }}(BaseModel):\n"
            "    \"\"\"{{ e.description or e.name }}\"\"\"\n"
            "{% for f in e.fields %}"
            "    {{ f.name }}: {{ f.type }}{% if not f.required %} | None{% endif %} = {% if f.required %}Field(...{% else %}Field(None{% endif %}, description=\"{{ f.description or '' }}\")\n"
            "{% endfor %}\n\n"
            "    @classmethod\n"
            "    def extract_uuid_connections(cls) -> Dict[str, str]:\n"
            "        return {\n"
            "{% for e in [e] %}{% for f in e.fields if f.is_uuid or f.is_connection %}"
            "            '{{ f.name }}': '{{ inventory.connections.get(f.name, 'unknown') }}',\n"
            "{% endfor %}{% endfor %}"
            "        }\n\n"
            "{% endfor %}\n"
        )

class RawSettlementEntities(RawRegionEntities):
    def analysis_prompt(self) -> str:
        return (
            "Analyze the supplied HTML/JSON snippets related to *settlements*.\n"
            "Return a JSON object with 'entities' describing normalized fields and UUID connections.\n"
            "Do not output code; output JSON only."
        )

    def model_template(self) -> str:
        hdr = "# Generated Pydantic models for settlements\n"
        return hdr + super().model_template()

class RawFactionEntities(RawRegionEntities):
    def analysis_prompt(self) -> str:
        return (
            "Analyze the supplied HTML/JSON snippets related to *factions*.\n"
            "Identify UUID fields that represent links to regions/settlements/dungeons/etc.\n"
            "Return JSON only."
        )

    def model_template(self) -> str:
        hdr = "# Generated Pydantic models for factions\n"
        return hdr + super().model_template()

class RawDungeonEntities(RawRegionEntities):
    def analysis_prompt(self) -> str:
        return (
            "Analyze the supplied HTML/JSON snippets related to *dungeons*.\n"
            "Detect area/monster/treasure relationships, but only emit field/connection metadata.\n"
            "Return JSON only."
        )

    def model_template(self) -> str:
        hdr = "# Generated Pydantic models for dungeons\n"
        return hdr + super().model_template()


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
    
    def add_entity(self, uuid: str, value: str):
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
    
    def generate_all_individual_models(self, models_dir: Path, templates_dir: Path, logger: logging.Logger) -> dict[str, GenerationResults]:
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
    
    def generate_container_models(self, models_dir: Path, templates_dir: Path, phase1_results: dict[str, GenerationResults], logger: logging.Logger) -> dict[str, GenerationResults]:
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
                # Fail fast to prevent partial artifacts
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
                # Fail fast to prevent partial artifacts
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


class AnalysisSummary(BaseModel):
    """Summary of analysis results for reporting."""
    total_entities_processed: int
    regions_count: int
    settlements_count: int
    factions_count: int
    dungeons_count: int
    uncategorized_count: int
    processing_timestamp: datetime = Field(default_factory=datetime.now)
    ready_for_processors: bool = True