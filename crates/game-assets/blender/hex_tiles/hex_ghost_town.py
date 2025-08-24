# Hex ghost town tile: generates a hex tile with a floating cube to suggest a distorted environment.
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
    master = bpy.data.objects.new("HexGhostTownTile", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Base", (0.2, 0.2, 0.25, 1.0))
    mat_float = new_mat("Float", (0.4, 0.4, 0.45, 1.0))
    # base tile
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.5, 0.5, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # upper layer (to give thickness)
    top = add('cylinder', vertices=6, location=(0, 0, 0.1))
    top.scale = (0.5, 0.5, 0.05)
    top.data.materials.append(mat_base)
    shade_flat(top)
    top.parent = master
    # floating cube representing ghostly distortion
    cube = add('cube', size=0.2, location=(0.0, 0.0, 0.3))
    cube.data.materials.append(mat_float)
    shade_flat(cube)
    cube.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "hex_ghost_town.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
tile = build_tile()
export_glb(tile)