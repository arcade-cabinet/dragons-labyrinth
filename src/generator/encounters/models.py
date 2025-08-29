"""
SQLModel tables and Pydantic models for encounters system.

Combines ORM definitions with encounter data models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime, Text


# ======================================
# Encounters Types and Enums
# ======================================

class EncounterType(str, Enum):
    """Types of encounters"""
    COMBAT = "combat"
    SCRIPTED_EVENT = "scripted_event"
    BEAST_ENCOUNTER = "beast_encounter"
    NPC_INTERACTION = "npc_interaction"
    ENVIRONMENTAL = "environmental"
    PUZZLE = "puzzle"
    TRAP = "trap"

class EncounterDifficulty(str, Enum):
    """Encounter difficulty levels"""
    TRIVIAL = "trivial"
    EASY = "easy"
    MODERATE = "moderate"
    HARD = "hard"
    DEADLY = "deadly"
    IMPOSSIBLE = "impossible"

class BeastBehavior(str, Enum):
    """Beast behavior patterns"""
    AGGRESSIVE = "aggressive"
    DEFENSIVE = "defensive"
    TERRITORIAL = "territorial"
    PREDATORY = "predatory"
    SCAVENGER = "scavenger"
    SOCIAL = "social"
    CORRUPTED = "corrupted"
    ANCIENT = "ancient"

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


# ======================================
# SQLModel ORM Tables
# ======================================

class EncountersTimestampedModel(SQLModel):
    """Base model with encounters-specific tracking."""
    
    created_at: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime), index=True)
    updated_at: datetime | None = SQLField(default=None, sa_column=Column(DateTime))
    generation_metadata: str = SQLField(default="{}", sa_column=Column(JSON), description="Generation process metadata")


class EncounterRecord(EncountersTimestampedModel, table=True):
    """Base encounter record for all encounter types"""
    __tablename__ = "encounters"
    
    # Primary key
    encounter_id: str = SQLField(primary_key=True, description="Unique encounter identifier")
    
    # Basic properties
    encounter_name: str = SQLField(description="Human-readable encounter name")
    encounter_type: str = SQLField(description="EncounterType enum value")
    difficulty: str = SQLField(description="EncounterDifficulty enum value")
    
    # Cross-system references
    source_entities: str = SQLField(default="[]", sa_column=Column(JSON), description="Entity IDs from entities subpackage")
    psychology_data: str = SQLField(default="{}", sa_column=Column(JSON), description="Psychology integration data")
    world_context: str = SQLField(default="{}", sa_column=Column(JSON), description="World subpackage context")
    map_references: str = SQLField(default="[]", sa_column=Column(JSON), description="Map hex locations")
    
    # Horror progression
    dread_level: int = SQLField(default=0, ge=0, le=4, description="Horror progression stage")
    corruption_stage: str = SQLField(default="CLEAN", description="Environmental corruption level")
    act_context: str = SQLField(default="PROLOGUE", description="Story act context")
    
    # Philosophy integration
    philosophy_approaches: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy-specific approaches")
    moral_choices: str = SQLField(default="[]", sa_column=Column(JSON), description="Moral decision points")
    
    # Cross-system coherence
    coherence_score: float = SQLField(default=0.0, description="Cross-system data coherence (0-1)")
    validation_notes: str = SQLField(default="[]", sa_column=Column(JSON), description="Cross-system validation results")


class CombatScenarioRecord(EncountersTimestampedModel, table=True):
    """Combat encounter scenarios with tactical elements"""
    __tablename__ = "combat_scenarios"
    
    # Primary key and relation
    encounter_id: str = SQLField(primary_key=True, foreign_key="encounters.encounter_id", description="Reference to base encounter")
    
    # Combat specifics
    scenario_name: str = SQLField(description="Combat scenario name")
    biome_context: str = SQLField(description="Biome from entities subpackage")
    entity_combatants: str = SQLField(default="[]", sa_column=Column(JSON), description="Entity IDs for combatants")
    
    # Tactical elements
    encounter_setup: str = SQLField(sa_column=Column(Text), description="Combat encounter description")
    tactical_elements: str = SQLField(default="[]", sa_column=Column(JSON), description="Tactical considerations")
    environmental_hazards: str = SQLField(default="[]", sa_column=Column(JSON), description="Environmental factors")
    victory_conditions: str = SQLField(default="[]", sa_column=Column(JSON), description="Ways to resolve encounter")
    
    # Cross-system integration
    psychology_tension: str = SQLField(default="{}", sa_column=Column(JSON), description="Psychological tension factors")
    map_location: str | None = SQLField(default=None, description="Hex location from maps")
    
    # Combat balance
    recommended_party_size: int = SQLField(default=4, description="Recommended party size")
    estimated_duration: int = SQLField(default=30, description="Estimated encounter duration (minutes)")
    encounter_cr: float = SQLField(default=1.0, description="Challenge rating estimate")
    
    # Asset generation hints
    visual_description: str = SQLField(sa_column=Column(Text), description="Visual description for asset generation")
    audio_cues: str = SQLField(default="[]", sa_column=Column(JSON), description="Audio elements needed")
    special_effects: str = SQLField(default="[]", sa_column=Column(JSON), description="Special effects required")


class ScriptedEventRecord(EncountersTimestampedModel, table=True):
    """Scripted story events for narrative progression"""
    __tablename__ = "scripted_events"
    
    # Primary key and relation
    event_id: str = SQLField(primary_key=True, foreign_key="encounters.encounter_id", description="Reference to base encounter")
    
    # Event specifics
    event_name: str = SQLField(description="Scripted event name")
    region_context: str = SQLField(description="World region context")
    
    # Story integration
    world_story_context: str = SQLField(default="{}", sa_column=Column(JSON), description="World progression context")
    psychology_character_development: str = SQLField(default="{}", sa_column=Column(JSON), description="Character psychology changes")
    involved_entities: str = SQLField(default="[]", sa_column=Column(JSON), description="NPCs/locations from entities")
    
    # Event content
    event_description: str = SQLField(sa_column=Column(Text), description="Detailed event description")
    story_significance: str = SQLField(description="Narrative importance level")
    character_development_opportunities: str = SQLField(default="[]", sa_column=Column(JSON), description="Character growth moments")
    plot_advancement: str = SQLField(sa_column=Column(Text), description="How event advances main plot")
    
    # Philosophy integration
    philosophy_choice_modifiers: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy-specific choices")
    moral_consequences: str = SQLField(default="[]", sa_column=Column(JSON), description="Long-term moral impacts")
    
    # Horror elements
    horror_elements: str = SQLField(default="[]", sa_column=Column(JSON), description="Specific horror techniques used")
    dread_escalation: bool = SQLField(default=False, description="Whether event escalates dread")
    
    # Presentation
    cutscene_required: bool = SQLField(default=False, description="Whether event needs cutscene")
    branching_paths: str = SQLField(default="{}", sa_column=Column(JSON), description="Multiple outcome paths")
    prerequisite_conditions: str = SQLField(default="[]", sa_column=Column(JSON), description="Conditions required to trigger event")


class BeastEncounterRecord(EncountersTimestampedModel, table=True):
    """Beast encounter records with ecological integration"""
    __tablename__ = "beast_encounters"
    
    # Primary key and relation
    encounter_id: str = SQLField(primary_key=True, foreign_key="encounters.encounter_id", description="Reference to base encounter")
    
    # Beast specifics
    encounter_name: str = SQLField(description="Beast encounter name")
    beast_entity_id: str = SQLField(description="Beast entity ID from entities subpackage")
    habitat_context: str = SQLField(description="Habitat biome from entities")
    
    # Behavior patterns
    beast_behavior: str = SQLField(description="BeastBehavior enum value")
    behavior_description: str = SQLField(sa_column=Column(Text), description="Beast behavior patterns")
    habitat_advantages: str = SQLField(default="[]", sa_column=Column(JSON), description="Environmental advantages beast has")
    
    # Interaction possibilities
    interaction_possibilities: str = SQLField(default="[]", sa_column=Column(JSON), description="Non-combat interaction options")
    philosophy_interactions: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy-specific beast interactions")
    taming_possibilities: str = SQLField(default="{}", sa_column=Column(JSON), description="Approach success probabilities")
    
    # Ecological context
    ecosystem_role: str = SQLField(sa_column=Column(Text), description="Beast's role in biome ecosystem")
    corruption_effects: str = SQLField(default="{}", sa_column=Column(JSON), description="Psychology corruption impact")
    corruption_symptoms: str = SQLField(default="[]", sa_column=Column(JSON), description="Visible corruption effects")
    
    # Territory and movement
    territory_hex_locations: str = SQLField(default="[]", sa_column=Column(JSON), description="Territory from maps")
    migration_patterns: str = SQLField(default="[]", sa_column=Column(JSON), description="Seasonal/corruption-based movement")
    territorial_aggression: float = SQLField(default=0.5, description="Territorial aggression level (0-1)")


class NPCInteractionRecord(EncountersTimestampedModel, table=True):
    """NPC interaction records with relationship tracking"""
    __tablename__ = "npc_interactions"
    
    # Primary key and relation
    interaction_id: str = SQLField(primary_key=True, foreign_key="encounters.encounter_id", description="Reference to base encounter")
    
    # NPC specifics
    interaction_name: str = SQLField(description="NPC interaction name")
    npc_entity_id: str = SQLField(description="NPC entity ID from entities subpackage")
    regional_context: str = SQLField(description="World region context")
    
    # Interaction properties
    interaction_category: str = SQLField(description="Type of interaction (dialogue, trade, quest, etc.)")
    relationship_stage: str = SQLField(default="first_meeting", description="Relationship development stage")
    
    # NPC psychology
    psychology_profile: str = SQLField(default="{}", sa_column=Column(JSON), description="NPC psychological state")
    personality_traits: str = SQLField(default="[]", sa_column=Column(JSON), description="Key personality characteristics")
    emotional_state: str = SQLField(description="Current emotional condition")
    goals_and_motivations: str = SQLField(default="[]", sa_column=Column(JSON), description="What drives this NPC")
    
    # Information and services
    information_available: str = SQLField(default="[]", sa_column=Column(JSON), description="Information NPC can provide")
    services_offered: str = SQLField(default="[]", sa_column=Column(JSON), description="Services NPC provides")
    items_available: str = SQLField(default="[]", sa_column=Column(JSON), description="Items for trade/sale")
    
    # Philosophy integration
    philosophy_affinities: str = SQLField(default="{}", sa_column=Column(JSON), description="NPC philosophy alignment")
    trust_modifiers: str = SQLField(default="{}", sa_column=Column(JSON), description="Trust based on player philosophy")
    
    # Progression
    relationship_progression: str = SQLField(default="{}", sa_column=Column(JSON), description="How relationship can develop")
    quest_hooks: str = SQLField(default="[]", sa_column=Column(JSON), description="Potential quests from this NPC")
    
    # Horror integration
    horror_awareness: float = SQLField(default=0.0, description="NPC's awareness of horror elements (0-1)")
    trauma_responses: str = SQLField(default="[]", sa_column=Column(JSON), description="How NPC responds to trauma")
    
    # Cultural context
    cultural_background: str = SQLField(default="{}", sa_column=Column(JSON), description="Regional cultural context")


class EncounterTableRecord(EncountersTimestampedModel, table=True):
    """Encounter probability tables for procedural generation"""
    __tablename__ = "encounter_tables"
    
    # Primary key
    table_id: str = SQLField(primary_key=True, description="Unique table identifier")
    
    # Table properties
    table_name: str = SQLField(description="Encounter table name")
    table_scope: str = SQLField(description="Scope (region, biome, level, etc.)")
    
    # Filtering criteria
    region_filter: str | None = SQLField(default=None, description="Region-specific encounters")
    biome_filter: str | None = SQLField(default=None, description="Biome-specific encounters")
    level_range_min: int = SQLField(description="Minimum level for this table")
    level_range_max: int = SQLField(description="Maximum level for this table")
    dread_level_filter: int | None = SQLField(default=None, description="Dread level filter")
    
    # Encounter distributions
    combat_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="Combat encounter IDs and weights")
    scripted_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="Scripted event IDs and weights")
    beast_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="Beast encounter IDs and weights")
    npc_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="NPC interaction IDs and weights")
    environmental_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="Environmental encounter IDs and weights")
    
    # Scaling parameters
    difficulty_scaling: str = SQLField(default="{}", sa_column=Column(JSON), description="Level to difficulty mapping")
    party_size_modifiers: str = SQLField(default="{}", sa_column=Column(JSON), description="Party size adjustment multipliers")
    
    # Philosophy integration
    philosophy_encounter_modifiers: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy-based encounter frequency")
    moral_encounter_frequency: str = SQLField(default="{}", sa_column=Column(JSON), description="Moral choice encounter rates")
    
    # Horror progression
    dread_escalation_encounters: str = SQLField(default="{}", sa_column=Column(JSON), description="Dread-escalating encounters")
    corruption_spread_encounters: str = SQLField(default="[]", sa_column=Column(JSON), description="Encounters that spread corruption")
    
    # Statistics and metadata
    total_encounters: int = SQLField(default=0, description="Total encounters in table")
    encounter_diversity_score: float = SQLField(default=0.0, description="Diversity of encounter types (0-1)")


class EncounterExtractionMetrics(EncountersTimestampedModel, table=True):
    """Metrics tracking for encounter generation and cross-system integration"""
    __tablename__ = "encounter_extraction_metrics"
    
    # Primary key
    extraction_id: str = SQLField(primary_key=True, description="Unique extraction run identifier")
    
    # Extraction metadata
    extraction_timestamp: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    extraction_type: str = SQLField(description="Type of extraction (full, incremental, specific)")
    
    # Cross-system integration metrics
    entities_integration_score: float = SQLField(default=0.0, description="Entities subpackage integration quality (0-1)")
    psychology_integration_score: float = SQLField(default=0.0, description="Psychology subpackage integration quality (0-1)")
    world_integration_score: float = SQLField(default=0.0, description="World subpackage integration quality (0-1)")
    maps_integration_score: float = SQLField(default=0.0, description="Maps subpackage integration quality (0-1)")
    overall_coherence_score: float = SQLField(default=0.0, description="Overall cross-system coherence (0-1)")
    
    # Generation metrics
    total_encounters_generated: int = SQLField(default=0, description="Total encounters generated")
    combat_scenarios_generated: int = SQLField(default=0, description="Combat scenarios generated")
    scripted_events_generated: int = SQLField(default=0, description="Scripted events generated")
    beast_encounters_generated: int = SQLField(default=0, description="Beast encounters generated")
    npc_interactions_generated: int = SQLField(default=0, description="NPC interactions generated")
    encounter_tables_generated: int = SQLField(default=0, description="Encounter tables generated")
    
    # Quality metrics
    encounter_diversity_score: float = SQLField(default=0.0, description="Diversity of encounter types (0-1)")
    philosophy_integration_score: float = SQLField(default=0.0, description="Philosophy integration quality (0-1)")
    horror_progression_score: float = SQLField(default=0.0, description="Horror progression authenticity (0-1)")
    moral_complexity_score: float = SQLField(default=0.0, description="Moral choice complexity (0-1)")
    
    # Performance metrics
    extraction_duration_seconds: float = SQLField(default=0.0, description="Total extraction time")
    ml_api_calls: int = SQLField(default=0, description="Number of ML API calls made")
    cross_system_queries: int = SQLField(default=0, description="Number of cross-system data queries")
    
    # Error tracking
    extraction_errors: str = SQLField(default="[]", sa_column=Column(JSON), description="Errors encountered during extraction")
    validation_failures: str = SQLField(default="[]", sa_column=Column(JSON), description="Validation failures")
    coherence_warnings: str = SQLField(default="[]", sa_column=Column(JSON), description="Cross-system coherence warnings")
    
    # Source data tracking
    source_subpackages: str = SQLField(default="[]", sa_column=Column(JSON), description="Subpackages used in generation")
    cross_system_dependencies: str = SQLField(default="{}", sa_column=Column(JSON), description="Inter-subpackage dependencies")
    entities_data_version: str | None = SQLField(default=None, description="Version of entities data used")
    psychology_data_version: str | None = SQLField(default=None, description="Version of psychology data used")
    world_data_version: str | None = SQLField(default=None, description="Version of world data used")
    maps_data_version: str | None = SQLField(default=None, description="Version of maps data used")


# ======================================
# Pydantic Data Models
# ======================================

class EncounterData(BaseModel):
    """Complete encounter data structure"""
    encounter_id: str
    name: str
    encounter_type: EncounterType
    difficulty: EncounterDifficulty
    
    # Cross-system integration
    source_entities: list[str]
    psychology_data: dict[str, Any]
    world_context: dict[str, Any]
    map_references: list[str]
    
    # Horror progression
    dread_level: DreadLevel
    corruption_stage: CorruptionStage
    act_context: ActStage
    
    # Philosophy integration
    philosophy_approaches: dict[str, str]
    moral_choices: list[str]
    
    # Quality metrics
    coherence_score: float
    
    model_config = ConfigDict(extra="forbid")


class CombatScenario(BaseModel):
    """Combat encounter scenario"""
    encounter_id: str
    name: str
    biome_context: BiomeType
    
    # Combat details
    entity_combatants: list[str]
    encounter_setup: str
    tactical_elements: list[str]
    environmental_hazards: list[str]
    victory_conditions: list[str]
    
    # Balance
    recommended_party_size: int
    encounter_cr: float
    estimated_duration: int
    
    # Cross-system data
    psychology_tension: dict[str, Any]
    map_location: str | None
    
    model_config = ConfigDict(extra="forbid")


class ScriptedEvent(BaseModel):
    """Scripted story event"""
    event_id: str
    name: str
    region_context: RegionType
    
    # Story content
    event_description: str
    story_significance: str
    plot_advancement: str
    
    # Cross-system integration
    world_story_context: dict[str, Any]
    psychology_character_development: dict[str, Any]
    involved_entities: list[str]
    
    # Philosophy integration
    philosophy_choice_modifiers: dict[str, Any]
    moral_consequences: list[str]
    
    # Horror elements
    horror_elements: list[str]
    dread_escalation: bool
    
    model_config = ConfigDict(extra="forbid")


class BeastEncounter(BaseModel):
    """Beast encounter data"""
    encounter_id: str
    name: str
    beast_entity_id: str
    habitat_context: BiomeType
    
    # Behavior
    beast_behavior: BeastBehavior
    behavior_description: str
    
    # Interactions
    interaction_possibilities: list[str]
    philosophy_interactions: dict[str, str]
    taming_possibilities: dict[str, float]
    
    # Ecology
    ecosystem_role: str
    corruption_effects: dict[str, str]
    territory_hex_locations: list[str]
    
    model_config = ConfigDict(extra="forbid")


class NPCInteraction(BaseModel):
    """NPC interaction data"""
    interaction_id: str
    name: str
    npc_entity_id: str
    regional_context: RegionType
    
    # Interaction properties
    interaction_category: str
    relationship_stage: str
    
    # Psychology
    psychology_profile: dict[str, Any]
    personality_traits: list[str]
    emotional_state: str
    
    # Services
    information_available: list[str]
    services_offered: list[str]
    items_available: list[str]
    
    # Philosophy integration
    philosophy_affinities: dict[str, float]
    trust_modifiers: dict[str, float]
    
    # Horror integration
    horror_awareness: float
    trauma_responses: list[str]
    
    model_config = ConfigDict(extra="forbid")
