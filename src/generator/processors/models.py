"""
Comprehensive processor models for Dragons Labyrinth entity processing.

Captures ALL rich data from HBF analysis including taverns, rumors, weather,
encounter tables, faction relationships, economic systems, and quest hooks.
"""

from __future__ import annotations

from typing import Any
from pydantic import BaseModel, Field


class ProcessingResult(BaseModel):
    """Standard result structure for all processors."""
    entity_type: str
    entity_name: str
    entity_uuid: str
    success: bool
    output_files: list[str] = Field(default_factory=list)
    entity_count: int = 0
    data: dict[str, Any] = Field(default_factory=dict)
    error_message: str | None = None


class AbilityScores(BaseModel):
    """D&D ability scores from stat blocks."""
    strength: int = 10
    dexterity: int = 10
    constitution: int = 10
    intelligence: int = 10
    wisdom: int = 10
    charisma: int = 10
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "str": self.strength,
            "dex": self.dexterity,
            "con": self.constitution,
            "int": self.intelligence,
            "wis": self.wisdom,
            "cha": self.charisma
        }


class NPCData(BaseModel):
    """Complete NPC data with full D&D stat blocks."""
    name: str
    level: int = 1
    class_type: str = "Commoner"
    race: str = "Human"
    description: str = ""
    personality: str = ""
    alignment: str = "Neutral"
    armor_class: int = 10
    hit_points: int = 1
    speed: int = 30
    proficiency_bonus: int = 2
    abilities: AbilityScores = Field(default_factory=AbilityScores)
    saving_throws: dict[str, int] = Field(default_factory=dict)
    skills: dict[str, int] = Field(default_factory=dict)
    languages: list[str] = Field(default_factory=list)
    equipment: list[str] = Field(default_factory=list)
    money: dict[str, int] = Field(default_factory=dict)  # {"cp": 3, "sp": 2}
    spells: dict[str, list[str]] = Field(default_factory=dict)  # {"cantrips": [], "1st": []}
    attacks: list[dict[str, Any]] = Field(default_factory=list)
    special_abilities: list[str] = Field(default_factory=list)
    faction_membership: str | None = None
    location_origin: str | None = None  # "of Dorith", "of Palemoon"
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "level": self.level,
            "class": self.class_type,
            "race": self.race,
            "description": self.description,
            "personality": self.personality,
            "alignment": self.alignment,
            "combat_stats": {
                "ac": self.armor_class,
                "hp": self.hit_points,
                "speed": self.speed,
                "proficiency": self.proficiency_bonus
            },
            "abilities": self.abilities.to_dict(),
            "saving_throws": self.saving_throws,
            "skills": self.skills,
            "languages": self.languages,
            "equipment": self.equipment,
            "money": self.money,
            "spells": self.spells,
            "attacks": self.attacks,
            "special_abilities": self.special_abilities,
            "faction": self.faction_membership,
            "origin": self.location_origin
        }


class MenuItemData(BaseModel):
    """Food/drink menu items with pricing."""
    name: str
    price: str
    description: str = ""
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "price": self.price,
            "description": self.description
        }


class RandomTable(BaseModel):
    """Random tables (food menus, rumors, encounters)."""
    table_type: str  # "food_menu", "rumors", "encounters"
    dice: str  # "1d4", "1d6", "1d8"
    entries: dict[str, str] = Field(default_factory=dict)  # {"1": "result", "2": "result"}
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "type": self.table_type,
            "dice": self.dice,
            "entries": self.entries
        }


class BulletinData(BaseModel):
    """Bulletin board postings and job offers."""
    title: str
    content: str
    poster: str | None = None
    reward: str | None = None
    location_target: str | None = None
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "title": self.title,
            "content": self.content,
            "poster": self.poster,
            "reward": self.reward,
            "target_location": self.location_target
        }


class TavernData(BaseModel):
    """Complete tavern data with all systems."""
    name: str
    owner: NPCData | None = None
    staff: list[NPCData] = Field(default_factory=list)
    drinks_menu: list[MenuItemData] = Field(default_factory=list)
    food_menu: RandomTable | None = None
    patrons: list[NPCData] = Field(default_factory=list)
    visitors: list[NPCData] = Field(default_factory=list)
    bulletin_board: list[BulletinData] = Field(default_factory=list)
    rumor_table: RandomTable | None = None
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "owner": self.owner.to_dict() if self.owner else None,
            "staff": [staff.to_dict() for staff in self.staff],
            "drinks_menu": [drink.to_dict() for drink in self.drinks_menu],
            "food_menu": self.food_menu.to_dict() if self.food_menu else None,
            "patrons": [patron.to_dict() for patron in self.patrons],
            "visitors": [visitor.to_dict() for visitor in self.visitors],
            "bulletin_board": [post.to_dict() for post in self.bulletin_board],
            "rumor_table": self.rumor_table.to_dict() if self.rumor_table else None
        }


class ShopData(BaseModel):
    """Shop/service establishment data."""
    name: str
    type: str  # "Tanner", "Bakery", "Physician", etc.
    owner: NPCData | None = None
    services: list[str] = Field(default_factory=list)
    inventory: list[dict[str, Any]] = Field(default_factory=list)
    prices: dict[str, str] = Field(default_factory=dict)
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "type": self.type,
            "owner": self.owner.to_dict() if self.owner else None,
            "services": self.services,
            "inventory": self.inventory,
            "prices": self.prices
        }


class WeatherData(BaseModel):
    """Weather systems and environmental conditions."""
    region: str
    conditions: list[str] = Field(default_factory=list)  # ["flood", "storm"]
    seasonal_effects: dict[str, str] = Field(default_factory=dict)
    hazards: list[str] = Field(default_factory=list)
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "region": self.region,
            "conditions": self.conditions,
            "seasonal_effects": self.seasonal_effects,
            "hazards": self.hazards
        }


class EncounterData(BaseModel):
    """Random encounter data from regions."""
    creature_type: str  # "Oni", "Medusa"
    location: str
    challenge_rating: str | None = None
    description: str = ""
    frequency: str | None = None
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "creature": self.creature_type,
            "location": self.location,
            "cr": self.challenge_rating,
            "description": self.description,
            "frequency": self.frequency
        }


class QuestHookData(BaseModel):
    """Quest hooks and plot threads."""
    title: str
    description: str
    quest_giver: str | None = None
    target_location: str | None = None
    reward: str | None = None
    difficulty: str | None = None
    type: str = "delivery"  # "delivery", "rescue", "exploration", "combat"
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "title": self.title,
            "description": self.description,
            "quest_giver": self.quest_giver,
            "target_location": self.target_location,
            "reward": self.reward,
            "difficulty": self.difficulty,
            "type": self.type
        }


class SettlementData(BaseModel):
    """Complete settlement data with all rich systems."""
    name: str
    uuid: str
    population: int = 0
    settlement_type: str = "village"  # village, town, city
    description: str = ""
    taverns: list[TavernData] = Field(default_factory=list)
    shops: list[ShopData] = Field(default_factory=list)
    guards: list[NPCData] = Field(default_factory=list)
    residents: list[NPCData] = Field(default_factory=list)
    quest_hooks: list[QuestHookData] = Field(default_factory=list)
    factions_present: list[str] = Field(default_factory=list)
    notable_features: list[str] = Field(default_factory=list)
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "uuid": self.uuid,
            "settlement_type": self.settlement_type,
            "population": self.population,
            "description": self.description,
            "taverns": [tavern.to_dict() for tavern in self.taverns],
            "shops": [shop.to_dict() for shop in self.shops],
            "guards": [guard.to_dict() for guard in self.guards],
            "residents": [resident.to_dict() for resident in self.residents],
            "quest_hooks": [hook.to_dict() for hook in self.quest_hooks],
            "factions": self.factions_present,
            "features": self.notable_features
        }


class MonsterData(BaseModel):
    """Monster stat blocks with complete D&D data."""
    name: str
    challenge_rating: str = "0"
    armor_class: str = "10"
    hit_points: str = "1"
    speed: str = "30"
    abilities: AbilityScores = Field(default_factory=AbilityScores)
    saving_throws: dict[str, str] = Field(default_factory=dict)
    skills: dict[str, str] = Field(default_factory=dict)
    damage_immunities: list[str] = Field(default_factory=list)
    senses: list[str] = Field(default_factory=list)
    languages: list[str] = Field(default_factory=list)
    special_abilities: list[dict[str, str]] = Field(default_factory=list)
    attacks: list[dict[str, str]] = Field(default_factory=list)
    legendary_actions: list[str] = Field(default_factory=list)
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "cr": self.challenge_rating,
            "ac": self.armor_class,
            "hp": self.hit_points,
            "speed": self.speed,
            "abilities": self.abilities.to_dict(),
            "saving_throws": self.saving_throws,
            "skills": self.skills,
            "immunities": self.damage_immunities,
            "senses": self.senses,
            "languages": self.languages,
            "special_abilities": self.special_abilities,
            "attacks": self.attacks,
            "legendary_actions": self.legendary_actions
        }


class TreasureData(BaseModel):
    """Comprehensive treasure and loot data."""
    type: str  # "currency", "artifact", "magic_item", "mundane"
    description: str
    value_gp: int = 0
    quantity: int = 1
    rarity: str | None = None  # "common", "uncommon", "rare", "legendary"
    magical: bool = False
    cursed: bool = False
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "type": self.type,
            "description": self.description,
            "value": self.value_gp,
            "quantity": self.quantity,
            "rarity": self.rarity,
            "magical": self.magical,
            "cursed": self.cursed
        }


class AreaData(BaseModel):
    """Dungeon area with complete environmental data."""
    uuid: str
    title: str
    area_number: int | None = None
    description: str = ""
    foreshadowing: str = ""
    area_type: str = "chamber"  # entrance, chamber, corridor, boss_room, treasure_room
    monsters: list[MonsterData] = Field(default_factory=list)
    treasures: list[TreasureData] = Field(default_factory=list)
    traps: list[dict[str, Any]] = Field(default_factory=list)
    environmental_details: dict[str, Any] = Field(default_factory=dict)
    connections: list[str] = Field(default_factory=list)  # connected area UUIDs
    lighting: str = "normal"  # bright, dim, dark, magical
    atmosphere: list[str] = Field(default_factory=list)  # ["fossils", "stalactites"]
    hazards: list[str] = Field(default_factory=list)  # ["poison gas", "unstable floor"]
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "uuid": self.uuid,
            "title": self.title,
            "area_number": self.area_number,
            "description": self.description,
            "foreshadowing": self.foreshadowing,
            "area_type": self.area_type,
            "monsters": [monster.to_dict() for monster in self.monsters],
            "treasures": [treasure.to_dict() for treasure in self.treasures],
            "traps": self.traps,
            "environmental_details": self.environmental_details,
            "connections": self.connections,
            "lighting": self.lighting,
            "atmosphere": self.atmosphere,
            "hazards": self.hazards
        }


class DungeonData(BaseModel):
    """Complete dungeon data with all areas and systems."""
    name: str
    uuid: str
    total_areas: int
    dungeon_type: str = "unknown"  # crypt, temple, hideout, caverns, shrine, tomb
    difficulty_level: int = 1
    areas: list[AreaData] = Field(default_factory=list)
    total_monsters: int = 0
    total_treasure_value: int = 0
    faction_control: str | None = None
    entrance_location: str | None = None
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "uuid": self.uuid,
            "type": self.dungeon_type,
            "total_areas": self.total_areas,
            "difficulty_level": self.difficulty_level,
            "total_monsters": self.total_monsters,
            "total_treasure_value": self.total_treasure_value,
            "faction_control": self.faction_control,
            "entrance_location": self.entrance_location,
            "areas": [area.to_dict() for area in self.areas]
        }


class HexTileData(BaseModel):
    """Individual hex tile with all geographic data."""
    uuid: str
    x: int
    y: int
    biome_type: str
    feature_type: str = "other"
    feature_uuid: str | None = None
    feature_label: str | None = None
    rivers: list[int] = Field(default_factory=list)
    trails: list[int] = Field(default_factory=list)
    roads: list[int] = Field(default_factory=list)
    harbor: int | None = None
    borderline: bool = False
    region_uuid: str | None = None
    realm_uuid: str | None = None
    elevation: str = "normal"  # low, normal, high
    vegetation: str = "normal"  # sparse, normal, dense
    water_access: bool = False
    settlement_access: bool = False
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "uuid": self.uuid,
            "coordinates": {"x": self.x, "y": self.y},
            "biome": self.biome_type,
            "feature": {
                "type": self.feature_type,
                "uuid": self.feature_uuid,
                "label": self.feature_label
            },
            "infrastructure": {
                "rivers": self.rivers,
                "trails": self.trails,
                "roads": self.roads,
                "harbor": self.harbor,
                "borderline": self.borderline
            },
            "geography": {
                "elevation": self.elevation,
                "vegetation": self.vegetation,
                "water_access": self.water_access,
                "settlement_access": self.settlement_access
            },
            "region_uuid": self.region_uuid,
            "realm_uuid": self.realm_uuid
        }


class RegionData(BaseModel):
    """Complete region data with hex tiles and systems."""
    name: str
    uuid: str
    total_hexes: int
    biome_distribution: dict[str, int] = Field(default_factory=dict)
    settlement_count: int = 0
    geographic_features: dict[str, int] = Field(default_factory=dict)
    hex_tiles: list[HexTileData] = Field(default_factory=list)
    weather_systems: WeatherData | None = None
    random_encounters: list[EncounterData] = Field(default_factory=list)
    travel_dangers: list[str] = Field(default_factory=list)
    notable_locations: list[str] = Field(default_factory=list)
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "uuid": self.uuid,
            "statistics": {
                "total_hexes": self.total_hexes,
                "settlement_count": self.settlement_count,
                "biome_distribution": self.biome_distribution,
                "geographic_features": self.geographic_features
            },
            "hex_tiles": [tile.to_dict() for tile in self.hex_tiles],
            "weather": self.weather_systems.to_dict() if self.weather_systems else None,
            "encounters": [enc.to_dict() for enc in self.random_encounters],
            "travel_dangers": self.travel_dangers,
            "notable_locations": self.notable_locations
        }


class FactionData(BaseModel):
    """Complete faction data with political systems."""
    name: str
    uuid: str
    description: str = ""
    faction_type: str = "guild"  # guild, order, cult, military, criminal
    territories: list[str] = Field(default_factory=list)
    strongholds: list[str] = Field(default_factory=list)
    allies: list[str] = Field(default_factory=list)
    enemies: list[str] = Field(default_factory=list)
    leadership: list[NPCData] = Field(default_factory=list)
    members: list[NPCData] = Field(default_factory=list)
    goals: list[str] = Field(default_factory=list)
    methods: list[str] = Field(default_factory=list)
    resources: dict[str, Any] = Field(default_factory=dict)
    reputation: dict[str, str] = Field(default_factory=dict)  # {"region": "hated/neutral/loved"}
    
    def to_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "uuid": self.uuid,
            "type": self.faction_type,
            "description": self.description,
            "territories": self.territories,
            "strongholds": self.strongholds,
            "political_relations": {
                "allies": self.allies,
                "enemies": self.enemies
            },
            "leadership": [leader.to_dict() for leader in self.leadership],
            "members": [member.to_dict() for member in self.members],
            "goals": self.goals,
            "methods": self.methods,
            "resources": self.resources,
            "reputation": self.reputation
        }
