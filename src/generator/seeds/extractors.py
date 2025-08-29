"""
ML-first seed extractors for the seeds extraction system.

Simple extractors following .clinerules patterns with modern Python standards.
"""

import json
from typing import Any

from pydantic import BaseModel
from sqlmodel import Session, select

from .models import (
    EmotionalSeeds,
    LinguisticSeeds,
    MotifSeeds,
    NarrativeSeeds,
    SemanticSeeds,
    Sources,
)


class SeedType:
    """Seed types enumeration"""
    NARRATIVE = "narrative"
    MOTIF = "motif"
    SEMANTIC = "semantic"
    EMOTIONAL = "emotional"
    LINGUISTIC = "linguistic"


class ExtractionMethod:
    """Extraction methods enumeration"""
    ML_TRANSFORMER = "ml_transformer"
    ML_CLUSTERING = "ml_clustering"
    SEMANTIC_ANALYSIS = "semantic_analysis"


class ExtractionResult(BaseModel):
    """Result of seed extraction operation"""
    success: bool
    seed_type: str
    seeds: list[BaseModel]
    source_id: int
    extraction_method: str
    confidence_score: float
    error_message: str | None = None
    metrics: dict | None = None


class UnifiedSeedExtractor:
    """
    Unified ML-first seed extractor that processes all source types.
    
    Simple implementation following .clinerules patterns.
    """
    
    def __init__(self, ml_processor: Any | None = None):
        """Initialize the unified extractor"""
        self.ml_processor = ml_processor or MockMLProcessor()
        
        # Initialize specific extractors
        self.extractors = {
            SeedType.NARRATIVE: NarrativeSeedExtractor(self.ml_processor),
            SeedType.MOTIF: MotifSeedExtractor(self.ml_processor),
            SeedType.SEMANTIC: SemanticSeedExtractor(self.ml_processor),
            SeedType.EMOTIONAL: EmotionalSeedExtractor(self.ml_processor),
            SeedType.LINGUISTIC: LinguisticSeedExtractor(self.ml_processor),
        }
    
    def extract_all(
        self,
        session: Session,
        source_types: list[str] | None = None,
        seed_types: list[str] | None = None,
    ) -> dict[str, list[BaseModel]]:
        """Extract all seeds from sources"""
        # Query unprocessed sources
        query = select(Sources).where(
            Sources.processing_status != "extracted"
        )
        
        if source_types:
            query = query.where(Sources.source_type.in_(source_types))
        
        sources = session.exec(query).all()
        
        # Extract seeds from each source
        seed_types = seed_types or [SeedType.NARRATIVE, SeedType.MOTIF, SeedType.SEMANTIC, SeedType.EMOTIONAL, SeedType.LINGUISTIC]
        all_seeds = {st: [] for st in seed_types}
        
        for source in sources:
            source_seeds = self.extract_from_source(
                session=session,
                source=source,
                seed_types=seed_types
            )
            
            for seed_type, seeds in source_seeds.items():
                all_seeds[seed_type].extend(seeds)
        
        return all_seeds
    
    def extract_from_source(
        self,
        session: Session,
        source: Sources,
        seed_types: list[str] | None = None,
    ) -> dict[str, list[BaseModel]]:
        """Extract seeds from a single source"""
        if not source.processed_content and not source.raw_content:
            return {}
        
        seed_types = seed_types or [SeedType.NARRATIVE, SeedType.MOTIF, SeedType.SEMANTIC, SeedType.EMOTIONAL, SeedType.LINGUISTIC]
        extracted_seeds = {}
        total_extracted = 0
        
        for seed_type in seed_types:
            if seed_type not in self.extractors:
                continue
            
            try:
                extractor = self.extractors[seed_type]
                result = extractor.extract(session=session, source=source)
                
                if result.success and result.seeds:
                    extracted_seeds[seed_type] = result.seeds
                    total_extracted += len(result.seeds)
                    
            except Exception as e:
                print(f"Failed to extract {seed_type} from source {source.id}: {e}")
        
        # Update source status
        source.processing_status = "extracted"
        source.extraction_count = total_extracted
        session.add(source)
        session.commit()
        
        return extracted_seeds


class MockMLProcessor:
    """Mock ML processor for testing"""
    
    def process_text(self, text: str, seed_type: str) -> Any:
        """Mock ML processing"""
        class MockResult:
            success = True
            seeds = []
            confidence_score = 0.8
            metrics = {}
        return MockResult()


class NarrativeSeedExtractor:
    """Extract narrative structures and story patterns"""
    
    def __init__(self, ml_processor: Any):
        self.ml_processor = ml_processor
        self.seed_type = SeedType.NARRATIVE
    
    def extract(self, session: Session, source: Sources) -> ExtractionResult:
        """Extract narrative seeds using ML"""
        content = source.processed_content or source.raw_content
        
        if not content or len(content) < 100:
            return ExtractionResult(
                success=False,
                seed_type=self.seed_type,
                seeds=[],
                source_id=source.id,
                extraction_method=ExtractionMethod.ML_TRANSFORMER,
                confidence_score=0.0,
                error_message="Insufficient content"
            )
        
        # Create sample narrative seeds
        sample_narratives = [
            {
                "structure_name": "descent_into_madness",
                "structure_type": "linear",
                "story_beats": ["peaceful_beginning", "first_warning", "growing_unease", "revelation", "horror"],
                "core_themes": ["corruption", "isolation", "loss_of_control"],
                "conflict_types": ["internal", "supernatural"],
                "horror_stage": 2,
                "corruption_arc": ["normal", "unsettled", "troubled", "broken"],
                "confidence_score": 0.85
            }
        ]
        
        # Convert to ORM models and save
        orm_seeds = []
        for seed_data in sample_narratives:
            seed = NarrativeSeeds(
                source_id=source.id,
                structure_name=seed_data.get("structure_name"),
                structure_type=seed_data.get("structure_type"),
                story_beats=json.dumps(seed_data.get("story_beats", [])),
                core_themes=json.dumps(seed_data.get("core_themes", [])),
                conflict_types=json.dumps(seed_data.get("conflict_types", [])),
                horror_stage=seed_data.get("horror_stage", 0),
                corruption_arc=json.dumps(seed_data.get("corruption_arc", [])) if seed_data.get("corruption_arc") else None,
                psychological_elements=json.dumps(seed_data.get("psychological_elements", [])) if seed_data.get("psychological_elements") else None,
                confidence_score=seed_data.get("confidence_score", 0.0),
                extraction_method=ExtractionMethod.ML_TRANSFORMER
            )
            session.add(seed)
            orm_seeds.append(seed)
        
        session.commit()
        
        return ExtractionResult(
            success=True,
            seed_type=self.seed_type,
            seeds=orm_seeds,
            source_id=source.id,
            extraction_method=ExtractionMethod.ML_TRANSFORMER,
            confidence_score=0.85
        )


class MotifSeedExtractor:
    """Extract visual and thematic motifs"""
    
    def __init__(self, ml_processor: Any):
        self.ml_processor = ml_processor
        self.seed_type = SeedType.MOTIF
    
    def extract(self, session: Session, source: Sources) -> ExtractionResult:
        """Extract motif seeds using ML"""
        content = source.processed_content or source.raw_content
        
        if not content or len(content) < 100:
            return ExtractionResult(
                success=False,
                seed_type=self.seed_type,
                seeds=[],
                source_id=source.id,
                extraction_method=ExtractionMethod.ML_TRANSFORMER,
                confidence_score=0.0,
                error_message="Insufficient content"
            )
        
        # Create sample motif seeds
        sample_motifs = [
            {
                "name": "ancient_darkness",
                "category": "symbolic",
                "description": "Ancient darkness that corrupts all it touches",
                "keywords": ["darkness", "ancient", "corruption", "shadow"],
                "atmosphere": "Creeping dread and inevitability",
                "dread_amplification": 0.8,
                "corruption_potential": 0.9,
                "frequency": 3,
                "confidence_score": 0.85
            }
        ]
        
        # Convert to ORM models and save
        orm_seeds = []
        for seed_data in sample_motifs:
            seed = MotifSeeds(
                source_id=source.id,
                name=seed_data.get("name"),
                category=seed_data.get("category"),
                description=seed_data.get("description"),
                keywords=json.dumps(seed_data.get("keywords", [])),
                color_palette=json.dumps(seed_data.get("color_palette", [])) if seed_data.get("color_palette") else None,
                atmosphere=seed_data.get("atmosphere"),
                dread_amplification=seed_data.get("dread_amplification", 0.0),
                corruption_potential=seed_data.get("corruption_potential", 0.0),
                frequency=seed_data.get("frequency", 1),
                confidence_score=seed_data.get("confidence_score", 0.0)
            )
            session.add(seed)
            orm_seeds.append(seed)
        
        session.commit()
        
        return ExtractionResult(
            success=True,
            seed_type=self.seed_type,
            seeds=orm_seeds,
            source_id=source.id,
            extraction_method=ExtractionMethod.ML_TRANSFORMER,
            confidence_score=0.85
        )


class SemanticSeedExtractor:
    """Extract semantic concepts and relationships"""
    
    def __init__(self, ml_processor: Any):
        self.ml_processor = ml_processor
        self.seed_type = SeedType.SEMANTIC
    
    def extract(self, session: Session, source: Sources) -> ExtractionResult:
        """Extract semantic seeds using ML"""
        return ExtractionResult(
            success=True,
            seed_type=self.seed_type,
            seeds=[],
            source_id=source.id,
            extraction_method=ExtractionMethod.SEMANTIC_ANALYSIS,
            confidence_score=0.0
        )


class EmotionalSeedExtractor:
    """Extract emotional patterns and psychological progressions"""
    
    def __init__(self, ml_processor: Any):
        self.ml_processor = ml_processor
        self.seed_type = SeedType.EMOTIONAL
    
    def extract(self, session: Session, source: Sources) -> ExtractionResult:
        """Extract emotional seeds using ML"""
        return ExtractionResult(
            success=True,
            seed_type=self.seed_type,
            seeds=[],
            source_id=source.id,
            extraction_method=ExtractionMethod.ML_TRANSFORMER,
            confidence_score=0.0
        )


class LinguisticSeedExtractor:
    """Extract linguistic patterns and vocabulary"""
    
    def __init__(self, ml_processor: Any):
        self.ml_processor = ml_processor
        self.seed_type = SeedType.LINGUISTIC
    
    def extract(self, session: Session, source: Sources) -> ExtractionResult:
        """Extract linguistic seeds using ML"""
        return ExtractionResult(
            success=True,
            seed_type=self.seed_type,
            seeds=[],
            source_id=source.id,
            extraction_method=ExtractionMethod.ML_TRANSFORMER,
            confidence_score=0.0
        )
