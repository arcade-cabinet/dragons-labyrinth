"""
Processing protocols for entity processors.

Following .clinerules standards:
- Protocol definitions for processor interfaces
- Modern Python typing (dict/list not Dict/List)
- Absolute imports only
"""

from __future__ import annotations

from typing import Protocol, Any, runtime_checkable

from generator.entities.types import (
    EntityId, HexCoordinate, ProcessorType,
    SettlementScale, DungeonType, BiomeType
)


@runtime_checkable
class EntityCluster(Protocol):
    """Protocol for entity cluster containers."""
    
    category: str
    name: str
    entities: list[dict[str, Any]]
    processor_type: str
    
    def add_entity(self, entity: dict[str, Any]) -> None:
        """Add entity to cluster."""
        ...
    
    def get_entity_count(self) -> int:
        """Get entity count."""
        ...


@runtime_checkable
class EntityProcessor(Protocol):
    """Protocol for specialized entity processors."""
    
    processor_type: str
    
    def process_cluster(self, cluster: EntityCluster, logger: Any, console: Any) -> dict[str, Any]:
        """Process entity cluster and return results."""
        ...
    
    def _extract_specific_data(self, cluster: EntityCluster, ml_results: dict[str, Any], logger: Any, console: Any) -> dict[str, Any]:
        """Extract processor-specific data."""
        ...
    
    def _generate_bevy_hooks(self, cluster: EntityCluster, specific_data: dict[str, Any]) -> dict[str, Any]:
        """Generate Bevy ECS integration hooks."""
        ...


@runtime_checkable
class TemplateRenderer(Protocol):
    """Protocol for Rust ECS template rendering."""
    
    def _prepare_template_data(self, result: dict[str, Any]) -> dict[str, Any]:
        """Prepare data for template rendering."""
        ...
    
    def _generate_rust_ecs_code(self, result: dict[str, Any], logger: Any, console: Any) -> str | None:
        """Generate Rust ECS code using templates."""
        ...


@runtime_checkable
class AnalysisExtractor(Protocol):
    """Protocol for extracting analysis data from entities."""
    
    def extract_scale_from_name(self, name: str) -> SettlementScale:
        """Extract settlement scale from name."""
        ...
    
    def extract_biome_from_content(self, content: str) -> BiomeType:
        """Extract biome type from content."""
        ...
    
    def extract_dungeon_type_from_name(self, name: str) -> DungeonType:
        """Extract dungeon type from name."""
        ...


@runtime_checkable
class RustCodeGenerator(Protocol):
    """Protocol for generating Rust ECS code."""
    
    def generate_components(self, data: list[dict[str, Any]]) -> str:
        """Generate Rust ECS components."""
        ...
    
    def generate_systems(self, data: list[dict[str, Any]]) -> str:
        """Generate Rust ECS systems."""
        ...
    
    def update_mod_file(self, module_names: list[str]) -> None:
        """Update mod.rs with generated modules."""
        ...


@runtime_checkable
class CrossValidator(Protocol):
    """Protocol for cross-validation between JSON and HTML processing."""
    
    def validate_processing_quality(self, html_results: list[dict[str, Any]], json_results: list[dict[str, Any]]) -> dict[str, Any]:
        """Validate processing quality using JSON as ground truth."""
        ...
    
    def generate_improvement_recommendations(self, validation_results: dict[str, Any]) -> dict[str, list[str]]:
        """Generate improvement recommendations from validation gaps."""
        ...
