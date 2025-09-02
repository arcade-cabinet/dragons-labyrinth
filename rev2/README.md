# Worldbuilder (Bevy Tilemap + Mapgen + Agent Bridge)

* Rust 1.88 • Edition 2024
* Bevy 0.16.1
* bevy_ecs_tilemap 0.16.0 (hex support)
* mapgen 0.6.0 (dungeon algorithms)
* Optional: bevy-agent 0.1.0 via `tools/agent_bridge` (feature-gated)

## Run
```bash
cargo run -p game
```

Controls: arrows (move) • E (encounter) • T (shop) • Q (quest log) • Space (talk) • Enter (spawn dungeon) • Esc (clear dungeon) • S (save).

## Optional: Build-time agent
```bash
BEVY_AGENT_AUTOGEN=1 cargo run -p game
# requires: cargo install bevy-agent
```
