# Gear: Helmet – builds a simple helmet that can be placed on a villager model.
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
    bsdf.inputs['Metallic'].default_value = 0.5
    bsdf.inputs['Roughness'].default_value = 0.5
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_helmet():
    master = bpy.data.objects.new("Helmet", None)
    bpy.context.collection.objects.link(master)
    mat_metal = new_mat("HelmetMetal", (0.5, 0.5, 0.55, 1.0))
    # main helmet shell
    shell = add('cylinder', vertices=16, location=(0, 0, 0))
    shell.scale = (0.15, 0.15, 0.12)
    shell.data.materials.append(mat_metal)
    shade_flat(shell)
    shell.parent = master
    # brim (slightly wider disc)
    brim = add('cylinder', vertices=16, location=(0, 0, -0.1))
    brim.scale = (0.18, 0.18, 0.02)
    brim.data.materials.append(mat_metal)
    shade_flat(brim)
    brim.parent = master
    # crest (small cone on top)
    crest = add('cone', vertices=8, location=(0, 0, 0.12))
    crest.scale = (0.05, 0.05, 0.1)
    crest.data.materials.append(mat_metal)
    shade_flat(crest)
    crest.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "helmet.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
helmet = build_helmet()
export_glb(helmet)