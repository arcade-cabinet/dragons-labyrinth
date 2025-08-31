from __future__ import annotations
from pathlib import Path
from PIL import Image
import json, io

from util import write_if_changed, ROOT

def pack_uniform_grid(src_dir: Path, out_png: Path, out_json: Path, tile_size: int = 512, cols: int = 8) -> None:
    tiles: list[tuple[str, Path]] = []
    for p in (src_dir / "biomes").rglob("*.png"):
        biome = p.parent.name
        tiles.append((f"{biome}/{p.stem}", p))
    if not tiles:
        return

    rows = (len(tiles) + cols - 1) // cols
    atlas_w, atlas_h = cols * tile_size, rows * tile_size
    atlas = Image.new("RGBA", (atlas_w, atlas_h), (0,0,0,0))

    index = {}
    for i, (name, path) in enumerate(sorted(tiles)):
        img = Image.open(path).convert("RGBA")
        img = img.resize((tile_size, tile_size))
        x = (i % cols) * tile_size
        y = (i // cols) * tile_size
        atlas.paste(img, (x, y))
        index[name] = {"u": x, "v": y, "w": tile_size, "h": tile_size}

    buf = io.BytesIO()
    atlas.save(buf, format="PNG")
    write_if_changed(out_png, buf.getvalue())
    payload = {"tiles": index, "tile_size": tile_size, "atlas_w": atlas_w, "atlas_h": atlas_h, "cols": cols, "rows": rows}
    write_if_changed(out_json, json.dumps(payload, indent=2).encode())

def main() -> None:
    src_dir = ROOT / "apps" / "game" / "assets"
    out_dir = ROOT / "build" / "atlas"
    out_dir.mkdir(parents=True, exist_ok=True)
    pack_uniform_grid(src_dir, out_dir / "atlas.png", out_dir / "atlas.json")

if __name__ == "__main__":
    main()
