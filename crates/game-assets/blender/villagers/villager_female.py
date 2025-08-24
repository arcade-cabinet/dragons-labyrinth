# Female villager chess‑piece: builds a low‑poly female villager with a dress and hair, then exports a GLB.
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
    master = bpy.data.objects.new("VillagerFemale", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Base", (0.12, 0.12, 0.15, 1.0))
    mat_skin = new_mat("Skin", (0.92, 0.75, 0.6, 1.0))
    mat_dress = new_mat("Dress", (0.3, 0.2, 0.4, 1.0))
    mat_apron = new_mat("Apron", (0.8, 0.8, 0.85, 1.0))
    mat_hair = new_mat("Hair", (0.3, 0.15, 0.1, 1.0))
    # base
    base = add('cylinder', vertices=6, location=(0, 0, 0))
    base.scale = (0.6, 0.6, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # dress (cone skirt)
    skirt = add('cone', vertices=16, location=(0, 0, 0.15))
    skirt.scale = (0.3, 0.3, 0.25)
    skirt.data.materials.append(mat_dress)
    shade_flat(skirt)
    skirt.parent = master
    # torso above skirt
    torso = add('cylinder', vertices=12, location=(0, 0, 0.32))
    torso.scale = (0.18, 0.18, 0.25)
    torso.data.materials.append(mat_dress)
    shade_flat(torso)
    torso.parent = master
    # apron (front panel) as a plane extruded
    apron = add('cube', size=0.1, location=(0, -0.15, 0.3))
    apron.scale = (0.8, 0.1, 0.6)
    apron.data.materials.append(mat_apron)
    shade_flat(apron)
    apron.parent = master
    # head
    bpy.ops.mesh.primitive_uv_sphere_add(segments=16, ring_count=8, location=(0, 0, 0.6))
    head = bpy.context.active_object
    head.scale = (0.12, 0.12, 0.12)
    head.data.materials.append(mat_skin)
    shade_flat(head)
    head.parent = master
    # hair (simple hemispheric cap)
    hair_cap = add('cone', vertices=16, location=(0, 0, 0.72))
    hair_cap.scale = (0.18, 0.18, 0.15)
    hair_cap.data.materials.append(mat_hair)
    shade_flat(hair_cap)
    hair_cap.parent = master
    # arms (narrow cylinders)
    for offset in [0.28, -0.28]:
        arm = add('cylinder', vertices=8, location=(offset, 0, 0.4))
        arm.scale = (0.045, 0.045, 0.25)
        arm.rotation_euler = (0, math.radians(90), 0)
        arm.data.materials.append(mat_dress)
        shade_flat(arm)
        arm.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "villager_female.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
villager = build_villager()
export_glb(villager)