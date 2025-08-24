# Dialogue & Narrative Architecture

## The Storage Problem

With Dragon's Labyrinth's branching narrative, we're looking at potentially:
- 12 companions × 5 dread levels × multiple conversation topics = 100s of dialogue trees
- Each tree having 10-50 nodes
- Total: 5,000-10,000 dialogue nodes

## The Solution: Hybrid Storage

### 1. YarnSpinner Files (Runtime)
```
assets/dialogue/
├── companions/
│   ├── therapist/
│   │   ├── intro.yarn
│   │   ├── dread_0_peace.yarn
│   │   ├── dread_1_unease.yarn
│   │   ├── dread_2_anxiety.yarn
│   │   ├── dread_3_horror.yarn
│   │   └── dread_4_madness.yarn
│   └── [other companions]/
├── quests/
│   ├── main/
│   └── side/
└── npcs/
```

### 2. SQLite Database (Build-time Generation Tracking)
```sql
-- Track what's been generated
CREATE TABLE dialogue_generation (
    id TEXT PRIMARY KEY,
    character TEXT NOT NULL,
    context TEXT NOT NULL,
    dread_level INTEGER,
    generated_at TIMESTAMP,
    token_count INTEGER,
    yarn_file TEXT,
    prompt_hash TEXT -- Avoid regenerating identical content
);

-- Track relationships between dialogues
CREATE TABLE dialogue_graph (
    source_node TEXT,
    target_node TEXT,
    condition TEXT,
    PRIMARY KEY (source_node, target_node)
);

-- Cache expensive computations
CREATE TABLE narrative_cache (
    context_hash TEXT PRIMARY KEY,
    generated_content TEXT,
    expires_at TIMESTAMP
);
```

### 3. Runtime Loading Strategy
```rust
pub struct DialogueManager {
    yarn_runner: YarnRunner,
    loaded_files: HashMap<String, YarnProject>,
    current_context: DialogueContext,
}

impl DialogueManager {
    pub fn load_contextual_dialogue(&mut self, 
        companion: &Companion,
        dread: u8,
        location: &str,
    ) -> Result<()> {
        // Only load relevant files
        let files = vec![
            format!("companions/{}/dread_{}.yarn", companion.name, dread),
            format!("locations/{}.yarn", location),
        ];
        
        for file in files {
            if !self.loaded_files.contains_key(&file) {
                self.yarn_runner.load_file(&file)?;
            }
        }
        Ok(())
    }
}
```

## AI Generation Strategy (Optimized)

### What STAYS in build-tools:
```rust
// Only narrative generation
pub enum GenerationTask {
    CompanionDialogue {
        archetype: CompanionArchetype,
        dread_level: u8,
        context: Vec<String>, // Previous conversations
    },
    QuestDialogue {
        quest_type: QuestType,
        moral_weight: MoralWeight,
        branches: Vec<String>,
    },
    RelationshipProgression {
        companion_a: String,
        companion_b: String,
        shared_trauma: Vec<String>,
    },
}
```

### Optimized Prompting:
```rust
pub struct DialoguePromptOptimizer {
    pub fn generate_batch(&self, tasks: Vec<GenerationTask>) -> String {
        // Batch similar requests
        // Use compression techniques
        // Reference previous context by ID not full text
        // Generate multiple variations in one call
    }
    
    pub fn use_templates(&self) -> String {
        // Reusable patterns to reduce tokens
        r#"
        [TEMPLATE: companion_breakdown]
        Character shows: [fear_response]
        Player options:
        1. Comfort -> [trust++]
        2. Ignore -> [trust--]
        3. Exploit -> [corruption++]
        "#
    }
}
```

## What Gets Generated Where

### During Asset Curation (Sonnet 1M):
- Identify missing 3D models
- Generate .bpy scripts for Blender
- Design level layouts
- Audio requirements

### During Build (build-tools + OpenAI):
- Dialogue trees (YarnSpinner format)
- Quest branching logic
- Relationship matrices
- Narrative event triggers

### During Development (Direct Rust):
- Map generation (using mapgen algorithms)
- Physics interactions (avian)
- Hex grid logic (hexx)
- Combat systems
- UI layouts

### Runtime Procedural (No AI):
- Terrain generation
- Corruption spread
- Environmental effects
- Particle systems

## Benefits of This Approach

1. **Focused AI Usage**: Only for truly creative content (dialogue/narrative)
2. **Efficient Storage**: YarnSpinner handles runtime, SQLite tracks generation
3. **Optimized Prompts**: Batch processing, templates, compression
4. **Clear Separation**: AI for content, algorithms for systems
5. **Performance**: Only load needed dialogue, not entire narrative

## Token Optimization Techniques

### 1. Context Compression
```python
# Instead of sending full dialogue history
"Previous: [300 words of dialogue]"

# Send summary + key decisions
"Context: Therapist revealed trauma, player chose empathy [ref:therapy_01]"
```

### 2. Batch Generation
```python
# Instead of 12 separate API calls for companions
"Generate intro dialogues for ALL companions with these archetypes:
- Therapist: professional, hiding trauma
- Child: innocent, observant
[...all 12 in one request]"
```

### 3. Template References
```python
# Define templates once, reference many times
"Use [TEMPLATE:breakdown] with FEAR=abandonment, RESPONSE=freeze"
```

### 4. Incremental Generation
```python
# Generate base dialogue first
base = generate_core_dialogue()

# Then generate variations
for dread_level in range(5):
    variation = generate_variation(base, dread_level)
```

## Implementation Priority

1. **First**: Get YarnSpinner working with basic dialogue
2. **Second**: Build dialogue generation pipeline with templates
3. **Third**: Implement batching and optimization
4. **Fourth**: Add relationship/quest generation
5. **Finally**: Polish with variations and edge cases

This way we get playable dialogue quickly, then enhance it.
