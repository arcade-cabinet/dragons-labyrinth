from __future__ import annotations
from openai import OpenAI
from ai.schemas import DialoguePack, QuestlinePack, WorldBook
from ai.util import write_if_changed, call_json_schema
from pathlib import Path
import json, os

def expand_npc_dialogue(wb: WorldBook, model: str, temperature: float = 0.8) -> dict[str, DialoguePack]:
    client = OpenAI()
    results: dict[str, DialoguePack] = {}
    world_json = wb.model_dump_json(indent=2)
    for region in wb.regions:
        for npc in region.npcs:
            prompt = f"""Context (WorldBook excerpt):
{world_json}

Write DialoguePack JSON for npc_id={npc.id} with short lines that fit the band's tone.
"""
            text = call_json_schema(client, model, prompt, DialoguePack, temperature)
            dp = DialoguePack.model_validate_json(text)
            results[npc.id] = dp
    return results

def expand_questlines(wb: WorldBook, model: str, temperature: float = 0.7) -> dict[str, QuestlinePack]:
    client = OpenAI()
    qmap: dict[str, QuestlinePack] = {}
    for region in wb.regions:
        for q in region.quests:
            prompt = f"""Write QuestlinePack JSON for quest_id={q.id}.
Include three levels: macro (arc), meso (chapters), micro (beats per step).
Keep consistent with quest summary and steps.
Quest:
{json.dumps(q.model_dump(), indent=2)}
"""
            text = call_json_schema(client, model, prompt, QuestlinePack, temperature)
            pack = QuestlinePack.model_validate_json(text)
            qmap[q.id] = pack
    return qmap

def write_dialogue_bundle(dialogues: dict[str, DialoguePack], questlines: dict[str, QuestlinePack], out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    for npc_id, pack in dialogues.items():
        write_if_changed(out_dir / f"dialogue_{npc_id}.json", pack.model_dump_json(indent=2).encode())
    for qid, pack in questlines.items():
        write_if_changed(out_dir / f"questline_{qid}.json", pack.model_dump_json(indent=2).encode())
