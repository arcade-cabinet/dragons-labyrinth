extends EditorPlugin

var _dock

func _enter_tree():
	if not ProjectSettings.has_setting("autoload/Worldbuilder"):
		ProjectSettings.set_setting("autoload/Worldbuilder", "res://addons/worldbuilder/Worldbuilder.gd")
		ProjectSettings.save()
	# Editor Preview Dock
	var dock_path = "res://addons/worldbuilder/editor/WorldbuilderDock.gd"
	if ResourceLoader.exists(dock_path):
		_dock = load(dock_path).new()
		add_control_to_dock(EditorPlugin.DOCK_SLOT_RIGHT_UL, _dock)

func _exit_tree():
	if _dock:
		remove_control_from_docks(_dock)
		_dock.queue_free()
		_dock = null
