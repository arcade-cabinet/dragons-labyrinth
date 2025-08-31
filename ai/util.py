from __future__ import annotations
import hashlib, json, pathlib
from typing import Any

ROOT = pathlib.Path(__file__).resolve().parents[1]
BUILD = ROOT / "build"
MANIFEST = BUILD / "manifest.json"

def sha256_bytes(b: bytes) -> str:
    return hashlib.sha256(b).hexdigest()

def write_if_changed(path: pathlib.Path, content: bytes) -> bool:
    BUILD.mkdir(parents=True, exist_ok=True)
    state: dict[str, Any] = {"files": {}}
    if MANIFEST.exists():
        try:
            state = json.loads(MANIFEST.read_text())
        except Exception:
            pass
    digest = sha256_bytes(content)
    rel = str(path.relative_to(ROOT))
    unchanged = state.get("files", {}).get(rel) == digest and path.exists()
    if unchanged:
        return False
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(content)
    state.setdefault("files", {})[rel] = digest
    MANIFEST.write_text(json.dumps(state, indent=2))
    return True
