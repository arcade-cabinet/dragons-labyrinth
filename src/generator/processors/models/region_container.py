```python
# Absolute imports - NO wildcards

from typing import Any, List, Dict, Optional

from pydantic import BaseModel

from generator.processors.models.regions import InnDrinkModel
from generator.processors.models.regions import InnFoodMenuModel
from generator.processors.models.regions import InnBulletinModel
from generator.processors.models.regions import InnRumorModel
from generator.processors.models.regions import InnNPCModel
from generator.processors.models.regions import InnPatronModel
from generator.processors.models.regions import RegionInnModel

from generator.processors.models.settlements import EstablishmentInventoryItem
from generator.processors.models.settlements import EstablishmentQuest
from generator.processors.models.settlements import EstablishmentNPC
from generator.processors.models.settlements import EstablishmentFactionMembership
from generator.processors.models.settlements import SettlementEstablishment

from generator.processors.models.dungeons import MonsterStatBlock
from generator.processors.models.dungeons import TreasureItem
from generator.processors.models.dungeons import Trap
from generator.processors.models.dungeons import DungeonArea

# --- Supporting Models for Integration ---

class HexTileModel(BaseModel):
    hex_id: str
    terrain_type: str
    features: List[str]  # e.g., ["settlement", "dungeon", "river"]
    feature_uuids: List[str]  # UUIDs of features present (settlements, dungeons, etc.)
    resource_tags: List[str]
    adjacent_hex_ids: List[str]
    travel_cost: int
    notes: Optional[str] = None

class FactionModel(BaseModel):
    faction_uuid: str
    name: str
    description: str
    influence_level: int  # 0-100
    controlled_hexes: List[str]  # hex_ids
    member_npc_uuids: List[str]
    alliances: List[str]  # faction_uuids
    conflicts: List[str]  # faction_uuids
    hidden_activities: Optional[List[str]] = None

class DungeonContainer(BaseModel):
    dungeon_uuid: str
    name: str
    entrance_hex_id: str
    areas: List[DungeonArea]
    monsters: List[MonsterStatBlock]
    treasures: List[TreasureItem]
    traps: List[Trap]
    notes: Optional[str] = None

class WeatherSystemModel(BaseModel):
    current_weather: str
    forecast: List[str]
    seasonal_patterns: Dict[str, Any]

class EncounterModel(BaseModel):
    encounter_id: str
    hex_id: str
    description: str
    involved_npcs: List[str]
    involved_monsters: List[str]
    trigger_conditions: Dict[str, Any]

class TradeRouteModel(BaseModel):
    route_id: str
    name: str
    hex_path: List[str]
    connected_settlements: List[str]
    trade_goods: List[str]
    danger_level: int

class RegionalEconomyModel(BaseModel):
    resource_availability: Dict[str, int]  # resource_name -> quantity/abundance
    scarcity: Dict[str, int]  # resource_name -> scarcity index
    trade_routes: List[TradeRouteModel]
    aggregated_services: Dict[str, Any]  # e.g., {"inns": [...], "markets": [...]}
    pricing_modifiers: Dict[str, float]  # e.g., {"food": 1.2, "iron": 0.8}

class NavigationModel(BaseModel):
    adjacent_region_ids: List[str]
    internal_routes: Dict[str, List[str]]  # hex_id -> list of connected hex_ids
    travel_services: Dict[str, Any]  # e.g., {"ferries": [...], "caravans": [...]}

class RegionalQuestModel(BaseModel):
    quest_id: str
    title: str
    description: str
    originating_settlement_uuid: Optional[str]
    related_npc_uuids: List[str]
    related_faction_uuids: List[str]
    target_location_uuid: Optional[str]
    rewards: Dict[str, Any]
    hooks: List[str]

# --- Master RegionContainer ---

class RegionContainer(BaseModel):
    # Core identity
    region_uuid: str
    name: str
    description: str

    # Spatial Integration
    hex_tiles: List[HexTileModel]  # All hexes with full data

    # Settlements and Establishments
    settlements: List[Dict[str, Any]]  # Each: {
                                        #   "settlement_uuid": str,
                                        #   "name": str,
                                        #   "districts": List[str],
                                        #   "establishments": List[SettlementEstablishment],
                                        #   "npcs": List[EstablishmentNPC],
                                        #   "faction_memberships": List[EstablishmentFactionMembership],
                                        #   "quests": List[EstablishmentQuest],
                                        #   "inventory": List[EstablishmentInventoryItem]
                                        # }
    inns: List[RegionInnModel]  # All inns in the region
    inn_menus: List[InnFoodMenuModel]
    inn_drinks: List[InnDrinkModel]
    inn_bulletins: List[InnBulletinModel]
    inn_rumors: List[InnRumorModel]
    inn_npcs: List[InnNPCModel]
    inn_patrons: List[InnPatronModel]

    # Dungeons
    dungeons: List[DungeonContainer]  # Each dungeon with full areas, monsters, treasures, traps

    # Factions and Politics
    factions: List[FactionModel]  # All factions with control/influence data

    # Economic Integration
    regional_economy: RegionalEconomyModel

    # Weather and Environmental Systems
    weather: WeatherSystemModel

    # Encounters and Travel
    encounters: List[EncounterModel]
    navigation: NavigationModel

    # Quests, Rumors, and Adventure Hooks
    regional_quests: List[RegionalQuestModel]
    rumors: List[InnRumorModel]  # All rumors, not just inn-specific

    # Adjacency and Routing
    adjacent_regions: List[str]  # region_uuids
    internal_routes: Dict[str, List[str]]  # hex_id -> connected hex_ids

    # Utility/Meta
    meta: Dict[str, Any] = {}

    class Config:
        arbitrary_types_allowed = True
        schema_extra = {
            "description": "Master container for all region data, integrating spatial, economic, political, and gameplay systems."
        }
```
This `RegionContainer` model provides a comprehensive, fully-integrated structure for regional gameplay, aggregating all spatial, economic, political, and adventure data using the provided models and absolute imports. Each section is extensible and references the appropriate Phase 1 models, with clear integration points for navigation, quests, factions, and more.