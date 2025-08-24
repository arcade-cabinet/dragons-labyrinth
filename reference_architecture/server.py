"""MCP server for Professor Pixel's asset library management."""

from __future__ import annotations

from fastmcp import FastMCP

from professor_pixel.database import (
    asset_db, 
    pattern_db,
    rebuild_asset_index,
    search_assets_semantic,
    Pattern,
    PatternParameter,
    PatternAssetCompatibility
)
from professor_pixel.database.engine import database_session
from professor_pixel.models import AssetSearchRequest, AssetSearchFilter  
from professor_pixel.settings import get_settings
from professor_pixel.types import GameLibrary, PatternCategory, AssetFileType
from professor_pixel.schemas.core_rules import parse_core_asset, get_supported_core_categories, demo_core_parsing

# Create the MCP server
mcp = FastMCP("Professor Pixel Development Tools 🎮🛠️")


@mcp.tool
def search_assets(query: str, limit: int = 8, category: str | None = None) -> list[dict[str, str | None]]:
    """
    Search the asset library using simple text matching.
    
    Args:
        query: Text search query (searches filenames, categories, etc.)
        limit: Maximum number of assets to return (default: 8)
        category: Optional category filter (aircraft, audio, tiles, etc.)
    
    Returns:
        List of matching assets with metadata including path, category, filename, etc.
    """
    # Use the centralized search function
    return search_assets_semantic(query=query, limit=limit, category=category)


@mcp.tool
def rebuild_index() -> dict[str, str | int]:
    """
    Rebuild the asset database from filesystem.
    
    This scans all assets in the library directory and rebuilds the SQLite database.
    Use this when new assets are added or the database becomes corrupted.
    
    Returns:
        Dictionary with rebuild status and asset count
    """
    # Use the centralized rebuild function
    return rebuild_asset_index()


@mcp.tool
def check_index_status() -> dict[str, str | bool | int]:
    """
    Check the status of the asset database.
    
    Returns:
        Dictionary with database status and statistics
    """
    try:
        stats = asset_db.get_stats()
        return {
            "status": "ok", 
            "database_exists": True,
            "total_assets": stats.total_assets,
            "categories": len(stats.categories),
            "message": f"Database ready with {stats.total_assets} assets"
        }
    except Exception as e:
        return {
            "status": "error",
            "database_exists": False, 
            "total_assets": 0,
            "message": str(e)
        }


@mcp.resource("asset://library/stats")
def get_library_stats() -> str:
    """Get statistics about the asset library."""
    try:
        stats = asset_db.get_stats()
        
        output = f"Asset Library Statistics:\n"
        output += f"Total assets: {stats.total_assets}\n"
        output += f"Categories: {len(stats.categories)}\n"
        output += f"Total size: {stats.total_size_mb:.1f} MB\n"
        
        if stats.last_indexed:
            output += f"Last indexed: {stats.last_indexed}\n"
        
        output += f"\nTop categories:\n"
        for category, count in stats.get_top_categories():
            output += f"- {category}: {count} assets\n"
        
        return output
    except Exception as e:
        return f"Error getting stats: {e}"


@mcp.tool
def search_patterns(
    skill_level: str = "beginner", 
    category: str | None = None,
    library: str = "arcade",
    limit: int = 20
) -> list[dict[str, str | int | bool]]:
    """
    Search for programming patterns by skill level and category.
    
    Args:
        skill_level: "beginner", "intermediate", "advanced", or "expert"
        category: Optional category filter ("visual", "sprites", "audio", "collision", "input", "motion", "game")
        library: Library name ("arcade", "pygame_ce", "pyglet", "pysdl2")
        limit: Maximum number of patterns to return
    
    Returns:
        List of patterns with metadata including complexity, parameters, and asset compatibility
    """
    try:
        library_enum = GameLibrary[library.upper()]
    except KeyError:
        return [{"error": f"Unknown library: {library}"}]
    
    # Define skill level complexity limits
    complexity_limits = {
        "beginner": 2,
        "intermediate": 3, 
        "advanced": 4,
        "expert": 5
    }
    
    max_complexity = complexity_limits.get(skill_level, 2)
    
    with database_session() as session:
        query = session.query(Pattern).filter(
            Pattern.library == library_enum,
            Pattern.complexity <= max_complexity
        )
        
        if category:
            try:
                category_enum = PatternCategory[category.upper()]
                query = query.filter(Pattern.category == category_enum)
            except KeyError:
                pass  # Invalid category, ignore filter
        
        patterns = query.limit(limit).all()
        
        # Convert to MCP format
        pattern_list = []
        for pattern in patterns:
            pattern_dict = {
                "opcode": pattern.opcode,
                "source_function": pattern.source_function,
                "category": pattern.category.name.lower(),
                "complexity": pattern.complexity,
                "is_beginner_safe": pattern.is_beginner_safe,
                "signature": pattern.signature,
                "docstring": pattern.docstring[:200] + "..." if len(pattern.docstring) > 200 else pattern.docstring,
            }
            pattern_list.append(pattern_dict)
        
        return pattern_list


@mcp.tool  
def get_pattern_details(opcode: str) -> dict[str, str | list | bool]:
    """
    Get complete details about a specific pattern including parameters and asset compatibility.
    
    Args:
        opcode: Pattern opcode (e.g., "DRAW_CIRCLE", "LOAD_SOUND", "CREATE_SPRITE")
    
    Returns:
        Dictionary with pattern details, parameters, and compatible asset types
    """
    with database_session() as session:
        pattern = session.query(Pattern).filter(Pattern.opcode == opcode).first()
        
        if not pattern:
            return {"error": f"Pattern not found: {opcode}"}
        
        # Get parameters
        parameters = []
        for param in pattern.parameters:
            param_dict = {
                "name": param.name,
                "type": param.param_type,
                "required": param.is_required,
                "default": param.default_value,
                "description": param.description,
                "suggested_asset_type": param.suggested_asset_type,
            }
            parameters.append(param_dict)
        
        # Get asset compatibility
        compatible_assets = []
        for compat in pattern.asset_compatibility:
            compat_dict = {
                "asset_type": compat.asset_file_type.name.lower(),
                "parameter": compat.parameter_name,
                "required": compat.is_required,
            }
            compatible_assets.append(compat_dict)
        
        return {
            "opcode": pattern.opcode,
            "source_function": pattern.source_function,
            "library": pattern.library.name.lower(),
            "category": pattern.category.name.lower(),
            "complexity": pattern.complexity,
            "is_beginner_safe": pattern.is_beginner_safe,
            "signature": pattern.signature,
            "docstring": pattern.docstring,
            "template": pattern.template_content,
            "parameters": parameters,
            "compatible_assets": compatible_assets,
        }


@mcp.tool
def get_compatible_assets(opcode: str, parameter_name: str | None = None) -> list[dict[str, str | None]]:
    """
    Find assets compatible with a specific pattern's parameters.
    
    Args:
        opcode: Pattern opcode (e.g., "DRAW_SPRITE", "LOAD_SOUND")
        parameter_name: Optional specific parameter name to check
    
    Returns:
        List of compatible assets from the asset library
    """
    with database_session() as session:
        pattern = session.query(Pattern).filter(Pattern.opcode == opcode).first()
        
        if not pattern:
            return [{"error": f"Pattern not found: {opcode}"}]
        
        # Get compatible asset types for this pattern
        compatible_types = []
        for compat in pattern.asset_compatibility:
            if parameter_name is None or compat.parameter_name == parameter_name:
                compatible_types.append(compat.asset_file_type)
        
        if not compatible_types:
            return [{"message": f"No asset compatibility defined for {opcode}"}]
        
        # Search for assets of compatible types
        compatible_assets = []
        for asset_type in compatible_types:
            filter_obj = AssetSearchFilter(file_type=asset_type)
            request = AssetSearchRequest(filter=filter_obj, limit=10)
            result = asset_db.search_assets(request)
            
            for asset in result.assets:
                asset_dict = {
                    "path": asset.path,
                    "category": asset.category,
                    "filename": asset.filename,
                    "file_type": asset.file_type.name if asset.file_type else None,
                    "compatible_parameter": parameter_name,
                }
                compatible_assets.append(asset_dict)
        
        return compatible_assets


@mcp.tool
def suggest_patterns_for_asset(asset_path: str) -> list[dict[str, str | int]]:
    """
    Suggest patterns that can use a specific asset.
    
    Args:
        asset_path: Relative path to asset (e.g., "sprites/hero.png")
    
    Returns:
        List of patterns that can use this asset type
    """
    # Find the asset in database
    filter_obj = AssetSearchFilter()
    request = AssetSearchRequest(query=asset_path, limit=1, filter=filter_obj)
    result = asset_db.search_assets(request)
    
    if not result.assets:
        return [{"error": f"Asset not found: {asset_path}"}]
    
    asset = result.assets[0]
    asset_file_type = asset.file_type
    
    # Find patterns compatible with this asset type
    with database_session() as session:
        compatible_patterns = session.query(Pattern).join(PatternAssetCompatibility).filter(
            PatternAssetCompatibility.asset_file_type == asset_file_type
        ).all()
        
        suggestions = []
        for pattern in compatible_patterns:
            suggestion = {
                "opcode": pattern.opcode,
                "source_function": pattern.source_function,
                "category": pattern.category.name.lower(),
                "complexity": pattern.complexity,
                "suggested_for": asset_file_type.name.lower(),
                "description": pattern.docstring[:100] + "..." if len(pattern.docstring) > 100 else pattern.docstring,
            }
            suggestions.append(suggestion)
        
        return suggestions


@mcp.resource("patterns://library/stats")
def get_pattern_stats() -> str:
    """Get statistics about available patterns."""
    with database_session() as session:
        total_patterns = session.query(Pattern).count()
        
        # Count by library
        library_counts = {}
        for library in GameLibrary:
            count = session.query(Pattern).filter(Pattern.library == library).count()
            if count > 0:
                library_counts[library.name.lower()] = count
        
        # Count by category  
        category_counts = {}
        for category in PatternCategory:
            count = session.query(Pattern).filter(Pattern.category == category).count()
            if count > 0:
                category_counts[category.name.lower()] = count
        
        # Count by skill level
        skill_counts = {
            "beginner": session.query(Pattern).filter(Pattern.complexity <= 2).count(),
            "intermediate": session.query(Pattern).filter(Pattern.complexity <= 3).count(),
            "advanced": session.query(Pattern).filter(Pattern.complexity <= 4).count(),
            "expert": session.query(Pattern).filter(Pattern.complexity <= 5).count(),
        }
        
        stats = f"Pattern Library Statistics:\n"
        stats += f"Total patterns: {total_patterns}\n\n"
        
        stats += f"By Library:\n"
        for lib, count in library_counts.items():
            stats += f"- {lib}: {count} patterns\n"
        
        stats += f"\nBy Category:\n"
        for cat, count in category_counts.items():
            stats += f"- {cat}: {count} patterns\n"
        
        stats += f"\nBy Skill Level:\n"
        for skill, count in skill_counts.items():
            stats += f"- {skill}: {count} patterns\n"
        
        return stats


@mcp.resource("asset://config/paths")
def get_asset_paths() -> str:
    """Get asset-related path configuration."""
    settings = get_settings()
    
    paths_info = "Asset Path Configuration:\n"
    paths_info += f"Library Directory: {settings.paths.library_dir}\n"
    paths_info += f"Core Assets Directory: {settings.paths.library_dir.parent / 'core'}\n"
    paths_info += f"Main Database: {settings.paths.main_database_path}\n"
    paths_info += f"Database Directory: {settings.paths.database_dir}\n"
    paths_info += f"Data Directory: {settings.paths.data_dir}\n"
    
    return paths_info


# ============================================================================
# CORE ASSET TOOLS (Two-Tier System)
# ============================================================================

@mcp.tool
def parse_core_asset_filename(category: str, filename: str) -> dict[str, str | bool | int]:
    """
    Test core asset filename parsing using intelligent extraction rules.
    
    Args:
        category: Core asset category (backgrounds, image_maps, professors, resources, typography)
        filename: Asset filename to parse (with descriptive metadata)
    
    Returns:
        Parsed metadata including coordinates, eras, dimensions, interactive areas, etc.
    """
    try:
        metadata = parse_core_asset(category, filename)
        
        # Convert to MCP-friendly format
        result = {
            "success": True,
            "category": category,
            "filename": filename,
            "display_name": metadata.get("display_name", ""),
            "description": metadata.get("description", ""),
            "parsed": metadata.get("parsed", False)
        }
        
        # Add category-specific parsed data
        if category == "image_maps" and "interactive_areas" in metadata:
            result["interactive_areas_count"] = len(metadata["interactive_areas"])
            result["dimensions"] = str(metadata.get("dimensions", ""))
            # Show first interactive area as example
            if metadata["interactive_areas"]:
                first_area = metadata["interactive_areas"][0]
                bounds = first_area.get("bounds", {})
                if bounds:
                    top_left = bounds.get("top_left", {})
                    bottom_right = bounds.get("bottom_right", {})
                    result["example_area"] = f"{first_area.get('name', 'Unknown')}: ({top_left.get('x', 0)},{top_left.get('y', 0)}) to ({bottom_right.get('x', 0)},{bottom_right.get('y', 0)})"
        
        elif category == "backgrounds":
            result["era"] = metadata.get("era", "")
            result["style"] = metadata.get("style", "")
            result["layout"] = metadata.get("layout", "")
            result["dimensions"] = str(metadata.get("dimensions", ""))
        
        elif category == "professors":
            result["asset_subtype"] = metadata.get("asset_subtype", "")
            result["era"] = metadata.get("era", "")
            result["style"] = metadata.get("style", "")
        
        elif category == "resources":
            result["resource_type"] = metadata.get("resource_type", "")
            result["usage"] = metadata.get("usage", "")
        
        elif category == "typography":
            result["font_format"] = metadata.get("font_format", "")
            result["font_category"] = metadata.get("font_category", "")
        
        return result
        
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "category": category,
            "filename": filename
        }


@mcp.tool  
def validate_core_structure() -> dict[str, str | bool | list]:
    """
    Validate the core assets directory structure and parsing capabilities.
    
    Returns:
        Validation results including found categories, asset counts, and parsing status.
    """
    from professor_pixel.schemas.core_rules import validate_core_asset_structure
    
    settings = get_settings()
    core_path = settings.paths.library_dir.parent / "core"
    
    try:
        validation = validate_core_asset_structure(core_path)
        
        return {
            "success": validation["valid"],
            "core_path": str(core_path),
            "categories_found": validation["categories_found"],
            "assets_per_category": validation["assets_per_category"],
            "errors": validation["errors"],
            "warnings": validation["warnings"],
            "total_categories": len(validation["categories_found"]),
            "total_assets": sum(validation["assets_per_category"].values())
        }
        
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "core_path": str(core_path)
        }


@mcp.tool
def test_core_workflow(workflow_type: str = "main_menu") -> dict[str, str | bool]:
    """
    Test core workflow generation and pattern-based specifications.
    
    Args:
        workflow_type: Type of core workflow to test (main_menu, title_screen, entrypoint)
    
    Returns:
        Workflow execution results and generated specification validation.
    """
    try:
        if workflow_type == "main_menu":
            from professor_pixel.schemas.ai.core_workflows.main_menu_workflow import MainMenuWorkflow
            
            # Create workflow instance
            workflow_instance = MainMenuWorkflow()
            
            # Test workflow creation
            workflow_graph = workflow_instance.build_workflow()
            nodes = list(workflow_graph.nodes.keys())
            
            result = {
                "success": True,
                "workflow_type": workflow_type,
                "status": "workflow_built",
                "nodes": nodes,
                "message": f"Main menu workflow created with {len(nodes)} nodes",
                "next_step": "Execute with proper state to generate specification"
            }
            
        else:
            result = {
                "success": False,
                "error": f"Workflow type '{workflow_type}' not yet implemented",
                "available_types": ["main_menu"]
            }
            
        return result
        
    except Exception as e:
        return {
            "success": False,
            "error": str(e),
            "workflow_type": workflow_type
        }


@mcp.tool
def demo_core_asset_parsing() -> str:
    """
    Run comprehensive demo of core asset parsing across all categories.
    
    Returns:
        Demo output showing intelligent parsing results for each core category.
    """
    import io
    import sys
    from contextlib import redirect_stdout
    
    try:
        # Capture the demo output
        f = io.StringIO()
        with redirect_stdout(f):
            demo_core_parsing()
        
        demo_output = f.getvalue()
        return demo_output
        
    except Exception as e:
        return f"❌ Error running core parsing demo: {e}"


@mcp.resource("core://asset/categories")
def get_core_categories() -> str:
    """
    Get supported core asset categories and their parsing rules.
    
    Returns:
        List of core categories with parsing capabilities and example filenames.
    """
    categories = get_supported_core_categories()
    
    category_info = "Core Asset Categories & Parsing Rules:\n\n"
    
    examples = {
        "backgrounds": "academy_centered_on_full_screen_16bit_academy_backdrop_1024x1024.png",
        "image_maps": "logo_with_start_button_from_155,825_to_640,990_1536x1024.png", 
        "professors": "professor_portrait_16bit_1024x1024.png",
        "resources": "16bit_gold_trophy_on_transparent_background_1024x1024.png",
        "typography": "PressStart2P-Regular.ttf"
    }
    
    for category in categories:
        category_info += f"📁 {category}:\n"
        if category in examples:
            category_info += f"   Example: {examples[category]}\n"
        category_info += f"   Use: parse_core_asset_filename('{category}', 'filename')\n\n"
    
    category_info += "Core assets are separate from library assets (116k+ CC0 game content).\n"
    category_info += "Use demo_core_asset_parsing() to see parsing examples.\n"
    category_info += "Use validate_core_structure() to check asset organization."
    
    return category_info


if __name__ == "__main__":
    mcp.run()
