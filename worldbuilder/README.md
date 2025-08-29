# Worldbuilder Name Pipeline (Godot + Offline Python) — MIT

**Drop-in, commercial-friendly pipeline** to generate authentic-feeling names for a medieval–horror RPG
by blending Old Norse (non), Old English (ang), Welsh (cy), Arabic (ar), and Hebrew (he) lemmas.

## Contents

- `prep/` — Offline Python tools to convert lexical sources → JSON (one-time).
- `godot_addon/` — A Godot addon that reads the JSONs and generates names at runtime.
- `sample_data/` — Small seed JSONs so it works out-of-the-box.
- `LICENSE` — MIT (OK for commercial use).

> This kit ships **without** third‑party data dumps. Use `prep/` to ingest OMW, Cleasby–Vigfusson, etc.
> Then commit the produced JSONs under `godot_addon/addons/worldbuilder/data/lingua/` and ship the addon alone.

---

## Quick Start (Zero Setup — uses the sample seed lists)

1) Copy the `godot_addon/addons` folder into your Godot project root.
2) In Godot 4: **Project → Project Settings → AutoLoad**: ensure `Worldbuilder.gd` is autoloaded (the addon does this automatically).
3) Run the demo script in `godot_addon/examples/ExampleNames.gd` (attach to a Node and run the scene).

You should see names like: `Ulfrcoed`, `Vetrwyn`, `Draigvetr`, `Nachtnir`.

---

## Full Pipeline (Recommended)

1. **Offline Python prep (once):**
   - Use OMW data dumps + CLTK to generate `en → {non, ang, cy, ar, he}` lemma tables for ~500–1000 fantasy seeds
     (forest, ash, iron, raven, night, winter, etc.).
   - From Cleasby–Vigfusson, add rich Old Norse thematic lists (sea, storm, kinship, battle).
   - Transliterate Arabic/Hebrew to ASCII forms (keep originals too).

2. **Commit JSONs + manifest** to:  
   `godot_addon/addons/worldbuilder/data/lingua/`

3. **In Godot**, select a language blend per region/act:  
   - Meadows (Act1): `non:0.2, ang:0.4, cy:0.4`  
   - Warfront (Act2): `non:0.5, ang:0.3, cy:0.2`  
   - Void (Act3): add Semitic edge: `ar/he:0.2`

4. **NameForge** composes 1–2 morphemes per blend; phonotactics polish final forms.

---

## Legal / Licensing Notes

- This repo is MIT-licensed. You are responsible for the licensing of the **source datasets** you ingest.
- Recommended: Open Multilingual Wordnet (various permissive licenses per language), Cleasby–Vigfusson (MIT-packaged),
  CLTK (permissive). Cache results, store provenance + checksums in `manifest.json`.

---

Generated on 2025-08-27.
