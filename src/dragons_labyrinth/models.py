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


# =============================================================================
# Asset Generation Models (extending existing architecture)
# =============================================================================

class AssetGenerationState(BaseModel):
    """State schema for asset generation workflow following professor-pixels patterns."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    # Input configuration
    asset_category: str = Field(description="Category of assets to generate (biome, character, etc.)")
    level_range: str = Field(description="Level range for assets (1-20, 21-40, etc.)")
    toml_spec_path: Path = Field(description="Path to TOML specification file")
    output_dir: Path = Field(description="Output directory for generated assets")
    
    # Generation configuration
    batch_size: int = Field(default=5, description="Number of assets to generate per batch")
    quality_level: str = Field(default="standard", description="Quality level (draft, standard, hd)")
    style_consistency: bool = Field(default=True, description="Enforce style consistency")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    
    # TOML processing
    raw_toml_data: dict[str, Any] = Field(default_factory=dict, description="Raw TOML specification data")
    parsed_asset_specs: list[dict[str, Any]] = Field(default_factory=list, description="Parsed asset specifications")
    prompt_templates: dict[str, str] = Field(default_factory=dict, description="DALL-E prompt templates")
    
    # Generation results
    generated_assets: dict[str, str] = Field(default_factory=dict, description="Generated asset file paths")
    generation_metadata: dict[str, Any] = Field(default_factory=dict, description="Generation metadata")
    failed_generations: list[str] = Field(default_factory=list, description="Failed asset names")
    
    # Human review
    human_approval: ApprovalStatus | None = Field(default=None, description="Human review status")
    human_feedback: dict[str, str] = Field(default_factory=dict, description="Human feedback")
    review_data: ReviewData = Field(default_factory=dict, description="Data for human review")
    
    # Quality validation
    validation_results: list[ValidationResult] = Field(default_factory=list, description="Asset validation results")
    consistency_scores: dict[str, float] = Field(default_factory=dict, description="Style consistency scores")
    
    # Integration with game engine
    asset_registry_updates: dict[str, Any] = Field(default_factory=dict, description="Updates to asset registry")
    bevy_integration_code: str | None = Field(default=None, description="Generated Bevy integration code")
    
    # Workflow metadata
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    current_stage: WorkflowStage = Field(default="initialization", description="Current workflow stage")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Number of completed steps")
    status: AnalysisStatus = Field(default=AnalysisStatus.NOT_STARTED, description="Current generation status")


class AssetSpecification(BaseModel):
    """Specification for a single asset following TOML structure."""
    model_config = ConfigDict(extra="forbid")
    
    # Basic asset info
    asset_name: str = Field(description="Asset filename (without extension)")
    asset_category: str = Field(description="Category (biome, character, monster, etc.)")
    asset_type: str = Field(description="Specific type within category")
    level_range: str = Field(description="Level range this asset applies to")
    
    # DALL-E generation
    prompt: str = Field(description="DALL-E prompt with consistency constraints")
    size: str = Field(default="1024x1024", description="Image dimensions")
    quality: str = Field(default="standard", description="Image quality")
    style: str = Field(default="natural", description="Style parameter")
    
    # Layer cake system
    layer_type: str = Field(description="Layer in compositing system (base, overlay, effect)")
    transparency: bool = Field(default=False, description="Requires transparent background")
    tileable: bool = Field(default=False, description="Must be seamlessly tileable")
    
    # Horror progression
    dread_level: int = Field(ge=0, le=4, default=0, description="Associated dread level")
    corruption_variant: str | None = Field(default=None, description="Corruption variant (cursed, nightmare, void)")
    philosophy_alignment: list[str] = Field(default_factory=list, description="Philosophy path alignment")
    
    # Technical constraints
    consistency_constraints: list[str] = Field(default_factory=list, description="Style consistency requirements")
    negative_prompts: list[str] = Field(default_factory=list, description="Elements to avoid")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    priority: int = Field(default=5, ge=1, le=10, description="Generation priority")


class AssetGenerationRequest(BaseModel):
    """Request for asset generation workflow."""
    model_config = ConfigDict(extra="forbid")
    
    asset_category: str = Field(description="Category of assets to generate")
    level_range: str = Field(description="Level range (1-20, 21-40, etc.)")
    toml_spec_path: Path = Field(description="Path to TOML specification")
    output_dir: Path = Field(description="Output directory")
    
    # Generation configuration
    batch_size: int = Field(default=5, description="Assets per batch")
    quality_level: str = Field(default="standard", description="Quality level")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    
    # Workflow configuration
    workflow_id: WorkflowID | None = Field(default=None, description="Optional workflow ID")
    checkpoint_interval: int = Field(default=3, description="Steps between checkpoints")
    max_retries: int = Field(default=2, description="Maximum retry attempts")
    
    # Generated fields
    created_at: datetime = Field(default_factory=datetime.now, description="Request creation time")
    
    def model_post_init(self, __context: Any) -> None:
        """Generate workflow ID if not provided."""
        if self.workflow_id is None:
            import uuid
            self.workflow_id = f"assets_{self.asset_category}_{uuid.uuid4().hex[:8]}"


class AssetGenerationResult(BaseModel):
    """Result from asset generation workflow."""
    model_config = ConfigDict(extra="forbid")
    
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    asset_category: str = Field(description="Category of assets generated")
    level_range: str = Field(description="Level range processed")
    status: AnalysisStatus = Field(description="Final generation status")
    
    # Generation results
    assets_requested: int = Field(description="Number of assets requested")
    assets_generated: int = Field(description="Number of assets successfully generated")
    assets_failed: int = Field(description="Number of failed generations")
    
    # Generated outputs
    asset_files: dict[str, str] = Field(default_factory=dict, description="Generated asset file paths")
    metadata_files: list[str] = Field(default_factory=list, description="Generation metadata files")
    integration_files: list[str] = Field(default_factory=list, description="Game engine integration files")
    
    # Quality metrics
    validation_passed: bool = Field(description="Whether validation passed")
    consistency_scores: dict[str, float] = Field(default_factory=dict, description="Style consistency scores")
    average_quality_score: float = Field(ge=0.0, le=1.0, default=0.0, description="Average quality score")
    
    # Human review
    human_reviewed: bool = Field(default=False, description="Whether human reviewed")
    human_approval: ApprovalStatus | None = Field(default=None, description="Human approval status")
    review_comments: list[str] = Field(default_factory=list, description="Human review comments")
    
    # Performance metrics
    processing_time_seconds: float = Field(description="Total processing time")
    api_calls_made: int = Field(default=0, description="Number of DALL-E API calls")
    total_cost_usd: float = Field(default=0.0, description="Total generation cost")
    
    # Metadata
    started_at: datetime = Field(description="Start timestamp")
    completed_at: datetime = Field(description="Completion timestamp")
    generator_version: str = Field(default="1.0.0", description="Generator version")


class TOMLAssetBatch(BaseModel):
    """Batch configuration for asset generation from TOML."""
    model_config = ConfigDict(extra="forbid")
    
    batch_name: str = Field(description="Batch identifier")
    level_range: str = Field(description="Level range for this batch")
    asset_category: str = Field(description="Primary asset category")
    
    # Asset specifications
    asset_specs: list[AssetSpecification] = Field(default_factory=list, description="Asset specifications")
    total_assets: int = Field(default=0, description="Total number of assets in batch")
    
    # Generation settings
    style_constraints: dict[str, Any] = Field(default_factory=dict, description="Global style constraints")
    consistency_rules: list[str] = Field(default_factory=list, description="Consistency rules")
    
    # Horror integration
    dread_progression: dict[str, Any] = Field(default_factory=dict, description="Dread level progression")
    corruption_mapping: dict[str, str] = Field(default_factory=dict, description="Corruption variant mapping")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Creation timestamp")
    priority: int = Field(default=5, ge=1, le=10, description="Batch priority")


# =============================================================================
# Variant Asset Generation Models (Revolutionary Architecture)
# =============================================================================

class VariantDimension(BaseModel):
    """Single variant dimension with possible values."""
    model_config = ConfigDict(extra="forbid")
    
    dimension_name: str = Field(description="Name of variant dimension (e.g., 'skin_tone', 'corruption')")
    possible_values: list[str] = Field(description="All possible values for this dimension")
    default_value: str | None = Field(default=None, description="Default value if not specified")
    description: str = Field(description="Human-readable description")
    
    # Constraints
    required: bool = Field(default=True, description="Whether this dimension is required")
    exclude_combinations: list[list[str]] = Field(default_factory=list, description="Value combinations to exclude")


class VariantConfiguration(BaseModel):
    """Configuration for variant generation from universal TOML."""
    model_config = ConfigDict(extra="forbid")
    
    # Variant dimensions
    dimensions: dict[str, VariantDimension] = Field(default_factory=dict, description="All variant dimensions")
    dimension_descriptors: dict[str, dict[str, str]] = Field(default_factory=dict, description="Value descriptors for substitution")
    
    # Generation rules
    max_variants_per_archetype: int = Field(default=30, description="Limit combinatorial explosion")
    priority_dimensions: list[str] = Field(default_factory=list, description="Generate these dimensions first")
    exclude_combinations: list[list[str]] = Field(default_factory=list, description="Global exclusion rules")
    
    # Sprite sheet configuration
    sprite_sheet_grouping: str = Field(description="How to group variants (archetype, category, etc.)")
    naming_convention: str = Field(description="Template for variant naming")
    
    # Resolution optimization
    resolution_tier: str = Field(description="Resolution tier for this asset type")


class ResolutionTier(BaseModel):
    """Resolution optimization tier configuration."""
    model_config = ConfigDict(extra="forbid")
    
    tier_name: str = Field(description="Name of resolution tier")
    resolution: str = Field(description="Image resolution (e.g., '512x512')")
    use_case: str = Field(description="What this resolution is optimized for")
    
    # Performance settings
    batch_size_multiplier: float = Field(default=1.0, description="Batch size adjustment for this resolution")
    sprite_sheet_compatible: bool = Field(default=True, description="Whether suitable for sprite sheets")
    
    # Quality settings
    quality_override: str | None = Field(default=None, description="Quality override for this tier")
    style_override: str | None = Field(default=None, description="Style override for this tier")


class VariantAssetSpec(BaseModel):
    """Individual variant asset specification generated from combinations."""
    model_config = ConfigDict(extra="forbid")
    
    # Asset identification
    asset_name: str = Field(description="Generated asset name with variant suffixes")
    base_archetype: str = Field(description="Base archetype (knight, goblin, plains, etc.)")
    variant_combination: dict[str, str] = Field(description="Specific variant values used")
    
    # Generation details
    final_prompt: str = Field(description="DALL-E prompt with all substitutions applied")
    resolution: str = Field(description="Optimized resolution for this asset")
    quality: str = Field(default="standard", description="Quality setting")
    style: str = Field(default="natural", description="Style setting")
    
    # Asset properties
    asset_category: str = Field(description="Category (character, biome, monster)")
    asset_type: str = Field(description="Specific type within category")
    layer_type: str = Field(description="Layer type (base, overlay, token, effect)")
    priority: int = Field(description="Layer cake priority")
    
    # Sprite sheet metadata
    sprite_sheet_group: str = Field(description="Which sprite sheet this belongs to")
    expected_cell_size: tuple[int, int] | None = Field(default=None, description="Expected cell size in sprite sheet")
    
    # Metadata
    generated_at: datetime = Field(default_factory=datetime.now, description="Generation timestamp")


class SpriteSheetCell(BaseModel):
    """Individual cell in a sprite sheet."""
    model_config = ConfigDict(extra="forbid")
    
    asset_name: str = Field(description="Name of asset in this cell")
    cell_index: int = Field(description="Index in sprite sheet")
    
    # Position and size
    x: int = Field(description="X coordinate in sprite sheet")
    y: int = Field(description="Y coordinate in sprite sheet")
    width: int = Field(description="Cell width in pixels")
    height: int = Field(description="Cell height in pixels")
    
    # Variant data
    variant_combination: dict[str, str] = Field(default_factory=dict, description="Variant values for this cell")
    base_archetype: str = Field(description="Base archetype")
    
    # Validation
    validated: bool = Field(default=False, description="Whether cell content was validated")
    validation_score: float = Field(default=0.0, ge=0.0, le=1.0, description="Validation confidence score")


class SpriteSheetMetadata(BaseModel):
    """Complete sprite sheet atlas metadata."""
    model_config = ConfigDict(extra="forbid")
    
    # Sheet identification
    sheet_name: str = Field(description="Sprite sheet filename")
    sheet_category: str = Field(description="Asset category for this sheet")
    base_archetype: str = Field(description="Base archetype (if single archetype)")
    
    # Dimensions
    sheet_size: tuple[int, int] = Field(description="Total sprite sheet dimensions (width, height)")
    cell_size: tuple[int, int] = Field(description="Individual cell dimensions")
    grid_size: tuple[int, int] = Field(description="Grid dimensions (cols, rows)")
    
    # Content
    cells: list[SpriteSheetCell] = Field(default_factory=list, description="All cells in sprite sheet")
    total_variants: int = Field(description="Total number of variants in sheet")
    
    # Generation metadata
    variant_dimensions_used: list[str] = Field(default_factory=list, description="Which variant dimensions were used")
    resolution_tier: str = Field(description="Resolution tier used")
    
    # Validation
    sheet_validated: bool = Field(default=False, description="Whether entire sheet was validated")
    missing_cells: list[int] = Field(default_factory=list, description="Indices of missing/failed cells")
    
    # Metadata
    generated_at: datetime = Field(default_factory=datetime.now, description="Generation timestamp")
    file_size_bytes: int | None = Field(default=None, description="Generated file size")


class CombinatorialGeneration(BaseModel):
    """Results of combinatorial variant generation."""
    model_config = ConfigDict(extra="forbid")
    
    # Source configuration
    base_archetype: str = Field(description="Base archetype being expanded")
    variant_config: VariantConfiguration = Field(description="Variant configuration used")
    
    # Generated specifications
    generated_specs: list[VariantAssetSpec] = Field(default_factory=list, description="All generated variant specs")
    total_combinations: int = Field(description="Total possible combinations")
    generated_combinations: int = Field(description="Actually generated combinations")
    excluded_combinations: int = Field(description="Combinations excluded by rules")
    
    # Sprite sheet planning
    sprite_sheets: list[SpriteSheetMetadata] = Field(default_factory=list, description="Planned sprite sheets")
    total_sprite_sheets: int = Field(description="Number of sprite sheets needed")
    
    # Performance estimates
    estimated_generation_time: float = Field(description="Estimated time in seconds")
    estimated_cost_usd: float = Field(description="Estimated generation cost")
    estimated_file_size_mb: float = Field(description="Estimated total file size")
    
    # Metadata
    generated_at: datetime = Field(default_factory=datetime.now, description="Generation timestamp")


class VariantAssetGenerationState(BaseModel):
    """Enhanced state for variant-based asset generation workflow."""
    model_config = ConfigDict(arbitrary_types_allowed=True)
    
    # Input configuration
    asset_category: str = Field(description="Category of assets to generate")
    level_range: str = Field(default="1-180", description="Universal level range")
    toml_spec_path: Path = Field(description="Path to universal variant TOML")
    output_dir: Path = Field(description="Output directory")
    
    # Variant configuration
    variant_config: VariantConfiguration | None = Field(default=None, description="Parsed variant configuration")
    resolution_tiers: dict[str, ResolutionTier] = Field(default_factory=dict, description="Available resolution tiers")
    
    # Combinatorial generation
    combinatorial_results: dict[str, CombinatorialGeneration] = Field(default_factory=dict, description="Results per archetype")
    total_variants_planned: int = Field(default=0, description="Total variants to generate")
    
    # Generation results
    generated_variants: dict[str, str] = Field(default_factory=dict, description="Generated variant file paths")
    generation_metadata: dict[str, Any] = Field(default_factory=dict, description="Per-variant metadata")
    failed_generations: list[str] = Field(default_factory=list, description="Failed variant names")
    
    # Sprite sheet processing
    sprite_sheets_planned: list[SpriteSheetMetadata] = Field(default_factory=list, description="Planned sprite sheets")
    sprite_sheets_generated: dict[str, str] = Field(default_factory=dict, description="Generated sprite sheet paths")
    atlas_metadata: dict[str, Any] = Field(default_factory=dict, description="Atlas JSON metadata")
    
    # Human review
    human_approval: ApprovalStatus | None = Field(default=None, description="Human review status")
    human_feedback: dict[str, str] = Field(default_factory=dict, description="Human feedback")
    
    # Workflow metadata
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    current_stage: WorkflowStage = Field(default="initialization", description="Current stage")
    started_at: datetime = Field(default_factory=datetime.now, description="Start time")
    completed_at: datetime | None = Field(default=None, description="Completion time")
    step_count: int = Field(default=0, description="Completed steps")
    
    # Performance tracking
    batch_size: int = Field(default=5, description="Variants per batch")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    api_calls_made: int = Field(default=0, description="DALL-E API calls")
    total_cost_usd: float = Field(default=0.0, description="Total generation cost")


class VariantAssetGenerationRequest(BaseModel):
    """Request for variant-based asset generation."""
    model_config = ConfigDict(extra="forbid")
    
    # Basic configuration
    asset_category: str = Field(description="Asset category to generate")
    toml_spec_path: Path = Field(description="Path to universal variant TOML")
    output_dir: Path = Field(description="Output directory")
    
    # Variant configuration
    max_variants_per_archetype: int | None = Field(default=None, description="Override max variants")
    priority_archetypes: list[str] = Field(default_factory=list, description="Generate these archetypes first")
    
    # Generation settings
    batch_size: int = Field(default=5, description="Variants per batch")
    quality_level: str = Field(default="standard", description="Quality level")
    enable_sprite_sheets: bool = Field(default=True, description="Generate sprite sheets")
    autonomous_mode: bool = Field(default=False, description="Skip human review")
    
    # Performance settings
    parallel_generation: bool = Field(default=False, description="Enable parallel generation")
    memory_limit_mb: int = Field(default=2048, description="Memory limit for sprite sheet processing")
    
    # Workflow configuration
    workflow_id: WorkflowID | None = Field(default=None, description="Optional workflow ID")
    checkpoint_interval: int = Field(default=10, description="Steps between checkpoints")
    
    # Metadata
    created_at: datetime = Field(default_factory=datetime.now, description="Request creation time")
    
    def model_post_init(self, __context: Any) -> None:
        """Generate workflow ID if not provided."""
        if self.workflow_id is None:
            import uuid
            self.workflow_id = f"variants_{self.asset_category}_{uuid.uuid4().hex[:8]}"


class VariantAssetGenerationResult(BaseModel):
    """Result from variant-based asset generation."""
    model_config = ConfigDict(extra="forbid")
    
    # Basic info
    workflow_id: WorkflowID = Field(description="Workflow identifier")
    asset_category: str = Field(description="Generated asset category")
    status: AnalysisStatus = Field(description="Final status")
    
    # Generation results
    archetypes_processed: int = Field(description="Number of base archetypes processed")
    variants_requested: int = Field(description="Total variants requested")
    variants_generated: int = Field(description="Variants successfully generated")
    variants_failed: int = Field(description="Failed variant generations")
    
    # Sprite sheet results
    sprite_sheets_generated: int = Field(description="Number of sprite sheets created")
    sprite_sheet_files: list[str] = Field(default_factory=list, description="Generated sprite sheet paths")
    atlas_files: list[str] = Field(default_factory=list, description="Generated atlas JSON files")
    
    # Quality metrics
    validation_passed: bool = Field(description="Whether validation passed")
    average_quality_score: float = Field(ge=0.0, le=1.0, description="Average quality score")
    sprite_sheet_efficiency: float = Field(ge=0.0, le=1.0, description="Sprite sheet space efficiency")
    
    # Performance metrics
    processing_time_seconds: float = Field(description="Total processing time")
    sprite_sheet_processing_time: float = Field(description="Time spent on sprite sheet generation")
    peak_memory_usage_mb: float | None = Field(default=None, description="Peak memory usage")
    total_api_calls: int = Field(description="Total DALL-E API calls")
    total_cost_usd: float = Field(description="Total generation cost")
    
    # File outputs
    individual_asset_files: dict[str, str] = Field(default_factory=dict, description="Individual variant files")
    metadata_files: list[str] = Field(default_factory=list, description="Metadata files")
    integration_files: list[str] = Field(default_factory=list, description="Game engine integration files")
    
    # Human review
    human_reviewed: bool = Field(default=False, description="Whether human reviewed")
    human_approval: ApprovalStatus | None = Field(default=None, description="Human approval status")
    
    # Metadata
    started_at: datetime = Field(description="Start timestamp")
    completed_at: datetime = Field(description="Completion timestamp")
    generator_version: str = Field(default="2.0.0", description="Variant generator version")
