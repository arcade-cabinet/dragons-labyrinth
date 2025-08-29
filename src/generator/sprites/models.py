"""
SQLModel tables and Pydantic models for sprites and character system.

Combines ORM definitions with character data models.
Uses SQLModel for Godot-compatible SQLite database.
"""

from datetime import datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field, ConfigDict
from sqlmodel import SQLModel, Field as SQLField, Relationship, Column, JSON, DateTime, Text


# ======================================
# Sprites Types and Enums
# ======================================

class CharacterType(str, Enum):
    """Types of characters"""
    NPC = "npc"
    COMPANION = "companion"
    MONSTER = "monster"
    MERCENARY = "mercenary"
    BOSS = "boss"
    VILLAIN = "villain"

class CompanionRole(str, Enum):
    """Companion role classifications"""
    WARRIOR = "warrior"
    HEALER = "healer"
    SCHOLAR = "scholar"
    ROGUE = "rogue"
    MYSTIC = "mystic"
    SURVIVOR = "survivor"
    GUIDE = "guide"
    CRAFTSMAN = "craftsman"

class MonsterCategory(str, Enum):
    """Monster classification types"""
    BEAST = "beast"
    CORRUPTED = "corrupted"
    UNDEAD = "undead"
    ELEMENTAL = "elemental"
    ABERRATION = "aberration"
    DRAGON_SPAWN = "dragon_spawn"
    VOID_ENTITY = "void_entity"

class TraumaType(str, Enum):
    """Types of trauma characters can experience"""
    PHYSICAL = "physical"
    EMOTIONAL = "emotional"
    PSYCHOLOGICAL = "psychological"
    SPIRITUAL = "spiritual"
    SOCIAL = "social"
    EXISTENTIAL = "existential"

class EmotionalState(str, Enum):
    """Character emotional states"""
    STABLE = "stable"
    STRESSED = "stressed"
    ANXIOUS = "anxious"
    DEPRESSED = "depressed"
    TRAUMATIZED = "traumatized"
    RECOVERING = "recovering"
    BROKEN = "broken"

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

class SpritesTimestampedModel(SQLModel):
    """Base model with sprites-specific tracking."""
    
    created_at: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime), index=True)
    updated_at: datetime | None = SQLField(default=None, sa_column=Column(DateTime))
    generation_metadata: str = SQLField(default="{}", sa_column=Column(JSON), description="Generation process metadata")


class CharacterRecord(SpritesTimestampedModel, table=True):
    """Base character record for all character types"""
    __tablename__ = "characters"
    
    # Primary key
    character_id: str = SQLField(primary_key=True, description="Unique character identifier")
    
    # Basic properties
    character_name: str = SQLField(description="Character name")
    character_type: str = SQLField(description="CharacterType enum value")
    age: int = SQLField(description="Character age")
    
    # Cross-system references
    base_entity_id: str | None = SQLField(default=None, description="Entity from entities subpackage")
    psychology_data: str = SQLField(default="{}", sa_column=Column(JSON), description="Psychology integration data")
    world_context: str = SQLField(default="{}", sa_column=Column(JSON), description="World subpackage context")
    regional_data: str = SQLField(default="{}", sa_column=Column(JSON), description="Regional context data")
    
    # Physical description
    physical_description: str = SQLField(sa_column=Column(Text), description="Physical appearance")
    clothing_style: str = SQLField(description="Clothing and fashion")
    distinguishing_features: str = SQLField(default="[]", sa_column=Column(JSON), description="Notable features")
    
    # Psychology integration
    emotional_profile: str = SQLField(default="{}", sa_column=Column(JSON), description="Emotional profile data")
    personality_traits: str = SQLField(default="[]", sa_column=Column(JSON), description="Personality characteristics")
    goals_and_motivations: str = SQLField(default="[]", sa_column=Column(JSON), description="Character motivations")
    
    # World integration
    home_region: str | None = SQLField(default=None, description="Home region")
    philosophy_alignment: str | None = SQLField(default=None, description="Philosophy path")
    dread_tolerance: int | None = SQLField(default=None, description="Dread tolerance level")
    corruption_stage: str = SQLField(default="CLEAN", description="Corruption level")
    
    # Cross-system coherence
    coherence_score: float = SQLField(default=0.0, description="Cross-system data coherence (0-1)")
    validation_notes: str = SQLField(default="[]", sa_column=Column(JSON), description="Cross-system validation results")


class NPCRecord(SpritesTimestampedModel, table=True):
    """NPC character records with social integration"""
    __tablename__ = "npc_characters"
    
    # Primary key and relation
    character_id: str = SQLField(primary_key=True, foreign_key="characters.character_id", description="Reference to base character")
    
    # NPC specifics
    npc_id: str = SQLField(unique=True, description="Unique NPC identifier")
    occupation: str = SQLField(description="Character's job or role")
    social_class: str = SQLField(description="Social standing")
    
    # Cultural integration
    cultural_background: str = SQLField(default="{}", sa_column=Column(JSON), description="Regional culture data")
    relationships: str = SQLField(default="{}", sa_column=Column(JSON), description="Relationships with other NPCs")
    reputation: str = SQLField(default="{}", sa_column=Column(JSON), description="Reputation with groups")
    
    # Social interaction
    dialogue_themes: str = SQLField(default="[]", sa_column=Column(JSON), description="Conversation topics")
    quest_potential: str = SQLField(default="[]", sa_column=Column(JSON), description="Potential quests")
    services_offered: str = SQLField(default="[]", sa_column=Column(JSON), description="Services provided")
    
    # Psychology integration
    fears_and_anxieties: str = SQLField(default="[]", sa_column=Column(JSON), description="Character fears")
    emotional_triggers: str = SQLField(default="[]", sa_column=Column(JSON), description="Emotional triggers")
    coping_mechanisms: str = SQLField(default="[]", sa_column=Column(JSON), description="Stress coping methods")
    
    # Gameplay integration
    interaction_complexity: float = SQLField(default=0.5, description="Complexity of interactions (0-1)")
    story_importance: float = SQLField(default=0.3, description="Importance to main story (0-1)")
    player_relationship_potential: float = SQLField(default=0.6, description="Potential for player relationships (0-1)")


class CompanionRecord(SpritesTimestampedModel, table=True):
    """Companion character records with trauma/therapy integration"""
    __tablename__ = "companion_characters"
    
    # Primary key and relation
    character_id: str = SQLField(primary_key=True, foreign_key="characters.character_id", description="Reference to base character")
    
    # Companion specifics
    companion_id: str = SQLField(unique=True, description="Unique companion identifier")
    companion_role: str = SQLField(description="CompanionRole enum value")
    origin_region: str = SQLField(description="Origin region")
    
    # Combat and abilities
    combat_specialization: str = SQLField(description="Combat focus")
    equipment_preferences: str = SQLField(default="[]", sa_column=Column(JSON), description="Preferred equipment")
    special_abilities: str = SQLField(default="[]", sa_column=Column(JSON), description="Unique abilities")
    
    # Trauma and therapy system
    trauma_vulnerabilities: str = SQLField(default="[]", sa_column=Column(JSON), description="Trauma susceptibilities")
    current_traumas: str = SQLField(default="[]", sa_column=Column(JSON), description="Active traumas")
    therapy_progress: str = SQLField(default="{}", sa_column=Column(JSON), description="Healing progress")
    therapy_responsiveness: float = SQLField(default=0.7, description="Response to therapy (0-1)")
    
    # Character development
    character_arc_milestones: str = SQLField(default="[]", sa_column=Column(JSON), description="Growth milestones")
    loyalty_factors: str = SQLField(default="[]", sa_column=Column(JSON), description="Loyalty influences")
    loyalty_level: float = SQLField(default=0.5, description="Current loyalty to player (0-1)")
    
    # Relationships
    therapeutic_relationships: str = SQLField(default="{}", sa_column=Column(JSON), description="Therapy effectiveness with others")
    relationship_dynamics: str = SQLField(default="{}", sa_column=Column(JSON), description="Relationships with other companions")
    
    # Horror progression
    corruption_resistance: float = SQLField(default=0.6, description="Resistance to corruption (0-1)")
    dread_adaptation: str = SQLField(default="{}", sa_column=Column(JSON), description="Adaptation to dread levels")
    
    # Story integration
    story_integration: str = SQLField(default="{}", sa_column=Column(JSON), description="World story integration data")
    character_arc_completion: float = SQLField(default=0.0, description="Character arc progress (0-1)")


class MonsterRecord(SpritesTimestampedModel, table=True):
    """Monster character records with corruption progression"""
    __tablename__ = "monster_characters"
    
    # Primary key and relation
    character_id: str = SQLField(primary_key=True, foreign_key="characters.character_id", description="Reference to base character")
    
    # Monster specifics
    monster_id: str = SQLField(unique=True, description="Unique monster identifier")
    monster_name: str = SQLField(description="Monster name")
    monster_category: str = SQLField(description="MonsterCategory enum value")
    size_category: str = SQLField(description="Physical size category")
    threat_level: float = SQLField(default=1.0, description="Threat level (0-10)")
    
    # Horror and corruption
    horror_theme: str = SQLField(description="Primary horror theme")
    corruption_variants: str = SQLField(default="{}", sa_column=Column(JSON), description="Corruption stage appearances")
    horror_escalation: str = SQLField(default="{}", sa_column=Column(JSON), description="Dread level behaviors")
    
    # Behavior and ecology
    behavior_patterns: str = SQLField(default="[]", sa_column=Column(JSON), description="Behavioral characteristics")
    environmental_preferences: str = SQLField(default="[]", sa_column=Column(JSON), description="Habitat preferences")
    social_structure: str = SQLField(default="solitary", description="Pack behavior type")
    
    # Combat characteristics
    combat_tactics: str = SQLField(default="[]", sa_column=Column(JSON), description="Combat behavior")
    weaknesses: str = SQLField(default="[]", sa_column=Column(JSON), description="Monster weaknesses")
    special_abilities: str = SQLField(default="[]", sa_column=Column(JSON), description="Unique abilities")
    
    # Cross-system integration
    habitat_regions: str = SQLField(default="[]", sa_column=Column(JSON), description="Regions where found")
    corruption_psychology: str = SQLField(default="{}", sa_column=Column(JSON), description="Corruption psychology data")
    horror_theme_integration: str = SQLField(default="{}", sa_column=Column(JSON), description="Horror theme integration")
    
    # Philosophy interaction
    philosophy_responses: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy-based responses")
    moral_complexity: str | None = SQLField(default=None, description="Moral aspects")
    
    # Encounter design
    encounter_frequency: float = SQLField(default=0.3, description="How often encountered (0-1)")
    scaling_difficulty: str = SQLField(default="{}", sa_column=Column(JSON), description="Difficulty scaling by level")


class MercenaryRecord(SpritesTimestampedModel, table=True):
    """Mercenary character records for hireable characters"""
    __tablename__ = "mercenary_characters"
    
    # Primary key and relation
    character_id: str = SQLField(primary_key=True, foreign_key="characters.character_id", description="Reference to base character")
    
    # Mercenary specifics
    mercenary_id: str = SQLField(unique=True, description="Unique mercenary identifier")
    specialization: str = SQLField(description="Professional specialization")
    experience_level: float = SQLField(default=0.5, description="Experience level (0-1)")
    regional_background: str = SQLField(description="Home region")
    
    # Skills and equipment
    equipment: str = SQLField(default="[]", sa_column=Column(JSON), description="Standard equipment")
    special_skills: str = SQLField(default="[]", sa_column=Column(JSON), description="Specialized abilities")
    utility_skills: str = SQLField(default="[]", sa_column=Column(JSON), description="Non-combat skills")
    
    # Hiring mechanics
    hire_cost: int = SQLField(default=100, description="Base hiring cost")
    loyalty_requirements: str = SQLField(default="[]", sa_column=Column(JSON), description="Loyalty requirements")
    contract_preferences: str = SQLField(default="[]", sa_column=Column(JSON), description="Contract preferences")
    deal_breakers: str = SQLField(default="[]", sa_column=Column(JSON), description="Things that make them leave")
    
    # Performance metrics
    combat_effectiveness: float = SQLField(default=0.6, description="Combat skill (0-1)")
    reliability: float = SQLField(default=0.7, description="Contract reliability (0-1)")
    adaptability: float = SQLField(default=0.5, description="Adaptability to situations (0-1)")
    
    # Psychology
    personality_overview: str = SQLField(sa_column=Column(Text), description="Personality description")
    motivations: str = SQLField(default="[]", sa_column=Column(JSON), description="What drives them")
    corruption_resistance: float = SQLField(default=0.5, description="Corruption resistance (0-1)")
    
    # Regional context
    specialization_context: str = SQLField(default="{}", sa_column=Column(JSON), description="Regional specialization context")
    local_reputation: float = SQLField(default=0.5, description="Local reputation (0-1)")


class CharacterRosterRecord(SpritesTimestampedModel, table=True):
    """Character roster metadata and organization"""
    __tablename__ = "character_rosters"
    
    # Primary key
    roster_id: str = SQLField(primary_key=True, description="Unique roster identifier")
    
    # Roster properties
    roster_name: str = SQLField(description="Character roster name")
    generation_method: str = SQLField(description="Generation approach used")
    generation_timestamp: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    
    # Character counts
    total_npcs: int = SQLField(default=0, description="Count of NPCs")
    total_companions: int = SQLField(default=0, description="Count of companions")
    total_monsters: int = SQLField(default=0, description="Count of monsters")
    total_mercenaries: int = SQLField(default=0, description="Count of mercenaries")
    total_characters: int = SQLField(default=0, description="Total character count")
    
    # Organization data
    characters_by_region: str = SQLField(default="{}", sa_column=Column(JSON), description="Regional organization")
    characters_by_dread_level: str = SQLField(default="{}", sa_column=Column(JSON), description="Dread level organization")
    characters_by_philosophy: str = SQLField(default="{}", sa_column=Column(JSON), description="Philosophy organization")
    characters_by_corruption: str = SQLField(default="{}", sa_column=Column(JSON), description="Corruption organization")
    
    # Relationship tracking
    therapeutic_relationships: str = SQLField(default="{}", sa_column=Column(JSON), description="Therapy relationships")
    social_networks: str = SQLField(default="{}", sa_column=Column(JSON), description="Social connections")
    
    # Cross-system metrics
    entities_integration_score: float = SQLField(default=0.0, description="Entities integration quality (0-1)")
    psychology_integration_score: float = SQLField(default=0.0, description="Psychology integration quality (0-1)")
    world_integration_score: float = SQLField(default=0.0, description="World integration quality (0-1)")
    overall_coherence_score: float = SQLField(default=0.0, description="Overall coherence (0-1)")
    
    # Quality metrics
    character_diversity_score: float = SQLField(default=0.0, description="Character diversity (0-1)")
    philosophy_integration_score: float = SQLField(default=0.0, description="Philosophy integration (0-1)")
    emotional_authenticity_score: float = SQLField(default=0.0, description="Emotional authenticity (0-1)")
    trauma_system_completeness: float = SQLField(default=0.0, description="Trauma system completeness (0-1)")
    
    # Source tracking
    source_subpackages: str = SQLField(default="[]", sa_column=Column(JSON), description="Source subpackages")
    cross_system_dependencies: str = SQLField(default="{}", sa_column=Column(JSON), description="Cross-system dependencies")


class SpriteExtractionMetrics(SpritesTimestampedModel, table=True):
    """Metrics tracking for character generation and cross-system integration"""
    __tablename__ = "sprite_extraction_metrics"
    
    # Primary key
    extraction_id: str = SQLField(primary_key=True, description="Unique extraction run identifier")
    
    # Extraction metadata
    extraction_timestamp: datetime = SQLField(default_factory=datetime.utcnow, sa_column=Column(DateTime))
    extraction_type: str = SQLField(description="Type of extraction (full, incremental, specific)")
    
    # Cross-system integration metrics
    entities_integration_score: float = SQLField(default=0.0, description="Entities subpackage integration quality (0-1)")
    psychology_integration_score: float = SQLField(default=0.0, description="Psychology subpackage integration quality (0-1)")
    world_integration_score: float = SQLField(default=0.0, description="World subpackage integration quality (0-1)")
    overall_coherence_score: float = SQLField(default=0.0, description="Overall cross-system coherence (0-1)")
    
    # Generation metrics
    total_characters_generated: int = SQLField(default=0, description="Total characters generated")
    npcs_generated: int = SQLField(default=0, description="NPCs generated")
    companions_generated: int = SQLField(default=0, description="Companions generated")
    monsters_generated: int = SQLField(default=0, description="Monsters generated")
    mercenaries_generated: int = SQLField(default=0, description="Mercenaries generated")
    
    # Quality metrics
    character_diversity_score: float = SQLField(default=0.0, description="Character diversity (0-1)")
    philosophy_integration_score: float = SQLField(default=0.0, description="Philosophy integration quality (0-1)")
    emotional_authenticity_score: float = SQLField(default=0.0, description="Emotional authenticity (0-1)")
    trauma_system_completeness: float = SQLField(default=0.0, description="Trauma system completeness (0-1)")
    
    # Therapy system metrics
    therapeutic_relationships_created: int = SQLField(default=0, description="Number of therapy relationships")
    therapy_approaches_defined: int = SQLField(default=0, description="Number of therapy approaches")
    trauma_types_covered: int = SQLField(default=0, description="Number of trauma types covered")
    
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


# ======================================
# Pydantic Data Models
# ======================================

class CharacterData(BaseModel):
    """Complete character data structure"""
    character_id: str
    name: str
    character_type: CharacterType
    age: int
    
    # Physical attributes
    physical_description: str
    clothing_style: str
    distinguishing_features: list[str]
    
    # Psychology integration
    emotional_profile: dict[str, Any]
    personality_traits: list[str]
    goals_and_motivations: list[str]
    
    # World integration
    home_region: RegionType | None
    philosophy_alignment: PhilosophyPath | None
    dread_tolerance: DreadLevel | None
    corruption_stage: CorruptionStage
    
    # Cross-system references
    base_entity_id: str | None
    psychology_data: dict[str, Any]
    world_context: dict[str, Any]
    
    model_config = ConfigDict(extra="forbid")


class CompanionCharacter(BaseModel):
    """Companion character with trauma/therapy system"""
    character_id: str
    companion_id: str
    name: str
    role: CompanionRole
    origin_region: RegionType
    
    # Abilities
    combat_specialization: str
    equipment_preferences: list[str]
    special_abilities: list[str]
    
    # Trauma system
    trauma_vulnerabilities: list[TraumaType]
    current_traumas: list[TraumaType]
    therapy_progress: dict[str, float]
    therapy_responsiveness: float
    
    # Development
    character_arc_milestones: list[str]
    loyalty_factors: list[str]
    loyalty_level: float
    
    # Relationships
    therapeutic_relationships: dict[str, float]
    relationship_dynamics: dict[str, str]
    
    # Horror progression
    corruption_resistance: float
    dread_adaptation: dict[str, str]
    
    model_config = ConfigDict(extra="forbid")


class NPCCharacter(BaseModel):
    """NPC character with social integration"""
    character_id: str
    npc_id: str
    name: str
    occupation: str
    social_class: str
    
    # Social integration
    cultural_background: dict[str, Any]
    relationships: dict[str, str]
    reputation: dict[str, float]
    
    # Interaction
    dialogue_themes: list[str]
    quest_potential: list[str]
    services_offered: list[str]
    
    # Psychology
    fears_and_anxieties: list[str]
    emotional_triggers: list[str]
    coping_mechanisms: list[str]
    
    # Gameplay
    interaction_complexity: float
    story_importance: float
    player_relationship_potential: float
    
    model_config = ConfigDict(extra="forbid")


class MonsterCharacter(BaseModel):
    """Monster character with corruption system"""
    character_id: str
    monster_id: str
    name: str
    category: MonsterCategory
    size_category: str
    threat_level: float
    
    # Horror system
    horror_theme: str
    corruption_variants: dict[str, str]
    horror_escalation: dict[str, str]
    
    # Behavior
    behavior_patterns: list[str]
    environmental_preferences: list[str]
    social_structure: str
    
    # Combat
    combat_tactics: list[str]
    weaknesses: list[str]
    special_abilities: list[str]
    
    # Integration
    habitat_regions: list[str]
    philosophy_responses: dict[str, str]
    moral_complexity: str | None
    
    model_config = ConfigDict(extra="forbid")
