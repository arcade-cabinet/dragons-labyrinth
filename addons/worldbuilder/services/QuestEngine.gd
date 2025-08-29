extends Node
class_name QuestEngine

signal quest_started(quest_id)
signal quest_advanced(quest_id, beat)
signal quest_completed(quest_id)

var repo: QuestRepo
var active := {}

func _ready():
	if repo == null:
		repo = QuestRepo.new()

func start(quest_id: String, region_id: String) -> void:
	active[quest_id] = {"region": region_id, "idx": 0}
	emit_signal("quest_started", quest_id)

func advance(event: String, quest_id: String) -> void:
	if not active.has(quest_id):
		return
	var ctx = active[quest_id]
	var region_id = ctx["region"]
	var quests = repo.get_quests(region_id)
	for q in quests:
		if str(q.get("id", "")) == quest_id:
			var idx = int(ctx["idx"])
			if idx < q.get("beats", []).size():
				var beat = q["beats"][idx]
				ctx["idx"] = idx + 1
				emit_signal("quest_advanced", quest_id, beat)
				if ctx["idx"] >= q["beats"].size():
					emit_signal("quest_completed", quest_id)
					active.erase(quest_id)
			return

