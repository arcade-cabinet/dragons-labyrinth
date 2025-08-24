# Gear: Light armour – creates a simple chest piece for the villager models.
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
    bsdf.inputs['Metallic'].default_value = 0.4
    bsdf.inputs['Roughness'].default_value = 0.6
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_armor():
    master = bpy.data.objects.new("LightArmor", None)
    bpy.context.collection.objects.link(master)
    mat_armor = new_mat("Armor", (0.4, 0.4, 0.45, 1.0))
    # chest plate (torso)
    chest = add('cylinder', vertices=12, location=(0, 0, 0))
    chest.scale = (0.16, 0.12, 0.22)
    chest.data.materials.append(mat_armor)
    shade_flat(chest)
    chest.parent = master
    # shoulder pads
    for offset in [0.2, -0.2]:
        pad = add('cylinder', vertices=8, location=(offset, 0, 0.08))
        pad.scale = (0.08, 0.05, 0.05)
        pad.data.materials.append(mat_armor)
        shade_flat(pad)
        pad.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "armor_light.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
armor = build_armor()
export_glb(armor)