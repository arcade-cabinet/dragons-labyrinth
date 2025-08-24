# Boss: Forsaken Knight – creates a rusted knight with sword and shield on a base and exports a GLB.
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


def new_mat(name, rgba, metallic=0.4, roughness=0.6):
    mat = bpy.data.materials.new(name=name)
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes.get("Principled BSDF")
    bsdf.inputs['Base Color'].default_value = rgba
    bsdf.inputs['Metallic'].default_value = metallic
    bsdf.inputs['Roughness'].default_value = roughness
    return mat


def shade_flat(obj):
    bpy.context.view_layer.objects.active = obj
    obj.select_set(True)
    bpy.ops.object.shade_flat()
    obj.select_set(False)


def add(objtype, **kwargs):
    getattr(bpy.ops.mesh, f'primitive_{objtype}_add')(**kwargs)
    return bpy.context.active_object


def build_knight():
    master = bpy.data.objects.new("ForsakenKnight", None)
    bpy.context.collection.objects.link(master)
    # materials
    mat_base = new_mat("Base", (0.12, 0.12, 0.15, 1.0), metallic=0.1, roughness=0.8)
    mat_armor = new_mat("Armor", (0.3, 0.32, 0.35, 1.0))
    mat_cloth = new_mat("Cloth", (0.2, 0.15, 0.25, 1.0))
    mat_metal = new_mat("Metal", (0.6, 0.6, 0.65, 1.0), metallic=0.8, roughness=0.4)
    # base
    base = add('cylinder', vertices=8, location=(0, 0, 0))
    base.scale = (0.5, 0.5, 0.1)
    base.data.materials.append(mat_base)
    shade_flat(base)
    base.parent = master
    # torso armour
    torso = add('cylinder', vertices=12, location=(0, 0, 0.2))
    torso.scale = (0.25, 0.25, 0.35)
    torso.data.materials.append(mat_armor)
    shade_flat(torso)
    torso.parent = master
    # head/helm
    bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=1, location=(0, 0, 0.55))
    head = bpy.context.active_object
    head.scale = (0.14, 0.14, 0.14)
    head.data.materials.append(mat_armor)
    shade_flat(head)
    head.parent = master
    # shoulder pads
    for offset in [0.22, -0.22]:
        shoulder = add('uv_sphere', location=(offset, 0, 0.45))
        shoulder.scale = (0.12, 0.07, 0.07)
        shoulder.data.materials.append(mat_armor)
        shade_flat(shoulder)
        shoulder.parent = master
    # arms
    for offset in [0.32, -0.32]:
        arm = add('cylinder', vertices=8, location=(offset, 0, 0.3))
        arm.scale = (0.05, 0.05, 0.25)
        arm.rotation_euler = (0, math.radians(90), math.radians(15 if offset>0 else -15))
        arm.data.materials.append(mat_armor)
        shade_flat(arm)
        arm.parent = master
    # cloak tail
    cloak = add('cone', vertices=12, location=(0, 0, 0.15))
    cloak.scale = (0.25, 0.25, 0.3)
    cloak.data.materials.append(mat_cloth)
    shade_flat(cloak)
    cloak.parent = master
    # sword (right hand)
    blade = add('cone', vertices=12, location=(0.38, 0, 0.25))
    blade.scale = (0.05, 0.05, 0.4)
    blade.rotation_euler = (0, math.radians(90), 0)
    blade.data.materials.append(mat_metal)
    shade_flat(blade)
    blade.parent = master
    grip = add('cylinder', vertices=12, location=(0.3, 0, 0.25))
    grip.scale = (0.02, 0.02, 0.12)
    grip.rotation_euler = (0, math.radians(90), 0)
    grip.data.materials.append(mat_cloth)
    shade_flat(grip)
    grip.parent = master
    # shield (left hand)
    shield = add('cylinder', vertices=16, location=(-0.38, 0, 0.3))
    shield.scale = (0.15, 0.02, 0.15)
    shield.rotation_euler = (math.radians(90), 0, 0)
    shield.data.materials.append(mat_armor)
    shade_flat(shield)
    shield.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "forsaken_knight.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
knight = build_knight()
export_glb(knight)