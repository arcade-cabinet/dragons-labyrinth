extends Node

# Loads 3D transition scenes by name using res://metadata/transitions.json if present,
# falling back to res://scenes/transitions/<name>.tscn

const MANIFEST_PATH := "res://metadata/transitions.json"

var _manifest: Dictionary = {}

func _ready() -> void:
    _load_manifest()

func _load_manifest() -> void:
    _manifest = {}
    if FileAccess.file_exists(MANIFEST_PATH):
        var f := FileAccess.open(MANIFEST_PATH, FileAccess.READ)
        if f:
            var text := f.get_as_text()
            f.close()
            var data := JSON.parse_string(text)
            if typeof(data) == TYPE_DICTIONARY:
                _manifest = data

func transition_scene_path(name: String) -> String:
    if _manifest.has(name):
        var p = String(_manifest[name])
        if p.begins_with("res://") and ResourceLoader.exists(p):
            return p
    var fallback := "res://scenes/transitions/%s.tscn" % name
    if ResourceLoader.exists(fallback):
        return fallback
    return ""

func start_transition(name: String) -> bool:
    var path := transition_scene_path(name)
    if path == "":
        push_error("Transition scene not found for '" + name + "'")
        return false
    var err := get_tree().change_scene_to_file(path)
    if err != OK:
        push_error("Failed to change scene to " + path)
        return false
    return true


