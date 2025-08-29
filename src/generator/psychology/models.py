"""
SQLModel tables and Pydantic models for psychology system.

Combines ORM definitions with psychology data models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime


# ======================================
# Psychology Types and Enums
# ======================================

class CompanionState(str, Enum):
    """Current psychological state of companion"""
    STABLE = "stable"
    STRESSED = "stressed"
    TRAUMATIZED = "traumatized"
    BREAKING = "breaking"
    BROKEN = "broken"
    ABANDONED = "abandoned"
    IN_THERAPY = "in_therapy"
    RECOVERING = "recovering"

class CompanionType(str, Enum):
    """Type classification for companions"""
    WARRIOR = "warrior"
    HEALER = "healer"
    SCHOLAR = "scholar"
    ROGUE = "rogue"
    MYSTIC = "mystic"
    SURVIVOR = "survivor"

class EmotionalTrigger(str, Enum):
    """Triggers that cause emotional responses"""
    VIOLENCE = "violence"
    ABANDONMENT = "abandonment"
    BETRAYAL = "betrayal"
    LOSS = "loss"
    CORRUPTION = "corruption"
    ISOLATION = "isolation"
    SUPERNATURAL = "supernatural"

class TherapyStage(str, Enum):
    """Stages of companion therapy"""
    DENIAL = "denial"
    ACCEPTANCE = "acceptance"
    PROCESSING = "processing"
    INTEGRATION = "integration"
    RECOVERY = "recovery"

class HorrorArchetype(str, Enum):
    """Horror archetypes from narrative analysis"""
    INEVITABILITY = "inevitability"
    CORRUPTION = "corruption"
    ISOLATION = "isolation"
    MADNESS = "madness"
    BETRAYAL = "betrayal"
    LOSS_OF_CONTROL = "loss_of_control"

class CognitiveBias(str, Enum):
    """Cognitive biases affecting companions"""
    OPTIMISM_BIAS = "optimism_bias"
    CONFIRMATION_BIAS = "confirmation_bias"
    SUNK_COST_FALLACY = "sunk_cost_fallacy"
    GROUPTHINK = "groupthink"
    DENIAL = "denial"

class RecoveryFactor(str, Enum):
    """Factors that help companion recovery"""
    PLAYER_SUPPORT = "player_support"
    COMPANION_BONDS = "companion_bonds"
    SAFE_ENVIRONMENT = "safe_environment"
    PROFESSIONAL_HELP = "professional_help"
    TIME = "time"
    MEANING_MAKING = "meaning_making"

class TraumaSeverity(str, Enum):
    """Severity levels of trauma"""
    MILD = "mild"
    MODERATE = "moderate"
    SEVERE = "severe"
    CRITICAL = "critical"

class LoyaltyBand(str, Enum):
    """Loyalty level bands"""
    DEVOTED = "devoted"
    LOYAL = "loyal"
    NEUTRAL = "neutral"
    WAVERING = "wavering"
    DISLOYAL = "disloyal"

class CorruptionStage(str, Enum):
    """Stages of world corruption"""
    CLEAN = "clean"
    WITHERED = "withered"
    SCORCHED = "scorched"
    VOID = "void"

class PhilosophyPath(str, Enum):
    """Player's philosophical alignment"""
    LIGHT = "light"
    NEUTRAL = "neutral"
    DARK = "dark"
    PRAGMATIC = "pragmatic"
    COMPASSIONATE = "compassionate"
    RUTHLESS = "ruthless"

# Type aliases
CompanionId = str
TraumaLevel = int  # 0-100
LoyaltyScore = float  # 0.0-1.0
HorrorProgression = float  # 0.0-1.0
EmotionalIntensity = float  # 0.0-1.0
DreadLevel = int  # 0-4


# ======================================
# SQLModel ORM Tables
# ======================================

class PsychologyTimestampedModel(SQLModel):
    """Base model with psychology-specific tracking."""
    
    created_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime), index=True)
    updated_at: datetime = SQLField(default_factory=datetime.now, sa_column=Column(DateTime), index=True)
    
    # ML tracking for psychology extraction
    extraction_confidence: float = SQLField(default=0.0, description="ML psychology extraction confidence")
    ml_model_version: str = SQLField(default="1.0", description="ML model version used")
    cross_system_validated: bool = SQLField(default=False, description="Validated against entities/seeds")


class CompanionProfiles(PsychologyTimestampedModel, table=True):
    """Psychological profiles for companion NPCs using entities + seeds data."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Link to entities system
    entity_id: str = SQLField(index=True, description="References entity from entities subpackage")
    companion_name: str = SQLField(index=True)
    
    # Companion classification
    companion_type: str = SQLField(index=True, description="CompanionType enum value")
    baseline_loyalty: float = SQLField(default=0.5, ge=0.0, le=1.0)
    loyalty_threshold: float = SQLField(default=0.3, description="Abandonment threshold")
    
    # Psychology state
    current_trauma_level: int = SQLField(default=0, ge=0, le=100)
    current_loyalty_score: float = SQLField(default=0.5, ge=0.0, le=1.0)
    companion_state: str = SQLField(default="STABLE", description="CompanionState enum value")
    
    # Horror progression
    dread_tolerance: int = SQLField(default=2, ge=0, le=4, description="Max dread level before trauma")
    horror_sensitivity: float = SQLField(default=0.3, ge=0.0, le=1.0)
    corruption_resistance: float = SQLField(default=0.5, ge=0.0, le=1.0)
    
    # Personality from seeds analysis
    dominant_emotion: str = SQLField(default="neutral", description="From seeds emotional analysis")
    narrative_archetype: str = SQLField(default="loyal_friend", description="From seeds narrative patterns")
    linguistic_patterns: str | None = SQLField(default=None, sa_column=Column(JSON), description="Speech patterns from seeds")
    
    # Psychological factors (JSON)
    emotional_triggers: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list of EmotionalTrigger values")
    cognitive_biases: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list of CognitiveBias values")
    recovery_factors: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list of RecoveryFactor values")
    therapy_requirements: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list of therapy needs")


class HorrorProgression(PsychologyTimestampedModel, table=True):
    """World-level dread escalation and horror progression tracking."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Location-based horror
    hex_coordinate: str = SQLField(index=True, description="Hex coordinate (BASE, N1, etc.)")
    region_name: str = SQLField(index=True, description="Region from entities analysis")
    biome_type: str = SQLField(index=True, description="Biome from entities analysis")
    
    # Horror levels
    base_dread_level: int = SQLField(default=0, ge=0, le=4, description="Baseline dread for location")
    current_dread_level: int = SQLField(default=0, ge=0, le=4, description="Current modified dread")
    corruption_stage: str = SQLField(default="CLEAN", description="CorruptionStage enum value")
    
    # Horror progression mechanics
    horror_intensity: float = SQLField(default=0.0, ge=0.0, le=1.0)
    dread_multiplier: float = SQLField(default=1.0, ge=0.0, le=5.0)
    corruption_spread_rate: float = SQLField(default=0.1, ge=0.0, le=1.0)
    
    # Environmental psychology from seeds
    dominant_horror_archetype: str = SQLField(default="INEVITABILITY", description="HorrorArchetype from seeds")
    environmental_triggers: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list of environmental triggers")
    narrative_themes: str = SQLField(default="[]", sa_column=Column(JSON), description="JSON list from seeds narrative analysis")
    
    # Distance-based calculations
    distance_from_start: int = SQLField(default=0, description="Hex distance from BASE")
    mathematical_dread_base: float = SQLField(default=0.0, description="Distance / 20 for dread calculation")
    
    # Cross-system integration
    entity_count: int = SQLField(default=0, description="Number of entities in this hex")
    seed_pattern_count: int = SQLField(default=0, description="Number of seed patterns matching location")


class PlayerPsychology(PsychologyTimestampedModel, table=True):
    """Player choice psychology and moral path tracking."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Player identification (for save games)
    player_session_id: str = SQLField(index=True, description="Game session identifier")
    
    # Moral alignment tracking
    philosophy_path: str = SQLField(default="NEUTRAL", description="PhilosophyPath enum value")
    moral_choices_made: int = SQLField(default=0, description="Total moral choices encountered")
    dark_choices: int = SQLField(default=0, description="Number of dark/evil choices made")
    light_choices: int = SQLField(default=0, description="Number of light/good choices made")
    
    # Psychological state
    player_trauma_level: int = SQLField(default=0, ge=0, le=100)
    horror_resistance: float = SQLField(default=0.5, ge=0.0, le=1.0)
    companion_abandonment_count: int = SQLField(default=0, description="Times abandoned companions")
    
    # World progression
    current_hex_coordinate: str = SQLField(index=True, description="Current location")
    max_dread_encountered: int = SQLField(default=0, ge=0, le=4)
    corruption_exposure_level: float = SQLField(default=0.0, ge=0.0, le=1.0)
    
    # Narrative psychology
    understanding_level: float = SQLField(default=0.0, ge=0.0, le=1.0, description="Understanding of true nature")
    fear_threshold: float = SQLField(default=0.3, ge=0.0, le=1.0, description="When player becomes afraid")
    
    # Companion relationships
    companions_met: int = SQLField(default=0)
    companions_active: int = SQLField(default=0)
    companions_traumatized: int = SQLField(default=0)
    companions_in_therapy: int = SQLField(default=0)


class PsychologyExtractionMetrics(PsychologyTimestampedModel, table=True):
    """Metrics for psychology ML extraction and cross-system integration."""
    
    id: int | None = SQLField(default=None, primary_key=True)
    
    # Extraction session
    extraction_type: str = SQLField(index=True, description="Type of psychology extraction")
    source_system: str = SQLField(description="Source system (entities, seeds, world)")
    
    # ML performance
    model_name: str = SQLField(description="ML model used for extraction")
    processing_time_seconds: float = SQLField(default=0.0)
    memory_usage_mb: float = SQLField(default=0.0)
    
    # Quality metrics
    extraction_accuracy: float = SQLField(default=0.0, ge=0.0, le=1.0)
    cross_system_coherence: float = SQLField(default=0.0, ge=0.0, le=1.0)
    narrative_consistency: float = SQLField(default=0.0, ge=0.0, le=1.0)
    
    # Results summary
    companions_generated: int = SQLField(default=0)
    horror_profiles_created: int = SQLField(default=0)
    player_profiles_updated: int = SQLField(default=0)
    
    # Error tracking
    extraction_errors: int = SQLField(default=0)
    validation_warnings: int = SQLField(default=0)
    cross_system_conflicts: int = SQLField(default=0)


# ======================================
# Pydantic Data Models
# ======================================

class CompanionPsychologyProfile(BaseModel):
    """Comprehensive companion psychology profile"""
    companion_id: str
    name: str
    companion_type: CompanionType
    
    # Current state
    trauma_level: TraumaLevel
    loyalty_score: LoyaltyScore
    state: CompanionState
    
    # Personality traits
    dominant_emotion: str
    emotional_triggers: list[EmotionalTrigger]
    cognitive_biases: list[CognitiveBias]
    recovery_factors: list[RecoveryFactor]
    
    # Horror progression
    dread_tolerance: DreadLevel
    horror_sensitivity: EmotionalIntensity
    corruption_resistance: float
    
    model_config = ConfigDict(extra="forbid")


class HorrorProgressionState(BaseModel):
    """Horror progression state for a location"""
    hex_coordinate: str
    region_name: str
    biome_type: str
    
    # Horror levels
    base_dread_level: DreadLevel
    current_dread_level: DreadLevel
    corruption_stage: CorruptionStage
    
    # Progression metrics
    horror_intensity: HorrorProgression
    dread_multiplier: float
    corruption_spread_rate: float
    
    # Narrative context
    dominant_archetype: HorrorArchetype
    environmental_triggers: list[EmotionalTrigger]
    narrative_themes: list[str]
    
    model_config = ConfigDict(extra="forbid")


class PlayerPsychologyState(BaseModel):
    """Player psychology and moral choices"""
    session_id: str
    philosophy_path: PhilosophyPath
    
    # Choice tracking
    moral_choices_made: int
    dark_choices: int
    light_choices: int
    
    # Psychological state
    trauma_level: TraumaLevel
    horror_resistance: float
    understanding_level: float
    
    # Current situation
    current_location: str
    max_dread_encountered: DreadLevel
    corruption_exposure: float
    
    # Companion relationships
    companions_met: int
    companions_active: int
    companions_traumatized: int
    companions_in_therapy: int
    
    model_config = ConfigDict(extra="forbid")


class TherapySession(BaseModel):
    """Therapy session for traumatized companions"""
    companion_id: str
    session_number: int
    therapy_stage: TherapyStage
    
    # Session details
    duration_minutes: int
    techniques_used: list[str]
    breakthrough_moments: list[str]
    
    # Progress metrics
    trauma_reduction: float
    loyalty_improvement: float
    stability_increase: float
    
    # Next steps
    recommended_actions: list[str]
    next_session_needed: bool
    
    model_config = ConfigDict(extra="forbid")
