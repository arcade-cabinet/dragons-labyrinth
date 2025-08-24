"""
SQLAlchemy ORM models for Professor Pixel's Arcade Academy database.

All database tables are defined here using SQLAlchemy declarative syntax.
"""

from __future__ import annotations

from datetime import datetime

from sqlalchemy import BigInteger, Enum, ForeignKey, Index, Text, func, JSON
from sqlalchemy.orm import DeclarativeBase, Mapped, mapped_column, relationship

from professor_pixel.types import AssetFileType, MediaType, GameLibrary, PatternCategory


class DatabaseBase(DeclarativeBase):
    """Base class for all database models."""
    pass


class Asset(DatabaseBase):
    """Asset file record in the database."""
    
    __tablename__ = "assets"
    
    # Primary key
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # File path information
    path: Mapped[str] = mapped_column(unique=True, index=True)
    category: Mapped[str] = mapped_column(index=True)
    subcategory: Mapped[str | None] = mapped_column(index=True)
    
    # File name components
    filename: Mapped[str] = mapped_column(index=True)
    stem: Mapped[str] = mapped_column(index=True)
    extension: Mapped[str] = mapped_column(index=True)
    
    # File classification
    file_type: Mapped[AssetFileType] = mapped_column(index=True)
    media_type: Mapped[str] = mapped_column(index=True)  # Store as string for simplicity
    
    # File metadata
    file_size: Mapped[int | None] = mapped_column(BigInteger)
    
    # Search optimization
    searchable_text: Mapped[str] = mapped_column(Text, default="")
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Relationships
    search_history: Mapped[list["AssetSearchHistory"]] = relationship(
        "AssetSearchHistory", back_populates="asset", cascade="all, delete-orphan"
    )
    
    # Table configuration
    __table_args__ = (
        # Composite indexes for common queries
        Index("ix_asset_category_subcategory", "category", "subcategory"),
        Index("ix_asset_category_file_type", "category", "file_type"),
        Index("ix_asset_media_type_category", "media_type", "category"),
        Index("ix_asset_stem_category", "stem", "category"),
        
        # Full-text search index (SQLite FTS if needed later)
        Index("ix_asset_searchable_text", "searchable_text"),
    )
    
    def __repr__(self) -> str:
        return f"Asset(id={self.id}, path='{self.path}', category='{self.category}')"
    
    @property
    def full_path_parts(self) -> list[str]:
        """Get path components as a list."""
        return self.path.split("/")
    
    @property
    def relative_directory(self) -> str:
        """Get the directory containing this asset."""
        return "/".join(self.path.split("/")[:-1])
    
    @property
    def display_name(self) -> str:
        """Get a human-readable display name."""
        return self.stem.replace("_", " ").replace("-", " ").title()


class AssetCategory(DatabaseBase):
    """Asset category metadata and statistics."""
    
    __tablename__ = "asset_categories"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    name: Mapped[str] = mapped_column(unique=True, index=True)
    description: Mapped[str | None] = mapped_column(Text)
    
    # Statistics (updated by background tasks)
    asset_count: Mapped[int] = mapped_column(default=0)
    total_size_bytes: Mapped[int] = mapped_column(BigInteger, default=0)
    
    # Timestamps  
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    def __repr__(self) -> str:
        return f"AssetCategory(name='{self.name}', asset_count={self.asset_count})"
    
    @property
    def total_size_mb(self) -> float:
        """Total size in megabytes."""
        return self.total_size_bytes / (1024 * 1024)


class AssetSearchHistory(DatabaseBase):
    """Track asset search queries for analytics and optimization."""
    
    __tablename__ = "asset_search_history"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Search details
    query: Mapped[str | None] = mapped_column(index=True)
    category_filter: Mapped[str | None] = mapped_column()
    file_type_filter: Mapped[str | None] = mapped_column()
    
    # Results
    result_count: Mapped[int] = mapped_column(default=0)
    query_time_ms: Mapped[float] = mapped_column(default=0.0)
    
    # Asset that was selected (if any)
    asset_id: Mapped[int | None] = mapped_column(ForeignKey("assets.id", ondelete="SET NULL"))
    asset: Mapped["Asset | None"] = relationship("Asset", back_populates="search_history")
    
    # Metadata
    user_agent: Mapped[str | None] = mapped_column()
    ip_address: Mapped[str | None] = mapped_column()
    
    # Timestamps
    searched_at: Mapped[datetime] = mapped_column(server_default=func.now(), index=True)
    
    def __repr__(self) -> str:
        return f"AssetSearchHistory(query='{self.query}', result_count={self.result_count})"


class DatabaseMetadata(DatabaseBase):
    """Store database metadata and version information."""
    
    __tablename__ = "database_metadata"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    key: Mapped[str] = mapped_column(unique=True, index=True)
    value: Mapped[str] = mapped_column(Text)
    
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    def __repr__(self) -> str:
        return f"DatabaseMetadata(key='{self.key}', value='{self.value}')"


class Pattern(DatabaseBase):
    """API pattern discovered from library scanning."""
    
    __tablename__ = "patterns"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Pattern identification
    opcode: Mapped[str] = mapped_column(unique=True, index=True)
    source_function: Mapped[str] = mapped_column(index=True)  # e.g., "arcade.draw_circle"
    library: Mapped[GameLibrary] = mapped_column(index=True)
    category: Mapped[PatternCategory] = mapped_column(index=True)
    
    # Function metadata
    signature: Mapped[str] = mapped_column(Text)
    docstring: Mapped[str] = mapped_column(Text, default="")
    return_type: Mapped[str] = mapped_column(default="None")
    
    # Educational metadata
    complexity: Mapped[int] = mapped_column(index=True)  # 1-5 scale
    is_beginner_safe: Mapped[bool] = mapped_column(default=False, index=True)
    
    # Template generation
    template_content: Mapped[str | None] = mapped_column(Text)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Relationships
    parameters: Mapped[list["PatternParameter"]] = relationship(
        "PatternParameter", back_populates="pattern", cascade="all, delete-orphan"
    )
    asset_compatibility: Mapped[list["PatternAssetCompatibility"]] = relationship(
        "PatternAssetCompatibility", back_populates="pattern", cascade="all, delete-orphan"
    )
    
    def __repr__(self) -> str:
        return f"Pattern(opcode='{self.opcode}', library={self.library.name}, category={self.category.name})"


class PatternParameter(DatabaseBase):
    """Parameter for a pattern function."""
    
    __tablename__ = "pattern_parameters"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Pattern relationship
    pattern_id: Mapped[int] = mapped_column(ForeignKey("patterns.id", ondelete="CASCADE"), index=True)
    pattern: Mapped["Pattern"] = relationship("Pattern", back_populates="parameters")
    
    # Parameter info
    name: Mapped[str] = mapped_column(index=True)
    param_type: Mapped[str] = mapped_column()  # e.g., "str", "int", "float", "tuple[int, int, int]"
    is_required: Mapped[bool] = mapped_column(default=True)
    default_value: Mapped[str | None] = mapped_column()
    
    # Educational hints
    description: Mapped[str] = mapped_column(Text, default="")
    suggested_asset_type: Mapped[str | None] = mapped_column()  # "image", "audio", "font", etc.
    
    def __repr__(self) -> str:
        return f"PatternParameter(name='{self.name}', type='{self.param_type}', required={self.is_required})"


class PatternAssetCompatibility(DatabaseBase):
    """Links patterns to compatible asset types."""
    
    __tablename__ = "pattern_asset_compatibility"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Relationships
    pattern_id: Mapped[int] = mapped_column(ForeignKey("patterns.id", ondelete="CASCADE"), index=True)
    pattern: Mapped["Pattern"] = relationship("Pattern", back_populates="asset_compatibility")
    
    # Compatibility info
    asset_file_type: Mapped[AssetFileType] = mapped_column(index=True)
    parameter_name: Mapped[str] = mapped_column()  # Which parameter uses this asset type
    is_required: Mapped[bool] = mapped_column(default=False)
    
    def __repr__(self) -> str:
        return f"PatternAssetCompatibility(pattern={self.pattern.opcode}, asset_type={self.asset_file_type.name})"


class TemplateRules(DatabaseBase):
    """AI-generated template rules for a specific library."""
    
    __tablename__ = "template_rules"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Library identification
    library: Mapped[GameLibrary] = mapped_column(index=True)
    library_version: Mapped[str] = mapped_column(index=True)
    
    # Rule generation metadata
    rules_version: Mapped[str] = mapped_column(index=True)  # Hash of library_rules + usage_analysis
    generated_at: Mapped[datetime] = mapped_column(server_default=func.now())
    
    # Template generation rules (JSON storage)
    category_templates: Mapped[dict] = mapped_column(JSON)  # Category -> style -> template patterns
    style_configurations: Mapped[dict] = mapped_column(JSON)  # Style -> configuration
    function_mappings: Mapped[dict] = mapped_column(JSON)  # Pattern -> function mapping
    
    # Educational structure
    complexity_progression: Mapped[dict] = mapped_column(JSON)  # Category -> complexity levels
    student_customization_points: Mapped[dict] = mapped_column(JSON)  # Category -> customization areas
    
    # Coverage metrics
    patterns_covered: Mapped[int] = mapped_column()
    categories_supported: Mapped[list[str]] = mapped_column(JSON)
    
    # Human approval
    is_approved: Mapped[bool] = mapped_column(default=False, index=True)
    approved_by: Mapped[str | None] = mapped_column()
    approval_notes: Mapped[str | None] = mapped_column(Text)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Relationships
    lesson_progressions: Mapped[list["LessonProgression"]] = relationship(
        "LessonProgression", back_populates="template_rules", cascade="all, delete-orphan"
    )
    
    # Unique constraint on library + rules_version
    __table_args__ = (
        Index("ix_template_rules_lib_rules_version", "library", "rules_version", unique=True),
        Index("ix_template_rules_lib_approved", "library", "is_approved"),
    )
    
    def __repr__(self) -> str:
        return f"TemplateRules(library={self.library.name}, version={self.rules_version}, approved={self.is_approved})"


class LessonProgression(DatabaseBase):
    """AI-generated lesson progression for a curriculum."""
    
    __tablename__ = "lesson_progressions"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Template rules relationship
    template_rules_id: Mapped[int] = mapped_column(ForeignKey("template_rules.id", ondelete="CASCADE"), index=True)
    template_rules: Mapped["TemplateRules"] = relationship("TemplateRules", back_populates="lesson_progressions")
    
    # Lesson identification
    lesson_id: Mapped[str] = mapped_column(index=True)
    title: Mapped[str] = mapped_column()
    description: Mapped[str] = mapped_column(Text)
    
    # Lesson structure
    estimated_duration_minutes: Mapped[int] = mapped_column()
    complexity_level: Mapped[int] = mapped_column(index=True)
    student_choice_points: Mapped[int] = mapped_column()
    
    # Pattern integration (JSON storage for flexibility)
    patterns_used: Mapped[list[str]] = mapped_column(JSON)  # Pattern opcodes
    new_patterns: Mapped[list[str]] = mapped_column(JSON)  # First-time patterns
    reinforced_patterns: Mapped[list[str]] = mapped_column(JSON)  # Review patterns
    
    # Dependencies
    prerequisites: Mapped[list[str]] = mapped_column(JSON)  # Prerequisite lesson IDs
    
    # Educational metadata
    teaches_concepts: Mapped[list[str]] = mapped_column(JSON)
    learning_objectives: Mapped[list[str]] = mapped_column(JSON)
    
    # Lesson content
    intro_dialogue: Mapped[str] = mapped_column(Text)
    success_dialogue: Mapped[str] = mapped_column(Text)
    hint_dialogue: Mapped[str | None] = mapped_column(Text)
    
    # Output files that will be generated
    output_files: Mapped[list[str]] = mapped_column(JSON)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Relationships
    curriculum_lessons: Mapped[list["CurriculumLesson"]] = relationship(
        "CurriculumLesson", back_populates="lesson_progression", cascade="all, delete-orphan"
    )
    
    def __repr__(self) -> str:
        return f"LessonProgression(lesson_id='{self.lesson_id}', title='{self.title}', complexity={self.complexity_level})"


class CurriculumStructure(DatabaseBase):
    """AI-generated curriculum structure for a complete course."""
    
    __tablename__ = "curriculum_structures"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Curriculum identification
    curriculum_id: Mapped[str] = mapped_column(unique=True, index=True)
    library: Mapped[GameLibrary] = mapped_column(index=True)
    title: Mapped[str] = mapped_column()
    description: Mapped[str] = mapped_column(Text)
    
    # Curriculum metadata
    total_lessons: Mapped[int] = mapped_column()
    estimated_total_hours: Mapped[float] = mapped_column()
    progression_type: Mapped[str] = mapped_column()  # "linear", "branching", "spiral"
    
    # Educational structure
    learning_objectives: Mapped[list[str]] = mapped_column(JSON)
    target_student_age_range: Mapped[tuple[int, int]] = mapped_column(JSON)
    
    # Generation metadata
    generation_parameters: Mapped[dict] = mapped_column(JSON)  # Parameters used for generation
    generation_hash: Mapped[str] = mapped_column(index=True)  # Hash for idempotency
    
    # Quality metrics
    patterns_count: Mapped[int] = mapped_column()
    templates_count: Mapped[int] = mapped_column()
    total_choice_points: Mapped[int] = mapped_column()
    
    # Human approval
    is_approved: Mapped[bool] = mapped_column(default=False, index=True)
    approved_by: Mapped[str | None] = mapped_column()
    approval_notes: Mapped[str | None] = mapped_column(Text)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Relationships
    lessons: Mapped[list["CurriculumLesson"]] = relationship(
        "CurriculumLesson", back_populates="curriculum", cascade="all, delete-orphan"
    )
    generated_templates: Mapped[list["GeneratedTemplate"]] = relationship(
        "GeneratedTemplate", back_populates="curriculum", cascade="all, delete-orphan"
    )
    
    def __repr__(self) -> str:
        return f"CurriculumStructure(id='{self.curriculum_id}', library={self.library.name}, approved={self.is_approved})"


class CurriculumLesson(DatabaseBase):
    """Individual lesson within a curriculum structure."""
    
    __tablename__ = "curriculum_lessons"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Curriculum relationship
    curriculum_id: Mapped[int] = mapped_column(ForeignKey("curriculum_structures.id", ondelete="CASCADE"), index=True)
    curriculum: Mapped["CurriculumStructure"] = relationship("CurriculumStructure", back_populates="lessons")
    
    # Lesson progression relationship
    lesson_progression_id: Mapped[int] = mapped_column(ForeignKey("lesson_progressions.id", ondelete="CASCADE"), index=True)
    lesson_progression: Mapped["LessonProgression"] = relationship("LessonProgression", back_populates="curriculum_lessons")
    
    # Lesson ordering
    lesson_order: Mapped[int] = mapped_column(index=True)
    
    # Lesson customization for this curriculum
    custom_parameters: Mapped[dict | None] = mapped_column(JSON)  # Curriculum-specific overrides
    
    def __repr__(self) -> str:
        return f"CurriculumLesson(curriculum={self.curriculum.curriculum_id}, lesson={self.lesson_progression.lesson_id}, order={self.lesson_order})"


class GeneratedTemplate(DatabaseBase):
    """AI-generated Jinja2 template for a specific pattern."""
    
    __tablename__ = "generated_templates"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Template identification
    pattern_opcode: Mapped[str] = mapped_column(index=True)
    library: Mapped[GameLibrary] = mapped_column(index=True)
    template_style: Mapped[str] = mapped_column(index=True)  # "beginner", "intermediate", "advanced"
    
    # Template content
    template_content: Mapped[str] = mapped_column(Text)
    template_file_path: Mapped[str] = mapped_column()  # Relative path for organization
    
    # Generation metadata
    generated_from_rules_version: Mapped[str] = mapped_column(index=True)  # Template rules version used
    generation_parameters: Mapped[dict] = mapped_column(JSON)  # Parameters used for generation
    
    # Template metadata
    choice_points: Mapped[list[str]] = mapped_column(JSON)  # Student choice variable names
    output_files: Mapped[list[str]] = mapped_column(JSON)  # Files this template generates
    
    # Quality metrics
    complexity_score: Mapped[int] = mapped_column()
    lines_of_code_estimate: Mapped[int] = mapped_column()
    student_concepts_count: Mapped[int] = mapped_column()
    
    # Curriculum relationship
    curriculum_id: Mapped[int | None] = mapped_column(ForeignKey("curriculum_structures.id", ondelete="CASCADE"), index=True)
    curriculum: Mapped["CurriculumStructure | None"] = relationship("CurriculumStructure", back_populates="generated_templates")
    
    # Human approval
    is_approved: Mapped[bool] = mapped_column(default=False, index=True)
    approved_by: Mapped[str | None] = mapped_column()
    approval_notes: Mapped[str | None] = mapped_column(Text)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Unique constraint per pattern/library/style
    __table_args__ = (
        Index("ix_template_pattern_library_style", "pattern_opcode", "library", "template_style", unique=True),
        Index("ix_template_approved", "library", "template_style", "is_approved"),
    )
    
    def __repr__(self) -> str:
        return f"GeneratedTemplate(pattern='{self.pattern_opcode}', library={self.library.name}, style='{self.template_style}')"


class APIAnalysisResult(DatabaseBase):
    """Results from API analysis for idempotency tracking."""
    
    __tablename__ = "api_analysis_results"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Analysis identification
    library: Mapped[GameLibrary] = mapped_column(index=True)
    library_version: Mapped[str] = mapped_column(index=True)
    analysis_version: Mapped[str] = mapped_column(index=True)  # Hash of inputs (library_rules + usage_analysis)
    
    # Analysis results (JSON storage)
    usage_patterns: Mapped[dict] = mapped_column(JSON)  # LibCST usage analysis results
    pattern_suggestions: Mapped[list[dict]] = mapped_column(JSON)  # AI-generated pattern suggestions
    lesson_progression_data: Mapped[list[dict]] = mapped_column(JSON)  # AI-generated lesson progression
    
    # Analysis metadata
    total_functions_analyzed: Mapped[int] = mapped_column()
    patterns_generated: Mapped[int] = mapped_column()
    lessons_designed: Mapped[int] = mapped_column()
    
    # Quality metrics
    pattern_coverage_percent: Mapped[float] = mapped_column()
    educational_safety_score: Mapped[float] = mapped_column()
    
    # Processing metadata
    ai_model_used: Mapped[str] = mapped_column()
    processing_time_seconds: Mapped[float] = mapped_column()
    tokens_consumed: Mapped[int | None] = mapped_column()
    
    # Human approval
    is_approved: Mapped[bool] = mapped_column(default=False, index=True)
    approved_by: Mapped[str | None] = mapped_column()
    approval_notes: Mapped[str | None] = mapped_column(Text)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    # Unique constraint for idempotency
    __table_args__ = (
        Index("ix_api_analysis_lib_version", "library", "library_version", "analysis_version", unique=True),
        Index("ix_api_analysis_approved", "library", "is_approved"),
    )
    
    def __repr__(self) -> str:
        return f"APIAnalysisResult(library={self.library.name}, version={self.analysis_version}, approved={self.is_approved})"


class WorkflowExecution(DatabaseBase):
    """Track workflow executions for debugging and idempotency."""
    
    __tablename__ = "workflow_executions"
    
    id: Mapped[int] = mapped_column(primary_key=True, autoincrement=True)
    
    # Workflow identification
    workflow_type: Mapped[str] = mapped_column(index=True)  # "template_rules", "curriculum_generation", etc.
    workflow_id: Mapped[str] = mapped_column(index=True)  # Thread ID
    library: Mapped[GameLibrary] = mapped_column(index=True)
    
    # Input tracking for idempotency
    input_hash: Mapped[str] = mapped_column(index=True)  # Hash of all inputs
    
    # Execution status
    status: Mapped[str] = mapped_column(index=True)  # "running", "completed", "failed", "cancelled"
    current_step: Mapped[str | None] = mapped_column()
    total_steps: Mapped[int] = mapped_column(default=0)
    
    # Results
    output_data: Mapped[dict | None] = mapped_column(JSON)  # Final workflow outputs
    error_message: Mapped[str | None] = mapped_column(Text)
    
    # Performance tracking
    started_at: Mapped[datetime] = mapped_column(server_default=func.now())
    completed_at: Mapped[datetime | None] = mapped_column()
    processing_time_seconds: Mapped[float | None] = mapped_column()
    
    # Human interaction
    human_interactions: Mapped[int] = mapped_column(default=0)  # Number of human review steps
    autonomous_mode: Mapped[bool] = mapped_column(default=False)
    
    # Timestamps
    created_at: Mapped[datetime] = mapped_column(server_default=func.now())
    updated_at: Mapped[datetime] = mapped_column(server_default=func.now(), onupdate=func.now())
    
    __table_args__ = (
        Index("ix_workflow_type_status", "workflow_type", "status"),
        Index("ix_workflow_input_hash", "input_hash"),
        Index("ix_workflow_library_type", "library", "workflow_type"),
    )
    
    def __repr__(self) -> str:
        return f"WorkflowExecution(type='{self.workflow_type}', library={self.library.name}, status='{self.status}')"


# Create a convenience mapping for dynamic access
MODEL_REGISTRY = {
    "Asset": Asset,
    "AssetCategory": AssetCategory, 
    "AssetSearchHistory": AssetSearchHistory,
    "DatabaseMetadata": DatabaseMetadata,
    "Pattern": Pattern,
    "PatternParameter": PatternParameter,
    "PatternAssetCompatibility": PatternAssetCompatibility,
    "TemplateRules": TemplateRules,
    "LessonProgression": LessonProgression,
    "CurriculumStructure": CurriculumStructure,
    "CurriculumLesson": CurriculumLesson,
    "GeneratedTemplate": GeneratedTemplate,
    "APIAnalysisResult": APIAnalysisResult,
    "WorkflowExecution": WorkflowExecution,
}
