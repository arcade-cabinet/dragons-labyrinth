from __future__ import annotations
import hashlib, json, pathlib
from typing import Any, Type
from pydantic import BaseModel
from openai import OpenAI

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

def call_json_schema(client: OpenAI, model: str, prompt: str, schema_model: Type[BaseModel], temperature: float | None = None) -> str:
    payload: dict[str, Any] = {"model": model, "input": prompt}
    if temperature is not None:
        payload["temperature"] = temperature
    # Use structured outputs via response_format in Responses API if available; fallback to plain text
    try:
        res = client.responses.create(
            response_format={
                "type": "json_schema",
                "json_schema": {"name": schema_model.__name__, "schema": schema_model.model_json_schema()},
            },
            **payload,
        )
        return extract_json_text(res.output_text)
    except TypeError:
        # Older SDKs may not support response_format; try Chat Completions with function tools
        try:
            tool = {
                "type": "function",
                "function": {
                    "name": schema_model.__name__,
                    "parameters": schema_model.model_json_schema(),
                },
            }
            kwargs: dict[str, Any] = {}
            if temperature is not None:
                kwargs["temperature"] = temperature
            chat = client.chat.completions.create(
                model=model,
                messages=[{"role": "user", "content": prompt}],
                tools=[tool],
                tool_choice={"type": "function", "function": {"name": schema_model.__name__}},
                **kwargs,
            )
            message = chat.choices[0].message
            if message.tool_calls and len(message.tool_calls) > 0:
                args = message.tool_calls[0].function.arguments
                return extract_json_text(args)
            # Fallback to raw content
            return extract_json_text(message.content or "")
        except Exception:
            # Last resort: ask model to return JSON only and trust schema_model to validate
            fallback_prompt = f"{prompt}\n\nReturn strictly valid JSON for schema {schema_model.__name__}."
            res = client.responses.create(model=model, input=fallback_prompt)
            return extract_json_text(res.output_text)

def extract_json_text(text: str) -> str:
    s = text.strip()
    # Remove surrounding code fences
    if s.startswith("```"):
        # strip opening fence line
        s = s.split("\n", 1)[1] if "\n" in s else s
        # remove optional language tag already removed by split
        if s.startswith("json\n"):
            s = s[5:]
        # remove trailing fence if present
        if s.endswith("```"):
            s = s.rsplit("```", 1)[0]
    # If still contains prose, extract first balanced JSON object/array
    start_idx_obj = s.find("{")
    start_idx_arr = s.find("[")
    start_idx = -1
    if start_idx_obj != -1 and start_idx_arr != -1:
        start_idx = min(start_idx_obj, start_idx_arr)
    else:
        start_idx = max(start_idx_obj, start_idx_arr)
    if start_idx > 0:
        s = s[start_idx:]
    # Try to find end
    end_idx_obj = s.rfind("}")
    end_idx_arr = s.rfind("]")
    end_idx = max(end_idx_obj, end_idx_arr)
    if end_idx != -1:
        s = s[: end_idx + 1]
    return s.strip()

def _normalize_bands(obj: Any) -> Any:
    if isinstance(obj, dict):
        new = {}
        for k, v in obj.items():
            if k == "band" and isinstance(v, str):
                vv = v.replace("–", "-").replace("—", "-")
                new[k] = vv
            else:
                new[k] = _normalize_bands(v)
        return new
    if isinstance(obj, list):
        return [_normalize_bands(x) for x in obj]
    return obj

def normalize_json_for_model(schema_model: Type[BaseModel], text: str) -> dict[str, Any]:
    data = json.loads(text)
    data = _normalize_bands(data)
    # Trim whitespace from literal enums like kinds
    def _trim(obj: Any) -> Any:
        if isinstance(obj, dict):
            return {k: _trim(v) for k, v in obj.items()}
        if isinstance(obj, list):
            return [_trim(x) for x in obj]
        if isinstance(obj, str):
            return obj.strip()
        return obj
    data = _trim(data)
    return data
