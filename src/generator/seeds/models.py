"""
SQLModel tables and Pydantic models for seeds extraction system.

Combines ORM definitions with data pipeline models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field, ConfigDict, field_validator
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime


# ======================================
# SQLModel ORM Tables
# ======================================

class Sources(SQLModel, table=True):
    """Master table for all loaded source data (NLTK, books, linguistic sources)"""
    __tablename__ = "sources"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_type: str = SQLField(index=True)  # "nltk", "book", "linguistic", "omw", "cleasby"
    source_name: str = SQLField(index=True)  # Specific corpus/book/dataset name
    language: str | None = SQLField(default=None, index=True)  # Language code if applicable
    content_type: str = SQLField(index=True)  # "text", "corpus", "lexicon", "dataset"
    
    # Content storage
    raw_content: str | None = SQLField(default=None)  # Raw text/data
    processed_content: str | None = SQLField(default=None)  # Preprocessed version
    source_metadata: str | None = SQLField(default=None)  # JSON metadata
    
    # Processing metadata
    loaded_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime))
    processing_status: str = SQLField(default="loaded")  # "loaded", "processed", "extracted"
    extraction_count: int = SQLField(default=0)  # Number of seeds extracted
    
    # Quality metrics
    quality_score: float | None = SQLField(default=None)  # 0.0 to 1.0
    completeness: float | None = SQLField(default=None)  # 0.0 to 1.0
    
    # Relationships
    narrative_seeds: list["NarrativeSeeds"] = Relationship(back_populates="source")
    motif_seeds: list["MotifSeeds"] = Relationship(back_populates="source")
    semantic_seeds: list["SemanticSeeds"] = Relationship(back_populates="source")
    emotional_seeds: list["EmotionalSeeds"] = Relationship(back_populates="source")
    linguistic_seeds: list["LinguisticSeeds"] = Relationship(back_populates="source")


class NarrativeSeeds(SQLModel, table=True):
    """Narrative structures and story patterns extracted from sources"""
    __tablename__ = "narrative_seeds"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_id: int = SQLField(foreign_key="sources.id", index=True)
    
    # Core narrative data
    structure_name: str = SQLField(index=True)  # "hero_journey", "descent", etc.
    structure_type: str = SQLField(index=True)  # "linear", "cyclical", "branching"
    
    # Story components
    story_beats: str = SQLField(sa_column=Column(JSON))  # JSON array of narrative beats
    core_themes: str = SQLField(sa_column=Column(JSON))  # JSON array of themes
    conflict_types: str = SQLField(sa_column=Column(JSON))  # JSON array of conflict types
    
    # Horror progression
    horror_stage: int = SQLField(default=0)  # 0-4 dread level
    corruption_arc: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON corruption progression
    psychological_elements: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON psych elements
    
    # ML metrics
    confidence_score: float = SQLField(default=0.0)
    extraction_method: str = SQLField(default="ml")
    
    # Relationships
    source: Sources = Relationship(back_populates="narrative_seeds")


class MotifSeeds(SQLModel, table=True):
    """Visual and thematic motifs extracted from sources"""
    __tablename__ = "motif_seeds"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_id: int = SQLField(foreign_key="sources.id", index=True)
    
    # Core motif data
    name: str = SQLField(index=True)
    category: str = SQLField(index=True)  # "visual", "symbolic", "narrative"
    description: str
    
    # Visual elements
    keywords: str = SQLField(sa_column=Column(JSON))  # JSON array of visual keywords
    color_palette: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON color suggestions
    atmosphere: str | None = SQLField(default=None)  # Mood/atmosphere description
    
    # Horror integration
    dread_amplification: float = SQLField(default=0.0)  # 0.0 to 1.0
    corruption_potential: float = SQLField(default=0.0)  # 0.0 to 1.0
    
    # ML metrics
    frequency: int = SQLField(default=1)  # How often found in sources
    confidence_score: float = SQLField(default=0.0)
    
    # Relationships
    source: Sources = Relationship(back_populates="motif_seeds")


class SemanticSeeds(SQLModel, table=True):
    """Semantic concepts and relationships extracted from sources"""
    __tablename__ = "semantic_seeds"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_id: int = SQLField(foreign_key="sources.id", index=True)
    
    # Core semantic data
    concept: str = SQLField(index=True)
    semantic_field: str = SQLField(index=True)  # "nature", "corruption", "magic", etc.
    
    # Relationships
    related_terms: str = SQLField(sa_column=Column(JSON))  # JSON array of related vocabulary
    synonyms: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array
    antonyms: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array
    hypernyms: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array of broader terms
    hyponyms: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array of narrower terms
    
    # Emotional and horror context
    emotional_weight: float = SQLField(default=0.0)  # -1.0 to 1.0 (negative to positive)
    horror_correlation: float = SQLField(default=0.0)  # 0.0 to 1.0
    
    # Etymology and cultural context
    etymology: str | None = SQLField(default=None)
    cultural_context: str | None = SQLField(default=None)
    
    # ML metrics
    embedding_vector: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array of floats
    cluster_id: int | None = SQLField(default=None)
    confidence_score: float = SQLField(default=0.0)
    
    # Relationships
    source: Sources = Relationship(back_populates="semantic_seeds")


class EmotionalSeeds(SQLModel, table=True):
    """Emotional patterns and psychological progressions extracted from sources"""
    __tablename__ = "emotional_seeds"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_id: int = SQLField(foreign_key="sources.id", index=True)
    
    # Core emotional data
    category: str = SQLField(index=True)  # "fear", "grief", "madness", etc.
    intensity_level: int = SQLField(default=1)  # 1-5 intensity scale
    
    # Progression patterns
    progression_stages: str = SQLField(sa_column=Column(JSON))  # JSON array of emotional stages
    trigger_events: str = SQLField(sa_column=Column(JSON))  # JSON array of triggering events
    resolution_paths: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array
    
    # Horror psychology
    horror_correlation: float = SQLField(default=0.0)  # 0.0 to 1.0
    trauma_potential: float = SQLField(default=0.0)  # 0.0 to 1.0
    companion_impact: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON companion effects
    
    # Character psychology
    personality_traits: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array
    behavioral_patterns: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON array
    dialogue_modifiers: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON dialogue effects
    
    # ML metrics
    confidence_score: float = SQLField(default=0.0)
    extraction_method: str = SQLField(default="ml")
    
    # Relationships
    source: Sources = Relationship(back_populates="emotional_seeds")


class LinguisticSeeds(SQLModel, table=True):
    """Linguistic patterns and vocabulary extracted from sources"""
    __tablename__ = "linguistic_seeds"
    
    id: int | None = SQLField(default=None, primary_key=True)
    source_id: int = SQLField(foreign_key="sources.id", index=True)
    
    # Core linguistic data
    pattern_type: str = SQLField(index=True)  # "morphological", "syntactic", "semantic"
    language: str = SQLField(index=True)  # Language code
    
    # Pattern content
    pattern: str  # The linguistic pattern itself
    description: str  # Human-readable description
    usage_examples: str = SQLField(sa_column=Column(JSON))  # JSON array of examples
    
    # Vocabulary and etymology
    vocabulary_pool: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON vocabulary items
    etymology_data: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON etymology info
    transliterations: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON transliterations
    
    # Cultural and thematic context
    cultural_context: str | None = SQLField(default=None)
    thematic_category: str | None = SQLField(default=None)  # "magic", "nature", etc.
    
    # Game integration
    name_generation_rules: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON rules
    dialogue_patterns: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON patterns
    
    # ML metrics
    frequency: int = SQLField(default=1)
    confidence_score: float = SQLField(default=0.0)
    
    # Relationships
    source: Sources = Relationship(back_populates="linguistic_seeds")


class SeedClusters(SQLModel, table=True):
    """Clusters of related seeds across different types"""
    __tablename__ = "seed_clusters"
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Cluster identity
    cluster_name: str = SQLField(index=True)
    cluster_type: str = SQLField(index=True)  # "thematic", "emotional", "narrative"
    centroid_concept: str  # Central concept of the cluster
    
    # Member seeds (JSON arrays of IDs)
    narrative_seed_ids: str | None = SQLField(default=None, sa_column=Column(JSON))
    motif_seed_ids: str | None = SQLField(default=None, sa_column=Column(JSON))
    semantic_seed_ids: str | None = SQLField(default=None, sa_column=Column(JSON))
    emotional_seed_ids: str | None = SQLField(default=None, sa_column=Column(JSON))
    linguistic_seed_ids: str | None = SQLField(default=None, sa_column=Column(JSON))
    
    # Cluster metrics
    coherence_score: float = SQLField(default=0.0)  # 0.0 to 1.0
    member_count: int = SQLField(default=0)
    average_confidence: float = SQLField(default=0.0)
    
    # Horror progression
    horror_stage: int = SQLField(default=0)  # 0-4 dread level
    recommended_usage: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON usage suggestions
    
    created_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime))
    updated_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime))


class ExtractionMetrics(SQLModel, table=True):
    """Metrics and statistics for seed extraction runs"""
    __tablename__ = "extraction_metrics"
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Run identification
    run_id: str = SQLField(unique=True, index=True)
    run_timestamp: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime))
    
    # Source statistics
    total_sources: int = SQLField(default=0)
    sources_processed: int = SQLField(default=0)
    sources_failed: int = SQLField(default=0)
    
    # Extraction statistics
    narrative_seeds_extracted: int = SQLField(default=0)
    motif_seeds_extracted: int = SQLField(default=0)
    semantic_seeds_extracted: int = SQLField(default=0)
    emotional_seeds_extracted: int = SQLField(default=0)
    linguistic_seeds_extracted: int = SQLField(default=0)
    
    # Quality metrics
    average_confidence: float = SQLField(default=0.0)
    clustering_quality: float = SQLField(default=0.0)
    coverage_percentage: float = SQLField(default=0.0)
    
    # Performance metrics
    extraction_duration_seconds: float = SQLField(default=0.0)
    ml_inference_time_seconds: float = SQLField(default=0.0)
    database_write_time_seconds: float = SQLField(default=0.0)
    
    # Error tracking
    error_count: int = SQLField(default=0)
    error_details: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON error details
    
    # Configuration used
    extraction_config: str | None = SQLField(default=None, sa_column=Column(JSON))  # JSON config snapshot


# ======================================
# Pydantic Data Models
# ======================================

# Data Source Types and Enums (extracted from seeds-owned types)
from enum import Enum

class DataSourceType(str, Enum):
    """Type of data source for seeds extraction"""
    BOOKS = "books"
    NLTK = "nltk"
    LINGUISTIC = "linguistic"
    OMW = "omw"
    CLEASBY = "cleasby"

class CacheStrategy(str, Enum):
    """Cache strategy for data loading"""
    NONE = "none"
    PERSISTENT = "persistent" 
    MEMORY = "memory"

class DataSourceStatus(str, Enum):
    """Status of data source operations"""
    NOT_STARTED = "not_started"
    IN_PROGRESS = "in_progress"
    COMPLETED = "completed"
    FAILED = "failed"

class ValidationLevel(str, Enum):
    """Validation level for data quality"""
    MINIMAL = "minimal"
    STANDARD = "standard"
    STRICT = "strict"

class ErrorSeverity(str, Enum):
    """Error severity levels"""
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"

class ProcessingStage(str, Enum):
    """Processing stage for error tracking"""
    DOWNLOAD = "download"
    VALIDATION = "validation"
    EXTRACTION = "extraction"
    STORAGE = "storage"

class BookSubject(str, Enum):
    """Book subjects for corpus building"""
    FANTASY = "fantasy"
    HORROR = "horror"
    MYTHOLOGY = "mythology"
    FOLKLORE = "folklore"

class LinguisticDataType(str, Enum):
    """Types of linguistic data"""
    LEXICON = "lexicon"
    CORPUS = "corpus"
    ETYMOLOGY = "etymology"
    MORPHOLOGY = "morphology"

class NLTKResourceType(str, Enum):
    """NLTK resource types"""
    CORPUS = "corpus"
    MODELS = "models"
    TOKENIZERS = "tokenizers"
    CHUNKERS = "chunkers"

class DataFormat(str, Enum):
    """Data format types"""
    JSON = "json"
    CSV = "csv"
    XML = "xml"
    TEXT = "text"


class DataSourceConfig(BaseModel):
    """Base configuration for all data sources"""
    source_type: DataSourceType = Field(description="Type of data source")
    enabled: bool = Field(default=True, description="Whether this source is enabled")
    timeout_seconds: float = Field(default=30.0, description="Request timeout in seconds")
    retry_count: int = Field(default=3, description="Number of retries on failure")
    cache_strategy: CacheStrategy = Field(default=CacheStrategy.PERSISTENT, description="Caching strategy")
    validation_level: ValidationLevel = Field(default=ValidationLevel.STANDARD, description="Validation strictness")
    
    model_config = ConfigDict(extra="forbid")


class BookEntry(BaseModel):
    """A single book entry from the corpus"""
    title: str = Field(description="Book title")
    authors: list[str] = Field(default_factory=list, description="List of author names")
    year: int | None = Field(default=None, description="First publication year")
    subjects: list[str] = Field(default_factory=list, description="Book subjects/categories")
    key: str | None = Field(default=None, description="OpenLibrary key")
    motifs: list[str] = Field(default_factory=list, description="Extracted narrative motifs")
    download_url: str | None = Field(default=None, description="Download URL if available")
    content_available: bool = Field(default=False, description="Whether full content is available")
    
    model_config = ConfigDict(extra="forbid")


class LinguisticEntry(BaseModel):
    """A single linguistic data entry"""
    term: str = Field(description="The linguistic term")
    language: str = Field(description="Source language code")
    semantic_field: str = Field(description="Semantic domain or field")
    definitions: list[str] = Field(default_factory=list, description="Definitions or meanings")
    related_terms: list[str] = Field(default_factory=list, description="Related vocabulary")
    cultural_context: str | None = Field(default=None, description="Cultural or historical context")
    etymology: str | None = Field(default=None, description="Etymological information")
    
    model_config = ConfigDict(extra="forbid")


class OMWRecord(BaseModel):
    """Single Open Multilingual Wordnet record from CSV input"""
    english_seed: str = Field(description="English seed word")
    language: str = Field(description="Language code") 
    lemma: str = Field(description="Translated lemma")
    
    @field_validator("english_seed", "language", "lemma")
    @classmethod
    def strip_whitespace(cls, v: str) -> str:
        """Remove leading/trailing whitespace"""
        return v.strip()
    
    @field_validator("english_seed")
    @classmethod
    def validate_seed(cls, v: str) -> str:
        """Ensure english seed is not empty and normalize"""
        if not v:
            raise ValueError("english_seed cannot be empty")
        return v.lower().replace(' ', '_').replace('-', '_')
    
    model_config = ConfigDict(extra="forbid")


class ValidationResult(BaseModel):
    """Result of data validation operations"""
    source_name: str = Field(description="Name of validated data source")
    validation_level: ValidationLevel = Field(description="Level of validation performed")
    passed: bool = Field(description="Whether validation passed")
    issues_found: list[str] = Field(default_factory=list, description="List of validation issues")
    warnings: list[str] = Field(default_factory=list, description="Non-critical warnings")
    statistics: dict[str, Any] = Field(default_factory=dict, description="Validation statistics")
    validated_at: datetime = Field(default_factory=datetime.now, description="When validation was performed")
    
    model_config = ConfigDict(extra="forbid")
