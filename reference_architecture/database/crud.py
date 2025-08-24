"""
CRUD operations for Professor Pixel's asset database.

Simple, fast SQLite queries - no vector search nonsense.
"""

from __future__ import annotations

import time
from pathlib import Path

from sqlalchemy import and_, or_, func, select
from sqlalchemy.orm import Session

from professor_pixel.database.engine import database_session, create_all_tables
from professor_pixel.database.models import (
    Asset, AssetCategory, AssetSearchHistory, DatabaseMetadata,
    Pattern, PatternParameter, PatternAssetCompatibility,
    TemplateRules, LessonProgression, CurriculumStructure, 
    CurriculumLesson, GeneratedTemplate, APIAnalysisResult, WorkflowExecution
)
from professor_pixel.models import (
    AssetRecord, AssetSearchFilter, AssetSearchRequest, AssetSearchResult, DatabaseStats
)
from professor_pixel.types import AssetFileType, MediaType, GameLibrary, PatternCategory
from professor_pixel.settings import get_settings


class AssetDatabase:
    """Simple asset database operations."""
    
    def __init__(self):
        self.settings = get_settings()
        self.settings.paths.ensure_directories()
        create_all_tables()
    
    def index_assets(self) -> int:
        """Index all assets from the library directory."""
        library_dir = self.settings.paths.library_dir
        
        if not library_dir.exists():
            raise FileNotFoundError(f"Asset library not found: {library_dir}")
        
        indexed_count = 0
        
        with database_session() as session:
            # Clear existing assets
            session.query(Asset).delete()
            session.commit()
            
            # Scan all assets
            for asset_path in library_dir.rglob("*"):
                if asset_path.is_file() and not asset_path.name.startswith('.'):
                    record = AssetRecord.from_file_path(library_dir, asset_path)
                    
                    asset = Asset(
                        path=record.path,
                        category=record.category,
                        subcategory=record.subcategory,
                        filename=record.filename,
                        stem=record.stem,
                        extension=record.extension,
                        file_type=record.file_type,
                        media_type=record.media_type,
                        file_size=record.file_size,
                        searchable_text=record.searchable_text
                    )
                    
                    session.add(asset)
                    indexed_count += 1
                    
                    # Commit in batches for better performance
                    if indexed_count % 1000 == 0:
                        session.commit()
            
            # Final commit
            session.commit()
            
            # Update metadata
            self._set_metadata(session, "last_indexed", str(time.time()))
            self._set_metadata(session, "asset_count", str(indexed_count))
            session.commit()
        
        return indexed_count
    
    def search_assets(
        self, 
        request: AssetSearchRequest
    ) -> AssetSearchResult:
        """Search assets with filtering and pagination."""
        start_time = time.time()
        
        with database_session() as session:
            query = select(Asset)
            
            # Build WHERE conditions
            conditions = []
            
            # Text search
            if request.query:
                conditions.append(
                    or_(
                        Asset.searchable_text.contains(request.query.lower()),
                        Asset.stem.contains(request.query.lower()),
                        Asset.category.contains(request.query.lower())
                    )
                )
            
            # Apply filters
            if request.filter:
                if request.filter.category:
                    conditions.append(Asset.category == request.filter.category)
                
                if request.filter.subcategory:
                    conditions.append(Asset.subcategory == request.filter.subcategory)
                
                if request.filter.file_type:
                    conditions.append(Asset.file_type == request.filter.file_type)
                
                if request.filter.media_type:
                    conditions.append(Asset.media_type == request.filter.media_type)
                
                if request.filter.extension:
                    conditions.append(Asset.extension == request.filter.extension)
                
                if request.filter.min_size:
                    conditions.append(Asset.file_size >= request.filter.min_size)
                
                if request.filter.max_size:
                    conditions.append(Asset.file_size <= request.filter.max_size)
            
            # Apply conditions
            if conditions:
                query = query.where(and_(*conditions))
            
            # Get total count
            count_query = select(func.count()).select_from(query.subquery())
            total_count = session.execute(count_query).scalar()
            
            # Apply pagination
            query = query.offset(request.offset).limit(request.limit)
            
            # Execute query
            result = session.execute(query)
            assets = result.scalars().all()
            
            # Convert to AssetRecord models
            asset_records = []
            for asset in assets:
                record = AssetRecord(
                    id=str(asset.id),
                    path=asset.path,
                    category=asset.category,
                    subcategory=asset.subcategory,
                    filename=asset.filename,
                    stem=asset.stem,
                    extension=asset.extension,
                    file_type=asset.file_type,
                    media_type=asset.media_type,  # type: ignore
                    file_size=asset.file_size,
                    searchable_text=asset.searchable_text,
                    created_at=asset.created_at,
                    updated_at=asset.updated_at
                )
                asset_records.append(record)
            
            # Log search
            if request.query:
                search_log = AssetSearchHistory(
                    query=request.query,
                    category_filter=request.filter.category if request.filter else None,
                    file_type_filter=request.filter.file_type.name if request.filter and request.filter.file_type else None,
                    result_count=len(asset_records),
                    query_time_ms=(time.time() - start_time) * 1000
                )
                session.add(search_log)
                session.commit()
            
            query_time = (time.time() - start_time) * 1000
            has_more = (request.offset + len(asset_records)) < total_count
            
            return AssetSearchResult(
                assets=asset_records,
                total_count=total_count,
                query_time_ms=query_time,
                has_more=has_more
            )
    
    def get_categories(self) -> list[str]:
        """Get all asset categories."""
        with database_session() as session:
            result = session.execute(
                select(Asset.category).distinct().order_by(Asset.category)
            )
            return [row[0] for row in result]
    
    def get_subcategories(self, category: str) -> list[str]:
        """Get subcategories for a category."""
        with database_session() as session:
            result = session.execute(
                select(Asset.subcategory)
                .where(and_(Asset.category == category, Asset.subcategory.is_not(None)))
                .distinct()
                .order_by(Asset.subcategory)
            )
            return [row[0] for row in result if row[0]]
    
    def get_stats(self) -> DatabaseStats:
        """Get database statistics."""
        with database_session() as session:
            # Total assets
            total_assets = session.execute(select(func.count(Asset.id))).scalar()
            
            # Categories
            category_result = session.execute(
                select(Asset.category, func.count(Asset.id))
                .group_by(Asset.category)
                .order_by(Asset.category)
            )
            categories = {row[0]: row[1] for row in category_result}
            
            # File types
            file_type_result = session.execute(
                select(Asset.file_type, func.count(Asset.id))
                .group_by(Asset.file_type)
                .order_by(Asset.file_type)
            )
            file_types = {row[0]: row[1] for row in file_type_result}
            
            # Media types
            media_type_result = session.execute(
                select(Asset.media_type, func.count(Asset.id))
                .group_by(Asset.media_type)
                .order_by(Asset.media_type)
            )
            media_types = {row[0]: row[1] for row in media_type_result}  # type: ignore
            
            # Total size
            total_size = session.execute(
                select(func.sum(Asset.file_size)).where(Asset.file_size.is_not(None))
            ).scalar() or 0
            
            # Last indexed
            last_indexed_str = self._get_metadata(session, "last_indexed")
            last_indexed = None
            if last_indexed_str:
                from datetime import datetime
                last_indexed = datetime.fromtimestamp(float(last_indexed_str))
            
            return DatabaseStats(
                total_assets=total_assets,
                categories=categories,
                file_types=file_types,
                media_types=media_types,
                total_size_bytes=total_size,
                last_indexed=last_indexed
            )
    
    def _get_metadata(self, session: Session, key: str) -> str | None:
        """Get a metadata value."""
        result = session.execute(
            select(DatabaseMetadata.value).where(DatabaseMetadata.key == key)
        )
        row = result.first()
        return row[0] if row else None
    
    def _set_metadata(self, session: Session, key: str, value: str) -> None:
        """Set a metadata value."""
        existing = session.execute(
            select(DatabaseMetadata).where(DatabaseMetadata.key == key)
        ).first()
        
        if existing:
            existing[0].value = value
        else:
            metadata = DatabaseMetadata(key=key, value=value)
            session.add(metadata)


class PatternDatabase:
    """Pattern database operations for library API patterns."""
    
    def __init__(self):
        self.settings = get_settings()
        self.settings.paths.ensure_directories()
        create_all_tables()
    
    def store_patterns(self, patterns_data: dict, library: GameLibrary) -> int:
        """Store discovered patterns in the database."""
        stored_count = 0
        
        with database_session() as session:
            # Clear existing patterns for this library
            session.query(Pattern).filter(Pattern.library == library).delete()
            session.commit()
            
            # Store new patterns
            for opcode, pattern_data in patterns_data.items():
                pattern = Pattern(
                    opcode=opcode,
                    source_function=pattern_data["source"],
                    library=library,
                    category=self._map_category(pattern_data["category"]),
                    signature=pattern_data["signature"],
                    docstring=pattern_data.get("docstring", ""),
                    return_type=pattern_data.get("return_type", "None"),
                    complexity=pattern_data["complexity"],
                    is_beginner_safe=pattern_data["complexity"] <= 3,
                    template_content=pattern_data.get("template", "")
                )
                
                session.add(pattern)
                session.flush()  # Get pattern ID
                
                # Store parameters
                for param_data in pattern_data.get("parameters", []):
                    parameter = PatternParameter(
                        pattern_id=pattern.id,
                        name=param_data["name"],
                        param_type=param_data.get("annotation", "any"),
                        is_required=param_data.get("default") is None,
                        default_value=param_data.get("default"),
                        suggested_asset_type=self._suggest_asset_type(param_data["name"], param_data.get("annotation", ""))
                    )
                    session.add(parameter)
                    
                    # Auto-create asset compatibility based on parameter types
                    asset_types = self._detect_asset_compatibility(param_data["name"], param_data.get("annotation", ""))
                    for asset_type in asset_types:
                        compatibility = PatternAssetCompatibility(
                            pattern_id=pattern.id,
                            asset_file_type=asset_type,
                            parameter_name=param_data["name"],
                            is_required=param_data.get("default") is None
                        )
                        session.add(compatibility)
                
                stored_count += 1
            
            session.commit()
        
        return stored_count
    
    def get_patterns_by_library(self, library: GameLibrary) -> list[Pattern]:
        """Get all patterns for a specific library."""
        with database_session() as session:
            return session.query(Pattern).filter(Pattern.library == library).all()
    
    def get_patterns_by_category(self, category: PatternCategory, library: GameLibrary | None = None) -> list[Pattern]:
        """Get patterns by category, optionally filtered by library."""
        with database_session() as session:
            query = session.query(Pattern).filter(Pattern.category == category)
            if library:
                query = query.filter(Pattern.library == library)
            return query.all()
    
    def get_beginner_safe_patterns(self, library: GameLibrary | None = None) -> list[Pattern]:
        """Get all beginner-safe patterns."""
        with database_session() as session:
            query = session.query(Pattern).filter(Pattern.is_beginner_safe == True)
            if library:
                query = query.filter(Pattern.library == library)
            return query.all()
    
    def get_patterns_for_asset_type(self, asset_type: AssetFileType) -> list[Pattern]:
        """Get patterns compatible with a specific asset type."""
        with database_session() as session:
            return session.query(Pattern).join(PatternAssetCompatibility).filter(
                PatternAssetCompatibility.asset_file_type == asset_type
            ).all()
    
    def _map_category(self, category_str: str) -> PatternCategory:
        """Map string category to PatternCategory enum."""
        mapping = {
            "visual": PatternCategory.VISUAL,
            "sprites": PatternCategory.SPRITES,
            "collision": PatternCategory.COLLISION,
            "audio": PatternCategory.AUDIO,
            "input": PatternCategory.INPUT,
            "motion": PatternCategory.MOTION,
            "game": PatternCategory.GAME,
            "state": PatternCategory.STATE,
            "general": PatternCategory.ADVANCED,
        }
        return mapping.get(category_str, PatternCategory.ADVANCED)
    
    def _suggest_asset_type(self, param_name: str, param_type: str) -> str | None:
        """Suggest asset type based on parameter name and type."""
        name_lower = param_name.lower()
        type_lower = param_type.lower()
        
        # Image-related parameters
        if any(word in name_lower for word in ["texture", "image", "sprite", "path", "file"]) and "str" in type_lower:
            return "image"
        
        # Audio-related parameters  
        if any(word in name_lower for word in ["sound", "audio", "music"]) and "str" in type_lower:
            return "audio"
        
        # Font-related parameters
        if any(word in name_lower for word in ["font", "text"]) and "str" in type_lower:
            return "font"
        
        return None
    
    def _detect_asset_compatibility(self, param_name: str, param_type: str) -> list[AssetFileType]:
        """Detect which asset types are compatible with this parameter."""
        name_lower = param_name.lower()
        type_lower = param_type.lower()
        compatible_types = []
        
        # Image assets
        if any(word in name_lower for word in ["texture", "image", "sprite", "path", "file"]) and "str" in type_lower:
            compatible_types.append(AssetFileType.IMAGE)
        
        # Audio assets
        if any(word in name_lower for word in ["sound", "audio", "music"]) and "str" in type_lower:
            compatible_types.append(AssetFileType.AUDIO)
        
        # Font assets
        if any(word in name_lower for word in ["font"]) and "str" in type_lower:
            compatible_types.append(AssetFileType.FONT)
        
        return compatible_types


class AIContentDatabase:
    """Database operations for AI-generated content with idempotency."""
    
    def __init__(self):
        self.settings = get_settings()
        create_all_tables()
    
    def get_template_rules(self, library: GameLibrary, rules_version: str) -> TemplateRules | None:
        """Get existing template rules for idempotency check."""
        with database_session() as session:
            return session.query(TemplateRules).filter(
                TemplateRules.library == library,
                TemplateRules.rules_version == rules_version,
                TemplateRules.is_approved == True
            ).first()
    
    def store_template_rules(
        self, 
        library: GameLibrary,
        library_version: str,
        rules_version: str,
        template_rules_data: dict,
        is_approved: bool = False,
        approved_by: str | None = None
    ) -> TemplateRules:
        """Store AI-generated template rules."""
        
        with database_session() as session:
            # Check if already exists
            existing = session.query(TemplateRules).filter(
                TemplateRules.library == library,
                TemplateRules.rules_version == rules_version
            ).first()
            
            if existing:
                # Update existing
                existing.library_version = library_version
                existing.category_templates = template_rules_data["category_templates"]
                existing.style_configurations = template_rules_data["style_configurations"]
                existing.function_mappings = template_rules_data["function_mappings"]
                existing.complexity_progression = template_rules_data["complexity_progression"]
                existing.student_customization_points = template_rules_data["student_customization_points"]
                existing.patterns_covered = template_rules_data["patterns_covered"]
                existing.categories_supported = template_rules_data["categories_supported"]
                existing.is_approved = is_approved
                existing.approved_by = approved_by
                template_rules = existing
            else:
                # Create new
                template_rules = TemplateRules(
                    library=library,
                    library_version=library_version,
                    rules_version=rules_version,
                    category_templates=template_rules_data["category_templates"],
                    style_configurations=template_rules_data["style_configurations"],
                    function_mappings=template_rules_data["function_mappings"],
                    complexity_progression=template_rules_data["complexity_progression"],
                    student_customization_points=template_rules_data["student_customization_points"],
                    patterns_covered=template_rules_data["patterns_covered"],
                    categories_supported=template_rules_data["categories_supported"],
                    is_approved=is_approved,
                    approved_by=approved_by
                )
                session.add(template_rules)
            
            session.commit()
            return template_rules
    
    def get_api_analysis_result(self, library: GameLibrary, analysis_version: str) -> APIAnalysisResult | None:
        """Get existing API analysis result for idempotency check."""
        with database_session() as session:
            return session.query(APIAnalysisResult).filter(
                APIAnalysisResult.library == library,
                APIAnalysisResult.analysis_version == analysis_version,
                APIAnalysisResult.is_approved == True
            ).first()
    
    def store_api_analysis_result(
        self,
        library: GameLibrary,
        library_version: str,
        analysis_version: str,
        analysis_data: dict,
        is_approved: bool = False
    ) -> APIAnalysisResult:
        """Store API analysis results for idempotency."""
        
        with database_session() as session:
            # Check if already exists
            existing = session.query(APIAnalysisResult).filter(
                APIAnalysisResult.library == library,
                APIAnalysisResult.analysis_version == analysis_version
            ).first()
            
            if existing:
                # Update existing
                existing.library_version = library_version
                existing.usage_patterns = analysis_data["usage_patterns"]
                existing.pattern_suggestions = analysis_data["pattern_suggestions"]
                existing.lesson_progression_data = analysis_data["lesson_progression"]
                existing.total_functions_analyzed = analysis_data["total_functions_analyzed"]
                existing.patterns_generated = analysis_data["patterns_generated"]
                existing.lessons_designed = len(analysis_data["lesson_progression"])
                existing.pattern_coverage_percent = analysis_data["pattern_coverage_percent"]
                existing.educational_safety_score = analysis_data["educational_safety_score"]
                existing.is_approved = is_approved
                result = existing
            else:
                # Create new
                result = APIAnalysisResult(
                    library=library,
                    library_version=library_version,
                    analysis_version=analysis_version,
                    usage_patterns=analysis_data["usage_patterns"],
                    pattern_suggestions=analysis_data["pattern_suggestions"],
                    lesson_progression_data=analysis_data["lesson_progression"],
                    total_functions_analyzed=analysis_data["total_functions_analyzed"],
                    patterns_generated=analysis_data["patterns_generated"],
                    lessons_designed=len(analysis_data["lesson_progression"]),
                    pattern_coverage_percent=analysis_data["pattern_coverage_percent"],
                    educational_safety_score=analysis_data["educational_safety_score"],
                    ai_model_used=analysis_data.get("ai_model_used", "gpt-4o-mini"),
                    processing_time_seconds=analysis_data.get("processing_time", 0.0),
                    tokens_consumed=analysis_data.get("tokens_consumed"),
                    is_approved=is_approved
                )
                session.add(result)
            
            session.commit()
            return result
    
    def get_curriculum_structure(self, library: GameLibrary, generation_hash: str) -> CurriculumStructure | None:
        """Get existing curriculum structure for idempotency check."""
        with database_session() as session:
            return session.query(CurriculumStructure).filter(
                CurriculumStructure.library == library,
                CurriculumStructure.generation_hash == generation_hash,
                CurriculumStructure.is_approved == True
            ).first()
    
    def store_curriculum_structure(
        self,
        curriculum_data: dict,
        generation_hash: str,
        is_approved: bool = False
    ) -> CurriculumStructure:
        """Store complete curriculum structure."""
        
        with database_session() as session:
            curriculum = CurriculumStructure(
                curriculum_id=curriculum_data["curriculum_id"],
                library=GameLibrary[curriculum_data["library"].upper()],
                title=curriculum_data["title"],
                description=curriculum_data["description"],
                total_lessons=curriculum_data["total_lessons"],
                estimated_total_hours=curriculum_data["estimated_total_hours"],
                progression_type=curriculum_data["progression_type"],
                learning_objectives=curriculum_data["learning_objectives"],
                target_student_age_range=curriculum_data.get("target_student_age_range", (13, 18)),
                generation_parameters=curriculum_data.get("generation_parameters", {}),
                generation_hash=generation_hash,
                patterns_count=curriculum_data.get("patterns_count", 0),
                templates_count=curriculum_data.get("templates_count", 0),
                total_choice_points=curriculum_data.get("total_choice_points", 0),
                is_approved=is_approved
            )
            
            session.add(curriculum)
            session.commit()
            return curriculum
    
    def track_workflow_execution(
        self,
        workflow_type: str,
        workflow_id: str,
        library: GameLibrary,
        input_hash: str,
        autonomous_mode: bool = False
    ) -> WorkflowExecution:
        """Track workflow execution for debugging and idempotency."""
        
        with database_session() as session:
            execution = WorkflowExecution(
                workflow_type=workflow_type,
                workflow_id=workflow_id,
                library=library,
                input_hash=input_hash,
                status="running",
                autonomous_mode=autonomous_mode
            )
            
            session.add(execution)
            session.commit()
            return execution
    
    def update_workflow_status(
        self,
        workflow_id: str,
        status: str,
        current_step: str | None = None,
        output_data: dict | None = None,
        error_message: str | None = None
    ) -> None:
        """Update workflow execution status."""
        
        with database_session() as session:
            execution = session.query(WorkflowExecution).filter(
                WorkflowExecution.workflow_id == workflow_id
            ).first()
            
            if execution:
                execution.status = status
                execution.current_step = current_step
                execution.output_data = output_data
                execution.error_message = error_message
                
                if status in ["completed", "failed", "cancelled"]:
                    execution.completed_at = func.now()
                    if execution.started_at:
                        execution.processing_time_seconds = (
                            execution.completed_at - execution.started_at
                        ).total_seconds()
                
                session.commit()
    
    def get_workflow_by_input_hash(self, input_hash: str, workflow_type: str) -> WorkflowExecution | None:
        """Get existing workflow execution by input hash for idempotency."""
        with database_session() as session:
            return session.query(WorkflowExecution).filter(
                WorkflowExecution.input_hash == input_hash,
                WorkflowExecution.workflow_type == workflow_type,
                WorkflowExecution.status == "completed"
            ).first()


# Global instances
asset_db = AssetDatabase()
pattern_db = PatternDatabase()
ai_content_db = AIContentDatabase()


# ============================================================================
# Index management functions (moved from assets/indexer.py)
# ============================================================================

def check_index_exists() -> None:
    """Check if the asset index has been built."""
    from professor_pixel.assets.errors import IndexNotBuiltError, AssetsDBError
    
    try:
        stats = asset_db.get_stats()
        if stats.total_assets == 0:
            raise IndexNotBuiltError("Asset index is empty. Run rebuild_asset_index() first.")
    except Exception as e:
        raise AssetsDBError(f"Asset index appears corrupted: {e}")


def build_asset_index() -> int:
    """Build the asset index from filesystem using the new database system."""
    return asset_db.index_assets()


def rebuild_asset_index() -> dict[str, str | int]:
    """Force rebuild of the asset index from filesystem."""
    try:
        count = asset_db.index_assets()
        return {"status": "success", "asset_count": count}
    except Exception as e:
        return {"status": "error", "message": str(e), "asset_count": 0}


def get_index_stats() -> dict[str, int]:
    """Get statistics about the current index."""
    stats = asset_db.get_stats()
    return {
        "total_assets": stats.total_assets,
        "categories": len(stats.categories),
        "total_size_bytes": stats.total_size_bytes
    }


# ============================================================================
# Asset search functions (moved from assets/search.py)
# ============================================================================

def search_assets_semantic(query: str, limit: int = 8, category: str | None = None) -> list[dict[str, str | None]]:
    """
    Search for assets using text matching (no more vector search nonsense).
    
    This is the primary tool for finding relevant assets.
    """
    from professor_pixel.models import AssetSearchRequest, AssetSearchFilter
    
    # Create search request using our new models
    filter_obj = AssetSearchFilter(category=category) if category else None
    request = AssetSearchRequest(query=query, limit=limit, filter=filter_obj)
    
    # Search using the database
    result = asset_db.search_assets(request)
    
    # Convert to the expected format for compatibility
    return [
        {
            "path": asset.path,
            "category": asset.category, 
            "subcategory": asset.subcategory,
            "filename": asset.filename,
            "stem": asset.stem,
            "ext": asset.extension,
            "full_path": f"{asset.category}/{asset.path}",  # Approximate full path
            "content": asset.searchable_text
        }
        for asset in result.assets
    ]


def get_asset_categories() -> list[str]:
    """Get all available asset categories."""
    return asset_db.get_categories()


def get_asset_subcategories(category: str) -> list[str]:
    """Get subcategories for a specific category."""
    return asset_db.get_subcategories(category)
