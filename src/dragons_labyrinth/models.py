"""
Pydantic models for Dragon's Labyrinth HBF analysis.

This module defines Pydantic models following professor-pixels standards with
comprehensive type hints, Field descriptions, and ConfigDict usage.
"""

from __future__ import annotations

import sqlite3
import logging
from datetime import datetime
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from rich.console import Console
import pandas as pd
import networkx as nx

from dragons_labyrinth.types import (
    SliceType, AnalysisDepth, IntegrationLevel, WorkflowStage, ApprovalStatus,
    ComponentType, GenerationType, EntityType, AnalysisStatus, WorkflowEvent,
    IntegrationPoint, PatternCategory, OutputFormat, ValidationLevel,
    DreadLevel, PhilosophyPath, CompanionStress, CorruptionLevel,
    EntityID, WorkflowID, SliceID, PatternID, ComponentID,
    EntityDict, PatternDict, ComponentDict, ReviewData, StateDict,
    HyperlinkRef, HTMLContent, JSONContent,
    EntityCollection, PatternCollection, ComponentCollection,
    IntegrationPointList, HyperlinkCollection, ProbabilityTableCollection
)


class HBFConfig(BaseModel):
    """Configuration for HBF loading and processing"""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    hbf_path: Path = Field(description="Path to HBF database file")
    output_dir: Path = Field(description="Output directory for results")
    batch_size: int = Field(default=1000, description="Batch size for processing")
    
    def model_post_init(self, __context: Any) -> None:
        """Ensure paths are Path objects and output dir exists"""
        self.hbf_path = Path(self.hbf_path)
        self.output_dir = Path(self.output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)


class OrchestratorState(BaseModel):
    """
    Shared state for all HBF orchestrator components.
    Passed to all mixins to provide standardized access to data and utilities.
    """
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    # Core data
    entities_df: pd.DataFrame | None = Field(default=None, description="Entities DataFrame")
    refs_df: pd.DataFrame | None = Field(default=None, description="References DataFrame")
    
    # Database connections
    sqlite_conn: Any = Field(default=None, description="Raw SQLite connection")  # sqlite3.Connection
    sqlalchemy_engine: Any = Field(default=None, description="SQLAlchemy engine")
    
    # Utilities
    log: Any = Field(default=None, description="Logger instance")  # logging.Logger
    console: Any = Field(default=None, description="Rich console instance")  # Console
    
    # Configuration
    config: HBFConfig = Field(description="HBF configuration")


class EntityData(BaseModel):
    """Parsed entity data from HBF"""
    model_config = ConfigDict(extra="allow")
    
    entity_type: str = Field(default="unknown", description="Type of entity")
    name: str | None = Field(default=None, description="Entity name")
    content: str | None = Field(default=None, description="HTML content")
    description: str | None = Field(default=None, description="Entity description")
    
    # Relationships
    faction: str | None = Field(default=None, description="Faction reference")
    leader: str | None = Field(default=None, description="Leader reference")
    location: str | None = Field(default=None, description="Location reference")
    parent: str | None = Field(default=None, description="Parent entity reference")
    dungeon: str | None = Field(default=None, description="Dungeon reference")
    region: str | None = Field(default=None, description="Region reference")
    hex: str | None = Field(default=None, description="Hex reference")
    
    # List fields
    members: list[str] = Field(default_factory=list, description="Member references")
    collaborators: list[str] = Field(default_factory=list, description="Collaborator references")
    connections: list[str] = Field(default_factory=list, description="Connection references")
    rooms: list[str] = Field(default_factory=list, description="Room references")
    items: list[str] = Field(default_factory=list, description="Item references")


class Entity(BaseModel):
    """Complete entity from HBF database"""
    uuid: str = Field(description="Entity UUID")
    value: str = Field(description="Raw JSON value")
    data: EntityData | dict = Field(default_factory=dict, description="Parsed entity data")
    entity_type: str = Field(default="unknown", description="Entity type extracted from data")


class Reference(BaseModel):
    """Reference between entities"""
    uuid: str = Field(description="Source entity UUID")
    value: str = Field(description="Target entity UUID")
    ref_type: str = Field(default="unknown", description="Type of reference")


class Cluster(BaseModel):
    """Cluster of entities for batch processing"""
    cluster_id: str = Field(description="Unique cluster identifier")
    entity_type: str = Field(description="Type of entities in cluster")
    entity_ids: list[str] = Field(default_factory=list, description="List of entity UUIDs")
    total_tokens: int = Field(default=0, description="Total token count")
    entity_count: int = Field(default=0, description="Number of entities")


class GraphMetrics(BaseModel):
    """Metrics for entity relationship graph"""
    node_count: int = Field(description="Total number of nodes")
    edge_count: int = Field(description="Total number of edges")
    density: float = Field(description="Graph density")
    components: int = Field(description="Number of weakly connected components")
    avg_in_degree: float | None = Field(default=None, description="Average in-degree")
    max_in_degree: int | None = Field(default=None, description="Maximum in-degree")
    avg_out_degree: float | None = Field(default=None, description="Average out-degree")
    max_out_degree: int | None = Field(default=None, description="Maximum out-degree")


class HBFSummary(BaseModel):
    """Summary statistics for HBF database"""
    total_entities: int = Field(description="Total number of entities")
    entities_with_content: int = Field(description="Entities with non-empty content")
    empty_entities: int = Field(description="Entities with empty content")
    unique_entity_types: int = Field(description="Number of unique entity types")
    total_references: int = Field(description="Total number of references")
    type_distribution: dict[str, int] = Field(
        default_factory=dict,
        description="Distribution of entity types"
    )


# ===========================
# Workflow and Analysis Models
# ===========================

class HBFSliceAnalysisState(BaseModel):
    """State schema for HBF slice analysis workflow."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    # Input configuration
    slice_type: SliceType = Field(description="Type of slice being analyzed")
    hbf_path: Path = Field(description="Path to HBF database")
    output_dir: Path = Field(description="Output directory for results")
    
    # Analysis configuration
    target_entity_count: int = Field(default=50, description="Expected entities in slice")
    html_analysis_depth: AnalysisDepth = Field(default="standard", description="Level of HTML analysis")
    integration_level: IntegrationLevel = Field(default="horror_aware", description="Horror integration level")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    
    # Entity processing
    raw_entities: EntityCollection = Field(default_factory=list, description="Raw entities from HBF")
    html_content: dict[str, HTMLContent] = Field(default_factory=dict, description="Extracted HTML content")
    json_content: dict[str, JSONContent] = Field(default_factory=dict, description="Extracted JSON content")
    probability_tables: dict[str, ProbabilityTableCollection] = Field(default_factory=dict, description="Probability tables found")
    hyperlink_refs: dict[str, HyperlinkCollection] = Field(default_factory=dict, description="Hyperlink relationships")
    
    # Analysis results
    slice_patterns: PatternCollection = Field(default_factory=list, description="Discovered patterns in slice")
    integration_points: IntegrationPointList = Field(default_factory=list, description="Horror RPG integration points")
    entity_relationships: dict[str, list[str]] = Field(default_factory=dict, description="Entity relationship mappings")
    
    # Human review
    human_approval: ApprovalStatus | None = Field(default=None, description="Human review status")
    human_feedback: dict[str, str] = Field(default_factory=dict, description="Human feedback")
    review_data: ReviewData = Field(default_factory=dict, description="Data for human review")
    
    # Transformation results
    bevy_components: ComponentCollection = Field(default_factory=list, description="Generated Bevy components")
    rust_code: str | None = Field(default=None, description="Generated Rust code")
    component_specs: list[ComponentSpecification] = Field(default_factory=list, description="Component specifications")
    
    # Final output
    processed_entity_ids: list[EntityID] = Field(default_factory=list, description="Successfully processed entities")
    output_files: list[str] = Field(default_factory=list, description="Generated output files")
    validation_results: list[ValidationResult] = Field(default_factory=list, description="Validation results")
    
    # Workflow metadata
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    current_stage: WorkflowStage = Field(default="initialization", description="Current workflow stage")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Number of completed steps")
    status: AnalysisStatus = Field(default=AnalysisStatus.NOT_STARTED, description="Current analysis status")


class PatternSuggestion(BaseModel):
    """AI-discovered pattern in HBF slice data."""
    model_config = ConfigDict(extra="forbid")
    
    pattern_id: PatternID = Field(description="Unique pattern identifier")
    pattern_name: str = Field(description="Human-readable pattern name")
    pattern_category: PatternCategory = Field(description="Category of pattern")
    description: str = Field(description="Pattern description")
    
    # Pattern data
    source_entities: list[EntityID] = Field(default_factory=list, description="Entities this pattern was found in")
    pattern_data: PatternDict = Field(default_factory=dict, description="Raw pattern data")
    confidence_score: float = Field(ge=0.0, le=1.0, description="Confidence in pattern accuracy")
    
    # Horror integration
    integration_points: list[IntegrationPoint] = Field(default_factory=list, description="Horror integration points")
    dread_level_impact: DreadLevel | None = Field(default=None, description="Impact on dread level")
    philosophy_alignment: list[PhilosophyPath] = Field(default_factory=list, description="Philosophy path alignment")
    companion_stress_triggers: list[CompanionStress] = Field(default_factory=list, description="Companion stress triggers")
    
    # Technical details
    complexity_level: int = Field(ge=1, le=5, description="Implementation complexity")
    dependencies: list[PatternID] = Field(default_factory=list, description="Dependent patterns")
    frequency_score: float = Field(ge=0.0, le=1.0, description="How frequently this pattern occurs")
    
    # Metadata
    discovered_at: datetime = Field(default_factory=datetime.now, description="Discovery timestamp")
    last_validated: datetime | None = Field(default=None, description="Last validation timestamp")


class ComponentSpecification(BaseModel):
    """Specification for generated Bevy component."""
    model_config = ConfigDict(extra="forbid")
    
    component_id: ComponentID = Field(description="Unique component identifier")
    component_name: str = Field(description="Component struct name")
    component_type: ComponentType = Field(description="Type of component")
    generation_type: GenerationType = Field(description="Type of generation")
    
    # Component structure
    fields: dict[str, dict[str, Any]] = Field(default_factory=dict, description="Component fields and types")
    derives: list[str] = Field(default_factory=list, description="Rust derive traits")
    attributes: list[str] = Field(default_factory=list, description="Rust attributes")
    
    # Horror integration
    horror_integrations: dict[str, Any] = Field(default_factory=dict, description="Horror system integrations")
    dread_responsive_fields: list[str] = Field(default_factory=list, description="Fields that respond to dread")
    corruption_effects: dict[str, Any] = Field(default_factory=dict, description="Corruption effect mappings")
    
    # Source patterns
    source_patterns: list[PatternID] = Field(default_factory=list, description="Patterns this component implements")
    pattern_data: PatternDict = Field(default_factory=dict, description="Pattern data used")
    
    # Generated code
    rust_struct_code: str | None = Field(default=None, description="Generated Rust struct code")
    impl_code: str | None = Field(default=None, description="Generated implementation code")
    system_code: str | None = Field(default=None, description="Generated system code")
    
    # Validation
    validation_level: ValidationLevel = Field(default=ValidationLevel.SYNTAX_CHECK, description="Validation level")
    validation_errors: list[str] = Field(default_factory=list, description="Validation errors")
    
    # Metadata
    generated_at: datetime = Field(default_factory=datetime.now, description="Generation timestamp")
    generator_version: str = Field(default="1.0.0", description="Generator version")


class IntegrationMapping(BaseModel):
    """Mapping between HBF data and horror RPG systems."""
    model_config = ConfigDict(extra="forbid")
    
    slice_type: SliceType = Field(description="Type of slice being integrated")
    integration_points: list[IntegrationPoint] = Field(description="Integration points used")
    
    # Dread system mappings
    dread_amplifiers: dict[str, float] = Field(default_factory=dict, description="Dread amplification mappings")
    dread_triggers: dict[str, list[str]] = Field(default_factory=dict, description="Dread trigger conditions")
    
    # Philosophy system mappings
    philosophy_alignments: dict[PhilosophyPath, float] = Field(default_factory=dict, description="Philosophy path alignments")
    moral_choice_points: list[dict[str, Any]] = Field(default_factory=list, description="Moral choice opportunities")
    
    # Companion system mappings
    stress_triggers: dict[CompanionStress, list[str]] = Field(default_factory=dict, description="Companion stress triggers")
    comfort_sources: dict[str, float] = Field(default_factory=dict, description="Companion comfort sources")
    
    # Environmental mappings
    corruption_spreaders: dict[str, CorruptionLevel] = Field(default_factory=dict, description="Corruption source mappings")
    decay_accelerators: list[str] = Field(default_factory=list, description="Decay acceleration triggers")
    
    # Narrative integration
    horror_moments: list[dict[str, Any]] = Field(default_factory=list, description="Key horror story moments")
    revelation_triggers: list[str] = Field(default_factory=list, description="Truth revelation conditions")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    validated: bool = Field(default=False, description="Whether mapping is validated")


class SliceAnalysisRequest(BaseModel):
    """Request for slice analysis workflow."""
    model_config = ConfigDict(extra="forbid")
    
    slice_type: SliceType = Field(description="Type of slice to analyze")
    hbf_path: Path = Field(description="Path to HBF database")
    output_dir: Path = Field(description="Output directory")
    
    # Analysis configuration
    target_entity_count: int | None = Field(default=None, description="Expected entity count")
    html_analysis_depth: AnalysisDepth = Field(default="standard", description="HTML analysis depth")
    integration_level: IntegrationLevel = Field(default="horror_aware", description="Integration level")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    
    # Workflow configuration
    workflow_id: WorkflowID | None = Field(default=None, description="Optional workflow ID")
    checkpoint_interval: int = Field(default=5, description="Steps between checkpoints")
    max_retries: int = Field(default=3, description="Maximum retry attempts")
    
    # Generated fields
    created_at: datetime = Field(default_factory=datetime.now, description="Request creation time")
    
    def model_post_init(self, __context: Any) -> None:
        """Generate workflow ID if not provided."""
        if self.workflow_id is None:
            import uuid
            self.workflow_id = f"{self.slice_type}_{uuid.uuid4().hex[:8]}"


class SliceAnalysisResult(BaseModel):
    """Result from slice analysis workflow."""
    model_config = ConfigDict(extra="forbid")
    
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    slice_type: SliceType = Field(description="Type of slice analyzed")
    status: AnalysisStatus = Field(description="Final analysis status")
    
    # Analysis results
    entities_processed: int = Field(description="Number of entities processed")
    patterns_discovered: int = Field(description="Number of patterns found")
    components_generated: int = Field(description="Number of components generated")
    integration_points_mapped: int = Field(description="Number of integration points")
    
    # Generated outputs
    output_files: list[str] = Field(default_factory=list, description="Generated file paths")
    rust_code_files: list[str] = Field(default_factory=list, description="Generated Rust files")
    analysis_files: list[str] = Field(default_factory=list, description="Analysis result files")
    
    # Quality metrics
    validation_passed: bool = Field(description="Whether validation passed")
    confidence_scores: dict[str, float] = Field(default_factory=dict, description="Confidence scores")
    coverage_percentage: float = Field(ge=0.0, le=100.0, description="Slice coverage percentage")
    
    # Human review
    human_reviewed: bool = Field(default=False, description="Whether human reviewed")
    human_approval: ApprovalStatus | None = Field(default=None, description="Human approval status")
    review_comments: list[str] = Field(default_factory=list, description="Human review comments")
    
    # Performance metrics
    processing_time_seconds: float = Field(description="Total processing time")
    memory_usage_mb: float | None = Field(default=None, description="Peak memory usage")
    api_calls_made: int = Field(default=0, description="Number of AI API calls")
    
    # Metadata
    started_at: datetime = Field(description="Start timestamp")
    completed_at: datetime = Field(description="Completion timestamp")
    generator_version: str = Field(default="1.0.0", description="Generator version")


class ValidationResult(BaseModel):
    """Result of validation check."""
    model_config = ConfigDict(extra="forbid")
    
    validation_type: ValidationLevel = Field(description="Type of validation performed")
    target_file: str = Field(description="File being validated")
    passed: bool = Field(description="Whether validation passed")
    
    # Error details
    errors: list[str] = Field(default_factory=list, description="Validation errors")
    warnings: list[str] = Field(default_factory=list, description="Validation warnings")
    suggestions: list[str] = Field(default_factory=list, description="Improvement suggestions")
    
    # Metrics
    error_count: int = Field(default=0, description="Number of errors")
    warning_count: int = Field(default=0, description="Number of warnings")
    confidence_score: float = Field(ge=0.0, le=1.0, default=1.0, description="Validation confidence")
    
    # Metadata
    validated_at: datetime = Field(default_factory=datetime.now, description="Validation timestamp")
    validator_version: str = Field(default="1.0.0", description="Validator version")


class WorkflowCheckpoint(BaseModel):
    """Checkpoint data for workflow resumption."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    checkpoint_id: str = Field(description="Checkpoint identifier")
    current_stage: WorkflowStage = Field(description="Current workflow stage")
    
    # State data
    workflow_state: StateDict = Field(description="Complete workflow state")
    intermediate_results: dict[str, Any] = Field(default_factory=dict, description="Intermediate results")
    error_recovery_data: dict[str, Any] = Field(default_factory=dict, description="Error recovery information")
    
    # Progress tracking
    completed_steps: list[str] = Field(default_factory=list, description="Completed workflow steps")
    failed_steps: list[str] = Field(default_factory=list, description="Failed workflow steps")
    retry_counts: dict[str, int] = Field(default_factory=dict, description="Retry counts per step")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Checkpoint creation time")
    resumable_until: datetime | None = Field(default=None, description="Checkpoint expiry time")
    checkpoint_version: str = Field(default="1.0.0", description="Checkpoint format version")


class MemoryBankEntry(BaseModel):
    """Entry in the workflow memory bank."""
    model_config = ConfigDict(extra="forbid")
    
    entry_id: str = Field(description="Memory entry identifier")
    entry_type: str = Field(description="Type of memory entry")
    slice_type: SliceType | None = Field(default=None, description="Associated slice type")
    
    # Content
    title: str = Field(description="Memory entry title")
    description: str = Field(description="Detailed description")
    content: dict[str, Any] = Field(default_factory=dict, description="Entry content")
    tags: list[str] = Field(default_factory=list, description="Searchable tags")
    
    # Relationships
    related_patterns: list[PatternID] = Field(default_factory=list, description="Related patterns")
    related_components: list[ComponentID] = Field(default_factory=list, description="Related components")
    cross_references: list[str] = Field(default_factory=list, description="Cross-references")
    
    # Quality metrics
    importance_score: float = Field(ge=0.0, le=1.0, default=0.5, description="Importance score")
    confidence_level: float = Field(ge=0.0, le=1.0, default=1.0, description="Confidence level")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    updated_at: datetime = Field(default_factory=datetime.now, description="Last update timestamp")
    access_count: int = Field(default=0, description="Number of times accessed")
    creator: str = Field(default="system", description="Entry creator")
