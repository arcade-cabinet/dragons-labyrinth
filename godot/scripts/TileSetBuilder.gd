extends Node

# Builds a TileSet from PNG assets listed in res://metadata/manifest.json
# Only includes entries under the given category (e.g., "biomes").
# Saves to res://tilesets/<category>.tres

const TILE_SIZE := Vector2i(128, 128)

func build_tileset(category: String) -> String:
    var manifest_path := "res://metadata/manifest.json"
    if not FileAccess.file_exists(manifest_path):
        return ""
    var f := FileAccess.open(manifest_path, FileAccess.READ)
    if f == null:
        return ""
    var text := f.get_as_text()
    f.close()
    var data := JSON.parse_string(text)
    if typeof(data) != TYPE_DICTIONARY:
        return ""
    var entries: Array = []
    if data.has(category):
        entries = data[category]
    if entries.is_empty():
        return ""

    var pngs: Array = []
    for rel in entries:
        if typeof(rel) == TYPE_STRING and rel.ends_with(".png") and ("/" + category + "/") in ("/" + rel):
            pngs.append(rel)
    if pngs.is_empty():
        return ""

    var ts := TileSet.new()
    ts.tile_shape = TileSet.TILE_SHAPE_HEXAGON

    for rel_path in pngs:
        var tex_path := "res://assets/" + String(rel_path)
        var tex: Texture2D = load(tex_path)
        if tex == null:
            continue
        var src := TileSetAtlasSource.new()
        src.texture = tex
        src.margins = Rect2i(0, 0, 0, 0)
        src.separation = Vector2i(0, 0)
        src.texture_region_size = TILE_SIZE
        # Single-tile atlas at origin
        src.create_tile(Vector2i(0, 0))
        var sid := ts.get_last_unused_source_id()
        ts.add_source(src, sid)

    var out_dir := "res://tilesets"
    DirAccess.make_dir_recursive_absolute("res://tilesets")
    var out_path := out_dir + "/" + category + ".tres"
    var err := ResourceSaver.save(ts, out_path)
    if err != OK:
        return ""
    return out_path


