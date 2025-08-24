"""LangChain tools for asset database functionality."""

from __future__ import annotations

from langchain.tools import StructuredTool
from pydantic import BaseModel, Field

from professor_pixel.database.crud import search_assets_semantic, rebuild_asset_index


class AssetSearchArgs(BaseModel):
    """Arguments for natural language asset search."""
    query: str = Field(description="Natural language description of desired assets (e.g., 'pixel art spaceship sprites', 'jump sound effects')")
    limit: int = Field(default=8, description="Maximum number of assets to return")
    category: str | None = Field(default=None, description="Optional category filter (sprites, audio, tiles, etc.)")


# Create the main StructuredTool for AI agents
asset_search_tool = StructuredTool.from_function(
    search_assets_semantic,
    name="search_assets",
    description=(
        "Search the asset library using text queries. "
        "Use this to find relevant sprites, sounds, tiles, or other game assets "
        "for lesson generation. Returns asset paths and metadata."
    ),
    args_schema=AssetSearchArgs,
)


# Export for use in AI agents and CLI
__all__ = ["asset_search_tool", "AssetSearchArgs", "rebuild_asset_index"]
