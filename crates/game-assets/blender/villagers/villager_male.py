# Male villager chess‑piece: builds a low‑poly male villager on a hex base and exports a GLB.
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


def build_villager():
    master = bpy.data.objects.new("VillagerMale", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Base", (0.12, 0.12, 0.15, 1.0))
    mat_skin = new_mat("Skin", (0.9, 0.72, 0.58, 1.0))
    mat_tunic = new_mat("Tunic", (0.2, 0.25, 0.4, 1.0))
    mat_pants = new_mat("Pants", (0.15, 0.18, 0.25, 1.0))
    mat_hat = new_mat("Hat", (0.2, 0.15, 0.1, 1.0))
    # hex base
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.6, 0.6, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # body (tunic)
    body = add('cylinder', vertices=12, location=(0, 0, 0.2))
    body.scale = (0.22, 0.22, 0.35)
    body.data.materials.append(mat_tunic)
    shade_flat(body)
    body.parent = master
    # legs (pants)
    for offset in [0.08, -0.08]:
        leg = add('cylinder', vertices=8, location=(offset, 0, 0.05))
        leg.scale = (0.06, 0.06, 0.2)
        leg.data.materials.append(mat_pants)
        shade_flat(leg)
        leg.parent = master
    # head (sphere)
    bpy.ops.mesh.primitive_uv_sphere_add(segments=16, ring_count=8, location=(0, 0, 0.55))
    head = bpy.context.active_object
    head.scale = (0.12, 0.12, 0.12)
    head.data.materials.append(mat_skin)
    shade_flat(head)
    head.parent = master
    # arms
    for angle, side in [(0.35, 'left'), (-0.35, 'right')]:
        arm = add('cylinder', vertices=8, location=(angle, 0, 0.3))
        arm.scale = (0.05, 0.05, 0.25)
        arm.rotation_euler = (0, math.radians(90), 0)
        arm.data.materials.append(mat_tunic)
        shade_flat(arm)
        arm.parent = master
    # hat (simple cone)
    hat = add('cone', vertices=12, location=(0, 0, 0.68))
    hat.scale = (0.15, 0.15, 0.15)
    hat.data.materials.append(mat_hat)
    shade_flat(hat)
    hat.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "villager_male.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
villager = build_villager()
export_glb(villager)