# Generator Subpackage Refactoring - Independent Execution Tasks

## Execution Strategy: Sequential Task Completion

**Execute multiple subpackages in single session when context allows. Create new_task handoff only when approaching context limits.**

## TASK QUEUE - Execute in Order

### ✅ COMPLETED TASKS
- [x] **T0-FOUNDATION**: Fixed main orchestrator architecture (single engine pattern)
- [x] **T0-DOCS**: Created systematic review process documentation

### 🎯 READY TO EXECUTE - Independent Tasks

#### **T1-ENTITIES**: Entities Subpackage Complete Refactor
**Status**: Ready to execute  
**Context Required**: ~20K tokens  
**Duration**: 15-20 minutes

#### **T2-SEEDS**: Seeds Subpackage Complete Refactor  
**Status**: Waiting for T1-ENTITIES  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T3-PSYCHOLOGY**: Psychology Subpackage Complete Refactor
**Status**: Waiting for T2-SEEDS  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T4-WORLD**: World Subpackage Complete Refactor
**Status**: Waiting for T3-PSYCHOLOGY  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T5-MAPS**: Maps Subpackage Complete Refactor
**Status**: Waiting for T4-WORLD  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T6-ENCOUNTERS**: Encounters Subpackage Complete Refactor
**Status**: Waiting for T5-MAPS  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T7-SPRITES**: Sprites Subpackage Complete Refactor
**Status**: Waiting for T6-ENCOUNTERS  
**Context Required**: ~15K tokens  
**Duration**: 10-15 minutes

#### **T8-ASSETS**: Assets Subpackage Complete Refactor
**Status**: Waiting for T7-SPRITES  
**Context Required**: ~20K tokens  
**Duration**: 15-20 minutes

#### **T9-CLEANUP**: Root Level File Decomissioning
**Status**: Waiting for T1-T8 completion  
**Context Required**: ~10K tokens  
**Duration**: 5-10 minutes

## EXECUTION PROTOCOL

### Per-Task Execution Pattern
1. **Read this document** - understand current status
2. **Execute task completely** - don't leave partially done
3. **Update task status** - mark completed with checkmark
4. **Commit progress** - checkpoint working state
5. **Continue to next task** OR create new_task if context approaching limits

### Context Window Management  
- **Monitor usage**: Track tokens consumed during task execution
- **Optimal range**: Execute 2-4 tasks per session (80-120K tokens total)
- **Handoff threshold**: Create new_task when approaching 150K+ tokens
- **Handoff content**: Copy this document + progress updates + next task details

## CRITICAL ARCHITECTURE ISSUES TO FIX

### Root Level Pollution
**Problem**: Constants, models, protocols, types at root belong in subpackages  
**Solution**: Extract content to appropriate subpackages during each task
**Exception**: Only statistics.py stays at root (self-contained)

### Duplicate ORM Systems  
**Problem**: Both models.py AND orm.py with SQLModel tables
**Solution**: Merge orm.py content into models.py (single source of truth)

### Import/Type Violations
**Problem**: Using Optional, Dict, List instead of modern syntax
**Solution**: Replace with dict[str, any], str | None, list[str] etc

### Manager Class Complexity
**Problem**: Complex manager classes instead of simple functions  
**Solution**: Replace with simple run(engine, logger, console) functions

### Defensive Programming
**Problem**: Unnecessary file checking when inputs are committed
**Solution**: Remove defensive code, use repository-committed paths directly

## Systematic Review Process

### Phase 1: File Structure Analysis
For each subpackage, review ALL files:
- `types.py` - Enums and type definitions using modern syntax
- `constants.py` - Subpackage-specific constants (not global)
- `protocols.py` - Using types from types.py
- `models.py` - SQLModel tables ONLY (merge orm.py content)
- `__init__.py` - Simple run() function + exports

### Phase 2: Coding Standards Enforcement
**STRICT Standards**:
- **Modern Type Syntax**: `dict[str, any]` NOT `Dict[str, Any]`
- **Union Types**: `str | None` NOT `Optional[str]`  
- **No Optional Anywhere**: Use `| None` syntax exclusively
- **Imports at TOP**: NEVER inside functions
- **Absolute Imports**: `from generator.subpackage.module import thing`
- **SQLModel Only**: No pydantic, merge orm.py into models.py

### Phase 3: Architecture Simplification
- **Remove Manager Classes**: Replace with simple run() functions
- **Engine Parameter**: Accept shared engine, don't create own
- **No Defensive Programming**: Files are committed, paths are constants
- **Flat Structure**: Avoid deep nesting and abstractions

### Phase 4: Content Review & Root Level Decomission
- **Repository Assets**: All inputs are committed (raw_game.hbf, GLOBAL_STYLE_GUIDE.toml, asset-prompts/)
- **Move Root Content**: Extract relevant parts from root files into subpackage files
- **Decomission Root Files**: Remove types.py, models.py, protocols.py, constants.py from root
- **Self-Contained Subpackages**: Each subpackage owns its types/models/protocols/constants

## File-by-File Review Template

### 1. types.py
- Modern enums with auto() where helpful
- Union types using `|` syntax
- No imports inside functions
- Clear type aliases for domain concepts

### 2. constants.py  
- Subpackage-specific paths and config
- Repository-relative paths (inputs/raw_game.hbf is ALWAYS there)
- No defensive path checking

### 3. protocols.py
- Use types from types.py
- Modern annotations throughout
- Protocol definitions for interfaces

### 4. models.py (MERGED from orm.py)
- SQLModel tables ONLY
- Use types and constants from subpackage
- Single source of truth for data models
- Table creation functions

### 5. __init__.py
- Simple run(engine, logger, console) function
- Accept shared engine parameter
- No path manipulation or file checking
- Return RunStatistics
- Export all models and types

## Current Status: Entities Subpackage Review

**Files Identified**:
- `types.py` ✓ (exists)
- `constants.py` ❌ (missing - need to create)
- `protocols.py` ✓ (exists)  
- `models.py` ✓ (exists but conflicts with orm.py)
- `orm.py` ❌ (duplicate - merge into models.py)
- `__init__.py` ⚠️ (partially refactored but has issues)

**Critical Issues in Entities**:
1. Both models.py AND orm.py have SQLModel tables
2. Constants in wrong location (global instead of entities/)
3. Defensive programming in run() function
4. Not using repository-committed files properly

## Next Steps Process

### Immediate: Entities Subpackage Complete Review
1. **Audit all files** - entities/ AND root level for entities-related content
2. **Extract root content** - move entities-specific types/constants from root files
3. **Merge orm.py into models.py** - single source of truth
4. **Create entities/constants.py** - move HBF_RAW_PATH etc from root
5. **Fix __init__.py run()** - remove defensive programming, accept engine
6. **Test entities subpackage** - ensure it works independently

### Subsequent: One Subpackage at a Time
- Pick next subpackage (seeds, psychology, world, etc)
- Apply same systematic review
- Update this document with findings
- Create handoff for next subpackage review

## Repository Structure Understanding

**Committed Inputs** (ALWAYS available):
```
inputs/
├── asset-prompts/
│   ├── universal-biome-variants.toml
│   ├── universal-bridges.toml
│   ├── universal-character-variants.toml
│   ├── universal-effect-variants.toml
│   ├── universal-feature-variants.toml
│   ├── universal-item-variants.toml
│   ├── universal-monster-variants.toml
│   ├── universal-paths.toml
│   └── universal-ui-variants.toml
├── GLOBAL_STYLE_GUIDE.toml
└── raw_game.hbf
```

**No Need For**:
- File existence checking
- Shutil copying
- Defensive programming
- Path discovery

## Success Criteria Per Subpackage

### Code Quality
- Zero import violations
- Zero type annotation violations  
- Zero Optional usage
- All imports at file top
- Modern Python throughout

### Architecture
- Single models.py (no orm.py)
- Clean types.py with enums
- Subpackage constants.py
- Simple run() function
- Shared engine usage
- **Root decomission**: Moved relevant content from root files

### Functionality
- Subpackage works independently
- Returns proper RunStatistics
- Integrates with main pipeline
- No defensive programming assumptions

### Root Level Cleanup
- Identify entities-owned content in root files
- Move to entities subpackage appropriately
- Mark root sections for removal after all subpackages reviewed

## T1-ENTITIES TASK SPECIFICATION

**EXECUTE THIS TASK COMPLETELY - Don't leave partially done**

### Task Scope
Complete entities subpackage refactor following systematic review process

### Execution Steps
1. **Audit entities files** - review all files in src/generator/entities/
2. **Extract root content** - identify entities-specific content in src/generator/ root files
3. **Create entities/constants.py** - move HBF_RAW_PATH and related constants
4. **Merge orm.py into models.py** - single source of truth for SQLModel tables
5. **Fix entities/__init__.py run()** - remove defensive programming, proper engine usage
6. **Update all imports** - modern syntax throughout entities subpackage
7. **Test entities functionality** - ensure run() function works with shared engine

### Files to Modify
- `src/generator/entities/constants.py` (CREATE - extract from root)
- `src/generator/entities/models.py` (MERGE orm.py content)
- `src/generator/entities/orm.py` (DELETE after merge)
- `src/generator/entities/__init__.py` (FIX run function)
- `src/generator/entities/types.py` (UPDATE modern syntax)
- `src/generator/entities/protocols.py` (UPDATE modern syntax)
- All other entities/*.py files (FIX imports/types)

### Success Criteria  
- ✅ Single models.py (no orm.py)
- ✅ entities/constants.py with subpackage constants
- ✅ Clean run(engine, logger, console) function
- ✅ Zero Optional/Dict/List usage
- ✅ All imports at file top
- ✅ No defensive programming
- ✅ Committed working state

### After Completion
- Mark **T1-ENTITIES** as completed with [x]
- Update this document with any insights discovered
- Proceed to **T2-SEEDS** OR create new_task if context approaching limits

## HANDOFF PROTOCOL (Use only when context approaching limits)

### When to Create New Task
- Context usage approaching 150K+ tokens
- Complex debugging requiring significant exploration
- Major architectural discoveries requiring strategy pivot

### Handoff Content Template
```
# Generator Subpackage Refactoring - Continuing Execution

## Progress Update
[Copy this entire task queue with updated completion status]

## Next Task to Execute  
[Specify exact next task - T2, T3, etc]

## Key Discoveries
[Document any important insights from completed tasks]

## Execution Context
[Any specific context needed for next agent to continue effectively]
```

## SYSTEMATIC REVIEW STANDARDS (Apply to All Tasks)

### Coding Standards (Non-Negotiable)
- **Modern Type Syntax**: `dict[str, any]` NOT `Dict[str, Any]`
- **Union Types**: `str | None` NOT `Optional[str]`
- **Imports at TOP**: NEVER inside functions
- **Absolute Imports**: `from generator.subpackage.module import thing`
- **SQLModel Only**: Merge orm.py into models.py

### File Organization Standards  
- **Subpackage ownership** - no root-level types/models/constants/protocols
- **Self-contained modules** - each subpackage owns its domain
- **Simple functions** - run() functions over manager classes
- **Shared engine** - accept engine parameter, don't create own

### Repository Understanding
**Committed inputs** (ALWAYS available): inputs/raw_game.hbf, inputs/GLOBAL_STYLE_GUIDE.toml, inputs/asset-prompts/
**No defensive programming needed** - files are guaranteed to exist
