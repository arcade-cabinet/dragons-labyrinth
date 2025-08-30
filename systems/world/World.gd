extends Node2D

func _ready() -> void:
    print("World ready - loading manifest and drawing a small grid...")
    _load_assets_and_paint()

func _load_assets_and_paint() -> void:
    var manifest_path := "res://metadata/manifest.json"
    if not FileAccess.file_exists(manifest_path):
        print("No manifest found yet at", manifest_path)
        return

    var f := FileAccess.open(manifest_path, FileAccess.READ)
    if f == null:
        push_error("Failed to open manifest")
        return
    var text := f.get_as_text()
    f.close()

    var data := JSON.parse_string(text)
    if typeof(data) != TYPE_DICTIONARY:
        push_error("Manifest is not a dictionary")
        return

    # Find first category with any assets
    var first_paths: Array = []
    for k in data.keys():
        var arr = data[k]
        if typeof(arr) == TYPE_ARRAY and arr.size() > 0:
            first_paths = arr
            break
    if first_paths.is_empty():
        print("Manifest has no assets yet")
        return

    # Try to build/load a TileSet for a plausible category
    var category := _guess_category(first_paths)
    if category != "":
        var builder := preload("res://scripts/TileSetBuilder.gd").new()
        var ts_path := builder.build_tileset(category)
        if ts_path != "":
            var tilemap := $TileMap
            if tilemap:
                tilemap.tile_set = load(ts_path)
                tilemap.cell_quadrant_size = 64
                # Paint a small hex grid
                for q in range(0, 5):
                    for r in range(0, 5):
                        tilemap.set_cell(Vector2i(q, r), 0, Vector2i(0, 0))
                return

    # Fallback: load first few textures as proof and draw them via Sprite2D grid
    _spawn_sprites_grid(first_paths)

func _spawn_sprites_grid(paths: Array) -> void:
    var count := min(9, paths.size())
    var cols := 3
    var spacing := Vector2(140, 140)
    for i in range(count):
        var rel_path: String = paths[i]
        var tex_path := "res://assets/" + rel_path
        var tex: Texture2D = load(tex_path)
        if tex == null:
            print("Failed to load", tex_path)
            continue
        var spr := Sprite2D.new()
        spr.texture = tex
        var row := i / cols
        var col := i % cols
        spr.position = Vector2(col * spacing.x, row * spacing.y)
        add_child(spr)

func _guess_category(paths: Array) -> String:
    # Paths look like "<category>/<archetype>/<file>.png"
    for p in paths:
        if typeof(p) == TYPE_STRING and "/" in p:
            return p.split("/")[0]
    return ""

func start_transition(name: String) -> void:
    var loader := preload("res://scripts/TransitionLoader.gd").new()
    if not loader.start_transition(name):
        print("Transition failed:", name)
