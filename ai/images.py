from __future__ import annotations
from openai import OpenAI
from pydantic import BaseModel
from pathlib import Path
from typing import Any
from PIL import Image
import io, base64, json

from ai.schemas import ImagePlan, BiomeTileset, IconJob
from ai.util import write_if_changed, ROOT

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
            out_path = biome_dir / f"{v.name}.png"
            if out_path.exists():
                continue
            size = f"{v.size}x{v.size}"
            # gpt-image-1 supports only certain sizes; coerce unsupported ones to 1024
            if size not in {"1024x1024", "1024x1536", "1536x1024", "auto"}:
                size = "1024x1024"
            bg = "transparent" if v.transparent else "opaque"
            resp = client.images.generate(
                model=model,
                prompt=v.prompt,
                size=size,
                background=bg
            )
            b64 = resp.data[0].b64_json
            _save_b64_png(b64, out_path)
    # Icons
    icon_dir = out_dir / "icons"
    icon_dir.mkdir(parents=True, exist_ok=True)
    for job in plan.icons:
        out_path = icon_dir / f"{job.name}.png"
        if out_path.exists():
            continue
        size = f"{job.size}x{job.size}"
        if size not in {"1024x1024", "1024x1536", "1536x1024", "auto"}:
            size = "1024x1024"
        bg = "transparent" if job.transparent else "opaque"
        resp = client.images.generate(
            model=model,
            prompt=job.prompt,
            size=size,
            background=bg
        )
        b64 = resp.data[0].b64_json
        _save_b64_png(b64, out_path)
