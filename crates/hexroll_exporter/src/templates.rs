//! Template engine for generating ORM models using minijinja2

use anyhow::Result;
use minijinja::{Environment, context};
use std::path::{Path, PathBuf};
use serde::Serialize;

/// Template engine for ORM model generation
pub struct TemplateEngine {
    template_dir: PathBuf,
}

impl TemplateEngine {
    pub fn new() -> Self {
        // Templates are in the templates/ directory relative to the crate root
        let template_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("templates");
        Self { template_dir }
    }
    
    /// Generate an ORM model file using templates
    pub async fn generate_model<T: Serialize>(
        &self,
        model_type: &str,
        data: &[T],
        output_path: &Path,
    ) -> Result<()> {
        let template_file = format!("{}_model.rs.j2", model_type);
        let template_path = self.template_dir.join(&template_file);
        
        if !template_path.exists() {
            return Err(anyhow::anyhow!("Template not found: {}", template_file));
        }
        
        let template_content = std::fs::read_to_string(&template_path)?;
        let mut env = Environment::new();
        env.add_template(model_type, &template_content)?;
        
        let template = env.get_template(model_type)?;
        
        // Analyze data structure to infer field types for template
        let field_analysis = self.analyze_data_structure(data);
        
        let generated_code = template.render(context! {
            model_type => model_type,
            data_sample => data.first(),
            field_info => field_analysis,
            entity_count => data.len(),
        })?;
        
        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(output_path, generated_code)?;
        
        println!("✅ Generated {} model: {}", model_type, output_path.display());
        Ok(())
    }
    
    /// Generate the models module file
    pub async fn generate_models_module(&self, output_path: &Path) -> Result<()> {
        let template_path = self.template_dir.join("models_mod.rs.j2");
        
        if !template_path.exists() {
            return Err(anyhow::anyhow!("Models module template not found"));
        }
        
        let template_content = std::fs::read_to_string(&template_path)?;
        let mut env = Environment::new();
        env.add_template("models_mod", &template_content)?;
        
        let template = env.get_template("models_mod")?;
        
        let generated_code = template.render(context! {
            modules => vec!["settlements", "hex_tiles", "npcs", "dungeons", "items", "factions"],
        })?;
        
        std::fs::write(output_path, generated_code)?;
        
        println!("✅ Generated models module: {}", output_path.display());
        Ok(())
    }
    
    /// Analyze data structure to provide field information to templates
    fn analyze_data_structure<T: Serialize>(&self, data: &[T]) -> serde_json::Value {
        if let Some(sample) = data.first() {
            if let Ok(json_value) = serde_json::to_value(sample) {
                if let Some(obj) = json_value.as_object() {
                    let mut field_info = serde_json::Map::new();
                    
                    for (key, value) in obj {
                        let field_type = match value {
                            serde_json::Value::String(_) => "String",
                            serde_json::Value::Number(n) if n.is_i64() => "i32",
                            serde_json::Value::Number(n) if n.is_f64() => "f32",
                            serde_json::Value::Number(_) => "f64", // catch-all for other numbers
                            serde_json::Value::Bool(_) => "bool",
                            serde_json::Value::Array(_) => "Vec<String>",
                            serde_json::Value::Object(_) => "serde_json::Value",
                            serde_json::Value::Null => "Option<String>",
                        };
                        
                        field_info.insert(key.clone(), serde_json::Value::String(field_type.to_string()));
                    }
                    
                    return serde_json::Value::Object(field_info);
                }
            }
        }
        
        serde_json::Value::Object(serde_json::Map::new())
    }
}
