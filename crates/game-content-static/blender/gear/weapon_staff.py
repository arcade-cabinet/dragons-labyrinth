# Weapon: Staff – builds a magical staff with a wooden shaft and a gem at the top.
import bpy
import os


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
    bsdf.inputs['Metallic'].default_value = 0.3
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


def build_staff():
    master = bpy.data.objects.new("Staff", None)
    bpy.context.collection.objects.link(master)
    mat_wood = new_mat("Wood", (0.3, 0.17, 0.08, 1.0))
    mat_gem = new_mat("Gem", (0.2, 0.4, 0.6, 1.0))
    # shaft
    shaft = add('cylinder', vertices=12, location=(0, 0, 0.3))
    shaft.scale = (0.03, 0.03, 0.6)
    shaft.data.materials.append(mat_wood)
    shade_flat(shaft)
    shaft.parent = master
    # gem holder (slightly wider at top)
    holder = add('cylinder', vertices=12, location=(0, 0, 0.65))
    holder.scale = (0.05, 0.05, 0.05)
    holder.data.materials.append(mat_wood)
    shade_flat(holder)
    holder.parent = master
    # gem (icosphere)
    bpy.ops.mesh.primitive_ico_sphere_add(subdivisions=2, location=(0, 0, 0.72))
    gem = bpy.context.active_object
    gem.scale = (0.08, 0.08, 0.08)
    gem.data.materials.append(mat_gem)
    shade_flat(gem)
    gem.parent = master
    return master


def export_glb(obj):
    outdir = bpy.path.abspath("//exports/")
    if not os.path.exists(outdir):
        os.makedirs(outdir, exist_ok=True)
    filepath = os.path.join(outdir, "weapon_staff.glb")
    bpy.ops.object.select_all(action='DESELECT')
    for ob in bpy.data.objects:
        if ob == obj or ob.parent == obj:
            ob.select_set(True)
    bpy.ops.export_scene.gltf(filepath=filepath, export_format='GLB', export_texcoords=False, export_normals=True, export_materials='EXPORT')
    print(f"Exported to {filepath}")


reset_scene()
staff = build_staff()
export_glb(staff)