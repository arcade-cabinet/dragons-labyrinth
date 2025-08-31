from __future__ import annotations
from pydantic import BaseModel, Field
from typing import Literal

# ---- Canon & Themes ----
class ProgressionBand(BaseModel):
    band: Literal["1-20","21-40","41-60","61-120","121-180"]
    statement: str
    goals: list[str]
    gating_mechanics: list[str]

class GameCanon(BaseModel):
    title: str
    pillars: list[str]
    world_rules: list[str]
    starting_hex: str
    progression: list[ProgressionBand]

class ThemeBand(BaseModel):
    band: str
    tone: str
    biome_palette: list[str]
    enemy_archetypes: list[str]
    quest_archetypes: list[str]
    art_prompts: list[str]

class ThemeBible(BaseModel):
    style_directives: list[str]
    content_constraints: list[str]
    bands: list[ThemeBand]

# ---- World ----
class HexPOI(BaseModel):
    axial: str
    kind: Literal["village","dungeon","shrine","lair","ruin","camp","forge","portal"]
    blurb: str

class Quest(BaseModel):
    id: str
    title: str
    summary: str
    type: Literal["main","side","companion","forge","moral"]
    steps: list[str]
    success_outcome: str
    failure_outcome: str

class NPC(BaseModel):
    id: str
    name: str
    role: str
    personality: str
    secrets: list[str]
    hooks: list[str]

class Creature(BaseModel):
    id: str
    name: str
    tags: list[str]
    behavior: str
    cr_hint: str

class RegionBand(BaseModel):
    band: Literal["1-20","21-40","41-60","61-120","121-180"]
    name: str
    theme_summary: str
    biome_palette: list[str]
    tone: str

class WorldPlan(BaseModel):
    title: str
    starting_hex: str
    region_bands: list[RegionBand]
    global_pillars: list[str]

class RegionBible(BaseModel):
    band: str
    name: str
    mood_board: list[str]
    biomes: list[str]
    hex_points: list[HexPOI]
    quests: list[Quest]
    npcs: list[NPC]
    creatures: list[Creature]

class WorldBook(BaseModel):
    plan: WorldPlan
    regions: list[RegionBible]

# ---- Dialogue & Questlines ----
class DialogueLine(BaseModel):
    speaker: str
    line: str
    intent: str
    stage: str = ""

class DialoguePack(BaseModel):
    npc_id: str
    context: str
    lines: list[DialogueLine]

class QuestlineLevel(BaseModel):
    level: Literal["macro","meso","micro"]
    beats: list[str]

class QuestlinePack(BaseModel):
    quest_id: str
    levels: list[QuestlineLevel]

# ---- Image specs ----
class TileVariant(BaseModel):
    name: str
    prompt: str
    size: int = 512
    tileable: bool = True
    transparent: bool = False

class BiomeTileset(BaseModel):
    biome: str
    variants: list[TileVariant]

class IconJob(BaseModel):
    name: str
    prompt: str
    size: int = 128
    transparent: bool = True

class ImagePlan(BaseModel):
    tilesets: list[BiomeTileset]
    icons: list[IconJob]
