# Dragons Labyrinth Database Layer

Modern SQLModel-based database management with comprehensive type safety and Python 3.13 features.

## Overview

The database layer provides unified game.db management using SQLModel, combining SQLAlchemy ORM with Pydantic validation in a single framework. This modernization includes comprehensive type safety, intelligent validation, and Python 3.13 pattern matching.

## Architecture

```
src/dragons_labyrinth/db/
├── types.py          # Modern enums with auto() and type aliases
├── protocols.py      # Runtime checkable protocols for type safety
├── models.py         # SQLModel tables with comprehensive validation
├── manager.py        # Modern database manager with protocol compliance
└── README.md         # This documentation
```

## Key Features

### 🚀 SQLModel Integration
- **Unified Models**: Single model definitions serve both SQLAlchemy ORM and Pydantic validation
- **Automatic Validation**: Built-in field validation with custom validators
- **Type Safety**: Full type checking with enum constraints and protocol compliance

### 🔧 Python 3.13 Modernizations
- **Enhanced Enums**: Using `enum.auto()` with smart factory methods
- **Match-Case Patterns**: Modern pattern matching instead of if/elif chains
- **Runtime Protocols**: `@runtime_checkable` protocols for type safety
- **Unicode-Safe Comparisons**: `casefold()` instead of `lower()` for proper string handling

### 📊 Three Core Tables

#### FileRecord
Idempotency tracking with validated hashes and safe paths:
```python
FileRecord(
    key="pipeline_stage_identifier",
    hash="sha256_content_hash",
    file_path="relative/safe/path.txt",
    pipeline_stage=PipelineStage.PROCESSING
)
```

#### EntityRecord  
All pipeline data with enum validation and consistency checks:
```python
EntityRecord(
    entity_id="biome_dark_forest",
    entity_type=EntityType.BIOME_SPEC,
    entity_data={"terrain": "forest", "climate": "temperate"},
    dread_level=DreadLevel.TERRIFYING,
    corruption_stage=CorruptionStage.WITHERED
)
```

#### AssetRecord
Scanned assets with type-specific validation:
```python
AssetRecord(
    asset_id="character_npc_merchant",
    file_path="sprites/characters/merchant.png",
    asset_type=AssetType.IMAGE,
    asset_category=AssetCategory.CHARACTER,
    image_width=64,
    image_height=64,
    dread_level=DreadLevel.PEACEFUL
)
```

## Type System

### Smart Enums with Validation

```python
from dragons_labyrinth.db.types import DreadLevel, AssetType, CorruptionStage

# Enum factory methods with match-case patterns
dread = DreadLevel.from_value(3)  # DreadLevel.HORRIFYING
asset_type = AssetType.from_string("image")  # AssetType.IMAGE
corruption = CorruptionStage.from_string("scorched")  # CorruptionStage.SCORCHED

# Unicode-safe string comparison
if asset_name.casefold() == "special_item".casefold():
    # Handles international characters properly
    pass
```

### Runtime Checkable Protocols

```python
from dragons_labyrinth.db.protocols import DatabaseManagerProtocol

def process_database(db: DatabaseManagerProtocol) -> None:
    """Function accepts any object implementing the protocol"""
    entities = db.get_entities_by_type("biome_spec")
    # Type checker knows this returns list[EntityRecord]
```

### Type-Safe Aliases

```python
from dragons_labyrinth.db.types import EntityId, ContentHash, FilePath

def store_entity(entity_id: EntityId, data_hash: ContentHash) -> None:
    """Clear semantic meaning with type safety"""
    pass
```

## Comprehensive Validation

### Field-Level Validation
Models include extensive validation rules:

```python
# Image assets require dimensions
AssetRecord(
    asset_type=AssetType.IMAGE,
    image_width=512,  # Required for images
    image_height=512,  # Required for images
    # duration_seconds=None  # Not allowed for images
)

# Audio assets require duration
AssetRecord(
    asset_type=AssetType.AUDIO,
    duration_seconds=45.2,  # Required for audio
    # image_width=None  # Not allowed for audio
)
```

### Cross-Field Validation
Root validators ensure consistency:

```python
# Entity type-category consistency
EntityRecord(
    entity_type=EntityType.NPC_SPEC,
    category="characters"  # Valid combination
    # category="world"  # Would raise ValidationError
)

# Asset type-category compatibility  
AssetRecord(
    asset_type=AssetType.MODEL_3D,
    asset_category=AssetCategory.CHARACTER  # Valid
    # asset_category=AssetCategory.AUDIO  # Would raise ValidationError
)
```

### Hash and Path Validation
Built-in security and format checking:

```python
FileRecord(
    hash="a1b2c3d4...",  # Must be valid SHA256 hex (64 chars)
    file_path="safe/relative/path.txt"  # No absolute paths or ".."
)
```

## Usage Examples

### Basic Database Operations

```python
from dragons_labyrinth.db.manager import DatabaseManager
from dragons_labyrinth.db.types import EntityType, DreadLevel

# Initialize database
db = DatabaseManager()

# Store entity with automatic validation
success = db.store_entity(
    entity_id="dark_forest_01",
    entity_type=EntityType.BIOME_SPEC,
    entity_data={
        "name": "Dark Forest",
        "terrain": "forest",
        "danger_level": "high"
    },
    source_pipeline="world_generation",
    dread_level=DreadLevel.TERRIFYING,
    corruption_stage="withered"  # Auto-converted to enum
)

# Retrieve validated entities
entity = db.get_entity("dark_forest_01")
# entity is EntityRecord with all validation applied
print(f"Dread level: {entity.dread_level}")  # DreadLevel.TERRIFYING
```

### Asset Management

```python
# Register asset with validation
db.register_asset(
    asset_id="forest_tile_withered",
    file_path="biomes/forest/withered.png",
    asset_name="Withered Forest Tile",
    asset_type="image",  # Auto-converted to AssetType.IMAGE
    asset_category="biome",
    image_width=256,
    image_height=256,
    dread_level=2,  # Auto-converted to DreadLevel.TERRIFYING
    corruption_stage="withered"
)

# Query assets with type safety
withered_assets = db.get_assets_by_context(
    dread_level=DreadLevel.TERRIFYING,
    corruption_stage=CorruptionStage.WITHERED
)
```

### File Tracking

```python
# Idempotency tracking with validation
db.store_file_record(
    key="world_gen_forest_biomes",
    content_hash="a1b2c3d4e5f6...",  # Validated SHA256
    file_path="generated/biomes/forests.json",
    pipeline_stage=PipelineStage.GENERATION
)

# Check if processing needed
if not db.file_unchanged("world_gen_forest_biomes", new_hash):
    # File changed, reprocess needed
    process_biomes()
```

## Error Handling

The models provide clear validation errors:

```python
from pydantic import ValidationError

try:
    # Invalid: image asset with duration
    asset = AssetRecord(
        asset_id="test",
        file_path="test.png", 
        asset_name="Test",
        asset_type=AssetType.IMAGE,
        duration_seconds=10.0  # Invalid for images
    )
except ValidationError as e:
    print(e.errors())
    # [{'msg': 'Image assets should not have duration', 'type': 'value_error'}]
```

## Migration from Legacy

### From SQLAlchemy + Pydantic
```python
# OLD: Separate models
class BiomeORM(Base):
    __tablename__ = "biomes"
    id = Column(Integer, primary_key=True)
    name = Column(String)

class BiomeModel(BaseModel):
    id: int
    name: str

# NEW: Unified SQLModel
class BiomeRecord(SQLModel, table=True):
    __tablename__ = "biomes"
    id: int | None = Field(primary_key=True)
    name: str = Field(min_length=1)
```

### From hasattr/getattr Patterns
```python
# OLD: Runtime attribute checking
if hasattr(entity, "dread_level"):
    level = getattr(entity, "dread_level", 0)

# NEW: Type-safe access
entity: EntityRecord = db.get_entity(entity_id)
level: DreadLevel = entity.dread_level  # Always typed, validated
```

### From String-Based Enums
```python
# OLD: String comparisons
if asset.asset_type.lower() == "image":
    process_image(asset)

# NEW: Enum matching with pattern matching
match asset.asset_type:
    case AssetType.IMAGE:
        process_image(asset)
    case AssetType.AUDIO:
        process_audio(asset)
    case _:
        handle_unknown(asset)
```

## Performance Considerations

### Lazy Loading
SQLModel maintains SQLAlchemy's lazy loading capabilities:

```python
# Relationships loaded on demand
entity = db.get_entity("forest_01")
# entity.dependent_entities loads only when accessed
```

### Validation Overhead
- Validation occurs at model creation/update
- Database queries return pre-validated instances
- Use `model_validate()` for external data validation

### Enum Conversions
- Factory methods cache enum instances
- String-to-enum conversion is optimized
- Match-case patterns compile to efficient jump tables

## Development Patterns

### Adding New Enums
```python
class NewGameEnum(str, Enum):
    """New game concept with auto values"""
    OPTION_A = auto()
    OPTION_B = auto()
    OPTION_C = auto()
    
    @classmethod
    def from_string(cls, value: str) -> NewGameEnum:
        """Convert string with casefold for Unicode safety"""
        normalized = value.casefold().replace(" ", "_")
        
        match normalized:
            case x if x.startswith("option_a"):
                return cls.OPTION_A
            case x if x.startswith("option_b"): 
                return cls.OPTION_B
            case _:
                return cls.OPTION_C
```

### Adding Validation
```python
@validator("new_field")
def validate_new_field(cls, v: str) -> str:
    """Add custom validation logic"""
    if not v or len(v.strip()) == 0:
        raise ValueError("Field cannot be empty")
    return v.strip()
    
@root_validator
def validate_consistency(cls, values):
    """Cross-field validation"""
    field_a = values.get("field_a")
    field_b = values.get("field_b") 
    
    if field_a and field_b and incompatible(field_a, field_b):
        raise ValueError("Fields A and B are incompatible")
    
    return values
```

## Testing

### Model Validation Testing
```python
import pytest
from pydantic import ValidationError

def test_asset_validation():
    """Test asset model validation"""
    # Valid asset
    asset = AssetRecord(
        asset_id="test_image",
        file_path="test.png",
        asset_name="Test Image",
        asset_type=AssetType.IMAGE,
        image_width=512,
        image_height=512
    )
    assert asset.asset_type == AssetType.IMAGE
    
    # Invalid: image with duration
    with pytest.raises(ValidationError) as exc_info:
        AssetRecord(
            asset_id="invalid",
            file_path="test.png", 
            asset_name="Invalid",
            asset_type=AssetType.IMAGE,
            duration_seconds=10.0
        )
    
    assert "Image assets should not have duration" in str(exc_info.value)
```

### Database Integration Testing
```python
def test_database_enum_conversion():
    """Test enum conversion in database operations"""
    db = DatabaseManager()
    
    # Store with string values (auto-converted)
    db.store_entity(
        entity_id="test_entity",
        entity_type="biome_spec",  # String
        entity_data={"test": "data"},
        source_pipeline="test",
        dread_level=2,  # Integer
        corruption_stage="withered"  # String
    )
    
    # Retrieve with enum types
    entity = db.get_entity("test_entity")
    assert isinstance(entity.entity_type, EntityType)
    assert isinstance(entity.dread_level, DreadLevel)
    assert isinstance(entity.corruption_stage, CorruptionStage)
```

## Best Practices

1. **Use Type Hints**: Always specify types for better IDE support and type checking
2. **Leverage Enums**: Use enum types instead of string constants for better validation
3. **Protocol Compliance**: Design functions to accept protocol types for flexibility
4. **Validation First**: Let SQLModel validation catch errors early in the pipeline
5. **Pattern Matching**: Use match-case for enum handling instead of if/elif chains
6. **Unicode Safety**: Use `casefold()` for string comparisons that may involve international text

## Contributing

When extending the database layer:

1. Add new enums to `types.py` with `auto()` values and factory methods
2. Update protocols in `protocols.py` for new interfaces
3. Add validation to models in `models.py` using decorators
4. Update manager methods to use new types
5. Add comprehensive tests for validation logic
6. Update this README with examples

## Dependencies

- **SQLModel**: Unified SQLAlchemy + Pydantic framework
- **Pydantic**: Data validation and settings management
- **SQLAlchemy**: SQL toolkit and ORM
- **Python 3.13**: Required for modern enum and match-case features

---

*The modernized database layer provides type-safe, validated, and efficient data management for the Dragons Labyrinth game engine.*
