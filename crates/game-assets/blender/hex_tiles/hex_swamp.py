# Hex swamp tile: generates a hex tile with murky water and lily pads.
import bpy
import os
import math


def reset_scene():
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    for m in bpy.data.meshes:
        if m.users == 0:
            bpy.data.meshes.remove(m)
    for mat in bpy.data.materials:
        if mat.users == 0:
            bpy.data.materials.remove(mat)


def new_mat(name, rgba):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    bsdf.inputs['Base Color'].default_value = rgba
    bsdf.inputs['Metallic'].default_value = 0.0
    bsdf.inputs['Roughness'].default_value = 0.9
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_tile():
    master = bpy.data.objects.new("HexSwampTile", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_mud = new_mat("Mud", (0.1, 0.08, 0.07, 1.0))
    mat_water = new_mat("Water", (0.05, 0.15, 0.1, 0.8))
    mat_lily = new_mat("Lily", (0.15, 0.4, 0.2, 1.0))
    # base mud
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.5, 0.5, 0.1)
    base.data.materials.append(mat_mud)
    shade_flat(base)
    base.parent = master
    # water layer (slightly smaller to simulate puddle)
    water = add('cylinder', vertices=6, location=(0, 0, 0.1))
    water.scale = (0.45, 0.45, 0.05)
    water.data.materials.append(mat_water)
    shade_flat(water)
    water.parent = master
    # lily pads (simple circles)
    for ang in [30, 150, 270]:
        lily = add('circle', vertices=8, location=(0.25 * math.cos(math.radians(ang)), 0.25 * math.sin(math.radians(ang)), 0.12))
        lily.scale = (0.08, 0.08, 1.0)
        lily.data.materials.append(mat_lily)
        shade_flat(lily)
        lily.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "hex_swamp.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
tile = build_tile()
export_glb(tile)