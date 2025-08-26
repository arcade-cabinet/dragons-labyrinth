from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Any, Dict

from jinja2 import Environment, select_autoescape
from importlib.resources import files as pkg_files


@dataclass
class CodeGenContext:
    category: str
    prompt_extra: Dict[str, Any]
    output_dir: Path


class CodeGenerator:
    def __init__(self) -> None:
        self.env = Environment(autoescape=select_autoescape(enabled_extensions=(".j2",)), trim_blocks=True, lstrip_blocks=True)

    def _load_template(self, name: str):
        src = pkg_files("dragons_labyrinth.workflows.asset_generation.templates").joinpath(name).read_text(encoding="utf-8")
        return self.env.from_string(src)

    def render_to(self, template_name: str, context: Dict[str, Any], out_path: Path) -> Path:
        tpl = self._load_template(template_name)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        out_path.write_text(tpl.render(**context), encoding="utf-8")
        return out_path


def generate_biome_rules(extra: Dict[str, Any], out_dir: Path) -> Path:
    """Generate biome rules module from extra code spec.

    Expected structure in `extra`:
    {
      "biome_rules": {
        "adjacency": { "plains": ["plains", "forest", ...], ... },
        "clustering": { "lava": {"max_cluster": 6}, ... },
        "movement": { "swamp": {"walk": 1.5, "ground_mount": 1.2, "flying": 1.0}, ... },
        "hazards": { "lava": {"dps": 8.0}, ... },
        "passability": { "mountain": {"walk": false, "ground_mount": false, "flying": true}, ... }
      }
    }
    """
    env = Environment(autoescape=select_autoescape(enabled_extensions=(".j2",)), trim_blocks=True, lstrip_blocks=True)
    src = pkg_files("dragons_labyrinth.workflows.asset_generation.templates").joinpath("biome_rules.rs.j2").read_text(encoding="utf-8")
    tpl = env.from_string(src)
    rendered = tpl.render(rules=extra.get("biome_rules", {}))
    out_path = out_dir / "systems" / "hex_rendering" / "biome_rules.rs"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(rendered, encoding="utf-8")
    return out_path


