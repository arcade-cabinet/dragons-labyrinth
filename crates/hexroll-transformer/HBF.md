# HBF.md — Analysis of the Hexroll HBF Export

This document summarizes the structure of the provided `.hbf` (SQLite) export and the patterns we exploit to build the world database.

## Tables

- `Entities(uuid TEXT PRIMARY KEY, value TEXT)` — Stores HTML or JSON fragments (or empty placeholders).
- `Refs(uuid TEXT, type TEXT, details TEXT)` — Cross references connecting UUIDs (hex ownership, membership, etc.).

## Counts (observed)
- ~70,801 total `Entities` rows.
- ~68,556 **blank** `value` rows.
- ~2,245 **non‑empty** rows (JSON or HTML).

The non‑empty rows include:
- **Hex pages** (often contain weather tables — dry/wet columns — and occasionally embedded NPC stat blocks). Document title: `Hex N2`, etc.
- **Settlement pages**: “Village of X”, “City of Y”, with shop/tavern pages (title `<h4>` with `(<em>Type</em>)` for shop type).
- **Taverns/Inns**: Tables labelled `Rumor` with `d6` column + rumor text; common hooks about deliveries, monster sightings, etc.
- **NPC pages**: Tables of ability scores (`STR DEX CON INT WIS CHA`) and sections like AC/HP/Speed/Senses/Languages/Actions.
- **Dungeon/Cave/Temple/Tomb**: Area pages with headings, room summaries, hazards/portals.
- **JSON map blobs**: Feature arrays with `poi`, `areas`, `portals`, `hazards`.

## Patterns we use

- `<div id="doc-title">…</div>` carries the main title; we split on “ in ” / “ from ” to extract context (settlement vs shop page).
- Shops: `<h4><span id="editable-title">Name</span> (<em>Type</em>)</h4>` ⇒ `shop.kind` and `shop.name`.
- Rumors: find `table` with header containing `Rumor`; gather rows (often `d6` + text).
- Weather: find table with headers `Dry` and `Wet` columns; store as JSON rows; detect weekly flood chance (“1-in-6 weekly”).
- NPC: extract AC/HP/Speed lines and ability table; parse `Actions` as list items.
- Dungeons: look for keywords “Cave”, “Temple”, “Tomb” in titles and headings; classify as `dungeon.kind` and create subordinate entries.

See the code in `src/extractor.rs` for concrete regex/DOM extraction heuristics.
