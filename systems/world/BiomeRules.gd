# BiomeRules.gd - generated
extends Node

class_name BiomeRules

var ADJACENCY: Dictionary = {
"desert": [
"desert",
"plains"
],
"forest": [
"forest",
"plains",
"mountain"
],
"mountain": [
"mountain",
"forest",
"snow"
],
"plains": [
"plains",
"forest",
"coast"
],
"swamp": [
"swamp",
"forest",
"coast"
]
}
var MOVEMENT: Dictionary = {
"desert": {
"flying": 1.0,
"ground_mount": 1.1,
"walk": 1.2
},
"forest": {
"flying": 1.0,
"ground_mount": 1.1,
"walk": 1.2
},
"mountain": {
"flying": 1.0,
"ground_mount": 1.6,
"walk": 1.8
},
"plains": {
"flying": 1.0,
"ground_mount": 1.0,
"walk": 1.0
},
"swamp": {
"flying": 1.0,
"ground_mount": 1.3,
"walk": 1.5
}
}
var HAZARDS: Dictionary = {
"lava": {
"dps": 8.0
},
"swamp": {
"dps": 1.0
}
}
var PASSABILITY: Dictionary = {
"mountain": {
"flying": true,
"ground_mount": false,
"walk": false
},
"plains": {
"flying": true,
"ground_mount": true,
"walk": true
}
}
var LAYER_PRIORITIES: Dictionary = {
"base_terrain": 5,
"characters": 10,
"features": 7,
"overlays": 6,
"ui_elements": 15
}

func get_neighbors(biome: String) -> Array:
    return ADJACENCY.get(biome, [])

func movement_cost(biome: String, mode: String) -> float:
    var m := MOVEMENT.get(biome, {})
    if typeof(m) == TYPE_DICTIONARY and m.has(mode):
        return float(m[mode])
    return 1.0

func hazard_dps(biome: String) -> float:
    var h := HAZARDS.get(biome, {})
    if typeof(h) == TYPE_DICTIONARY and h.has("dps"):
        return float(h["dps"])
    return 0.0

func is_passable(biome: String, mode: String) -> bool:
    var p := PASSABILITY.get(biome, {})
    if typeof(p) == TYPE_DICTIONARY and p.has(mode):
        return bool(p[mode])
    return true

func layer_priority(layer: String) -> int:
    return int(LAYER_PRIORITIES.get(layer, 0))