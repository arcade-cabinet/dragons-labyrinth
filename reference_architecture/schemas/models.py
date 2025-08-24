"""
Pydantic models for build tools schema system.

This module defines structured data models for pattern schemas, student choices,
lesson specifications, and template compilation.
"""

from __future__ import annotations

from pathlib import Path
from typing import Any

from pydantic import BaseModel, Field, field_validator

from professor_pixel.types import (
    SchemaID, TemplateFile, OutputFile, AssetQuery, ValidationRule,
    ChoiceType, ComplexityLevel, PatternCategory, TemplateEngine,
    ValidationLevel, AssetType, LessonPhase, PatternID, ChoiceID
)


class StudentChoice(BaseModel):
    """Represents a choice point for students."""
    
    id: ChoiceID
    prompt: str
    choice_type: ChoiceType
    options: list[str] = Field(default_factory=list)
    default: str | None = None
    asset_query: AssetQuery | None = None
    validation: ValidationRule | None = None
    affects: list[str] = Field(default_factory=list)
    
    @field_validator("options")
    @classmethod
    def validate_options(cls, v: list[str], info) -> list[str]:
        """Validate options based on choice type."""
        if "choice_type" not in info.data:
            return v
            
        choice_type = info.data["choice_type"]
        if choice_type in ("select_one", "select_many") and len(v) < 2:
            raise ValueError(f"Choice type {choice_type} requires at least 2 options")
        elif choice_type in ("text_input", "number_input") and len(v) > 0:
            raise ValueError(f"Choice type {choice_type} should not have predefined options")
        
        return v


class PatternSchema(BaseModel):
    """Base schema for all patterns."""
    
    opcode: str
    category: PatternCategory
    title: str
    description: str
    
    # Learning objectives
    teaches: list[str] = Field(default_factory=list)
    prerequisites: list[str] = Field(default_factory=list)
    complexity: ComplexityLevel = 1
    
    # Student interaction
    choices: list[StudentChoice] = Field(default_factory=list)
    
    # Template generation
    template_file: TemplateFile = ""
    output_files: list[OutputFile] = Field(default_factory=list)
    template_engine: TemplateEngine = TemplateEngine.JINJA2
    
    # Arcade API integration
    arcade_functions: list[str] = Field(default_factory=list)
    arcade_classes: list[str] = Field(default_factory=list)
    
    # Asset integration
    asset_categories: list[str] = Field(default_factory=list)
    asset_types: list[AssetType] = Field(default_factory=list)
    
    def to_json_schema(self) -> dict[str, Any]:
        """Generate JSON schema for AI structured output."""
        properties = {}
        required = []
        
        for choice in self.choices:
            if choice.choice_type == "select_one":
                properties[choice.id] = {
                    "type": "string",
                    "enum": choice.options,
                    "description": choice.prompt
                }
            elif choice.choice_type == "select_many":
                properties[choice.id] = {
                    "type": "array",
                    "items": {"type": "string", "enum": choice.options},
                    "description": choice.prompt
                }
            elif choice.choice_type == "text_input":
                properties[choice.id] = {
                    "type": "string",
                    "description": choice.prompt
                }
            elif choice.choice_type == "number_input":
                properties[choice.id] = {
                    "type": "number",
                    "description": choice.prompt
                }
            elif choice.choice_type == "asset_search":
                properties[choice.id] = {
                    "type": "string",
                    "description": f"{choice.prompt} (asset search: {choice.asset_query})"
                }
            
            if choice.default is None:
                required.append(choice.id)
        
        return {
            "type": "object",
            "properties": properties,
            "required": required,
            "title": f"{self.opcode} Pattern Choices",
            "description": self.description
        }
    
    def get_template_context(self, student_choices: dict[str, Any]) -> dict[str, Any]:
        """Get template context from student choices."""
        context = {
            "opcode": self.opcode,
            "category": self.category.name.lower(),
            "choices": student_choices,
        }
        
        # Add individual choice values for easier template access
        for choice in self.choices:
            if choice.id in student_choices:
                context[choice.id] = student_choices[choice.id]
            elif choice.default:
                context[choice.id] = choice.default
        
        return context


class LessonSpec(BaseModel):
    """Specification for a single lesson."""
    
    lesson_id: str
    title: str
    description: str
    
    # Dialogue system
    intro_dialogue: str
    success_dialogue: str
    hint_dialogue: str = ""
    
    # Pattern progression
    patterns: list[str] = Field(default_factory=list)
    output_files: list[OutputFile] = Field(default_factory=list)
    
    # Student interaction
    total_choices: int = 0
    choice_points: list[str] = Field(default_factory=list)
    
    # Asset integration
    required_assets: list[str] = Field(default_factory=list)
    asset_searches: list[AssetQuery] = Field(default_factory=list)
    
    # Lesson flow
    current_phase: LessonPhase = LessonPhase.INTRO
    estimated_duration_minutes: int = Field(default=15, ge=5, le=60)


class SchemaRegistry(BaseModel):
    """Registry for all pattern schemas."""
    
    schemas: dict[str, PatternSchema] = Field(default_factory=dict)
    categories: dict[PatternCategory, list[str]] = Field(default_factory=dict)
    
    def register(self, schema: PatternSchema) -> None:
        """Register a pattern schema."""
        self.schemas[schema.opcode] = schema
        
        if schema.category not in self.categories:
            self.categories[schema.category] = []
        self.categories[schema.category].append(schema.opcode)
    
    def get_schema(self, opcode: str) -> PatternSchema | None:
        """Get a pattern schema by opcode."""
        return self.schemas.get(opcode)
    
    def get_by_category(self, category: PatternCategory) -> list[PatternSchema]:
        """Get all schemas in a category."""
        opcodes = self.categories.get(category, [])
        return [self.schemas[opcode] for opcode in opcodes]
    
    def generate_all_json_schemas(self) -> dict[str, dict[str, Any]]:
        """Generate JSON schemas for all patterns."""
        return {
            opcode: schema.to_json_schema() 
            for opcode, schema in self.schemas.items()
        }
    
    def save_schemas(self, output_dir: Path) -> tuple[Path, Path]:
        """Save all schemas to JSON files."""
        import json
        
        output_dir = Path(output_dir)
        output_dir.mkdir(parents=True, exist_ok=True)
        
        # Save individual schemas
        for opcode, schema in self.schemas.items():
            schema_file = output_dir / f"{opcode.lower()}.json"
            with open(schema_file, 'w') as f:
                json.dump(schema.to_json_schema(), f, indent=2)
        
        # Save consolidated schemas
        all_schemas = output_dir / "all_patterns.json"
        with open(all_schemas, 'w') as f:
            json.dump(self.generate_all_json_schemas(), f, indent=2)
        
        # Save registry metadata
        metadata = output_dir / "registry.json"
        with open(metadata, 'w') as f:
            json.dump({
                "total_patterns": len(self.schemas),
                "categories": {cat.name.lower(): len(opcodes) for cat, opcodes in self.categories.items()},
                "opcodes": list(self.schemas.keys())
            }, f, indent=2)
        
        return all_schemas, metadata


class TemplateCompilationResult(BaseModel):
    """Result of compiling a pattern template."""
    
    opcode: str
    template_file: TemplateFile
    output_file: OutputFile
    compiled_code: str
    student_choices: dict[str, Any] = Field(default_factory=dict)
    compilation_time: float = 0.0
    success: bool = True
    error_message: str | None = None


class PatternCompiler(BaseModel):
    """Compiles pattern templates with student choices."""
    
    template_dir: Path
    output_dir: Path
    template_engine: TemplateEngine = TemplateEngine.JINJA2
    
    model_config = {"arbitrary_types_allowed": True}
    
    def compile_pattern(self, schema: PatternSchema, student_choices: dict[str, Any]) -> TemplateCompilationResult:
        """Compile a single pattern with student choices."""
        import time
        from jinja2 import Environment, FileSystemLoader, Template
        
        start_time = time.time()
        
        try:
            if self.template_engine == TemplateEngine.JINJA2:
                env = Environment(loader=FileSystemLoader(self.template_dir))
                template = env.get_template(schema.template_file)
                
                context = schema.get_template_context(student_choices)
                compiled_code = template.render(**context)
                
                return TemplateCompilationResult(
                    opcode=schema.opcode,
                    template_file=schema.template_file,
                    output_file=schema.output_files[0] if schema.output_files else f"{schema.opcode.lower()}.py",
                    compiled_code=compiled_code,
                    student_choices=student_choices,
                    compilation_time=time.time() - start_time,
                    success=True
                )
            else:
                raise ValueError(f"Unsupported template engine: {self.template_engine}")
                
        except Exception as e:
            return TemplateCompilationResult(
                opcode=schema.opcode,
                template_file=schema.template_file,
                output_file="",
                compiled_code="",
                student_choices=student_choices,
                compilation_time=time.time() - start_time,
                success=False,
                error_message=str(e)
            )
    
    def save_compiled_code(self, result: TemplateCompilationResult) -> Path:
        """Save compiled code to output file."""
        output_path = self.output_dir / result.output_file
        output_path.parent.mkdir(parents=True, exist_ok=True)
        
        output_path.write_text(result.compiled_code)
        return output_path
