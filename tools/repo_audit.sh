#!/usr/bin/env bash
set -euo pipefail

# --- Patch queue: auto-apply repo_diffs.patch if present --------------------
apply_patch_queue() {
  # Find repo root even if script is run from a subdir
  local root
  if root=$(git rev-parse --show-toplevel 2>/dev/null); then
    :
  else
    root=$(pwd)
  fi

  local patch_file="$root/repo_diffs.patch"
  local completed_dir="$root/audit/completed_patches"

  # Auto-stash any local work before applying the patch
  local stashed=0
  if command -v git >/dev/null 2>&1 && git -C "$root" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    if ! git -C "$root" diff --quiet || ! git -C "$root" diff --cached --quiet; then
      echo "[patch-queue] Detected local changes — creating autostash..."
      git -C "$root" stash push -u -k -m "[patch-queue] autostash before applying repo_diffs.patch $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
      stashed=1
    fi
  fi

  # Nothing to do if file missing or empty
  if [[ ! -s "$patch_file" ]]; then
    echo "[patch-queue] No repo_diffs.patch to apply (missing or empty)."
    return 0
  fi

  echo "[patch-queue] Found $patch_file — attempting to apply..."
  mkdir -p "$completed_dir"

  # Try git apply first (respects index), fall back to `patch`
  if git apply --index --whitespace=fix --reject "$patch_file" 2>/dev/null; then
    echo "[patch-queue] Applied with git apply."
  else
    echo "[patch-queue] git apply failed; attempting POSIX patch..."
    # `-p1` assumes diffs are relative to repo root
    if command -v patch >/dev/null 2>&1; then
      (cd "$root" && patch -p1 --backup --verbose < "$patch_file")
      echo "[patch-queue] Applied with patch(1)."
    else
      echo "[patch-queue] ERROR: neither git apply (successfully) nor patch(1) available. Aborting." >&2
      exit 1
    fi
  fi

  # Rotate the patch into completed bucket with UTC timestamp
  local ts
  ts=$(date -u +"%Y%m%dT%H%M%SZ")
  local dest="$completed_dir/repo_diffs-$ts.patch"
  mv "$patch_file" "$dest"
  echo "[patch-queue] Moved applied patch to $dest"

  # Recreate a blank patch file for the next batch
  : > "$patch_file"
  echo "[patch-queue] Created fresh blank repo_diffs.patch"

  # Restore any autostashed work
  if [[ "$stashed" -eq 1 ]]; then
    echo "[patch-queue] Restoring working tree from autostash..."
    if ! git -C "$root" stash pop --index; then
      echo "[patch-queue] WARNING: 'git stash pop' reported conflicts. Resolve and continue." >&2
    fi
  fi
}

apply_patch_queue

# --- End patch queue ---------------------------------------------------------

from typing import Any

def load_entities_from_sqlite(db_path: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    # ...

def _is_hex_cell(v: dict[str, Any]) -> bool:
    # ...

def _biome_of(v: dict[str, Any]) -> str:
    # ...

def _edge_list(v: dict[str, Any], key: str) -> list[int]:
    out: list[int] = []
    # ...

def _classify_feature(v: dict[str, Any]) -> str:
    # ...

def build_world_outputs(
    db_path: str = "",
    raw_json_entities: list[dict[str, Any]] | None = None,
) -> dict[str, Any]:
    entities: list[dict[str, Any]] = raw_json_entities if raw_json_entities is not None else ([] if not db_path else load_entities_from_sqlite(db_path))
    biomes_layer: list[dict[str, Any]] = []
    rivers_layer: list[dict[str, Any]] = []
    trails_layer: list[dict[str, Any]] = []
    settlements_layer: list[dict[str, Any]] = []
    dungeons_layer: list[dict[str, Any]] = []

    biomes_coll: list[dict[str, Any]] = []
    regions_coll: list[dict[str, Any]] = []
    settlements_coll: list[dict[str, Any]] = []
    dungeons_coll: list[dict[str, Any]] = []
    hexcells_coll: list[dict[str, Any]] = []
    rivers_coll: list[dict[str, Any]] = []
    trails_coll: list[dict[str, Any]] = []
    tokens_coll: list[dict[str, Any]] = []
    # ...

pairs: list[tuple[str, str]] = []
