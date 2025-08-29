extends Resource
class_name HexTileData

@export var coordinate: String = "BASE"
@export var biome: String = "forest"
@export var dread_level: int = 0
@export var features: Array[String] = []
@export var distance_from_base: int = 0
@export var corruption_stage: int = 0
@export var has_encounter: bool = false

func get_horror_description() -> String:
    match dread_level:
        0:
            return "The area feels peaceful and untouched by darkness."
        1:
            return "Something feels slightly off about this place."
        2:
            return "An oppressive atmosphere weighs upon you."
        3:
            return "Terror seeps from every shadow."
        4:
            return "Reality itself seems to writhe in agony."
        _:
            return "The horror is beyond description."

func apply_corruption(delta_corruption: float) -> void:
    corruption_stage = min(corruption_stage + int(delta_corruption), 4)
    dread_level = corruption_stage

func has_feature(feature_name: String) -> bool:
    return feature_name in features
