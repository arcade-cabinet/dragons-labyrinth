"""
Core asset parsing rules for Professor Pixel's Arcade Academy.

Each category of core assets has different filename conventions and metadata
extraction requirements. This module defines the parsing rules for each category.
"""

import re
from typing import Any, Protocol
from pathlib import Path

from professor_pixel.models import UICoordinate, UIRectangle, InteractiveArea


class CoreCategoryRules(Protocol):
    """Protocol for core asset category parsing rules."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse filename and extract metadata."""
        ...
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate extracted metadata."""
        ...


class BackgroundRules:
    """Parsing rules for background assets."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse background filename for metadata."""
        name_without_ext = Path(filename).stem
        
        metadata = {
            "display_name": name_without_ext.replace('_', ' ').title(),
            "description": f"Background: {name_without_ext.replace('_', ' ')}"
        }
        
        # Extract dimensions
        dim_match = re.search(r'(\d+)x(\d+)', name_without_ext)
        if dim_match:
            metadata["dimensions"] = (int(dim_match.group(1)), int(dim_match.group(2)))
        
        # Detect era/style
        if "8bit" in name_without_ext.lower():
            metadata["era"] = "1980"
            metadata["style"] = "8bit"
        elif "16bit" in name_without_ext.lower():
            metadata["era"] = "1995" 
            metadata["style"] = "16bit"
        elif "32bit" in name_without_ext.lower():
            metadata["era"] = "2000"
            metadata["style"] = "32bit"
        else:
            metadata["era"] = "1995"  # Default
            metadata["style"] = "16bit"
        
        # Detect transparency
        if "transparent" in name_without_ext.lower():
            metadata["has_transparency"] = True
        else:
            metadata["has_transparency"] = False
            
        # Detect layout type
        if "centered" in name_without_ext.lower():
            metadata["layout"] = "centered"
        elif "full_screen" in name_without_ext.lower():
            metadata["layout"] = "full_screen"
        else:
            metadata["layout"] = "standard"
        
        return metadata
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate background metadata."""
        required_fields = ["dimensions", "era", "style"]
        return all(field in metadata for field in required_fields)


class ImageMapRules:
    """Parsing rules for interactive image map assets."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse image map filename for interactive areas and metadata."""
        name_without_ext = Path(filename).stem
        
        # Extract main description (before _with_)
        main_desc_match = re.match(r'^([^_]+(?:_[^_]+)*?)(?:_with_|_\d+x\d+)', filename)
        description = main_desc_match.group(1).replace('_', ' ') if main_desc_match else "Unknown image map"
        
        # Extract dimensions
        dim_match = re.search(r'_(\d+)x(\d+)(?:\.\w+)?$', filename)
        dimensions = (int(dim_match.group(1)), int(dim_match.group(2))) if dim_match else (1024, 1024)
        
        # Extract interactive areas
        interactive_areas = []
        
        # Find everything after _with_
        areas_section = filename
        if '_with_' in areas_section:
            areas_section = areas_section.split('_with_', 1)[1]
        
        # Remove dimensions from the end
        areas_section = re.sub(r'_\d+x\d+\.\w+$', '', areas_section)
        
        # Split on __and__ for multiple areas
        individual_areas = [area.strip('_') for area in areas_section.split('__and_') if area.strip('_')]
        
        for area_section in individual_areas:
            # Pattern: area_name_from_x,y_to_x,y
            area_pattern = r'(\w+(?:_\w+)*?)_from_(\d+),(\d+)_to_(\d+),(\d+)'
            area_match = re.search(area_pattern, area_section)
            
            if area_match:
                area_name, x1, y1, x2, y2 = area_match.groups()
                
                top_left = UICoordinate(x=int(x1), y=int(y1))
                bottom_right = UICoordinate(x=int(x2), y=int(y2))
                bounds = UIRectangle(top_left=top_left, bottom_right=bottom_right)
                
                # Clean up area name
                clean_name = area_name.replace('_', ' ')
                
                # Determine action type
                action_type = "navigate" if "button" in clean_name else "interact"
                
                area = InteractiveArea(
                    name=clean_name,
                    description=f"Interactive {clean_name} area",
                    bounds=bounds,
                    action_type=action_type,
                    action_data={"target": area_name}
                )
                
                interactive_areas.append(area)
        
        metadata = {
            "display_name": description.title(),
            "description": f"Interactive image map: {description}",
            "dimensions": dimensions,
            "interactive_areas": [area.model_dump() for area in interactive_areas],
            "area_count": len(interactive_areas)
        }
        
        return metadata
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate image map metadata."""
        required_fields = ["dimensions", "interactive_areas"]
        return (all(field in metadata for field in required_fields) and 
                len(metadata["interactive_areas"]) > 0)


class ProfessorRules:
    """Parsing rules for professor assets (portraits and videos)."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse professor filename for era, type, and dimensions."""
        name_without_ext = Path(filename).stem
        
        metadata = {
            "display_name": name_without_ext.replace('_', ' ').title(),
            "description": f"Professor asset: {name_without_ext.replace('_', ' ')}"
        }
        
        # Determine asset type
        extension = Path(filename).suffix.lower()
        if extension in ['.mp4', '.avi', '.mov']:
            metadata["asset_subtype"] = "video"
            metadata["is_intro_video"] = "intro" in name_without_ext.lower()
        else:
            metadata["asset_subtype"] = "portrait"
            metadata["is_portrait"] = True
        
        # Extract era information
        if "8bit" in name_without_ext.lower():
            metadata["era"] = "1980"
            metadata["style"] = "8bit"
        elif "16bit" in name_without_ext.lower():
            metadata["era"] = "1995"
            metadata["style"] = "16bit"
        elif "32bit" in name_without_ext.lower():
            metadata["era"] = "2000"
            metadata["style"] = "32bit"
        else:
            # Default for videos or undetermined portraits
            metadata["era"] = "1995"
            metadata["style"] = "modern"
        
        # Extract dimensions if present
        dim_match = re.search(r'(\d+)x(\d+)', name_without_ext)
        if dim_match:
            metadata["dimensions"] = (int(dim_match.group(1)), int(dim_match.group(2)))
        
        # Special handling for intro videos
        if metadata.get("is_intro_video"):
            metadata["play_behavior"] = "play_once_then_pause"
            metadata["interaction"] = "click_to_restart"
        
        return metadata
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate professor metadata."""
        return "era" in metadata and "style" in metadata


class ResourceRules:
    """Parsing rules for resource assets (trophies, icons, etc.)."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse resource filename for type and metadata."""
        name_without_ext = Path(filename).stem
        
        metadata = {
            "display_name": name_without_ext.replace('_', ' ').title(),
            "description": f"Resource: {name_without_ext.replace('_', ' ')}"
        }
        
        # Extract dimensions
        dim_match = re.search(r'(\d+)x(\d+)', name_without_ext)
        if dim_match:
            metadata["dimensions"] = (int(dim_match.group(1)), int(dim_match.group(2)))
        
        # Detect era/style
        if "8bit" in name_without_ext.lower():
            metadata["era"] = "1980"
            metadata["style"] = "8bit"
        elif "16bit" in name_without_ext.lower():
            metadata["era"] = "1995"
            metadata["style"] = "16bit"
        elif "32bit" in name_without_ext.lower():
            metadata["era"] = "2000"
            metadata["style"] = "32bit"
        
        # Detect transparency
        if "transparent" in name_without_ext.lower():
            metadata["has_transparency"] = True
        
        # Detect resource type
        if "trophy" in name_without_ext.lower():
            metadata["resource_type"] = "trophy"
            metadata["usage"] = "achievement_display"
        elif "icon" in name_without_ext.lower():
            metadata["resource_type"] = "icon"
            metadata["usage"] = "ui_element"
        else:
            metadata["resource_type"] = "general"
            metadata["usage"] = "decoration"
        
        return metadata
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate resource metadata."""
        return "resource_type" in metadata


class TypographyRules:
    """Parsing rules for typography assets (fonts)."""
    
    def parse_filename(self, filename: str) -> dict[str, Any]:
        """Parse font filename for format and metadata."""
        name_without_ext = Path(filename).stem
        extension = Path(filename).suffix.lower()
        
        metadata = {
            "display_name": name_without_ext.replace('_', ' ').title(),
            "description": f"Font: {name_without_ext.replace('_', ' ')}"
        }
        
        # Determine font format
        if extension == '.ttf':
            metadata["font_format"] = "truetype"
            metadata["web_compatible"] = False
        elif extension == '.otf':
            metadata["font_format"] = "opentype"
            metadata["web_compatible"] = False
        elif extension in ['.woff', '.woff2']:
            metadata["font_format"] = "web"
            metadata["web_compatible"] = True
        else:
            metadata["font_format"] = "unknown"
        
        # Detect style hints from name
        if "regular" in name_without_ext.lower():
            metadata["font_style"] = "regular"
        elif "bold" in name_without_ext.lower():
            metadata["font_style"] = "bold"
        elif "italic" in name_without_ext.lower():
            metadata["font_style"] = "italic"
        else:
            metadata["font_style"] = "regular"
        
        # Special detection for retro/pixel fonts
        if any(term in name_without_ext.lower() for term in ["pixel", "press", "start", "retro", "8bit", "16bit"]):
            metadata["font_category"] = "retro_pixel"
            metadata["ui_usage"] = "game_ui"
        else:
            metadata["font_category"] = "standard"
            metadata["ui_usage"] = "general"
        
        return metadata
    
    def validate_metadata(self, metadata: dict[str, Any]) -> bool:
        """Validate typography metadata."""
        return "font_format" in metadata and "font_style" in metadata


# Category mapping
CORE_CATEGORY_RULES = {
    "backgrounds": BackgroundRules(),
    "image_maps": ImageMapRules(),
    "professors": ProfessorRules(),
    "resources": ResourceRules(),
    "typography": TypographyRules(),
}


def get_core_rules(category: str) -> CoreCategoryRules | None:
    """Get parsing rules for a specific core asset category."""
    return CORE_CATEGORY_RULES.get(category.lower())


def parse_core_asset(category: str, filename: str) -> dict[str, Any]:
    """Parse a core asset filename using category-specific rules."""
    rules = get_core_rules(category)
    if not rules:
        # Fallback for unknown categories
        return {
            "display_name": Path(filename).stem.replace('_', ' ').title(),
            "description": f"Unknown category asset: {filename}",
            "parsed": False
        }
    
    metadata = rules.parse_filename(filename)
    metadata["parsed"] = True
    metadata["category"] = category
    metadata["filename"] = filename
    
    # Validate the parsed metadata
    if not rules.validate_metadata(metadata):
        metadata["validation_error"] = f"Invalid metadata for {category} asset"
    
    return metadata


def get_supported_core_categories() -> list[str]:
    """Get list of supported core asset categories."""
    return list(CORE_CATEGORY_RULES.keys())


def validate_core_asset_structure(core_path: Path) -> dict[str, Any]:
    """Validate that core asset directory has expected structure."""
    validation_result = {
        "valid": True,
        "errors": [],
        "warnings": [],
        "categories_found": [],
        "assets_per_category": {}
    }
    
    expected_categories = get_supported_core_categories()
    
    if not core_path.exists():
        validation_result["valid"] = False
        validation_result["errors"].append(f"Core assets directory not found: {core_path}")
        return validation_result
    
    for category in expected_categories:
        category_path = core_path / category
        if category_path.exists():
            validation_result["categories_found"].append(category)
            # Count assets in category
            asset_count = len([f for f in category_path.iterdir() if f.is_file()])
            validation_result["assets_per_category"][category] = asset_count
        else:
            validation_result["warnings"].append(f"Optional category missing: {category}")
    
    # Check for unexpected categories
    for item in core_path.iterdir():
        if item.is_dir() and item.name not in expected_categories:
            validation_result["warnings"].append(f"Unexpected category found: {item.name}")
    
    return validation_result


def demo_core_parsing():
    """Demonstrate core asset parsing for all categories."""
    test_files = {
        "backgrounds": "arcade_academy_centered_on_full_screen_16bit_academy_backdrop_1024x1024.png",
        "image_maps": "logo_and_academy_building_with_start_button_from_155,825_to_640,990__and_continue_button_from_910,825_to_1380,990_1536x1024.png",
        "professors": "professor_portrait_16bit_1024x1024.png",
        "resources": "16bit_gold_trophy_on_transparent_background_1024x1024.png",
        "typography": "PressStart2P-Regular.ttf"
    }
    
    print("🎯 Core Asset Parsing Demo")
    print("=" * 60)
    
    for category, filename in test_files.items():
        print(f"\n📁 Category: {category}")
        print(f"📄 File: {filename}")
        
        metadata = parse_core_asset(category, filename)
        
        print(f"✅ Display Name: {metadata.get('display_name', 'N/A')}")
        print(f"📝 Description: {metadata.get('description', 'N/A')}")
        
        # Show category-specific metadata
        if category == "backgrounds":
            print(f"🎨 Era: {metadata.get('era', 'N/A')} ({metadata.get('style', 'N/A')})")
            print(f"📐 Dimensions: {metadata.get('dimensions', 'N/A')}")
            print(f"🔍 Layout: {metadata.get('layout', 'N/A')}")
        elif category == "image_maps":
            print(f"📐 Dimensions: {metadata.get('dimensions', 'N/A')}")
            print(f"🎯 Interactive Areas: {metadata.get('area_count', 0)}")
            if 'interactive_areas' in metadata:
                for area_data in metadata['interactive_areas']:
                    area = InteractiveArea(**area_data)
                    print(f"  • {area.name}: {area.bounds.width}x{area.bounds.height} at ({area.bounds.top_left.x},{area.bounds.top_left.y})")
        elif category == "professors":
            print(f"🎭 Type: {metadata.get('asset_subtype', 'N/A')}")
            print(f"🎨 Era: {metadata.get('era', 'N/A')} ({metadata.get('style', 'N/A')})")
            if metadata.get('dimensions'):
                print(f"📐 Dimensions: {metadata['dimensions']}")
        elif category == "resources":
            print(f"🏆 Type: {metadata.get('resource_type', 'N/A')}")
            print(f"🎯 Usage: {metadata.get('usage', 'N/A')}")
            if metadata.get('era'):
                print(f"🎨 Era: {metadata['era']} ({metadata.get('style', 'N/A')})")
        elif category == "typography":
            print(f"📝 Format: {metadata.get('font_format', 'N/A')}")
            print(f"🎨 Style: {metadata.get('font_style', 'N/A')}")
            print(f"🎮 Category: {metadata.get('font_category', 'N/A')}")
        
        print(f"✅ Validation: {'PASS' if metadata.get('parsed') and not metadata.get('validation_error') else 'FAIL'}")


if __name__ == "__main__":
    demo_core_parsing()
