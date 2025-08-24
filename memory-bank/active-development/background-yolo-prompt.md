# Background Agent YOLO Mode Prompt
## For Overnight/Extended Execution WITHOUT MCP

## CORE PRINCIPLES

### 1. COMPLETE SOLUTIONS ONLY
- NEVER create minimal implementations
- NEVER use stubs or placeholders  
- NEVER skip features to save time
- Build EVERYTHING properly the first time

### 2. NO TIME CONSTRAINTS
- Build times can be hours if needed
- Asset generation can process thousands of items
- Database operations can be comprehensive
- Take the time to do it RIGHT

### 3. DIRECT API ACCESS
You have access to these environment variables:
- `OPENAI_API_KEY` - Use for GPT-4, DALL-E, TTS generation
- `FREESOUND_API_KEY` - Use for audio asset downloads
- Any other keys in ~/.bash_profile

Use these DIRECTLY in Rust code via:
```rust
std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set")
```

### 4. NO MCP DEPENDENCY
- You do NOT have MCP tools available
- Use standard file I/O, grep, find commands
- Read files directly with std::fs
- Execute shell commands with std::process::Command

## YOLO MODE EXECUTION PATTERN

### Phase 1: Read Everything First
```bash
# Get oriented
find . -name "*.rs" -type f | head -20
grep -r "TODO" --include="*.rs" | head -20
cargo build 2>&1 | grep error | head -20
```

### Phase 2: Implement Comprehensively
For EACH task:
1. Read the ENTIRE relevant file
2. Understand ALL dependencies
3. Implement COMPLETE solution
4. Test thoroughly
5. Move to next

### Phase 3: Generate All Assets
When generating assets:
- Generate HUNDREDS not samples
- Create ALL variations (dread 0-4)
- Build complete sets, not examples
- Use parallel processing when possible

## SPECIFIC PATTERNS FOR DRAGON'S LABYRINTH

### Database Operations Pattern
```rust
async fn implement_all_44_methods(&self) -> DatabaseResult<()> {
    // Don't skip ANY method
    // Don't use unimplemented!() 
    // Actually write the database queries
    // Test each one works
}
```

### Asset Generation Pattern
```rust
async fn generate_all_assets() -> Result<()> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    
    // Generate ALL dread levels
    for dread in 0..=4 {
        // Generate ALL asset types
        for asset_type in &["ui", "audio", "dialogue", "environment"] {
            // Generate MULTIPLE variations
            for variation in 0..100 {
                // Actually generate, don't mock
                generate_real_asset(api_key, dread, asset_type, variation).await?;
            }
        }
    }
    Ok(())
}
```

### File Operations Without MCP
```rust
// Read files directly
let content = std::fs::read_to_string("path/to/file.rs")?;

// Write files directly  
std::fs::write("path/to/output.rs", generated_code)?;

// Search codebase
let output = std::process::Command::new("grep")
    .args(&["-r", "pattern", "--include=*.rs"])
    .output()?;
```

## CRITICAL: WHAT TO ACTUALLY DO

### Current Priority: 44 GameDatabaseOperations Methods
Location: `crates/game-database/src/engine.rs`

IMPLEMENT ALL OF THESE (not stubs!):
- create_player
- update_player_progression
- get_save_slots
- delete_save_slot
- get_horror_state
- record_horror_event
- ... (all 44 methods)

Each method should:
1. Connect to database properly
2. Use SeaORM entities from database-orm
3. Handle errors correctly
4. Return real data, not mock data

### After Database: Complete Asset Generation
1. Read all agent files in crates/build-tools/src/agents/
2. For EACH agent, generate FULL production assets:
   - UIAgent: Generate 100+ UI variations
   - AudioAgent: Download 500+ Freesound clips
   - DecayAgent: Generate 1000+ decay rules
   - DialogueAgent: Generate complete dialogue trees
   - MapsAgent: Generate 50+ map layouts

### Testing Without Asking
```rust
// Don't ask "should I test this?"
// Just test it:
cargo test
cargo build --release
cargo run --bin mcp-server

// If it fails, FIX IT
// If it passes, move on
```

## OVERNIGHT EXECUTION CHECKLIST

### Hour 1-4: Database Implementation
- [ ] Read traits.rs completely
- [ ] Read engine.rs completely  
- [ ] Implement all 44 methods
- [ ] Test compilation
- [ ] Fix all errors

### Hour 5-8: Asset Generation
- [ ] Set up OpenAI client
- [ ] Generate UI assets (all dread levels)
- [ ] Generate audio configs
- [ ] Generate dialogue trees
- [ ] Generate environment decay rules

### Hour 9-12: Integration & Testing
- [ ] Run full build
- [ ] Test all systems
- [ ] Generate missing assets
- [ ] Document what was completed

## NO COMPROMISES MODE

### Things you MUST NOT DO:
- ❌ "For brevity, I'll show just one example"
- ❌ "To save time, let's use a stub"
- ❌ "This is good enough for now"
- ❌ "We can add more later"
- ❌ unimplemented!() or todo!()
- ❌ Mock data or fake responses
- ❌ Partial implementations

### Things you MUST DO:
- ✅ Implement every single method
- ✅ Generate complete asset sets
- ✅ Handle all error cases
- ✅ Test everything works
- ✅ Build production-ready code
- ✅ Take hours if needed
- ✅ Use real API calls

## ENVIRONMENT SETUP

Before starting, verify:
```bash
echo $OPENAI_API_KEY  # Should show key
echo $FREESOUND_API_KEY  # Should show key
cargo --version  # Should work
rustc --version  # Should work
```

## SUCCESS CRITERIA

You are successful when:
1. All 44 database methods are FULLY implemented
2. game-database compiles with ZERO errors
3. Hundreds of assets are generated (not samples)
4. Everything is production-ready
5. No stubs, no placeholders, no shortcuts

## FINAL NOTES

This is an OVERNIGHT run. You have 8-12 hours. Use them ALL if needed.

Build time can be 10 minutes, 30 minutes, an hour - it doesn't matter.

What matters is COMPLETENESS and CORRECTNESS.

Ship a working game, not a prototype.
