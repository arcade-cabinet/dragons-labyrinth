"""
SQLModel tables and Pydantic models for assets system.

Combines ORM definitions with asset data models including OpenAI integration.
Uses SQLModel for Godot-compatible SQLite database with blob storage.
"""

from datetime import datetime
from enum import Enum
from typing import Any
import re

from pydantic import BaseModel, Field, ConfigDict, field_validator, model_validator
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime, Text, LargeBinary


# ======================================
# Assets Types and Enums
# ======================================

class AssetType(str, Enum):
    """Types of assets"""
    IMAGE = "image"
    AUDIO = "audio"
    MODEL_3D = "model_3d"
    FONT = "font"
    TEXTURE = "texture"
    ANIMATION = "animation"
    SHADER = "shader"

class AssetCategory(str, Enum):
    """Asset categories"""
    BIOME = "biome"
    CHARACTER = "character"
    ITEM = "item"
    UI = "ui"
    AUDIO = "audio"
    MUSIC = "music"
    STRUCTURE = "structure"

class AssetResolution(str, Enum):
    """Asset resolution presets"""
    LOW = "256x256"
    MEDIUM = "512x512"
    HIGH = "1024x1024"
    ULTRA = "1792x1024"
    PORTRAIT = "1024x1792"

class BiomeType(str, Enum):
    """Biome types from entities"""
    GRASSLAND = "grassland"
    FOREST = "forest"
    MOUNTAIN = "mountain"
    DESERT = "desert"
    SWAMP = "swamp"
    TUNDRA = "tundra"
    CORRUPTED = "corrupted"

class RegionType(str, Enum):
    """Region types from world system"""
    WILDERNESS = "wilderness"
    SETTLEMENT = "settlement"
    DUNGEON_COMPLEX = "dungeon_complex"
    CORRUPTED_ZONE = "corrupted_zone"
    SAFE_HAVEN = "safe_haven"

class CorruptionStage(str, Enum):
    """Corruption stages from psychology"""
    CLEAN = "clean"
    WITHERED = "withered"
    SCORCHED = "scorched"
    VOID = "void"

class ActStage(str, Enum):
    """Story act stages from world"""
    PROLOGUE = "prologue"
    ACT_1_PEACE = "act_1_peace"
    ACT_1_UNEASE = "act_1_unease"
    ACT_2_DREAD = "act_2_dread"
    ACT_2_TERROR = "act_2_terror"
    ACT_3_HORROR = "act_3_horror"
    EPILOGUE = "epilogue"

class PhilosophyPath(str, Enum):
    """Philosophy paths from psychology"""
    LIGHT = "light"
    NEUTRAL = "neutral"
    DARK = "dark"
    PRAGMATIC = "pragmatic"
    COMPASSIONATE = "compassionate"
    RUTHLESS = "ruthless"

# Type aliases
DreadLevel = int  # 0-4
AssetId = str
FilePath = str
ContentHash = str
RegionContext = str


# ======================================
# SQLModel ORM Tables
# ======================================

class AssetsTimestampedModel(SQLModel):
    """Base model with assets-specific tracking."""
    
    created_at: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime), index=True)
    updated_at: datetime | None = SQLField(default=None, sa_column=Column(DateTime))
    generation_metadata: str = SQLField(default="{}", sa_column=Column(JSON), description="Generation process metadata")


class AssetRecord(AssetsTimestampedModel, table=True):
    """Assets table with cross-system integration and OpenAI generation tracking"""
    __tablename__ = "assets"
    
    # Primary key
    asset_id: str = SQLField(primary_key=True, description="Unique asset identifier")
    
    # Basic properties
    asset_name: str = SQLField(description="Human-readable asset name")
    asset_type: str = SQLField(description="AssetType enum value")
    asset_category: str = SQLField(description="AssetCategory enum value")
    resolution: str = SQLField(description="AssetResolution enum value")
    
    # File metadata
    file_path: str | None = SQLField(default=None, description="Path to asset file on disk")
    file_size: int | None = SQLField(default=None, description="File size in bytes")
    file_hash: str | None = SQLField(default=None, description="SHA256 hash of file content")
    mime_type: str | None = SQLField(default=None, description="MIME type")
    
    # Cross-system references
    source_entities: str = SQLField(default="[]", sa_column=Column(JSON), description="Entity IDs from entities subpackage")
    psychology_context: str = SQLField(default="{}", sa_column=Column(JSON), description="Psychology context data")
    world_context: str = SQLField(default="{}", sa_column=Column(JSON), description="World context data")
    maps_context: str = SQLField(default="{}", sa_column=Column(JSON), description="Maps context data")
    encounters_context: str = SQLField(default="{}", sa_column=Column(JSON), description="Encounters context data")
    sprites_context: str = SQLField(default="{}", sa_column=Column(JSON), description="Sprites context data")
    
    # Horror progression context
    dread_level: int = SQLField(description="Associated dread level")
    corruption_stage: str = SQLField(description="Corruption context")
    act_context: str = SQLField(description="Story act context")
    
    # Regional and philosophy context
    region_context: str | None = SQLField(default=None, description="Regional context")
    philosophy_alignment: str | None = SQLField(default=None, description="Philosophy context")
    
    # OpenAI generation metadata
    generation_prompt: str = SQLField(sa_column=Column(Text), description="Complete prompt used for generation")
    base_prompt: str | None = SQLField(default=None, sa_column=Column(Text), description="Original base prompt")
    enhanced_prompt: str | None = SQLField(default=None, sa_column=Column(Text), description="Cross-system enhanced prompt")
    openai_model: str = SQLField(description="OpenAI model used")
    openai_quality: str = SQLField(default="high", description="Quality setting used")
    generation_cost: float = SQLField(default=0.0, description="Actual generation cost")
    
    # Generation parameters
    style_constraints: str = SQLField(default="[]", sa_column=Column(JSON), description="Style requirements")
    transparency_required: bool = SQLField(default=True, description="Transparent background required")
    sprite_sheet_compatible: bool = SQLField(default=False, description="Sprite sheet compatible")
    
    # Cross-system coherence
    coherence_score: float = SQLField(default=0.0, description="Cross-system context coherence (0-1)")
    context_richness_score: float = SQLField(default=0.0, description="Richness of cross-system context (0-1)")
    validation_notes: str = SQLField(default="[]", sa_column=Column(JSON), description="Cross-system validation results")
    
    # Usage and quality tracking
    usage_count: int = SQLField(default=0, description="Times asset has been referenced")
    quality_score: float = SQLField(default=0.0, description="Asset quality assessment (0-1)")
    last_used: datetime | None = SQLField(default=None, sa_column=Column(DateTime), description="Last usage timestamp")


class AssetBlobStorage(AssetsTimestampedModel, table=True):
    """SQLite blob storage for generated assets"""
    __tablename__ = "asset_blobs"
    
    # Primary key and relation
    asset_id: str = SQLField(primary_key=True, foreign_key="assets.asset_id", description="Reference to asset record")
    
    # Blob storage
    asset_data: bytes = SQLField(sa_column=Column(LargeBinary), description="Asset binary data")
    
    # Blob metadata
    blob_size: int = SQLField(description="Size of blob in bytes")
    compression_used: str | None = SQLField(default=None, description="Compression algorithm used")
    checksum: str = SQLField(description="Checksum of blob data")
    
    # Storage metadata
    stored_at: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    accessed_count: int = SQLField(default=0, description="Number of times blob has been accessed")
    last_accessed: datetime | None = SQLField(default=None, sa_column=Column(DateTime), description="Last access timestamp")


class AssetRequestRecord(AssetsTimestampedModel, table=True):
    """Asset request tracking with cross-system context"""
    __tablename__ = "asset_requests"
    
    # Primary key
    request_id: str = SQLField(primary_key=True, description="Unique request identifier")
    
    # Request properties
    asset_name: str = SQLField(description="Requested asset name")
    asset_type: str = SQLField(description="Requested asset type")
    asset_category: str = SQLField(description="Requested asset category")
    resolution: str = SQLField(description="Requested resolution")
    
    # Generation prompts
    base_prompt: str = SQLField(sa_column=Column(Text), description="Base generation prompt")
    enhanced_prompt: str = SQLField(sa_column=Column(Text), description="Cross-system enhanced prompt")
    style_constraints: str = SQLField(default="[]", sa_column=Column(JSON), description="Style requirements")
    
    # Cross-system context
    source_subpackages: str = SQLField(default="[]", sa_column=Column(JSON), description="Source subpackages")
    context_entities: str = SQLField(default="[]", sa_column=Column(JSON), description="Referenced entities")
    context_data: str = SQLField(default="{}", sa_column=Column(JSON), description="Cross-system context")
    
    # Horror progression context
    dread_level: int = SQLField(description="Associated dread level")
    corruption_stage: str = SQLField(description="Corruption context")
    philosophy_context: str | None = SQLField(default=None, description="Philosophy alignment")
    region_context: str | None = SQLField(default=None, description="Regional context")
    
    # Generation parameters
    openai_model: str = SQLField(default="dall-e-3", description="OpenAI model to use")
    quality: str = SQLField(default="high", description="Generation quality")
    transparency_required: bool = SQLField(default=True, description="Transparent background needed")
    priority: int = SQLField(default=5, description="Generation priority (1-10)")
    
    # Status and tracking
    request_status: str = SQLField(default="pending", description="Request status")
    generated_asset_id: str | None = SQLField(default=None, description="ID of generated asset if completed")
    cost_estimate: float = SQLField(default=0.04, description="Estimated generation cost")
    actual_cost: float | None = SQLField(default=None, description="Actual generation cost")
    
    # Cross-system metrics
    coherence_score: float = SQLField(default=0.0, description="Cross-system context coherence (0-1)")
    context_enhancement_score: float = SQLField(default=0.0, description="Quality of context enhancement (0-1)")
    
    # Metadata
    requested_at: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    completed_at: datetime | None = SQLField(default=None, sa_column=Column(DateTime), description="When request was completed")
    processing_errors: str = SQLField(default="[]", sa_column=Column(JSON), description="Errors during processing")


class SpriteSheetRecord(AssetsTimestampedModel, table=True):
    """Sprite sheet metadata and organization"""
    __tablename__ = "sprite_sheets"
    
    # Primary key
    sheet_id: str = SQLField(primary_key=True, description="Unique sprite sheet identifier")
    
    # Sheet properties
    sheet_name: str = SQLField(description="Sprite sheet name")
    sheet_category: str = SQLField(description="Category of sprites in sheet")
    grid_width: int = SQLField(description="Number of sprites horizontally")
    grid_height: int = SQLField(description="Number of sprites vertically")
    sprite_width: int = SQLField(description="Width of individual sprites")
    sprite_height: int = SQLField(description="Height of individual sprites")
    
    # Component assets
    component_asset_ids: str = SQLField(default="[]", sa_column=Column(JSON), description="Asset IDs that make up this sheet")
    sprite_positions: str = SQLField(default="{}", sa_column=Column(JSON), description="Position of each sprite in sheet")
    
    # Cross-system organization
    source_subpackages: str = SQLField(default="[]", sa_column=Column(JSON), description="Subpackages contributing sprites")
    cross_system_coherence: float = SQLField(default=0.0, description="Coherence across component sprites (0-1)")
    
    # Sheet metadata
    total_sprites: int = SQLField(default=0, description="Total number of sprites in sheet")
    sheet_file_path: str | None = SQLField(default=None, description="Path to completed sprite sheet file")
    sheet_file_size: int | None = SQLField(default=None, description="Size of sprite sheet file")
    
    # Usage tracking
    usage_count: int = SQLField(default=0, description="Times sprite sheet has been referenced")
    last_used: datetime | None = SQLField(default=None, sa_column=Column(DateTime), description="Last usage timestamp")


class AssetExtractionMetrics(AssetsTimestampedModel, table=True):
    """Metrics tracking for asset generation and cross-system integration"""
    __tablename__ = "asset_extraction_metrics"
    
    # Primary key
    extraction_id: str = SQLField(primary_key=True, description="Unique extraction run identifier")
    
    # Extraction metadata
    extraction_timestamp: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    extraction_type: str = SQLField(description="Type of extraction (cross_system_openai, bulk, etc.)")
    
    # Cross-system integration metrics
    entities_integration_score: float = SQLField(default=0.0, description="Entities subpackage integration quality (0-1)")
    psychology_integration_score: float = SQLField(default=0.0, description="Psychology subpackage integration quality (0-1)")
    world_integration_score: float = SQLField(default=0.0, description="World subpackage integration quality (0-1)")
    maps_integration_score: float = SQLField(default=0.0, description="Maps subpackage integration quality (0-1)")
    encounters_integration_score: float = SQLField(default=0.0, description="Encounters subpackage integration quality (0-1)")
    sprites_integration_score: float = SQLField(default=0.0, description="Sprites subpackage integration quality (0-1)")
    overall_coherence_score: float = SQLField(default=0.0, description="Overall cross-system coherence (0-1)")
    
    # Generation metrics
    total_assets_generated: int = SQLField(default=0, description="Total assets generated")
    assets_by_type: str = SQLField(default="{}", sa_column=Column(JSON), description="Assets by type")
    assets_by_category: str = SQLField(default="{}", sa_column=Column(JSON), description="Assets by category")
    assets_by_resolution: str = SQLField(default="{}", sa_column=Column(JSON), description="Assets by resolution")
    
    # OpenAI integration metrics
    openai_api_calls: int = SQLField(default=0, description="Total OpenAI API calls made")
    openai_success_count: int = SQLField(default=0, description="Successful OpenAI generations")
    openai_failure_count: int = SQLField(default=0, description="Failed OpenAI generations")
    total_generation_cost: float = SQLField(default=0.0, description="Total OpenAI generation cost")
    average_cost_per_asset: float = SQLField(default=0.0, description="Average cost per generated asset")
    
    # Quality metrics
    asset_diversity_score: float = SQLField(default=0.0, description="Diversity of asset types (0-1)")
    visual_consistency_score: float = SQLField(default=0.0, description="Visual style consistency (0-1)")
    horror_progression_coverage: float = SQLField(default=0.0, description="Coverage of horror progression (0-1)")
    cross_system_enhancement_effectiveness: float = SQLField(default=0.0, description="Effectiveness of cross-system enhancement (0-1)")
    
    # Performance metrics
    extraction_duration_seconds: float = SQLField(default=0.0, description="Total extraction time")
    average_generation_time: float = SQLField(default=0.0, description="Average generation time per asset")
    blob_storage_time: float = SQLField(default=0.0, description="Time spent on blob storage operations")
    
    # Error tracking
    generation_errors: str = SQLField(default="[]", sa_column=Column(JSON), description="Generation errors encountered")
    openai_errors: str = SQLField(default="[]", sa_column=Column(JSON), description="OpenAI API errors")
    cross_system_errors: str = SQLField(default="[]", sa_column=Column(JSON), description="Cross-system coordination errors")
    validation_failures: str = SQLField(default="[]", sa_column=Column(JSON), description="Asset validation failures")
    
    # Source data tracking
    source_subpackages: str = SQLField(default="[]", sa_column=Column(JSON), description="Subpackages used for context")
    cross_system_dependencies: str = SQLField(default="{}", sa_column=Column(JSON), description="Inter-subpackage dependencies")
    openai_integration_version: str = SQLField(default="1.0", description="OpenAI integration version used")
    
    # Context enhancement tracking
    prompt_enhancements_applied: str = SQLField(default="[]", sa_column=Column(JSON), description="Types of prompt enhancements applied")
    context_data_sources: str = SQLField(default="{}", sa_column=Column(JSON), description="Amount of context data from each source")
    enhancement_effectiveness: str = SQLField(default="{}", sa_column=Column(JSON), description="Effectiveness of each enhancement type")


# ======================================
# Pydantic Data Models
# ======================================

class AssetData(BaseModel):
    """Complete asset data structure"""
    asset_id: str
    name: str
    asset_type: AssetType
    category: AssetCategory
    resolution: AssetResolution
    
    # File metadata
    file_path: str | None
    file_size: int | None
    file_hash: str | None
    mime_type: str | None
    
    # Cross-system integration
    source_entities: list[str]
    psychology_context: dict[str, Any]
    world_context: dict[str, Any]
    maps_context: dict[str, Any]
    encounters_context: dict[str, Any]
    sprites_context: dict[str, Any]
    
    # Horror progression
    dread_level: DreadLevel
    corruption_stage: CorruptionStage
    act_context: ActStage
    
    # OpenAI generation
    generation_prompt: str
    openai_model: str
    generation_cost: float
    
    # Quality metrics
    coherence_score: float
    context_richness_score: float
    
    model_config = ConfigDict(extra="forbid")


class AssetQuery(BaseModel):
    """Query parameters for asset search"""
    
    asset_type: str | None = Field(default=None, description="Filter by asset type")
    asset_category: str | None = Field(default=None, description="Filter by category")
    asset_subcategory: str | None = Field(default=None, description="Filter by subcategory")
    
    biome_context: str | None = Field(default=None, description="Filter by biome")
    character_context: str | None = Field(default=None, description="Filter by character")
    region_context: str | None = Field(default=None, description="Filter by region")
    dread_level: int | None = Field(default=None, description="Filter by dread level")
    corruption_stage: str | None = Field(default=None, description="Filter by corruption stage")
    philosophy_alignment: str | None = Field(default=None, description="Filter by philosophy")
    
    accessible_only: bool = Field(default=True, description="Only return accessible assets")
    validated_only: bool = Field(default=True, description="Only return validated assets")
    
    limit: int | None = Field(default=None, description="Maximum results to return")
    offset: int = Field(default=0, description="Results offset for pagination")

    model_config = ConfigDict(extra="forbid")


class AssetScanResult(BaseModel):
    """Result of scanning assets directory"""
    
    total_files_found: int = Field(description="Total files discovered")
    new_assets: int = Field(description="New assets added to database")
    updated_assets: int = Field(description="Assets with updated metadata")
    removed_assets: int = Field(description="Assets removed (files no longer exist)")
    
    assets_by_type: dict[str, int] = Field(description="Asset count by type")
    assets_by_category: dict[str, int] = Field(description="Asset count by category")
    
    scan_duration_seconds: float = Field(description="Time taken to scan")
    scan_timestamp: datetime = Field(description="When scan was performed")
    
    errors_encountered: list[str] = Field(default_factory=list, description="Scan errors")
    warnings: list[str] = Field(default_factory=list, description="Scan warnings")

    model_config = ConfigDict(extra="forbid")


class OpenAIGenerationSpec(BaseModel):
    """OpenAI generation specification with cross-system enhancement"""
    
    # Basic generation parameters
    asset_name: str = Field(description="Name of asset to generate")
    asset_type: AssetType = Field(description="Type of asset")
    category: AssetCategory = Field(description="Asset category")
    resolution: AssetResolution = Field(description="Target resolution")
    
    # Prompt components
    base_prompt: str = Field(description="Base description prompt")
    enhanced_prompt: str = Field(description="Cross-system enhanced prompt")
    style_constraints: list[str] = Field(default_factory=list, description="Style requirements")
    
    # Cross-system context
    entities_context: dict[str, Any] = Field(default_factory=dict, description="Context from entities")
    psychology_context: dict[str, Any] = Field(default_factory=dict, description="Context from psychology")
    world_context: dict[str, Any] = Field(default_factory=dict, description="Context from world")
    maps_context: dict[str, Any] = Field(default_factory=dict, description="Context from maps")
    encounters_context: dict[str, Any] = Field(default_factory=dict, description="Context from encounters")
    sprites_context: dict[str, Any] = Field(default_factory=dict, description="Context from sprites")
    
    # Horror progression
    dread_level: DreadLevel = Field(description="Target dread level")
    corruption_stage: CorruptionStage = Field(description="Target corruption stage")
    act_context: ActStage = Field(description="Story act context")
    
    # OpenAI parameters
    model: str = Field(default="dall-e-3", description="OpenAI model")
    quality: str = Field(default="high", description="Quality setting")
    transparency: bool = Field(default=True, description="Transparent background")
    
    model_config = ConfigDict(extra="forbid")


class AssetGenerationResult(BaseModel):
    """Result of asset generation with OpenAI"""
    
    success: bool = Field(description="Whether generation succeeded")
    asset_id: str | None = Field(default=None, description="Generated asset ID")
    
    # Generation details
    openai_response_url: str | None = Field(default=None, description="OpenAI response URL")
    generation_cost: float = Field(default=0.0, description="Actual generation cost")
    generation_time_seconds: float = Field(default=0.0, description="Time taken for generation")
    
    # File details
    file_path: str | None = Field(default=None, description="Local file path")
    file_size: int | None = Field(default=None, description="File size in bytes")
    file_hash: str | None = Field(default=None, description="File hash")
    
    # Quality metrics
    coherence_score: float = Field(default=0.0, description="Cross-system coherence")
    context_richness_score: float = Field(default=0.0, description="Context richness")
    
    # Error handling
    error_message: str | None = Field(default=None, description="Error message if failed")
    retry_suggested: bool = Field(default=False, description="Whether retry is suggested")
    
    model_config = ConfigDict(extra="forbid")
