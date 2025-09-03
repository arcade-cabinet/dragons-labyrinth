"""Entity cluster models for AI-powered model generation.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

import json
import logging
from abc import ABC, abstractmethod
from pathlib import Path
from typing import Any

from jinja2 import Template
from pydantic import BaseModel, Field

from generator.analysis.constants import (
    HTML_ENTITIES_SAMPLE_THRESHOLD,
    JSON_ENTITIES_SAMPLE_THRESHOLD,
    DEFAULT_MODEL,
)
from generator.analysis.models.raw import RawEntity
from generator.analysis.models.results import GenerationResults, ModelConnections
from generator.utils import generate_with_openai


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
        """Add entity to cluster if it matches category and name."""
        if entity.category == self.category and entity.entity_name == self.cluster_name:
            self.entities.append(entity)
            return True
        return False

    def write_entities_to_disk(self, analysis_dir: Path) -> None:
        """Write all entities to disk and collect file paths."""
        for entity in self.entities:
            file_path = entity.write_to_disk(analysis_dir)
            if entity.entity_type == "json":
                self.json_files.append(file_path)
            else:
                self.html_files.append(file_path)

    def can_generate_models(self) -> bool:
        """Check if cluster has enough samples for generation."""
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
        """Gather HTML and JSON samples within thresholds."""
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
            "- Do NOT output code, markdown fences, or commentary. JSON only."
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
        """Extract connection information from inventory for container phases."""
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
        """Generate AI-powered models using two-stage process (idempotent)."""
        if not self.can_generate_models():
            return GenerationResults(success=False, analysis_notes=["No sample files for analysis"]) 

        models_file = models_dir / f"{self.category}.py"
        
        # Check if model file already exists (idempotent)
        if models_file.exists():
            logger.info(f"Model already exists for {self.category}: {models_file}")
            
            # Still need to return connections info for container generation
            # Try to infer connections from existing file (simplified approach)
            html_samples, json_samples = self._gather_samples()
            self.generation_results = GenerationResults(
                models_generated=[str(models_file)],
                analysis_notes=[f"Model already exists, skipped generation"],
                connections=ModelConnections(
                    import_path=f"generator.processors.models.{self.category}",
                    exported_classes=[self.category.title()]
                ),
                success=True,
            )
            return self.generation_results

        try:
            inventory = self._stage_a_inventory(templates_dir, logger)
            models_content = self._stage_b_render_code(inventory)

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
