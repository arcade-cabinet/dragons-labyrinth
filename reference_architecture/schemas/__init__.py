"""
Build Tools Schema System

This module provides the schema specification system that transforms
Arcade's API into teachable patterns with student choice points.

The schema system is the foundation for generating:
- JSON schemas for AI structured output
- Jinja2 templates for pattern compilation  
- Lesson specifications with choice points
- Asset library integration points

This module integrates with the orchestrator package for pattern compilation.
"""

from professor_pixel.schemas.models import (
    StudentChoice,
    PatternSchema,
    LessonSpec,
    SchemaRegistry,
    TemplateCompilationResult,
    PatternCompiler,
)

from professor_pixel.schemas.scanner import (
    LibraryAPIScanner,
    APIFunction,
    APIClass,
)

# Import shared types from main project
from professor_pixel.types import (
    PatternCategory,
    ChoiceType,
    ComplexityLevel,
    TemplateEngine,
    ValidationLevel,
    AssetType,
    LessonPhase,
    SchemaID,
    TemplateFile,
    OutputFile,
    AssetQuery,
    ValidationRule,
)

# Schema modules will be generated dynamically from library API scans
# For now, create empty registry that will be populated by generated schemas

# Global registry instance  
registry = SchemaRegistry()

# Registry will be populated dynamically after schema generation
def load_generated_schemas(library: str = "arcade"):
    """Load schemas generated from library API scan."""
    # This will be implemented to load from generated schema files
    pass

__all__ = [
    # Models
    "StudentChoice",
    "PatternSchema", 
    "LessonSpec",
    "SchemaRegistry",
    "TemplateCompilationResult",
    "PatternCompiler",
    # Scanner components
    "LibraryAPIScanner",
    "APIFunction", 
    "APIClass",
    # Shared types
    "PatternCategory",
    "ChoiceType", 
    "ComplexityLevel",
    "TemplateEngine",
    "ValidationLevel",
    "AssetType",
    "LessonPhase",
    "SchemaID",
    "TemplateFile",
    "OutputFile",
    "AssetQuery",
    "ValidationRule",
    # Global instances
    "registry",
    "load_generated_schemas",
]
