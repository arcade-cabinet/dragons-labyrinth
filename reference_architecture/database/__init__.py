"""
Database package for Professor Pixel's Arcade Academy.

Provides SQLAlchemy-based database access for:
- Asset library management
- Student progress tracking  
- Lesson data storage
- Application state management

All database models and operations are centralized here for use by
other packages like assets/, orchestrator/, etc.
"""

from __future__ import annotations

# Import core database components
from professor_pixel.database.engine import (
    get_database_engine, 
    get_database_session,
    create_all_tables,
    drop_all_tables
)

from professor_pixel.database.models import (
    # Asset models
    Asset,
    AssetCategory as AssetCategoryModel,
    AssetSearchHistory,
    
    # Pattern models
    Pattern,
    PatternParameter,
    PatternAssetCompatibility,
    
    # AI-generated content models
    TemplateRules,
    LessonProgression,
    CurriculumStructure,
    CurriculumLesson,
    GeneratedTemplate,
    APIAnalysisResult,
    WorkflowExecution,
    
    # Base model
    DatabaseBase
)

from professor_pixel.database.crud import (
    # Database classes
    AssetDatabase,
    PatternDatabase,
    AIContentDatabase,
    asset_db,
    pattern_db,
    ai_content_db,
    
    # Index management (moved from assets/indexer.py)
    check_index_exists,
    build_asset_index,
    rebuild_asset_index, 
    get_index_stats,
    
    # Asset search (moved from assets/search.py)
    search_assets_semantic,
    get_asset_categories,
    get_asset_subcategories
)

from professor_pixel.database.tools import (
    # LangChain tools (moved from assets/tools.py)
    asset_search_tool,
    AssetSearchArgs
)

__all__ = [
    # Engine management
    "get_database_engine",
    "get_database_session",
    "create_all_tables",
    "drop_all_tables",
    
    # Models
    "Asset",
    "AssetCategoryModel", 
    "AssetSearchHistory",
    "Pattern",
    "PatternParameter",
    "PatternAssetCompatibility",
    "TemplateRules",
    "LessonProgression",
    "CurriculumStructure",
    "CurriculumLesson", 
    "GeneratedTemplate",
    "APIAnalysisResult",
    "WorkflowExecution",
    "DatabaseBase",
    
    # Database classes
    "AssetDatabase",
    "PatternDatabase",
    "AIContentDatabase",
    "asset_db",
    "pattern_db",
    "ai_content_db",
    
    # Functions
    "check_index_exists",
    "build_asset_index",
    "rebuild_asset_index",
    "get_index_stats",
    "search_assets_semantic",
    "get_asset_categories",
    "get_asset_subcategories",
    "asset_search_tool",
    "AssetSearchArgs"
]
