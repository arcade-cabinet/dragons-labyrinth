"""
Schema Builder for Pattern Generation

This module integrates the new schema system with the existing build_tools
architecture to generate patterns, templates, and JSON schemas from Arcade's API.
"""

import json
import sys
from pathlib import Path

from professor_pixel.schemas.scanner import ArcadeAPIScanner
from professor_pixel.schemas.models import PatternSchema, StudentChoice
from professor_pixel.schemas import registry
from professor_pixel.types import PatternCategory, ChoiceType, AssetType, GameLibrary

# Add main project to Python path for imports
src_dir = Path(__file__).parent.parent
if str(src_dir) not in sys.path:
    sys.path.insert(0, str(src_dir))


class SchemaBuilder:
    """
    Builds pattern schemas from Arcade API data and existing pattern definitions.
    
    This integrates with the existing build system to:
    1. Scan Arcade's API for teachable functions
    2. Generate pattern schemas with student choice points
    3. Create JSON schemas for AI structured output
    4. Generate Jinja2 templates for code compilation
    """
    
    def __init__(self, output_dir: Path = None):
        self.output_dir = output_dir or Path("arcade-schemas")
        self.scanner = ArcadeAPIScanner()
        self.schemas_generated = {}
        
    def scan_and_build(self) -> dict[str, any]:
        """Scan Arcade API and build comprehensive schema system."""
        print("🔍 Scanning Arcade API...")
        
        # Scan Arcade's API
        api_data = self.scanner.scan()
        
        # Build schemas from existing patterns (like sprites)
        self._load_existing_schemas()
        
        # Generate new schemas from API data
        self._generate_schemas_from_api(api_data)
        
        # Save all schemas to disk
        schemas_path, metadata_path = registry.save_schemas(self.output_dir)
        
        # Generate consolidated outputs for AI consumption
        self._generate_ai_schemas()
        
        return {
            "total_schemas": len(registry.schemas),
            "categories": {cat.name: len(opcodes) for cat, opcodes in registry.categories.items()},
            "api_functions": len(api_data.get("functions", {})),
            "schemas_file": str(schemas_path),
            "metadata_file": str(metadata_path)
        }
    
    def _load_existing_schemas(self):
        """Load existing pattern schemas (like sprites)."""
        # Schema modules are already imported at module level
        print(f"✅ Loaded {len(registry.schemas)} existing schemas")
    
    def _generate_schemas_from_api(self, api_data: dict[str, any]):
        """Generate new schemas from Arcade API functions."""
        functions = api_data.get("functions", {})
        generated_count = 0
        
        for func_name, func_data in functions.items():
            if self._should_create_schema(func_name, func_data):
                schema = self._create_schema_from_function(func_name, func_data)
                if schema:
                    registry.register(schema)
                    generated_count += 1
        
        print(f"✅ Generated {generated_count} new schemas from API")
    
    def _should_create_schema(self, func_name: str, func_data: dict[str, any]) -> bool:
        """Determine if an API function should become a teaching pattern."""
        name = func_name.split(".")[-1]
        
        # Skip if we already have a schema for this
        opcode = name.upper()
        if registry.get_schema(opcode):
            return False
        
        # Include drawing functions
        if name.startswith("draw_"):
            return True
            
        # Include common game functions
        teachable_keywords = [
            "sprite", "collision", "physics", "sound", "input", 
            "key", "mouse", "texture", "animation"
        ]
        
        if any(keyword in name.lower() for keyword in teachable_keywords):
            return True
            
        return False
    
    def _create_schema_from_function(self, func_name: str, func_data: dict[str, any]) -> PatternSchema:
        """Create a pattern schema from an Arcade API function."""
        name = func_name.split(".")[-1]
        opcode = name.upper()
        
        # Determine category from function name/module
        category = self._categorize_function(func_name, func_data)
        
        # Create basic choices based on parameters
        choices = self._generate_choices_from_parameters(func_data.get("parameters", []))
        
        return PatternSchema(
            opcode=opcode,
            category=category,
            title=f"{name.replace('_', ' ').title()}",
            description=func_data.get("docstring", f"Use {name} in your game."),
            
            teaches=[name.replace("_", " "), "arcade api"],
            prerequisites=[],
            complexity=func_data.get("complexity", 1),
            
            choices=choices,
            
            template_file=f"{category.name.lower()}/{name}.jinja2",
            output_files=[f"{name}.py"],
            
            arcade_functions=[func_name],
            arcade_classes=[],
            
            asset_categories=[],
            asset_types=[]
        )
    
    def _categorize_function(self, func_name: str, func_data: dict[str, any]) -> PatternCategory:
        """Categorize a function into a pattern category."""
        name = func_name.lower()
        
        if "draw" in name or "render" in name:
            return PatternCategory.VISUAL
        elif "sprite" in name:
            return PatternCategory.SPRITES
        elif "physics" in name or "collision" in name:
            return PatternCategory.COLLISION
        elif "sound" in name or "audio" in name:
            return PatternCategory.AUDIO
        elif "key" in name or "mouse" in name or "input" in name:
            return PatternCategory.INPUT
        elif "motion" in name or "move" in name:
            return PatternCategory.MOTION
        else:
            return PatternCategory.GAME
    
    def _generate_choices_from_parameters(self, parameters: list[dict[str, any]]) -> list[StudentChoice]:
        """Generate student choices from function parameters."""
        choices = []
        
        for param in parameters[:3]:  # Limit to first 3 params to avoid overwhelming students
            param_name = param.get("name", "")
            param_type = param.get("annotation", "any")
            
            # Skip common parameters that aren't interesting choices
            if param_name in ["self", "delta_time", "ctx"]:
                continue
                
            choice = StudentChoice(
                id=param_name,
                prompt=f"Set {param_name.replace('_', ' ')}:",
                choice_type="text_input",  # Default to text input
                default=str(param.get("default", "")),
                affects=[param_name]
            )
            choices.append(choice)
        
        return choices
    
    def _generate_ai_schemas(self):
        """Generate consolidated schemas for AI consumption."""
        output_dir = Path(self.output_dir)
        
        # Generate all JSON schemas
        all_schemas = registry.generate_all_json_schemas()
        
        # Save consolidated AI schema file
        ai_schema_file = output_dir / "ai_patterns.json"
        with open(ai_schema_file, 'w') as f:
            json.dump({
                "description": "Pattern schemas for AI lesson generation",
                "total_patterns": len(all_schemas),
                "categories": list({schema.category.name for schema in registry.schemas.values()}),
                "schemas": all_schemas
            }, f, indent=2)
        
        print(f"✅ Generated AI schema file: {ai_schema_file}")


def build_schemas(output_dir: str = "arcade-schemas") -> dict[str, any]:
    """Main entry point for schema building."""
    builder = SchemaBuilder(Path(output_dir))
    return builder.scan_and_build()


if __name__ == "__main__":
    # Command line usage
    import sys
    output_dir = sys.argv[1] if len(sys.argv) > 1 else "arcade-schemas"
    result = build_schemas(output_dir)
    
    print("\n🎯 Schema Build Complete!")
    print(f"   Total schemas: {result['total_schemas']}")
    print(f"   Categories: {len(result['categories'])}")
    print(f"   Output: {result['schemas_file']}")
