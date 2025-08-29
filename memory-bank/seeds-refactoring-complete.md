# Seeds Subpackage Refactoring Complete

## Date: 2025-08-28

## Summary
Successfully refactored the seeds subpackage following the same ML-first patterns used for entities. The seeds extraction system now uses a unified Sources table and proper ORM for all operations.

## Architecture Changes

### 1. Directory Structure
```
src/generator/db/
└── seeds/           # SELF-CONTAINED (renamed from grammar)
    ├── __init__.py
    ├── types.py     # Seeds enums and types
    ├── protocols.py # Seeds interfaces
    ├── models.py    # Pydantic models (existing)
    ├── orm.py       # Sources, NarrativeSeeds, etc. tables
    ├── constants.py # Seeds constants (updated)
    ├── errors.py    # Custom exceptions
    ├── extractors.py # ML-first extractors from Sources table
    ├── manager.py   # Seeds coordinator
    ├── nltk.py      # NLTK loader (to be refactored)
    ├── books.py     # Books loader (to be refactored)
    ├── linguistic.py # Linguistic loader (to be refactored)
    ├── cleasby.py   # Cleasby loader (to be refactored)
    └── omw.py       # OMW loader (to be refactored)
```

### 2. Database Schema

#### Sources Table (Master table for all loaded data)
- `id`: Primary key
- `source_type`: Type of source (nltk, book, linguistic, etc.)
- `source_name`: Specific corpus/book/dataset name
- `language`: Language code if applicable
- `content_type`: Type of content (text, corpus, lexicon, dataset)
- `raw_content`: Raw text/data
- `processed_content`: Preprocessed version
- `metadata`: JSON metadata
- `processing_status`: Current status (loaded, processed, extracted)
- `extraction_count`: Number of seeds extracted
- `quality_score`: Quality metric (0.0 to 1.0)
- `completeness`: Completeness metric (0.0 to 1.0)

#### Seed Tables
- `NarrativeSeeds`: Story structures and patterns
- `MotifSeeds`: Visual and thematic motifs
- `SemanticSeeds`: Semantic concepts and relationships
- `EmotionalSeeds`: Emotional patterns and progressions
- `LinguisticSeeds`: Linguistic patterns and vocabulary

#### Aggregation Tables
- `SeedClusters`: Clusters of related seeds
- `ExtractionMetrics`: Metrics for extraction runs

### 3. Key Components

#### types.py
- Extended with seed-specific enums
- `SourceType`, `SeedType`, `ExtractionMethod`
- `HorrorStage`, `EmotionalCategory`, `LinguisticPattern`
- Result types: `ExtractionResult`, `ValidationResult`, `ClusteringResult`

#### protocols.py
- `SourceLoader`: Interface for loading data into Sources
- `SeedExtractor`: Interface for extracting seeds
- `MLProcessor`: Interface for ML operations
- `SeedValidator`: Interface for validation
- `SeedAggregator`: Interface for clustering
- `ExtractionPipeline`: Complete pipeline interface

#### errors.py
- Custom exceptions for all failure scenarios
- ML-first with no fallbacks
- Fail-fast semantics
- Helper functions for validation

#### extractors.py
- `UnifiedSeedExtractor`: Main extractor coordinator
- Individual extractors for each seed type
- ALL extractors use ML (no conditionals)
- Query from Sources table
- Write to appropriate seed tables
- Validation and quality checks

#### manager.py
- `SeedsExtractionManager`: Main coordinator
- Load sources functionality
- Preprocessing pipeline
- Extraction orchestration
- Clustering operations
- Query interfaces for other systems
- Metrics and reporting

### 4. ML-First Principles Applied

1. **No Fallbacks**: Everything uses ML, no conditionals
2. **Fail Fast**: Raise exceptions immediately on errors
3. **Unified Source**: All data goes through Sources table
4. **Proper ORM**: SQLModel for all database operations
5. **Self-Contained**: Seeds subpackage has everything it needs
6. **Python 3.13**: Modern Python patterns throughout

### 5. Integration Points

The seeds system integrates with:
- **ML Processor**: Required for all extraction operations
- **Database**: SQLite with SQLModel ORM
- **Entities System**: Can provide seeds for entity generation
- **Game Systems**: Seeds used for content generation

## Testing Approach

To test the refactored system:

1. **Load Sources**: Load NLTK, books, and linguistic data into Sources table
2. **Preprocess**: Clean and prepare content
3. **Extract Seeds**: Use ML to extract all seed types
4. **Cluster**: Group related seeds
5. **Query**: Test various query interfaces
6. **Report**: Generate extraction metrics

## Next Steps

### Still Need Source Loader Refactoring
The following loaders need to be updated to use the Sources table:
- `nltk.py` - Load NLTK data into Sources
- `books.py` - Load book corpus into Sources
- `linguistic.py` - Load linguistic data into Sources
- `cleasby.py` - Load Cleasby dictionary into Sources
- `omw.py` - Load OMW data into Sources

These loaders should:
1. Implement the `SourceLoader` protocol
2. Load data into Sources table
3. Set appropriate metadata
4. Handle preprocessing
5. Mark sources as ready for extraction

## Key Benefits

1. **Unified Data Model**: All source data in one table
2. **ML-First**: No fallbacks or conditionals
3. **Better Organization**: Clear separation of concerns
4. **Scalability**: Can easily add new source types
5. **Query Flexibility**: Rich query interfaces for game systems
6. **Metrics Tracking**: Built-in performance monitoring

## Example Usage

```python
from src.generator.db.seeds.manager import SeedsExtractionManager
from src.generator.db.seeds.types import SourceType, SeedType

# Initialize manager with ML processor
manager = SeedsExtractionManager(
    db_path="game.db",
    ml_processor=ml_processor  # Required
)

# Load sources
manager.load_sources(
    source_type=SourceType.BOOK,
    source_data={
        "dracula": {
            "content": "...",
            "language": "en",
            "metadata": {"author": "Bram Stoker"}
        }
    }
)

# Preprocess
manager.preprocess_sources()

# Extract seeds
results = manager.extract_seeds(
    seed_types=[SeedType.NARRATIVE, SeedType.MOTIF]
)

# Cluster seeds
clustering = manager.cluster_seeds(SeedType.NARRATIVE)

# Query for horror stage
horror_seeds = manager.get_seeds_for_horror_stage(
    horror_stage=2,  # Dread level
    limit=10
)

# Generate report
report = manager.generate_summary_report()
```

## Conclusion

The seeds subpackage has been successfully refactored to follow ML-first principles with proper ORM and fail-fast semantics. The system is now ready for integration with the game's content generation pipeline.
