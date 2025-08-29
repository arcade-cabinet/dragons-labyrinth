# Python Preprocessing Strategy - Smart Analysis Before Transformation

## The Insight
Before diving into Rust transformer development, we can use Python to thoroughly analyze, optimize, and prepare the HBF data. This gives us better understanding, visual reports, and potentially an optimized intermediate format.

## Python Analysis Pipeline

### Phase 1: Deep Analysis
```python
# src/dragons_labyrinth/analyze_hbf_db.py
- Complete entity analysis with pandas
- HTML reference graph visualization
- Clustering similar pages
- Rich terminal output with progress
- HTML reports for review
```

### Phase 2: Data Optimization
```python
# src/dragons_labyrinth/compress_hbf.py
- Remove truly empty entities
- Normalize HTML formatting
- Extract and deduplicate common patterns
- Create optimized.hbf with only useful data
```

### Phase 3: Clustering & Grouping
```python
# src/dragons_labyrinth/cluster_entities.py
- Group NPCs by location
- Group dungeons by type
- Group factions by conspiracy type
- Output batching strategy for AI
```

### Phase 4: Relationship Mapping
```python
# src/dragons_labyrinth/map_relationships.py
- Build complete reference graph
- Identify orphaned entities
- Find circular dependencies
- Generate relationship manifest
```

### Phase 5: Report Generation
```python
# src/dragons_labyrinth/generate_reports.py
- HTML visual reports with charts
- Network graphs of relationships
- Statistics and metrics
- Missing data identification
```

## Tools to Add

```toml
# pyproject.toml additions
[tool.uv.dependencies]
pandas = "^2.0.0"          # DataFrames for analysis
rich = "^13.0.0"           # Beautiful terminal output
plotly = "^5.0.0"          # Interactive charts
networkx = "^3.0.0"       # Graph analysis
beautifulsoup4 = "^4.12"  # HTML parsing
lxml = "^5.0.0"           # Fast XML/HTML processing
tqdm = "^4.66"            # Progress bars
jinja2 = "^3.1"           # HTML report templates
```

## Benefits of Python Preprocessing

### 1. Better Understanding
- See ALL the data patterns before coding
- Identify edge cases early
- Understand relationship complexity
- Find optimization opportunities

### 2. Visual Reports
```python
# Generate interactive HTML reports
df = pd.DataFrame(entities)
fig = px.sunburst(df, path=['type', 'subtype', 'location'])
fig.write_html("entity_hierarchy.html")
```

### 3. AI Batching Strategy
```python
# Analyze token counts and group efficiently
def optimize_batches(npcs, max_tokens=3500):
    batches = []
    current_batch = []
    current_tokens = 0
    
    for npc in npcs:
        tokens = count_tokens(npc)
        if current_tokens + tokens > max_tokens:
            batches.append(current_batch)
            current_batch = [npc]
            current_tokens = tokens
        else:
            current_batch.append(npc)
            current_tokens += tokens
    
    return batches
```

### 4. Intermediate Format
```python
# Create optimized intermediate format
class OptimizedHBF:
    def __init__(self, original_path):
        self.conn = sqlite3.connect(original_path)
        self.entities = self.load_and_optimize()
        
    def load_and_optimize(self):
        # Remove empty entities
        # Parse HTML once
        # Extract relationships
        # Build indices
        return optimized_data
        
    def save(self, path="optimized.hbf"):
        # Save cleaned, indexed version
```

### 5. Validation & Testing
```python
# Validate before transformation
def validate_world_completeness():
    issues = []
    
    # Check every hex has coordinates
    # Check every dungeon has entrance
    # Check every NPC has name
    # Check faction membership consistency
    
    return issues
```

## Execution Plan

### Step 1: Initial Analysis
```bash
# Run basic analysis
python src/dragons_labyrinth/analyze_hbf_db.py game.hbf

# Output:
# - entity_counts.json
# - relationship_graph.html
# - missing_data.txt
```

### Step 2: Optimization
```bash
# Create optimized version
python src/dragons_labyrinth/compress_hbf.py game.hbf optimized.hbf

# Output:
# - optimized.hbf (30% smaller)
# - compression_report.txt
```

### Step 3: Clustering
```bash
# Generate AI batching strategy
python src/dragons_labyrinth/cluster_entities.py optimized.hbf

# Output:
# - npc_batches.json
# - faction_groups.json
# - location_clusters.json
```

### Step 4: Reports
```bash
# Generate visual reports
python src/dragons_labyrinth/generate_reports.py optimized.hbf

# Output:
# - reports/index.html
# - reports/entities.html
# - reports/relationships.html
# - reports/statistics.html
```

## Smart Optimizations We Can Do

### 1. HTML Normalization
- Strip unnecessary whitespace
- Normalize tag formatting
- Extract inline styles to classes
- Remove empty tags

### 2. Pattern Extraction
```python
# Find repeated patterns
patterns = {}
for entity in entities:
    if entity.html:
        # Extract stat blocks
        stat_block = extract_stat_block(entity.html)
        if stat_block:
            pattern_hash = hash(normalize(stat_block))
            patterns[pattern_hash] = stat_block
```

### 3. Reference Resolution
```python
# Pre-resolve all references
resolved = {}
for entity in entities:
    refs = extract_refs(entity.html)
    for ref in refs:
        target = find_entity(ref)
        if target:
            resolved[ref] = target.uuid
```

### 4. AI Prompt Optimization
```python
# Pre-generate optimized prompts
def generate_prompt_batch(npcs):
    # Remove redundant info
    # Compress descriptions
    # Structure for efficiency
    return optimized_prompt
```

## Expected Outcomes

After Python preprocessing:
1. **Clear understanding** of all 70,801 entities
2. **Visual reports** showing world structure
3. **Optimized HBF** with 30-50% size reduction
4. **Batching strategy** for AI calls
5. **Validation report** of data completeness
6. **Relationship graph** fully mapped
7. **Asset requirements** pre-calculated

## Integration with Rust Transformer

The Rust transformer can then:
```rust
// Read preprocessed data
let optimized = OptimizedHBF::load("optimized.hbf")?;
let batches = load_json("npc_batches.json")?;
let relationships = load_json("relationships.json")?;

// Generate with confidence
generate_world(optimized, batches, relationships)?;
```

## Memory for Next Agent
Before starting Rust transformer development, use Python to thoroughly analyze and optimize the HBF data. This gives us visual reports, clustering, relationship mapping, and an optimized intermediate format. The Python preprocessing makes the Rust transformation simpler, faster, and more reliable.
