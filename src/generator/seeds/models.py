"""
Rust-compatible Pydantic models for seeds world generation.

Redesigned from SQLite/Godot architecture to Rust world crate generation.
Follows .clinerules modern Python standards with Rust-compatible data structures.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict


# ======================================
# Enums for Type Safety
# ======================================

class HorrorStage(int, Enum):
    """Horror progression stages (0-4 bands)"""
    PEACE = 0      # Band 1-20: Pastoral decay
    UNEASE = 1     # Band 21-40: World darkens  
    DREAD = 2      # Band 41-60: Trauma defines play
    TERROR = 3     # Band 61-120: Warped ecosystems
    HORROR = 4     # Band 121-180: Total collapse

class SeedCategory(str, Enum):
    """Categories of seeds for world generation"""
    NARRATIVE = "narrative"
    MOTIF = "motif" 
    SEMANTIC = "semantic"
    EMOTIONAL = "emotional"
    LINGUISTIC = "linguistic"

class StructureType(str, Enum):
    """Narrative structure types"""
    LINEAR = "linear"
    CYCLICAL = "cyclical"
    BRANCHING = "branching"

class MotifCategory(str, Enum):
    """Motif categories"""
    VISUAL = "visual"
    SYMBOLIC = "symbolic"
    NARRATIVE = "narrative"

class EmotionCategory(str, Enum):
    """Emotional categories"""
    FEAR = "fear"
    GRIEF = "grief"
    MADNESS = "madness"
    DESPAIR = "despair"
    CORRUPTION = "corruption"


# ======================================
# Core Seed Data Models
# ======================================

class NarrativeSeed(BaseModel):
    """Narrative structures and story patterns for world generation"""
    name: str = Field(description="Unique narrative structure name")
    structure_type: StructureType = Field(description="Type of narrative structure")
    
    # Story components
    story_beats: list[str] = Field(description="Sequence of narrative beats")
    core_themes: list[str] = Field(description="Central thematic elements")
    conflict_types: list[str] = Field(description="Types of conflicts involved")
    
    # Horror progression
    horror_stage: HorrorStage = Field(description="Which horror stage this applies to")
    corruption_progression: list[str] = Field(description="How corruption escalates")
    psychological_elements: list[str] = Field(description="Psychological horror components")
    
    # World generation metadata
    biome_affinity: list[str] = Field(default_factory=list, description="Preferred biomes for this narrative")
    encounter_weight: float = Field(default=1.0, ge=0.0, le=1.0, description="Likelihood of use in encounters")
    companion_impact: dict[str, float] = Field(default_factory=dict, description="Effects on companion psychology")
    
    model_config = ConfigDict(extra="forbid")


class MotifSeed(BaseModel):
    """Visual and thematic motifs for world atmosphere"""
    name: str = Field(description="Unique motif name")
    category: MotifCategory = Field(description="Type of motif")
    description: str = Field(description="Detailed motif description")
    
    # Visual elements
    visual_keywords: list[str] = Field(description="Keywords for visual representation")
    color_suggestions: list[str] = Field(default_factory=list, description="Suggested color palette")
    atmosphere_descriptor: str = Field(description="Mood/atmosphere this creates")
    
    # Horror integration
    dread_amplification: float = Field(ge=0.0, le=1.0, description="How much this increases dread")
    corruption_potential: float = Field(ge=0.0, le=1.0, description="Potential for corrupting elements")
    horror_stage: HorrorStage = Field(description="Which horror stage this fits")
    
    # World generation
    biome_compatibility: list[str] = Field(description="Compatible biomes")
    poi_affinity: list[str] = Field(default_factory=list, description="POI types that use this motif")
    generation_frequency: float = Field(default=0.5, ge=0.0, le=1.0, description="How often to generate")
    
    model_config = ConfigDict(extra="forbid")


class SemanticSeed(BaseModel):
    """Semantic concepts and vocabulary for world building"""
    concept: str = Field(description="Core semantic concept")
    semantic_field: str = Field(description="Semantic domain (nature, corruption, magic, etc.)")
    
    # Relationships
    related_terms: list[str] = Field(description="Related vocabulary")
    synonyms: list[str] = Field(default_factory=list, description="Synonym terms")
    antonyms: list[str] = Field(default_factory=list, description="Opposite terms")
    broader_concepts: list[str] = Field(default_factory=list, description="More general concepts")
    narrower_concepts: list[str] = Field(default_factory=list, description="More specific concepts")
    
    # Emotional and horror context
    emotional_weight: float = Field(ge=-1.0, le=1.0, description="Negative to positive emotional value")
    horror_correlation: float = Field(ge=0.0, le=1.0, description="Correlation with horror themes")
    corruption_association: float = Field(default=0.0, ge=0.0, le=1.0, description="Association with corruption")
    
    # Cultural context  
    etymology_notes: str | None = Field(default=None, description="Etymology and origin notes")
    cultural_context: str | None = Field(default=None, description="Cultural or historical context")
    
    # World generation usage
    name_generation_weight: float = Field(default=0.3, ge=0.0, le=1.0, description="Weight for name generation")
    description_weight: float = Field(default=0.5, ge=0.0, le=1.0, description="Weight for descriptions")
    dialogue_weight: float = Field(default=0.2, ge=0.0, le=1.0, description="Weight for dialogue")
    
    model_config = ConfigDict(extra="forbid")


class EmotionalSeed(BaseModel):
    """Emotional patterns and psychological progressions"""
    name: str = Field(description="Unique emotional pattern name")
    category: EmotionCategory = Field(description="Type of emotion")
    intensity_level: int = Field(ge=1, le=5, description="Emotional intensity (1-5 scale)")
    
    # Progression patterns
    progression_stages: list[str] = Field(description="Stages of emotional progression")
    trigger_events: list[str] = Field(description="Events that trigger this emotion")
    resolution_paths: list[str] = Field(default_factory=list, description="Ways this emotion can resolve")
    
    # Horror psychology
    horror_correlation: float = Field(ge=0.0, le=1.0, description="Correlation with horror themes")
    trauma_potential: float = Field(ge=0.0, le=1.0, description="Potential to cause lasting trauma")
    contagion_factor: float = Field(default=0.3, ge=0.0, le=1.0, description="How much this spreads to other companions")
    
    # Companion effects
    companion_behavior_changes: dict[str, float] = Field(default_factory=dict, description="Behavioral changes in companions")
    dialogue_modifiers: dict[str, float] = Field(default_factory=dict, description="Dialogue tone modifications")
    relationship_effects: dict[str, float] = Field(default_factory=dict, description="Effects on relationships")
    
    # World integration
    environmental_triggers: list[str] = Field(default_factory=list, description="Environmental conditions that trigger this")
    recovery_requirements: list[str] = Field(default_factory=list, description="What's needed for recovery")
    
    model_config = ConfigDict(extra="forbid")


class LinguisticSeed(BaseModel):
    """Linguistic patterns for name generation and dialogue"""
    name: str = Field(description="Unique linguistic pattern name")
    pattern_type: str = Field(description="Type of linguistic pattern (morphological, syntactic, etc.)")
    language_family: str = Field(default="fantasy", description="Language family or style")
    
    # Pattern content
    pattern_rules: list[str] = Field(description="Rules for applying this pattern")
    usage_examples: list[str] = Field(description="Example applications")
    phonetic_constraints: list[str] = Field(default_factory=list, description="Sound pattern constraints")
    
    # Vocabulary and generation
    vocabulary_pool: list[str] = Field(default_factory=list, description="Associated vocabulary")
    morpheme_components: list[str] = Field(default_factory=list, description="Morphological components")
    syllable_patterns: list[str] = Field(default_factory=list, description="Syllable structure patterns")
    
    # Thematic integration
    thematic_category: str = Field(description="Thematic category (magic, nature, corruption, etc.)")
    horror_stage_preference: list[HorrorStage] = Field(description="Preferred horror stages for use")
    emotional_resonance: dict[str, float] = Field(default_factory=dict, description="Emotional associations")
    
    # Name generation rules
    prefix_probability: float = Field(default=0.3, ge=0.0, le=1.0, description="Likelihood of prefix use")
    suffix_probability: float = Field(default=0.3, ge=0.0, le=1.0, description="Likelihood of suffix use")
    combination_rules: list[str] = Field(default_factory=list, description="How to combine with other patterns")
    
    model_config = ConfigDict(extra="forbid")


# ======================================
# Cluster and Organization Models  
# ======================================

class SeedCluster(BaseModel):
    """Thematic clusters of related seeds across categories"""
    name: str = Field(description="Unique cluster name")
    cluster_type: str = Field(description="Type of clustering (thematic, emotional, progression)")
    central_concept: str = Field(description="Core concept binding the cluster")
    
    # Member seeds by category
    narrative_seeds: list[str] = Field(default_factory=list, description="Narrative seed names in cluster")
    motif_seeds: list[str] = Field(default_factory=list, description="Motif seed names in cluster")  
    semantic_seeds: list[str] = Field(default_factory=list, description="Semantic seed names in cluster")
    emotional_seeds: list[str] = Field(default_factory=list, description="Emotional seed names in cluster")
    linguistic_seeds: list[str] = Field(default_factory=list, description="Linguistic seed names in cluster")
    
    # Cluster properties
    coherence_score: float = Field(ge=0.0, le=1.0, description="How coherent the cluster is")
    horror_stage: HorrorStage = Field(description="Primary horror stage for this cluster")
    usage_weight: float = Field(default=1.0, ge=0.0, description="Relative usage weight")
    
    # World generation guidance
    biome_affinities: list[str] = Field(description="Biomes where this cluster works well")
    generation_contexts: list[str] = Field(description="Contexts where this cluster should be used")
    exclusion_rules: list[str] = Field(default_factory=list, description="When NOT to use this cluster")
    
    model_config = ConfigDict(extra="forbid")


# ======================================
# World Generation Output Models
# ======================================

class BiomeSeeding(BaseModel):
    """Seed configuration for a specific biome"""
    biome_name: str = Field(description="Target biome name")
    horror_stage: HorrorStage = Field(description="Horror progression stage")
    
    # Seed selections
    primary_narratives: list[str] = Field(description="Primary narrative seeds to use")
    accent_motifs: list[str] = Field(description="Motifs for atmospheric touches")
    vocabulary_seeds: list[str] = Field(description="Semantic seeds for naming/description")
    emotional_baseline: list[str] = Field(description="Base emotional seeds")
    linguistic_patterns: list[str] = Field(description="Linguistic seeds for generation")
    
    # Generation parameters
    narrative_density: float = Field(ge=0.0, le=1.0, description="How often narrative seeds appear")
    motif_saturation: float = Field(ge=0.0, le=1.0, description="Atmospheric motif intensity")
    emotional_pressure: float = Field(ge=0.0, le=1.0, description="Emotional tension level")
    
    model_config = ConfigDict(extra="forbid")


class SeedsConfiguration(BaseModel):
    """Complete seeds configuration for world generation"""
    version: str = Field(default="1.0.0", description="Seeds configuration version")
    generated_at: datetime = Field(default_factory=datetime.now, description="Generation timestamp")
    
    # All seed collections
    narrative_seeds: list[NarrativeSeed] = Field(description="All narrative seeds")
    motif_seeds: list[MotifSeed] = Field(description="All motif seeds") 
    semantic_seeds: list[SemanticSeed] = Field(description="All semantic seeds")
    emotional_seeds: list[EmotionalSeed] = Field(description="All emotional seeds")
    linguistic_seeds: list[LinguisticSeed] = Field(description="All linguistic seeds")
    
    # Organization
    seed_clusters: list[SeedCluster] = Field(description="Thematic clusters")
    biome_configurations: list[BiomeSeeding] = Field(description="Per-biome seed configurations")
    
    # Generation metadata
    total_seeds: int = Field(description="Total number of seeds")
    horror_coverage: dict[str, int] = Field(description="Seeds per horror stage")
    generation_statistics: dict[str, Any] = Field(default_factory=dict, description="Generation stats")
    
    model_config = ConfigDict(extra="forbid")


# ======================================
# Processing and Quality Models
# ======================================

class SeedExtractionResult(BaseModel):
    """Result of seeds extraction process"""
    extraction_id: str = Field(description="Unique extraction run identifier")
    timestamp: datetime = Field(default_factory=datetime.now, description="When extraction completed")
    
    # Extraction statistics
    sources_processed: int = Field(description="Number of source materials processed")
    total_seeds_extracted: int = Field(description="Total seeds extracted")
    seeds_by_category: dict[SeedCategory, int] = Field(description="Seeds count by category")
    
    # Quality metrics
    average_confidence: float = Field(ge=0.0, le=1.0, description="Average extraction confidence")
    clustering_quality: float = Field(ge=0.0, le=1.0, description="Quality of seed clustering")
    coherence_score: float = Field(ge=0.0, le=1.0, description="Overall thematic coherence")
    
    # Processing metadata  
    processing_duration: float = Field(description="Processing time in seconds")
    errors_encountered: list[str] = Field(default_factory=list, description="Processing errors")
    warnings: list[str] = Field(default_factory=list, description="Processing warnings")
    
    model_config = ConfigDict(extra="forbid")
