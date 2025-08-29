# Dragon's Labyrinth Manager Patterns & Coding Standards

## Global Coding Standards (Enforced Globally)

### 1. No Relative Imports - Always Use Absolute Imports
```python
# ❌ WRONG - Relative imports
from .types import SomeType
from ..base import BasePipeline

# ✅ CORRECT - Absolute imports
from dragons_labyrinth.psychology.types import SomeType
from dragons_labyrinth.base import BasePipeline
```

### 2. No Optional Types - Use Union Syntax
```python
# ❌ WRONG - Optional
from typing import Optional
value: Optional[str] = None

# ✅ CORRECT - Union with None
value: str | None = None
```

### 3. No Uppercase Types - Use Lowercase Built-In Types
```python
# ❌ WRONG - Uppercase types
from typing import Dict, List, Any
data: Dict[str, List[Any]] = {}

# ✅ CORRECT - Lowercase built-in types
data: dict[str, list[any]] = {}
```

### 4. Subpackage Structure Standards
When building new subpackages, follow this exact structure:

```
src/dragons_labyrinth/new_subpackage/
├── types.py          # Enums with auto() - all type definitions
├── models.py         # Pydantic-2 models using types from types.py  
├── constants.py      # All hardcoded values to keep code clean
├── manager.py        # Manager following established pattern
├── extractors/       # Processing components if needed
└── __init__.py       # Clean exports
```

**types.py Pattern:**
```python
from enum import Enum, auto

class MyEnum(Enum):
    VALUE_ONE = auto()
    VALUE_TWO = auto()
    VALUE_THREE = auto()
```

**models.py Pattern:**
```python
from pydantic import BaseModel, Field
from dragons_labyrinth.new_subpackage.types import MyEnum

class MyModel(BaseModel):
    name: str = Field(description="Model name")
    enum_field: MyEnum = Field(description="Model type")
```

## Core Manager Architecture Philosophy

Dragon's Labyrinth uses a **Manager Pattern** across all subpackages to coordinate pipeline execution and cross-pipeline data sharing. This pattern ensures clean separation of concerns, proper dependency injection, and consistent Entity storage patterns.

### Manager Dependency Chain (Critical Sequence)
```
DatabaseManager (creates unified game.db)
    ↓ provides db_manager to
DataManager (collects external data sources)
    ↓ db_manager contains raw data for
SeedsManager (processes patterns from data)  
    ↓ db_manager contains processed seeds for
PsychologyManager (generates stable rules from seeds + data)
    ↓ db_manager contains psychology for
WorldBuildingManager (creates psychologically-informed content)
```

**Why This Sequence Matters:**
- Each manager provides specialized APIs for accessing their data
- Managers use dependency injection to access earlier pipeline results
- No manager creates their own DatabaseManager (only DatabaseManager does this)
- Cross-pipeline queries go through manager APIs, not direct database queries

## Manager Pattern Standards

### 1. Manager Class Structure

All managers must follow this exact pattern:

```python
class XxxManager(BasePipeline):
    """Manager for the Xxx pipeline."""
    
    def __init__(self):
        super().__init__("xxx_manager")  # Pipeline identifier
        
        # Initialize extractors/processors (NO external managers)
        self.extractor_a = ExtractorA()
        self.extractor_b = ExtractorB()
    
    def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs) -> dict[str, Any]:
        """
        Execute the main pipeline logic.
        
        Args:
            db_manager: DatabaseManager instance (injected dependency)
            **kwargs: Pipeline-specific parameters
            
        Returns:
            Dictionary with results, statistics, and metadata
        """
        # Pipeline execution logic here
        pass
    
    # Public API methods for cross-pipeline integration
    def get_xxx_for_yyy(self, db_manager: DatabaseManager, context: str) -> Any:
        """Public interface method for other pipelines."""
        pass
```

### 2. Critical Rules

**❌ NEVER CREATE YOUR OWN DatabaseManager:**
```python
# ❌ WRONG - Do not create your own db_manager
def __init__(self):
    super().__init__("my_manager")
    self.db_manager = DatabaseManager()  # BAD!

# ✅ CORRECT - Receive db_manager as parameter
def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs):
    # Use the injected db_manager
    db_manager.store_entity(...)
```

**✅ Always Follow Pipeline Method Signature:**
```python
def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs) -> dict[str, Any]:
    """EXACT signature required for pipeline coordination."""
```

**✅ Public API Methods Take db_manager Parameter:**
```python
def get_data_for_other_pipeline(self, db_manager: DatabaseManager, filter_param: str) -> Any:
    """All public interface methods receive db_manager."""
    entity = db_manager.get_entity("my_data")
    return process(entity, filter_param)
```

### 3. Entity Storage vs Flat Files

#### Use Entity Storage (DatabaseManager) When:
- **Cross-pipeline data sharing** (seeds → psychology → world_building)
- **Structured data** that other pipelines need to query
- **Idempotency tracking** for expensive operations
- **Versioned data** that changes over time
- **Relational data** with complex queries

```python
# ✅ Entity storage for cross-pipeline data
db_manager.store_entity(
    entity_id="seeds_bundle",
    entity_type="seeds_bundle", 
    entity_data=bundle.model_dump(),
    source_pipeline="seeds_manager",
    entity_name="Complete Seeds Bundle"
)
```

#### Use Flat Files When:
- **Output artifacts** for human consumption (reports, CSVs)
- **Debug information** and analysis results  
- **Final deliverables** that don't need querying
- **Large binary data** (images, audio) that just need file paths

```python
# ✅ Flat files for output artifacts
output_path = self.output_dir / "analysis_report.json"
with open(output_path, 'w') as f:
    json.dump(analysis_results, f, indent=2)
```

## Established Manager Examples

### 1. DatabaseManager (Database Creation)

**Role**: Creates the SQLite database and provides interface for all other managers.

**Pattern**:
- `_execute_pipeline(self, tracker: None, **kwargs)` - tracker is None because IT IS the storage system
- Creates its own database connection (only manager allowed to do this)
- Provides `store_entity()`, `get_entity()` methods for all other managers

```python
class DatabaseManager(BasePipeline):
    def __init__(self, db_path: Path | None = None):
        super().__init__("database")
        self.db_path = db_path or GAME_DB_PATH
        self.engine = create_engine(f"sqlite:///{self.db_path}")
        self.SessionLocal = sessionmaker(bind=self.engine)
        self._setup_database()
    
    def _execute_pipeline(self, tracker: None, **kwargs) -> dict[str, Any]:
        # Database setup and asset scanning
        return {"database_state": state, "scan_result": result}
    
    def store_entity(self, entity_id: str, entity_type: str, entity_data: dict, source_pipeline: str, **context) -> bool:
        # Entity storage with idempotency checking
```

### 2. SeedsManager (Proper Manager Pattern)

**Role**: Coordinates seed extractors and provides processed patterns to other pipelines.

**Pattern**:
- `_execute_pipeline(self, db_manager: DatabaseManager, **kwargs)` - receives db_manager
- Uses injected db_manager for all database operations
- Public API methods take db_manager parameter

```python
class SeedsManager(BasePipeline):
    def __init__(self):
        super().__init__("seeds_manager")
        # Initialize extractors only (NO external managers)
        self.motif_extractor = MotifExtractor()
        self.semantic_extractor = SemanticExtractor()
    
    def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs) -> dict[str, Any]:
        # Use injected db_manager
        bundle_updated = db_manager.store_entity(...)
        return {"seeds_bundle": bundle, "statistics": stats}
    
    def get_seeds_bundle(self, db_manager: DatabaseManager) -> SeedsBundle | None:
        """Public API for other pipelines."""
        entity = db_manager.get_entity("seeds_bundle")
        return SeedsBundle.model_validate(entity.entity_data) if entity else None
```

### 3. DataManager (Resource Collection)

**Role**: Downloads and processes external data sources for the seeds pipeline.

**Pattern**:
- `_execute_pipeline(self, db_manager: DatabaseManager, **kwargs)` - receives db_manager
- Coordinates data source collection
- Stores results via injected db_manager

```python
class DataManager(BasePipeline):
    def __init__(self):
        super().__init__("data")
        self.books_source = BooksDataSource()  # Internal data sources only
    
    def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs) -> dict[str, Any]:
        books_result = self.books_source.build_corpus()
        
        # Store via injected db_manager
        db_manager.store_entity(
            entity_id="books_corpus",
            entity_type="books_corpus",
            entity_data=books_result,
            source_pipeline="data"
        )
        
        return {"data_bundle": bundle, "statistics": stats}
```

### 4. PsychologyManager (Multi-Dependency Manager)

**Role**: Generates stable psychology rules using seeds and data manager APIs.

**Critical Pattern** - Uses multiple manager dependencies:
- `_execute_pipeline(self, db_manager, seeds_manager, data_manager, **kwargs)`
- Uses specialized manager APIs instead of direct database queries
- Demonstrates proper cross-manager integration

```python
class PsychologyManager(BasePipeline):
    def __init__(self):
        super().__init__("psychology_manager")
        self.prompt_builder = PsychologyPromptBuilder()  # Internal only
    
    def _execute_pipeline(self, db_manager: DatabaseManager, seeds_manager, data_manager, **kwargs) -> dict[str, Any]:
        # ✅ CORRECT - Use manager APIs
        seeds_bundle = seeds_manager.get_seeds_bundle(db_manager)
        # TODO: data_context = data_manager.get_books_corpus(db_manager)
        
        # Process and store results
        psychology_system = self._generate_psychology_system(seeds_bundle)
        
        db_manager.store_entity(
            entity_id="psychology_system",
            entity_type="psychology_system", 
            entity_data=psychology_system.model_dump(),
            source_pipeline="psychology_manager"
        )
        
        return {"psychology_system": psychology_system, "statistics": stats}
    
    def get_psychology_for_world_builder(self, db_manager: DatabaseManager, world_act: int, region_id: str) -> dict[str, any]:
        """Public API for world building integration."""
        stored = db_manager.get_entity("psychology_system")
        if stored:
            psychology_system = PsychologySystem.model_validate(stored.entity_data)
            return psychology_system.get_psychology_for_context(world_act, region_id)
        return {}
```

## Orchestrator Integration Pattern

The orchestrator coordinates all managers in the correct dependency order:

```python
class PipelineOrchestrator:
    def execute(self) -> dict[str, Any]:
        # Correct dependency sequence
        try:
            # 1. Data collection (raw corpus)
            if self.config.run_data:
                self.results["data"] = self.run_data_pipeline()
            
            # 2. Seeds (processes patterns from data)  
            if self.config.run_seeds:
                self.results["seeds"] = self.run_seeds_pipeline()
            
            # 3. Psychology (generates rules from seeds + data)
            if self.config.run_psychology:
                self.results["psychology"] = self.run_psychology_pipeline()
            
            # 4. World generation (uses psychology context)
            if self.config.run_world:
                self.results["world"] = self.run_world_pipeline()
```

**Psychology Pipeline Integration:**
```python
def run_psychology_pipeline(self) -> dict[str, Any]:
    # Ensure all dependencies are available
    if not self.data_manager:
        self.data_manager = DataManager()
    if not self.seeds_manager:
        self.seeds_manager = SeedsManager()
    
    # Initialize psychology manager
    if not self.psychology_manager:
        self.psychology_manager = PsychologyManager()
    
    # Execute with all dependencies
    results = self.psychology_manager._execute_pipeline(
        db_manager=self.db_manager,
        seeds_manager=self.seeds_manager,
        data_manager=self.data_manager
    )
    
    return results
```

## Cross-Pipeline Integration Patterns

### Manager Dependency Chain

```
DatabaseManager (creates database)
    ↓ provides db_manager to
DataManager (collects external data)
    ↓ db_manager contains data for
SeedsManager (processes patterns from data)  
    ↓ db_manager contains seeds for
PsychologyManager (generates stable rules from seeds)
    ↓ db_manager contains psychology for
WorldBuildingManager (creates informed content)
```

### Public API Pattern

All managers provide public API methods for other pipelines:

```python
class XxxManager(BasePipeline):
    # Public API methods that other pipelines call
    def get_xxx_for_yyy(self, db_manager: DatabaseManager, context: str) -> Any:
        """Get specific data for another pipeline."""
        entity = db_manager.get_entity("xxx_data")
        return filter_and_transform(entity, context)
    
    def get_xxx_by_theme(self, db_manager: DatabaseManager, theme: str) -> list[Any]:
        """Get filtered data by theme."""
        bundle = self._get_bundle(db_manager)
        return [item for item in bundle if matches_theme(item, theme)]
```

### Entity Naming Conventions

- **entity_id**: Descriptive identifier (`"seeds_bundle"`, `"psychology_rules"`, `"hex_tiles_data"`)
- **entity_type**: Category identifier (`"seeds_bundle"`, `"psychology_system"`, `"hbf_analysis"`) 
- **source_pipeline**: Manager that created it (`"seeds_manager"`, `"psychology"`, `"hbf_analyzer"`)
- **entity_name**: Human-readable name (`"Complete Seeds Bundle"`, `"Psychology Rules"`)

## Manager Creation Checklist

When creating a new manager, ensure:

- [ ] Inherits from `BasePipeline`
- [ ] Constructor: `super().__init__("manager_name")`
- [ ] Has `_execute_pipeline(self, db_manager: DatabaseManager, **kwargs) -> dict[str, Any]` method
- [ ] Does NOT create its own DatabaseManager instance
- [ ] Uses injected db_manager for all database operations
- [ ] Public API methods take db_manager as parameter
- [ ] Stores results using `db_manager.store_entity()` with proper naming
- [ ] Returns results dictionary with statistics and metadata
- [ ] Follows Entity storage for cross-pipeline data, flat files for output artifacts

## Common Manager Pattern Issues

### ❌ Anti-Patterns

1. **Creating Own DatabaseManager:**
   ```python
   def __init__(self):
       self.db_manager = DatabaseManager()  # BAD!
   ```

2. **Wrong Method Signature:**
   ```python
   def run(self) -> dict:  # BAD! Should be _execute_pipeline
   ```

3. **Missing db_manager Parameter:**
   ```python
   def get_data(self) -> Any:  # BAD! Missing db_manager parameter
   ```

4. **Using Flat Files for Cross-Pipeline Data:**
   ```python
   with open("data.json", "w") as f:  # BAD for cross-pipeline data
       json.dump(data, f)
   ```

### ✅ Correct Patterns

1. **Dependency Injection:**
   ```python
   def _execute_pipeline(self, db_manager: DatabaseManager, **kwargs):
       # Use injected db_manager
   ```

2. **Public API with DatabaseManager:**
   ```python
   def get_data_for_other_pipeline(self, db_manager: DatabaseManager):
       return db_manager.get_entity("my_data")
   ```

3. **Entity Storage for Cross-Pipeline Data:**
   ```python
   db_manager.store_entity(
       entity_id="my_bundle",
       entity_type="my_type",
       entity_data=data.model_dump(),
       source_pipeline="my_manager"
   )
   ```

This pattern ensures clean architecture, proper dependency management, and reliable cross-pipeline data sharing throughout Dragon's Labyrinth.
