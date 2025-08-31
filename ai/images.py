from __future__ import annotations
from openai import OpenAI
from pydantic import BaseModel
from pathlib import Path
from typing import Any
from PIL import Image
import io, base64, json

from schemas import ImagePlan, BiomeTileset, IconJob
from util import write_if_changed, ROOT

def _save_b64_png(b64: str, path: Path) -> None:
    raw = base64.b64decode(b64)
    write_if_changed(path, raw)

def generate_images(plan: ImagePlan, out_dir: Path, model: str) -> None:
    client = OpenAI()
    out_dir.mkdir(parents=True, exist_ok=True)
    # Tilesets
    for ts in plan.tilesets:
        biome_dir = out_dir / "biomes" / ts.biome
        biome_dir.mkdir(parents=True, exist_ok=True)
        for v in ts.variants:
            resp = client.images.generate(
                model=model,
                prompt=v.prompt,
                size=f"{v.size}x{v.size}",
                background="transparent" if v.transparent else "white"
            )
            b64 = resp.data[0].b64_json
            _save_b64_png(b64, biome_dir / f"{v.name}.png")
    # Icons
    icon_dir = out_dir / "icons"
    icon_dir.mkdir(parents=True, exist_ok=True)
    for job in plan.icons:
        resp = client.images.generate(
            model=model,
            prompt=job.prompt,
            size=f"{job.size}x{job.size}",
            background="transparent" if job.transparent else "white"
        )
        b64 = resp.data[0].b64_json
        _save_b64_png(b64, icon_dir / f"{job.name}.png")
