extends RefCounted
class_name QuestRepo

var base_path := "res://metadata/quests"
var quests_by_region := {}
var validation_errors := []

func load_region(region_id: String) -> void:
	var dir_path = base_path + "/" + region_id
	var qpath = dir_path + "/quests.json"
	if not FileAccess.file_exists(qpath):
		validation_errors.append("Missing quests.json for region: " + region_id)
		return
	var data = _read_json(qpath)
	if not data is Dictionary:
		validation_errors.append("Invalid quests.json for region: " + region_id)
		return
	quests_by_region[region_id] = data.get("quests", [])

func get_quests(region_id: String) -> Array:
	return quests_by_region.get(region_id, [])

func _read_json(path: String):
	var f = FileAccess.open(path, FileAccess.READ)
	if f == null:
		return {}
	var txt = f.get_as_text()
	var json = JSON.new()
	if json.parse(txt) != OK:
		return {}
	return json.data

