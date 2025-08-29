extends RefCounted
class_name MapRepo

var base_path := "res://metadata/maps"
var atlas_by_region := {}
var validation_errors := []

func load_region(region_id: String) -> void:
	var dir_path = base_path + "/" + region_id
	var apath = dir_path + "/atlas.json"
	if not FileAccess.file_exists(apath):
		validation_errors.append("Missing atlas.json for region: " + region_id)
		return
	var data = _read_json(apath)
	if not data is Dictionary:
		validation_errors.append("Invalid atlas.json for region: " + region_id)
		return
	atlas_by_region[region_id] = data

func get_cell(region_id: String, x: int, y: int) -> Dictionary:
	var atlas = atlas_by_region.get(region_id, {})
	if atlas.is_empty():
		return {}
	var size = atlas.get("size", [0, 0])
	if x < 0 or y < 0 or x >= int(size[0]) or y >= int(size[1]):
		return {}
	# cells are stored as a flat list; compute index
	var idx = y * int(size[0]) + x
	var cells = atlas.get("cells", [])
	if idx < 0 or idx >= cells.size():
		return {}
	return cells[idx]

func get_features(region_id: String) -> Array:
	var atlas = atlas_by_region.get(region_id, {})
	return atlas.get("features", []) if atlas is Dictionary else []

func get_rivers(region_id: String) -> Array:
	var atlas = atlas_by_region.get(region_id, {})
	return atlas.get("rivers", []) if atlas is Dictionary else []

func get_roads(region_id: String) -> Array:
	var atlas = atlas_by_region.get(region_id, {})
	return atlas.get("roads", []) if atlas is Dictionary else []

func _read_json(path: String):
	var f = FileAccess.open(path, FileAccess.READ)
	if f == null:
		return {}
	var txt = f.get_as_text()
	var json = JSON.new()
	if json.parse(txt) != OK:
		return {}
	return json.data

