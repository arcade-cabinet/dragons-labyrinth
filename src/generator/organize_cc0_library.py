#!/usr/bin/env python3
"""
Organize CC0 library assets for Dragon's Labyrinth.
- Scans vendor-specific sources: kenney-assets-source/ (k-) and quaternius-assets-source/ (q-)
- Unzips .zip archives into a staging area and rescans
- Categorizes into technical categories (models, textures, audio, fonts, sprites, docs, other)
- Derives logical subcategories from path/filename tokens (theme/domain)
- Builds readable, attributable filenames with vendor prefixes (k-/q-)
- RPG relevance filter optional via --no-filter
- ALWAYS converts 3D models (.obj/.fbx/.gltf) to GLB (no scaling)

Run example:
  python3 src/generator/organize_cc0_library.py --kenney-source ~/Downloads/kenney-assets-source --quaternius-source ~/Downloads/quaternius-assets-source --target .
"""

import argparse
import hashlib
import json
import os
import re
import shutil
import tempfile
import zipfile
from pathlib import Path

from generator.bpy_processor import convert_model_files_to_glb

# ---------------------------
# Technical categories & extensions
# ---------------------------
ASSET_CATEGORIES: dict[str, set[str]] = {
    'models': {'.glb', '.gltf', '.obj', '.mtl', '.fbx'},
    'textures': {'.png', '.jpg', '.jpeg', '.tga', '.bmp', '.gif', '.webp', '.tif', '.tiff'},
    'audio': {'.ogg', '.mp3', '.wav', '.flac'},
    'fonts': {'.ttf', '.otf', '.woff', '.woff2'},
    'sprites': {'.svg', '.psd', '.ai'},
    'docs': {'.txt', '.md', '.pdf', '.html', '.url', '.rtf'},
}
DEFAULT_OTHER = 'other'

# ---------------------------
# Relevance & exclusions (toggle with --no-filter)
# ---------------------------
RELEVANT_KEYWORDS: set[str] = {
    'zombie','monster','skeleton','ghost','demon','evil','dark','horror','creepy','nightmare','dead','undead','cursed',
    'medieval','castle','dungeon','knight','sword','shield','armor','dragon','goblin','orc','troll','wizard','magic','spell',
    'village','tavern','blacksmith','church','ruins','tower',
    'character','hero','warrior','rogue','mage','priest',
    'weapon','potion','chest','treasure','coin','key',
    'forest','swamp','mountain','cave','river','bridge','tree','rock','grass','path','road',
    'wall','door','window','roof','floor','stairs','furniture','table','chair','bed','barrel','crate'
}
EXCLUDE_KEYWORDS: set[str] = {
    'cyberpunk','sci-fi','scifi','space','spaceship','robot','cyber','futuristic',
    'car','vehicle','rifle','pistol','modern','city','computer','tech','electronic','neon','laser'
}

# ---------------------------
# Logical subcategory vocab (theme/domain)
# ---------------------------
THEME_KEYWORDS: dict[str, set[str]] = {
    'medieval': {'medieval','village','tavern','castle','keep','knight','squire','blacksmith','church'},
    'horror': {'horror','cursed','undead','zombie','skeleton','ghost','demon','nightmare','forsaken'},
    'fantasy': {'wizard','magic','spell','dragon','orc','goblin','troll','rune','altar'},
}
DOMAIN_KEYWORDS: dict[str, set[str]] = {
    'dungeon': {'dungeon','crypt','catacomb','ruin','ruins'},
    'village': {'village','town','hamlet','market','stall'},
    'weapons': {'sword','axe','mace','bow','dagger','shield','weapon'},
    'doors': {'door','gate','portcullis'},
    'floors': {'floor','tile','flagstone','plank'},
    'walls': {'wall','rampart','battlement'},
    'furniture': {'table','chair','bed','stool','shelf','shelves','cabinet','wardrobe','bench'},
    'props': {'barrel','crate','box','book','scroll','torch','lantern','bucket','anvil'},
    'architecture': {'tower','church','house','hut','roof','window','stairs','bridge'},
    'characters': {'character','npc','villager','man','woman','knight','goblin','orc','zombie','skeleton'},
    'nature': {'tree','rock','bush','grass','swamp','mushroom'},
    'ui': {'ui','icon','cursor','border'},
    'audio': {'ambience','sfx','voice','music','footstep'},
}

STRIP_TOKENS: set[str] = {
    'pack','megakit','kit','bundle','ultimate','updated','modular','standard',
    'redux','legacy','classic','sample','samples','sources','source'
}


OBJ_FAMILY = {'.obj', '.mtl'}

# ---------------------------
# Tokenize / naming helpers
# ---------------------------

def tokenize(s: str) -> list[str]:
    s = s.replace('[', ' ').replace(']', ' ')
    s = re.sub(r'[_\-]+', ' ', s)
    s = re.sub(r'[^\w\s]', ' ', s)
    out = []
    for t in s.lower().split():
        if t and t not in STRIP_TOKENS:
            out.append(t)
    return out




def is_relevant_asset(file_path: Path, filter_on: bool) -> bool:
    if not filter_on:
        return True
    stem = file_path.stem.lower()
    path_str = '/'.join(p.lower() for p in file_path.parts)
    if any(k in stem or k in path_str for k in EXCLUDE_KEYWORDS):
        return False
    return any(k in stem or k in path_str for k in RELEVANT_KEYWORDS)


def get_category(file_path: Path) -> str:
    ext = file_path.suffix.lower()
    for cat, exts in ASSET_CATEGORIES.items():
        if ext in exts:
            return cat
    return DEFAULT_OTHER


def theme_from_tokens(tokens: list[str]) -> str:
    for theme, keys in THEME_KEYWORDS.items():
        if any(k in tokens for k in keys):
            return theme
    return ''


def domain_from_tokens(tokens: list[str]) -> str:
    for domain, keys in DOMAIN_KEYWORDS.items():
        if any(k in tokens for k in keys):
            return domain
    return ''


def derive_subcategory(file_path: Path, category: str) -> str:
    folder_tokens: list[str] = []
    for part in file_path.parts:
        folder_tokens.extend(tokenize(part))
    stem_tokens = tokenize(file_path.stem)
    tokens = folder_tokens + stem_tokens

    theme = theme_from_tokens(tokens)
    domain = domain_from_tokens(tokens)

    if category == 'audio' and not domain:
        domain = 'audio'
    if category == 'fonts' and not domain:
        domain = 'fonts'

    if theme and domain:
        return f"{theme}/{domain}"
    if theme:
        return theme
    if domain:
        return domain
    return 'misc'


def clean_name(s: str) -> str:
    s = re.sub(r'[\s\-]+', '_', s.lower())
    s = re.sub(r'[^a-z0-9_]+', '', s)
    s = re.sub(r'_+', '_', s).strip('_')
    return s


def build_smart_name(file_path: Path, vendor_prefix: str) -> str:
    stem_tokens = tokenize(file_path.stem)
    path_tokens = tokenize('/'.join(file_path.parts))

    theme = theme_from_tokens(stem_tokens) or theme_from_tokens(path_tokens)
    domain = domain_from_tokens(stem_tokens) or domain_from_tokens(path_tokens)

    pieces: list[str] = [vendor_prefix]
    if theme:
        pieces.append(theme)
    if domain:
        pieces.append(domain)

    body_tokens = [t for t in stem_tokens if (len(t) > 1 or t.isdigit())]
    if not body_tokens:
        body_tokens = [clean_name(file_path.stem)]
    descriptor = '-'.join(body_tokens[:6])
    pieces.append(descriptor)

    base = clean_name('-'.join(pieces))
    return f"{base}{file_path.suffix.lower()}"

# ---------------------------
# ZIP staging & scanning
# ---------------------------

def unzip_all(root: Path, staging: Path, stats: dict[str, int]) -> None:
    for zip_path in list(root.rglob('*.zip')):
        rel = zip_path.relative_to(root)
        out_dir = staging / rel.parent
        out_dir.mkdir(parents=True, exist_ok=True)
        try:
            with zipfile.ZipFile(zip_path, 'r') as zf:
                zf.extractall(out_dir)
            stats['zips_extracted'] += 1
        except Exception as e:
            print(f"  Error extracting {zip_path}: {e}")
    nested = list(staging.rglob('*.zip'))
    if nested:
        for zip_path in nested:
            rel = zip_path.relative_to(staging)
            out_dir = staging / rel.parent
            try:
                with zipfile.ZipFile(zip_path, 'r') as zf:
                    zf.extractall(out_dir)
                stats['zips_extracted'] += 1
            except Exception as e:
                print(f"  Error extracting nested {zip_path}: {e}")

# ---------------------------
# Core organizer
# ---------------------------

def organize_assets(sources: list[tuple[Path, str]], target_dir: Path, filter_relevant: bool = True) -> dict[str, int]:
    stats: dict[str, int] = {
        'total_scanned': 0,
        'total_copied': 0,
        'models': 0, 'textures': 0, 'audio': 0, 'fonts': 0, 'sprites': 0, 'docs': 0,
        DEFAULT_OTHER: 0,
        'skipped_irrelevant': 0, 'skipped_duplicate': 0,
        'zips_extracted': 0
    }

    library_dir = target_dir / 'assets' / 'library'
    library_dir.mkdir(parents=True, exist_ok=True)

    with tempfile.TemporaryDirectory(prefix='asset_stage_') as stage_tmp:
        stage_root = Path(stage_tmp)
        # Unzip all sources into staging area
        for src, prefix in sources:
            unzip_all(src, stage_root / src.name, stats)

        # Build search_roots: (Path, vendor_prefix) for both original and staged
        search_roots: list[tuple[Path, str]] = []
        for src, prefix in sources:
            search_roots.append((src, prefix))
            search_roots.append((stage_root / src.name, prefix))

        processed: set[str] = set()
        to_convert: list[dict[str, str]] = []

        for root, vendor_prefix in search_roots:
            if not root.exists():
                continue
            for file_path in root.rglob('*'):
                if not file_path.is_file():
                    continue
                stats['total_scanned'] += 1

                category = get_category(file_path)
                if category == DEFAULT_OTHER:
                    if filter_relevant and not is_relevant_asset(file_path, True):
                        stats['skipped_irrelevant'] += 1
                        continue
                else:
                    if not is_relevant_asset(file_path, filter_relevant):
                        stats['skipped_irrelevant'] += 1
                        continue

                subcat = derive_subcategory(file_path, category)
                final_name = build_smart_name(file_path, vendor_prefix)

                target_path = library_dir / category / subcat / final_name
                target_path.parent.mkdir(parents=True, exist_ok=True)

                key = str(target_path).lower()
                if key in processed:
                    stats['skipped_duplicate'] += 1
                    continue

                try:
                    shutil.copy2(file_path, target_path)
                    processed.add(key)
                    stats['total_copied'] += 1
                    if category in stats:
                        stats[category] += 1
                    else:
                        stats[DEFAULT_OTHER] += 1

                    # Always queue conversion for .obj, .fbx, .gltf models
                    if category == 'models' and file_path.suffix.lower() in ('.obj', '.fbx', '.gltf'):
                        glb_root = library_dir / 'models_glb' / subcat
                        glb_root.mkdir(parents=True, exist_ok=True)
                        out_name = Path(final_name).with_suffix('.glb').name
                        to_convert.append({
                            'src': str(target_path),
                            'dst_filename': out_name,
                            'scale': 1.0,
                        })

                    if stats['total_copied'] % 200 == 0:
                        print(f"  Processed {stats['total_copied']} assets...")
                except Exception as e:
                    print(f"  Error copying {file_path}: {e}\n  ➤ target: {target_path}")

        # Always perform conversion for queued models
        if not to_convert:
            print("ℹ️  No models queued for conversion.")
        else:
            manifest = library_dir / '.cache' / 'convert_manifest.json'
            glb_out = library_dir / 'models_glb'
            glb_out.mkdir(parents=True, exist_ok=True)
            try:
                res = convert_model_files_to_glb(
                    files=to_convert,
                    output_dir=str(glb_out),
                    scale=1.0,
                    manifest_path=str(manifest)
                )
                converted = res.get('converted', 0)
                print(f"  ✅ Converted {converted} model(s) → GLB")
            except Exception as e:
                print(f"  ⚠️ Conversion failed: {e}")

    return stats

# ---------------------------
# CLI
# ---------------------------

def main() -> int:
    parser = argparse.ArgumentParser(description="Organize CC0 assets (Kenney + Quaternius) for Dragon's Labyrinth.")
    parser.add_argument('--kenney-source', type=Path, help='Kenney source directory')
    parser.add_argument('--quaternius-source', type=Path, help='Quaternius source directory')
    parser.add_argument('--target', type=Path, default=Path.cwd(), help='Target project root (default: CWD)')
    parser.add_argument('--no-filter', action='store_true', help='Do not filter by RPG relevance (import everything)')
    args = parser.parse_args()

    sources: list[tuple[Path, str]] = []
    if args.kenney_source:
        sources.append((args.kenney_source, 'k'))
    if args.quaternius_source:
        sources.append((args.quaternius_source, 'q'))
    if not sources:
        print("Error: At least one of --kenney-source or --quaternius-source must be provided.")
        return 1

    missing = [src for src, _ in sources if not src.exists()]
    if missing:
        print("Error: Missing source dirs:")
        for m in missing:
            print(f" - {m}")
        return 1

    print("🐉 Dragon's Labyrinth CC0 Asset Organizer — v3 (multi-source, unzip, smart subfolders, always convert)")
    print("="*88)
    print("Sources:")
    for src, prefix in sources:
        print(f"  • {src.resolve()} [{prefix}-]")
    print(f"Target:  {args.target.resolve()}")
    print(f"Filter:  {'Yes (RPG-relevant only)' if not args.no_filter else 'No (all assets)'}")
    print()

    stats = organize_assets(
        sources=sources,
        target_dir=args.target,
        filter_relevant=(not args.no_filter),
    )

    print("\n📊 Done!")
    print("="*88)
    print(f"Total scanned:          {stats['total_scanned']:,}")
    print(f"Total copied:           {stats['total_copied']:,}")
    print(f"  Models:               {stats['models']:,}")
    print(f"  Textures:             {stats['textures']:,}")
    print(f"  Sprites:              {stats['sprites']:,}")
    print(f"  Audio:                {stats['audio']:,}")
    print(f"  Fonts:                {stats['fonts']:,}")
    print(f"  Docs:                 {stats['docs']:,}")
    print(f"  Other:                {stats['other']:,}")
    if stats['skipped_irrelevant'] > 0:
        print(f"Skipped (irrelevant):   {stats['skipped_irrelevant']:,}")
    if stats['skipped_duplicate'] > 0:
        print(f"Skipped (duplicates):   {stats['skipped_duplicate']:,}")
    if stats['zips_extracted'] > 0:
        print(f"ZIPs extracted:         {stats['zips_extracted']:,}")

    print("\n✅ Assets organized under: assets/library/")
    print("📝 Next: run 'cargo build' to index assets into the database (idempotent).")
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
