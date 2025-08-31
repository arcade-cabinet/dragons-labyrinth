from __future__ import annotations
from openai import OpenAI
from schemas import DialoguePack, QuestlinePack, WorldBook
from util import write_if_changed
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
            res = client.responses.create(
                model=model,
                input=prompt,
                temperature=temperature,
                response_format={"type":"json_schema","json_schema":{
                    "name":"DialoguePack","schema":DialoguePack.model_json_schema()
                }},
            )
            dp = DialoguePack.model_validate_json(res.output_text)
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
            res = client.responses.create(
                model=model,
                input=prompt,
                temperature=temperature,
                response_format={"type":"json_schema","json_schema":{
                    "name":"QuestlinePack","schema":QuestlinePack.model_json_schema()
                }},
            )
            pack = QuestlinePack.model_validate_json(res.output_text)
            qmap[q.id] = pack
    return qmap

def write_dialogue_bundle(dialogues: dict[str, DialoguePack], questlines: dict[str, QuestlinePack], out_dir: Path) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    for npc_id, pack in dialogues.items():
        write_if_changed(out_dir / f"dialogue_{npc_id}.json", pack.model_dump_json(indent=2).encode())
    for qid, pack in questlines.items():
        write_if_changed(out_dir / f"questline_{qid}.json", pack.model_dump_json(indent=2).encode())
