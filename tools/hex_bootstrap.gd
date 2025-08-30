# Godot 4.4 — run with:
# godot4 --headless --script res://tools/hex_bootstrap.gd -- \
#   atlas=res://art/biomes_atlas.png tile_w=512 tile_h=512 \
#   layout=res://config/atlas_layout.json map=res://maps/demo_axial.json \
#   out_tileset=res://tilesets/biomes_hex_tileset.tres \
#   out_scene=res://scenes/HexWorld.tscn \
#   orientation=pointy # or flat

extends SceneTree

func _initialize():
	# Parse CLI args
	var args := OS.get_cmdline_user_args()
	var kv := {}
	for a in args:
		if a.contains("="):
			var p = a.split("=")
			if p.size() == 2:
				kv[p[0]] = p[1]

	var atlas_path      := kv.get("atlas", "res://art/biomes_atlas.png")
	var tile_w          := int(kv.get("tile_w", "512"))
	var tile_h          := int(kv.get("tile_h", "512"))
	var layout_json     := kv.get("layout", "res://config/atlas_layout.json")
	var map_json        := kv.get("map", "res://maps/demo_axial.json")
	var out_tileset     := kv.get("out_tileset", "res://tilesets/biomes_hex_tileset.tres")
	var out_scene       := kv.get("out_scene", "res://scenes/HexWorld.tscn")
	var orientation_str := kv.get("orientation", "pointy") # or "flat"

	# 1) Ensure Zehir plugin is enabled (so TileMapLayer class is available headless)
	_enable_plugin("res://addons/hexagon_tile_map_layer/plugin.cfg")

	# 2) Build the TileSet from atlas + layout json
	var biome_to_atlas: Dictionary = _load_json(layout_json)
	if biome_to_atlas.is_empty():
		printerr("❌ atlas_layout.json not found or empty: ", layout_json)
		quit(1)

	var tileset := _build_tileset(atlas_path, Vector2i(tile_w, tile_h), biome_to_atlas)
	var save_err := ResourceSaver.save(tileset, out_tileset)
	if save_err != OK:
		printerr("❌ Failed saving TileSet: ", out_tileset, " err=", save_err)
		quit(1)
	print("✅ Saved TileSet: ", out_tileset)

	# 3) Create a scene with TileMapLayer, assign tileset, set hex orientation
	var root := Node.new()
	root.name = "HexWorld"
	var tilemap_layer := _new_hex_layer()
	root.add_child(tilemap_layer)

	tilemap_layer.tile_set = tileset
	tilemap_layer.tile_shape = TileMap.TILE_SHAPE_HEXAGON

	if orientation_str.to_lower() == "flat":
		tilemap_layer.hex_side_length = 0 # use defaults; engine handles projection
		tilemap_layer.layout = TileMap.LAYOUT_HEXAGONAL_FLAT_TOP
	else:
		tilemap_layer.layout = TileMap.LAYOUT_HEXAGONAL_POINTY_TOP

	# 4) Paint cells from axial map (q,r) -> biome name
	var axial_map: Dictionary = _load_json(map_json)
	if axial_map.is_empty():
		print("⚠️ Map JSON empty; writing an empty scene")

	# find source_id (first source)
	var source_id := -1
	for id in tileset.get_source_ids():
		source_id = id
		break

	for key in axial_map.keys():
		var parts := str(key).split(",")
		if parts.size() != 2:
			continue
		var q := int(parts[0])
		var r := int(parts[1])
		var biome := str(axial_map[key]).to_lower()

		if not biome_to_atlas.has(biome):
			printerr("⚠️ Unknown biome in map: ", biome)
			continue

		var atlas_coords := _to_vec2i(biome_to_atlas[biome])
		tilemap_layer.set_cell(Vector2i(q, r), source_id, atlas_coords, 0)

	# 5) Save the scene
	var packed := PackedScene.new()
	var pack_ok := packed.pack(root)
	if pack_ok != OK:
		printerr("❌ Failed to pack scene: ", pack_ok)
		quit(1)

	var scene_err := ResourceSaver.save(packed, out_scene)
	if scene_err != OK:
		printerr("❌ Failed saving scene: ", out_scene, " err=", scene_err)
		quit(1)

	print("✅ Saved Scene: ", out_scene)
	quit(0)

func _enable_plugin(cfg_path: String) -> void:
	var cfg := ConfigFile.new()
	var proj_path := "res://project.godot"
	var err := cfg.load(proj_path)
	if err != OK:
		printerr("❌ Cannot load project.godot to enable plugin")
		return
	cfg.set_value("editor_plugins", "enabled", PackedStringArray([cfg_path]))
	cfg.save(proj_path)

func _build_tileset(atlas_path: String, cell_size: Vector2i, biome_to_atlas: Dictionary) -> TileSet:
	var ts := TileSet.new()
	var src := TileSetAtlasSource.new()
	src.texture = load(atlas_path)
	src.texture_region_size = cell_size
	ts.add_source(src)
	var source_id := ts.get_last_added_source_id()

	for biome in biome_to_atlas.keys():
		var v := _to_vec2i(biome_to_atlas[biome])
		src.create_tile(v) # alt 0
		var td := (ts.get_source(source_id) as TileSetAtlasSource).get_tile_data(v, 0)
		td.set_custom_data("biome", str(biome).to_lower())

	return ts

func _load_json(path: String) -> Dictionary:
	var f := FileAccess.open(path, FileAccess.READ)
	if f == null:
		return {}
	var txt := f.get_as_text(); f.close()
	var data = JSON.parse_string(txt)
	return (typeof(data) == TYPE_DICTIONARY) ? data : {}

func _to_vec2i(x):
	if x is Array and x.size() >= 2:
		return Vector2i(int(x[0]), int(x[1]))
	if x is Vector2i:
		return x
	return Vector2i(0, 0)

func _new_hex_layer() -> Node:
	# Class is provided by Zehir's plugin; registered as "TileMapLayer"
	var n := ClassDB.instantiate("TileMapLayer")
	if n == null:
		# fallback to built-in TileMap if plugin missing
		n = TileMap.new()
	n.name = "Hex"
	return n