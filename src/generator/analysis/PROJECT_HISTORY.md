

# Project History: Dragons' Labyrinth Entity Generator

## Early Development: Sprite Sheets and NPC Integration

- **NPC Sprite Sheets:**  
  Initial project phases focused on assembling and integrating sprite sheets for non-playable characters (NPCs). The goal was to create a flexible system for managing diverse NPC visual assets, supporting multiple animation states and directions.
    - Sprite sheets were organized by entity type.
    - Asset pipeline established for importing and referencing sprites in the engine.

- **Godot Integration:**  
  Early on, the generator was designed to output data compatible with the Godot game engine. This included:
    - Exporting entity definitions in formats easily parsed by Godot (e.g., JSON, CSV).
    - Ensuring sprite sheet layouts aligned with Godot's animation frame requirements.
    - Initial Godot scripts for dynamically loading and assigning entity data.

## Data Extraction: SQLite and HBF

- **Entity Data Sourcing:**  
  To populate a rich world, the project incorporated data extraction from SQLite databases and legacy HBF (Hierarchical Binary Format) files.
    - Developed Python scripts to extract, clean, and normalize entity attributes from SQLite.
    - Reverse-engineered HBF file structure to access additional legacy NPC data.
    - Unified extracted data into a common schema for further processing.

## Addressing Training Data Gaps

- **Data Gaps Identified:**  
  During entity generation, inconsistencies and missing values were discovered, particularly in attributes required for NPC logic and appearance.
    - Implemented fallback defaults and validation routines.
    - Logged and tracked missing fields for future data sourcing or manual curation.
    - Designed the system to gracefully degrade or randomize missing traits when necessary.

## Performance: Processor Optimizations

- **Optimization Efforts:**  
  As the volume of entities and sprites grew, performance bottlenecks appeared, especially in batch processing and image manipulation.
    - Refactored core loops to use vectorized operations (NumPy, Pillow).
    - Parallelized image processing tasks where feasible.
    - Profiled and cached expensive computations to improve throughput.

## Migration Toward Dual-Mode Manager

- **CLI and API Unification:**  
  To streamline both command-line and programmatic usage, the system evolved toward a dual-mode manager.
    - Introduced a unified interface for invoking generators as CLI tools or as importable Python modules.
    - Adopted [Typer](https://typer.tiangolo.com/) for concise, type-safe CLI definitions.
    - Modularized entity management logic to support both interactive and automated workflows.

## Summary

This project has moved from basic asset organization through complex data extraction and validation, toward a robust, optimized, and flexible entity generator. The current architecture supports modern development practices, efficient processing, and user-friendly interfaces, positioning the system as a core tool for Dragons' Labyrinth content creation.
