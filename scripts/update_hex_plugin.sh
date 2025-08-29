#!/usr/bin/env bash
set -euo pipefail
REPO=https://github.com/Zehir/godot-hexagon-tile-map-layer.git
PREFIX=vendor/godot-hexagon-tile-map-layer
BRANCH=main

echo 'Updating subtree...'
git subtree pull --prefix "$PREFIX" "$REPO" "$BRANCH" --squash

echo 'Syncing addon to godot/addons/hexagon_tilemaplayer...'
rsync -a --delete "$PREFIX"/addons/hexagon_tilemaplayer/ godot/addons/hexagon_tilemaplayer/

echo 'Done.'
