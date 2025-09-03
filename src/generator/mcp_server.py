from __future__ import annotations

"""
MCP server entrypoint for Dragons' Labyrinth.

This uses FastMCP 2.0 and exposes a small, stable set of tools that call into
our generator packages using **absolute imports only**.

Console script entry point: `dl_mcp` -> `generator.mcp_server:main`.
"""

from typing import Any

from fastmcp import FastMCP

# --- Absolute imports into our repo ---
# Entities toolchain (kept narrowly-scoped to avoid heavy imports at startup)
from generator.entities.image_generator import (
    generate_image,
    generate_images,
    IMAGE_CATEGORIES,
)
from generator.entities.godot_generator import (
    generate_godot_entity,
    generate_godot_entities,
)
from generator.entities.html_entities import (
    generate_html_entity,
    generate_html_entities,
)
from generator.entities.json_entities import (
    generate_json_entity,
    generate_json_entities,
)
from generator.entities.extractors import (
    extract_entities_from_hbf,
    extract_entities_from_json,
)
from generator.entities.types import Entity, EntityCategory


# ----------------------------------------------------------------------------
# FastMCP server definition
# ----------------------------------------------------------------------------

mcp = FastMCP("dragons-labyrinth")


@mcp.tool
def health_check() -> str:
    """Simple readiness probe."""
    return "ok"


@mcp.tool
def list_image_categories() -> list[str]:
    """Return the available IMAGE_CATEGORIES for the image generator."""
    return list(IMAGE_CATEGORIES)


@mcp.tool
def entities_from_json(payload: str) -> list[dict[str, Any]]:
    """Extract entities from a JSON payload (stringified JSON)."""
    entities: list[Entity] = extract_entities_from_json(payload)
    # Convert to plain dicts for transport over MCP
    return [e.model_dump() if hasattr(e, "model_dump") else dict(e) for e in entities]


@mcp.tool
def entities_from_hbf(markup: str) -> list[dict[str, Any]]:
    """Extract entities from HBF/HTML-like markup."""
    entities: list[Entity] = extract_entities_from_hbf(markup)
    return [e.model_dump() if hasattr(e, "model_dump") else dict(e) for e in entities]


@mcp.tool
def render_godot_entities(entities: list[dict[str, Any]]) -> str:
    """Render a list of entity dicts into Godot scene/text (returns a string)."""
    return generate_godot_entities(entities)


@mcp.tool
def render_godot_entity(entity: dict[str, Any]) -> str:
    """Render a single entity dict into a Godot node/scene string."""
    return generate_godot_entity(entity)


@mcp.tool
def render_html_entities(entities: list[dict[str, Any]]) -> str:
    """Render entities as an HTML snippet."""
    return generate_html_entities(entities)


@mcp.tool
def render_html_entity(entity: dict[str, Any]) -> str:
    """Render a single entity as an HTML snippet."""
    return generate_html_entity(entity)


@mcp.tool
def render_json_entities(entities: list[dict[str, Any]]) -> str:
    """Render entities as a normalized JSON string."""
    return generate_json_entities(entities)


@mcp.tool
def render_json_entity(entity: dict[str, Any]) -> str:
    """Render a single entity as a normalized JSON string."""
    return generate_json_entity(entity)


@mcp.tool
def make_image(prompt: str, category: str | None = None) -> str:
    """Generate a single image and return a path or identifier.

    `category` should be one of `list_image_categories()`; if omitted we let the
    underlying generator choose defaults.
    """
    return generate_image(prompt=prompt, category=category)


@mcp.tool
def make_images(prompts: list[str], category: str | None = None) -> list[str]:
    """Generate multiple images; returns a list of paths/identifiers."""
    return generate_images(prompts=prompts, category=category)


# ----------------------------------------------------------------------------
# Entrypoint
# ----------------------------------------------------------------------------

def main() -> None:
    """Run the FastMCP server."""
    mcp.run()


if __name__ == "__main__":
    main()