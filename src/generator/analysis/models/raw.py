"""Raw entity models for HBF data extraction.

Following .clinerules standards with modern Python type hints.
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from pydantic import BaseModel

from generator.analysis.constants import (
    REGIONS,
    SETTLEMENTS,
    FACTIONS,
    DUNGEONS,
)


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
    def create(cls, uuid: str, raw_value: str) -> RawEntity:
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
        """Write entity to disk and return file path (idempotent)."""
        if not self.category or not self.entity_name:
            raise ValueError(f"Entity {self.uuid} not properly categorized")
            
        # Create directory structure
        entity_dir = analysis_dir / self.category / self.get_sanitized_name()
        entity_dir.mkdir(parents=True, exist_ok=True)
        
        # Check if file already exists (idempotent)
        ext = "json" if self.entity_type == "json" else "html"
        filename = f"entity_{self.uuid}.{ext}"
        file_path = entity_dir / filename
        
        if file_path.exists():
            # File already exists, skip writing
            self.file_path = file_path
            return file_path
        
        # Write file only if it doesn't exist
        if self.entity_type == "json":
            with open(file_path, 'w', encoding='utf-8') as f:
                json.dump(self.data, f, indent=2, ensure_ascii=False)
        else:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(self.raw_value)
        
        self.file_path = file_path
        return file_path
