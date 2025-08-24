//! Generate 3D models from RON definitions using blr
//! 
//! This module reads RON files and generates GLB models using the blr crate

use std::path::Path;
use serde::{Deserialize, Serialize};
use pyo3::Python;
use blr::{BlendProject, types::Object};
use crate::error::BlenderBridgeError;

/// Material definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialDef {
    pub name: String,
    pub base_color: (f32, f32, f32, f32),
    pub metallic: f32,
    pub roughness: f32,
}

/// Object definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectDef {
    #[serde(rename = "type")]
    pub object_type: String,
    pub name: String,
    #[serde(default)]
    pub location: (f32, f32, f32),
    #[serde(default = "default_scale")]
    pub scale: (f32, f32, f32),
    #[serde(default)]
    pub rotation: (f32, f32, f32),
    #[serde(default)]
    pub vertices: u32,
    #[serde(default)]
    pub segments: u32,
    #[serde(default)]
    pub rings: u32,
    #[serde(default)]
    pub subdivisions: u32,
    pub material: String,
    #[serde(default = "default_shading")]
    pub shading: String,
}

fn default_scale() -> (f32, f32, f32) {
    (1.0, 1.0, 1.0)
}

fn default_shading() -> String {
    "flat".to_string()
}

/// Model definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDef {
    pub name: String,
    pub materials: Vec<MaterialDef>,
    pub objects: Vec<ObjectDef>,
}

/// Generate a GLB model from a RON definition
pub fn generate_model_from_ron(ron_path: &Path, output_path: &Path) -> Result<(), BlenderBridgeError> {
    // Use the simple GLTF generator instead of blr
    crate::simple_gltf::generate_simple_gltf_from_ron(ron_path, output_path)
}

/// Generate a GLB model from a RON definition using blr (requires Blender)
#[allow(dead_code)]
pub fn generate_model_from_ron_with_blr(ron_path: &Path, output_path: &Path) -> Result<(), BlenderBridgeError> {
    // Read and parse RON file
    let ron_content = std::fs::read_to_string(ron_path)?;
    let model_def: ModelDef = ron::from_str(&ron_content)
        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to parse RON: {}", e)))?;
    
    Python::with_gil(|py| {
        // Create a new Blender project
        let _project = BlendProject::empty(py)
            .map_err(|e| BlenderBridgeError::BlrError(e.to_string()))?;
        
        // Create materials via Python script
        for mat_def in &model_def.materials {
            // Create material and set properties using Python script
            let script = format!(r#"
import bpy
mat = bpy.data.materials.new(name="{}")
mat.use_nodes = True
bsdf = mat.node_tree.nodes.get("Principled BSDF")
if bsdf:
    bsdf.inputs['Base Color'].default_value = ({}, {}, {}, {})
    bsdf.inputs['Metallic'].default_value = {}
    bsdf.inputs['Roughness'].default_value = {}
"#, mat_def.name, 
    mat_def.base_color.0, mat_def.base_color.1, mat_def.base_color.2, mat_def.base_color.3,
    mat_def.metallic, mat_def.roughness);
            
            py.run(&script, None, None)
                .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create material: {}", e)))?;
        }
        
        // Create objects
        for obj_def in &model_def.objects {
            // Create the appropriate primitive based on type
            let _obj = match obj_def.object_type.as_str() {
                "Cube" => {
                    Object::new_mesh_primitive_cube(
                        py, 
                        2.0,  // size
                        false,  // calc_uvs
                        [obj_def.location.0, obj_def.location.1, obj_def.location.2],
                        [obj_def.rotation.0, obj_def.rotation.1, obj_def.rotation.2],
                        [obj_def.scale.0, obj_def.scale.1, obj_def.scale.2],
                    )
                    .map_err(|e| BlenderBridgeError::BlrError(e.to_string()))?
                },
                "Cylinder" => {
                    let script = format!(r#"
import bpy
bpy.ops.mesh.primitive_cylinder_add(
    vertices={},
    location=({}, {}, {})
)
obj = bpy.context.active_object
obj.name = "{}"
"#, obj_def.vertices.max(3), 
    obj_def.location.0, obj_def.location.1, obj_def.location.2,
    obj_def.name);
                    
                    py.run(&script, None, None)
                        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create cylinder: {}", e)))?;
                    
                    // Get the created object
                    let obj_script = format!("bpy.data.objects['{}']", obj_def.name);
                    py.eval(&obj_script, None, None)
                        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to get object: {}", e)))?;
                    
                    continue; // Skip the rest for now as we handled it via script
                },
                "UVSphere" => {
                    let script = format!(r#"
import bpy
bpy.ops.mesh.primitive_uv_sphere_add(
    segments={},
    ring_count={},
    location=({}, {}, {})
)
obj = bpy.context.active_object
obj.name = "{}"
"#, obj_def.segments.max(3), obj_def.rings.max(3),
    obj_def.location.0, obj_def.location.1, obj_def.location.2,
    obj_def.name);
                    
                    py.run(&script, None, None)
                        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create UV sphere: {}", e)))?;
                    continue;
                },
                "IcoSphere" => {
                    let script = format!(r#"
import bpy
bpy.ops.mesh.primitive_ico_sphere_add(
    subdivisions={},
    location=({}, {}, {})
)
obj = bpy.context.active_object
obj.name = "{}"
"#, obj_def.subdivisions,
    obj_def.location.0, obj_def.location.1, obj_def.location.2,
    obj_def.name);
                    
                    py.run(&script, None, None)
                        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create ico sphere: {}", e)))?;
                    continue;
                },
                "Cone" => {
                    let script = format!(r#"
import bpy
bpy.ops.mesh.primitive_cone_add(
    vertices={},
    location=({}, {}, {})
)
obj = bpy.context.active_object
obj.name = "{}"
"#, obj_def.vertices.max(3),
    obj_def.location.0, obj_def.location.1, obj_def.location.2,
    obj_def.name);
                    
                    py.run(&script, None, None)
                        .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to create cone: {}", e)))?;
                    continue;
                },
                _ => {
                    return Err(BlenderBridgeError::BlrError(format!("Unknown object type: {}", obj_def.object_type)));
                }
            };
            
            // Object properties are set when creating the primitives
        }
        
        // Apply object transforms and materials via script
        let finalize_script = format!(r#"
import bpy
for obj_def in {:?}:
    obj = bpy.data.objects.get(obj_def['name'])
    if obj:
        # Set scale
        obj.scale = obj_def['scale']
        # Set rotation
        obj.rotation_euler = obj_def['rotation']
        # Apply material
        mat = bpy.data.materials.get(obj_def['material'])
        if mat:
            if obj.data.materials:
                obj.data.materials[0] = mat
            else:
                obj.data.materials.append(mat)
        # Set shading
        if obj_def['shading'] == 'flat':
            bpy.context.view_layer.objects.active = obj
            obj.select_set(True)
            bpy.ops.object.shade_flat()
            obj.select_set(False)
        elif obj_def['shading'] == 'smooth':
            bpy.context.view_layer.objects.active = obj
            obj.select_set(True)
            bpy.ops.object.shade_smooth()
            obj.select_set(False)
"#, model_def.objects.iter().map(|o| {
    format!("{{'name': '{}', 'scale': {:?}, 'rotation': {:?}, 'material': '{}', 'shading': '{}'}}",
        o.name, o.scale, o.rotation, o.material, o.shading)
}).collect::<Vec<_>>());
        
        py.run(&finalize_script, None, None)
            .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to finalize objects: {}", e)))?;
        
        // Export to GLB
        let export_script = format!(r#"
import bpy
bpy.ops.export_scene.gltf(
    filepath="{}",
    export_format='GLB',
    export_apply=True
)
"#, output_path.to_string_lossy());
        
        py.run(&export_script, None, None)
            .map_err(|e| BlenderBridgeError::BlrError(format!("Failed to export GLB: {}", e)))?;
        
        Ok(())
    })
}

/// Check if a RON model definition exists
pub fn ron_model_exists(ron_path: &Path) -> bool {
    ron_path.exists() && ron_path.extension().map_or(false, |ext| ext == "ron")
}
