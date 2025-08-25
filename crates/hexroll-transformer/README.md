# hexroll_transformer

**hexroll_transformer** converts a Hexroll `.hbf` SQLite export into a clean, normalized **game.db** with a rich world schema (regions, biomes, hexes, settlements, dungeons, dwellings, factions, monsters, NPCs, rumors, weather, dialogues). It also provides a CLI to run the export and documentation for wiring the data into Bevy via **bevy_sqlx** and Yarn Spinner (**bevy_yarnspinner**).

- Deterministic transformation of every viable HTML/JSON fragment into tables.
- AI-assisted classification for messy/ambiguous pages (no feature gate; on by default).
- Dialogues: generates Yarn scripts for each NPC and stores them in `dialogue`.
- Keeps original `Entities` and `Refs` UUIDs for traceability.

## Install

This crate is a normal Rust project. You can copy its contents into your workspace as a member crate.

```bash
cargo build
```

> You will need an `OPENAI_API_KEY` set in your environment for the AI analyzer to function.

## CLI

Transform an HBF export and generate a normalized world database:

```bash
cargo run --bin hexroll_transformer -- /path/to/export.hbf --export-db game.db
```

This will:
1. Read `Entities`/`Refs` from the HBF (using `rusqlite` for speed).
2. Cluster pages by structure and run heuristics + AI classification.
3. Extract deep fields (hex coords/biomes, settlements, NPC statlines, rumors, weather, dungeon types, dwellings, factions).
4. Create `game.db` with SeaORM entities defined in `src/orm.rs`.
5. Generate Yarn dialogue stubs per NPC into the `dialogue` table.

## Bevy usage (with bevy_sqlx)

Add `bevy_sqlx` to your Bevy app and register the components/tables you want to hydrate. Because we keep original UUIDs, you can build an **ERM** (Entity Relationship Mapper) that resolves DB foreign keys to ECS `Entity` IDs.

## Yarn Spinner

Use `bevy_yarnspinner` to load Yarn scripts from the `dialogue` table. Each NPC record has one or more scripts keyed by `npc_id` and `node_name`.

See `HBF.md` for a deep analysis of the underlying export, including how we derived each table.
