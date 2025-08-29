extends VBoxContainer

var _wb: Worldbuilder
var _repo_info: RichTextLabel
var _blend_picker: OptionButton
var _seed_line: LineEdit
var _region_line: LineEdit
var _result_line: LineEdit

func _ready():
	name = "Worldbuilder Preview"
	_repo_info = RichTextLabel.new()
	_repo_info.bbcode_enabled = true
	add_child(_repo_info)

	var h1 = HBoxContainer.new()
	_seed_line = LineEdit.new()
	_seed_line.placeholder_text = "seed (e.g., forest)"
	h1.add_child(_seed_line)
	_region_line = LineEdit.new()
	_region_line.placeholder_text = "region (optional)"
	h1.add_child(_region_line)
	add_child(h1)

	_blend_picker = OptionButton.new()
	add_child(_blend_picker)

	var gen_btn = Button.new()
	gen_btn.text = "Generate Name"
	gen_btn.pressed.connect(_on_generate)
	add_child(gen_btn)

	_result_line = LineEdit.new()
	_result_line.editable = false
	add_child(_result_line)

	_call_deferred("_initialize")

func _initialize():
	if Engine.has_singleton("Worldbuilder"):
		_wb = Engine.get_singleton("Worldbuilder")
	else:
		_wb = get_node_or_null("/root/Worldbuilder")
	if _wb == null:
		_repo_info.text = "[color=red]Worldbuilder autoload not found[/color]"
		return
	# Populate blends
	_blend_picker.clear()
	for key in _wb.blend_presets.keys():
		_blend_picker.add_item(str(key))
	# Repo stats
	var info = _wb.repo.validate_integrity()
	_repo_info.text = "Loaded seeds: %d, Norse themes: %d" % [
		int(info["statistics"]["total_seeds"]), int(info["statistics"]["total_norse_themes"])
	]

func _on_generate():
	if _wb == null:
		return
	var seed = _seed_line.text.strip_edges()
	if seed == "":
		return
	var region = _region_line.text.strip_edges()
	var key = _blend_picker.get_item_text(_blend_picker.get_selected()) if _blend_picker.get_item_count() > 0 else "peace"
	var blend = _wb.get_blend_for_key(key, region)
	var forge = _wb.forge
	var name = forge.forge(seed, blend, region)
	_result_line.text = name

