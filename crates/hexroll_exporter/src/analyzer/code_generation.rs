//! AI-powered code generation for SeaORM models and minijinja templates

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use ai_bridge::{
    agent_spec::{AgentSpec, AgentSpecLoader, AgentConfigValue},
    agent_executor::{AgentExecutor, AgentExecutionRequest},
    context::BuildContext,
    openai_client::OpenAIClient,
};

use super::types::{AnalysisReport, TableInfo, ColumnInfo};

/// AI-powered code generator for SeaORM models and templates
pub struct AICodeGenerator {
    agent_spec: AgentSpec,
    executor: AgentExecutor,
    build_context: BuildContext,
}

impl AICodeGenerator {
    /// Create new AI code generator
    pub async fn new<P: AsRef<Path>>(agent_spec_path: P) -> Result<Self> {
        println!("🤖 Loading code generation agent specification...");
        
        // Load the agent specification
        let mut loader = AgentSpecLoader::new();
        loader.load_spec_from_file(agent_spec_path)
            .map_err(|e| anyhow::anyhow!("Failed to load agent spec: {}", e))?;
        
        // Get the loaded spec
        let agent_spec = loader.get_spec("hbf-analyzer")
            .ok_or_else(|| anyhow::anyhow!("hbf-analyzer spec not found after loading"))?
            .clone();
        
        // Create OpenAI client and build context
        let openai_client = OpenAIClient::new()?;
        let executor = AgentExecutor::new(openai_client);
        let build_context = BuildContext::new("./generated_code")?;
        
        println!("   ✅ Code generator ready");
        
        Ok(Self { agent_spec, executor, build_context })
    }
    
    /// Generate SeaORM models based on HBF analysis
    pub async fn generate_seaorm_models(&mut self, report: &AnalysisReport) -> Result<Vec<GeneratedModel>> {
        println!("🤖 Generating SeaORM models from HBF analysis...");
        
        let mut generated_models = Vec::new();
        
        for (table_name, table_info) in &report.table_info {
            println!("   🏗️  Generating model for table: {}", table_name);
            
            // Analyze HTML content patterns in the table
            let html_patterns = self.analyze_html_patterns_in_table(table_info).await?;
            
            // Generate SeaORM model
            let model = self.generate_model_for_table(table_name, table_info, &html_patterns, report).await?;
            generated_models.push(model);
        }
        
        println!("✅ Generated {} SeaORM models", generated_models.len());
        Ok(generated_models)
    }
    
    /// Analyze HTML patterns in a table using AI
    async fn analyze_html_patterns_in_table(&mut self, table_info: &TableInfo) -> Result<Vec<HTMLPattern>> {
        println!("     🔍 AI analyzing HTML patterns...");
        
        let mut html_patterns = Vec::new();
        
        // Extract HTML content from sample data
        let html_samples = self.extract_html_samples_from_table(table_info)?;
        
        if !html_samples.is_empty() {
            // Use AI to analyze HTML patterns
            let patterns = self.ai_analyze_html_fragments(&html_samples).await?;
            html_patterns.extend(patterns);
        }
        
        Ok(html_patterns)
    }
    
    /// Extract HTML samples from table data
    fn extract_html_samples_from_table(&self, table_info: &TableInfo) -> Result<Vec<String>> {
        let mut html_samples = Vec::new();
        
        if let Some(sample_data) = &table_info.sample_data {
            if let Some(rows) = sample_data.as_array() {
                for row in rows {
                    if let Some(row_obj) = row.as_object() {
                        for (col_name, value) in row_obj {
                            if let Some(text_value) = value.as_str() {
                                if text_value.contains("<") && text_value.contains(">") && text_value.len() > 50 {
                                    html_samples.push(format!("Column {}: {}", col_name, text_value));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(html_samples)
    }
    
    /// Use AI to analyze HTML fragments and extract patterns
    async fn ai_analyze_html_fragments(&mut self, html_samples: &[String]) -> Result<Vec<HTMLPattern>> {
        let html_content = html_samples.join("\n\n---\n\n");
        
        // Create execution request for HTML analysis
        let mut inputs = HashMap::new();
        inputs.insert("html_content".to_string(), AgentConfigValue::String(html_content));
        inputs.insert("analysis_type".to_string(), AgentConfigValue::String("pattern_extraction".to_string()));
        
        let exec_request = AgentExecutionRequest {
            agent_name: self.agent_spec.metadata.name.clone(),
            inputs,
            context: HashMap::new(),
            config_overrides: HashMap::new(),
            prompt_template: Some("html_parsing".to_string()),
        };
        
        // Execute HTML pattern analysis
        match self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await {
            Ok(result) => {
                if let Some(analysis_text) = result.outputs.get("result").and_then(|v| v.as_string()) {
                    self.parse_html_patterns_from_ai_response(analysis_text)
                } else {
                    Ok(Vec::new())
                }
            }
            Err(e) => {
                println!("     ⚠️  AI HTML analysis failed: {}", e);
                Ok(Vec::new())
            }
        }
    }
    
    /// Parse HTML patterns from AI response
    fn parse_html_patterns_from_ai_response(&self, response: &str) -> Result<Vec<HTMLPattern>> {
        let mut patterns = Vec::new();
        
        // Look for pattern descriptions in the AI response
        for line in response.lines() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("table") || line_lower.contains("list") || 
               line_lower.contains("div") || line_lower.contains("paragraph") {
                
                patterns.push(HTMLPattern {
                    pattern_type: self.extract_pattern_type(&line_lower),
                    description: line.trim().to_string(),
                    frequency: 1, // Would be calculated from actual analysis
                    confidence: 0.8,
                });
            }
        }
        
        Ok(patterns)
    }
    
    /// Extract pattern type from AI description
    fn extract_pattern_type(&self, line: &str) -> String {
        if line.contains("table") {
            "html_table".to_string()
        } else if line.contains("list") {
            "html_list".to_string()
        } else if line.contains("div") {
            "html_div".to_string()
        } else if line.contains("paragraph") {
            "html_paragraph".to_string()
        } else {
            "html_mixed".to_string()
        }
    }
    
    /// Generate SeaORM model for a specific table
    async fn generate_model_for_table(&mut self, table_name: &str, table_info: &TableInfo, 
                                     html_patterns: &[HTMLPattern], report: &AnalysisReport) -> Result<GeneratedModel> {
        println!("     🏗️  AI generating SeaORM model...");
        
        // Prepare table schema information
        let schema_info = self.prepare_schema_info(table_name, table_info, html_patterns, report)?;
        
        // Create execution request for model generation
        let mut inputs = HashMap::new();
        inputs.insert("table_name".to_string(), AgentConfigValue::String(table_name.to_string()));
        inputs.insert("schema_info".to_string(), AgentConfigValue::String(schema_info));
        inputs.insert("generation_type".to_string(), AgentConfigValue::String("seaorm_model".to_string()));
        
        let exec_request = AgentExecutionRequest {
            agent_name: self.agent_spec.metadata.name.clone(),
            inputs,
            context: HashMap::new(),
            config_overrides: HashMap::new(),
            prompt_template: Some("semantic_analysis".to_string()), // Reuse for code generation
        };
        
        // Execute model generation
        match self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await {
            Ok(result) => {
                if let Some(model_code) = result.outputs.get("result").and_then(|v| v.as_string()) {
                    Ok(GeneratedModel {
                        table_name: table_name.to_string(),
                        model_name: self.to_pascal_case(table_name),
                        rust_code: model_code.to_string(),
                        file_path: format!("src/entities/{}.rs", table_name.to_lowercase()),
                        relationships: self.extract_relationships_for_table(table_name, report),
                        html_patterns: html_patterns.to_vec(),
                    })
                } else {
                    Err(anyhow::anyhow!("No model code generated for {}", table_name))
                }
            }
            Err(e) => {
                println!("     ⚠️  Model generation failed for {}: {}", table_name, e);
                Err(e)
            }
        }
    }
    
    /// Prepare schema information for AI model generation
    fn prepare_schema_info(&self, table_name: &str, table_info: &TableInfo, 
                          html_patterns: &[HTMLPattern], report: &AnalysisReport) -> Result<String> {
        let mut schema_info = format!("Table: {}\n", table_name);
        schema_info.push_str(&format!("Records: {}\n\n", table_info.record_count));
        
        // Add column information
        schema_info.push_str("Columns:\n");
        for column in &table_info.columns {
            schema_info.push_str(&format!("  - {}: {} (nullable: {}, primary_key: {})\n", 
                column.name, column.data_type, !column.not_null, column.primary_key));
        }
        
        // Add HTML patterns
        if !html_patterns.is_empty() {
            schema_info.push_str("\nHTML Patterns Found:\n");
            for pattern in html_patterns {
                schema_info.push_str(&format!("  - {}: {} (confidence: {:.1}%)\n", 
                    pattern.pattern_type, pattern.description, pattern.confidence * 100.0));
            }
        }
        
        // Add relationship information
        let relationships = self.extract_relationships_for_table(table_name, report);
        if !relationships.is_empty() {
            schema_info.push_str("\nRelationships:\n");
            for rel in relationships {
                schema_info.push_str(&format!("  - {} -> {} (confidence: {:.1}%)\n", 
                    rel.from_field, rel.to_field, rel.confidence * 100.0));
            }
        }
        
        // Add sample data if available
        if let Some(sample_data) = &table_info.sample_data {
            schema_info.push_str("\nSample Data (first 3 records):\n");
            if let Some(rows) = sample_data.as_array() {
                for (i, row) in rows.iter().take(3).enumerate() {
                    schema_info.push_str(&format!("Record {}:\n", i + 1));
                    if let Some(row_obj) = row.as_object() {
                        for (col_name, value) in row_obj {
                            let display_value = if let Some(text) = value.as_str() {
                                if text.len() > 100 {
                                    format!("{}...", text.chars().take(100).collect::<String>())
                                } else {
                                    text.to_string()
                                }
                            } else {
                                format!("{:?}", value)
                            };
                            schema_info.push_str(&format!("  {}: {}\n", col_name, display_value));
                        }
                    }
                    schema_info.push('\n');
                }
            }
        }
        
        Ok(schema_info)
    }
    
    /// Extract relationships relevant to a specific table
    fn extract_relationships_for_table(&self, table_name: &str, report: &AnalysisReport) -> Vec<TableRelationship> {
        let mut relationships = Vec::new();
        
        // Extract from implicit relationships
        for rel in &report.implicit_relationships {
            if rel.from_table == table_name {
                relationships.push(TableRelationship {
                    from_field: rel.from_column.clone(),
                    to_field: format!("{}.{}", rel.to_table, rel.to_column),
                    relationship_type: "foreign_key".to_string(),
                    confidence: rel.confidence,
                });
            }
            if rel.to_table == table_name {
                relationships.push(TableRelationship {
                    from_field: format!("{}.{}", rel.from_table, rel.from_column),
                    to_field: rel.to_column.clone(),
                    relationship_type: "inverse_foreign_key".to_string(),
                    confidence: rel.confidence,
                });
            }
        }
        
        relationships
    }
    
    /// Convert table name to PascalCase for model names
    fn to_pascal_case(&self, input: &str) -> String {
        input.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }
    
    /// Generate minijinja templates for data transformation
    pub async fn generate_transformation_templates(&mut self, report: &AnalysisReport, 
                                                  models: &[GeneratedModel]) -> Result<Vec<GeneratedTemplate>> {
        println!("🤖 Generating minijinja transformation templates...");
        
        let mut templates = Vec::new();
        
        for model in models {
            println!("   📝 Generating template for {}", model.model_name);
            
            let template = self.generate_template_for_model(model, report).await?;
            templates.push(template);
        }
        
        println!("✅ Generated {} transformation templates", templates.len());
        Ok(templates)
    }
    
    /// Generate template for a specific model
    async fn generate_template_for_model(&mut self, model: &GeneratedModel, 
                                        report: &AnalysisReport) -> Result<GeneratedTemplate> {
        // Prepare template generation context
        let template_context = self.prepare_template_context(model, report)?;
        
        // Create execution request for template generation
        let mut inputs = HashMap::new();
        inputs.insert("model_info".to_string(), AgentConfigValue::String(template_context));
        inputs.insert("template_type".to_string(), AgentConfigValue::String("minijinja".to_string()));
        
        let exec_request = AgentExecutionRequest {
            agent_name: self.agent_spec.metadata.name.clone(),
            inputs,
            context: HashMap::new(),
            config_overrides: HashMap::new(),
            prompt_template: Some("semantic_analysis".to_string()),
        };
        
        // Execute template generation
        match self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await {
            Ok(result) => {
                if let Some(template_code) = result.outputs.get("result").and_then(|v| v.as_string()) {
                    Ok(GeneratedTemplate {
                        model_name: model.model_name.clone(),
                        template_name: format!("{}_transform.jinja", model.table_name.to_lowercase()),
                        template_content: template_code.to_string(),
                        file_path: format!("templates/{}_transform.jinja", model.table_name.to_lowercase()),
                        variables: self.extract_template_variables(template_code),
                    })
                } else {
                    Err(anyhow::anyhow!("No template generated for {}", model.model_name))
                }
            }
            Err(e) => {
                println!("     ⚠️  Template generation failed for {}: {}", model.model_name, e);
                Err(e)
            }
        }
    }
    
    /// Prepare context for template generation
    fn prepare_template_context(&self, model: &GeneratedModel, report: &AnalysisReport) -> Result<String> {
        let mut context = format!("Model: {}\n", model.model_name);
        context.push_str(&format!("Table: {}\n\n", model.table_name));
        
        // Add model code snippet
        context.push_str("Generated SeaORM Model:\n");
        context.push_str("```rust\n");
        context.push_str(&model.rust_code.lines().take(20).collect::<Vec<_>>().join("\n"));
        context.push_str("\n...\n```\n\n");
        
        // Add HTML patterns
        if !model.html_patterns.is_empty() {
            context.push_str("HTML Patterns to Handle:\n");
            for pattern in &model.html_patterns {
                context.push_str(&format!("  - {}: {}\n", pattern.pattern_type, pattern.description));
            }
            context.push('\n');
        }
        
        // Add relationships
        if !model.relationships.is_empty() {
            context.push_str("Relationships:\n");
            for rel in &model.relationships {
                context.push_str(&format!("  - {} -> {} ({})\n", 
                    rel.from_field, rel.to_field, rel.relationship_type));
            }
            context.push('\n');
        }
        
        // Add transformation requirements
        context.push_str("Transformation Requirements:\n");
        context.push_str("- Convert raw HBF data to SeaORM entity instances\n");
        context.push_str("- Handle HTML content parsing and sanitization\n");
        context.push_str("- Maintain referential integrity\n");
        context.push_str("- Apply Dragon's Labyrinth horror theme transformations\n");
        
        Ok(context)
    }
    
    /// Extract template variables from generated template code
    fn extract_template_variables(&self, template_code: &str) -> Vec<String> {
        let mut variables = Vec::new();
        
        // Simple regex to find {{variable}} patterns
        if let Ok(regex) = regex::Regex::new(r"\{\{\s*(\w+)\s*\}\}") {
            for captures in regex.captures_iter(template_code) {
                if let Some(var_name) = captures.get(1) {
                    let var_str = var_name.as_str().to_string();
                    if !variables.contains(&var_str) {
                        variables.push(var_str);
                    }
                }
            }
        }
        
        variables
    }
    
    /// Save all generated models and templates to files
    pub async fn save_generated_code(&self, models: &[GeneratedModel], 
                                    templates: &[GeneratedTemplate], 
                                    output_dir: &Path) -> Result<()> {
        println!("💾 Saving generated code to: {}", output_dir.display());
        
        // Create output directories
        std::fs::create_dir_all(output_dir.join("src/entities"))?;
        std::fs::create_dir_all(output_dir.join("templates"))?;
        
        // Save SeaORM models
        for model in models {
            let model_path = output_dir.join(&model.file_path);
            std::fs::write(&model_path, &model.rust_code)?;
            println!("   📄 Saved model: {}", model_path.display());
        }
        
        // Save minijinja templates
        for template in templates {
            let template_path = output_dir.join(&template.file_path);
            std::fs::write(&template_path, &template.template_content)?;
            println!("   📄 Saved template: {}", template_path.display());
        }
        
        // Generate mod.rs file for entities
        self.generate_entities_mod_file(models, &output_dir.join("src/entities/mod.rs"))?;
        
        println!("✅ All generated code saved successfully");
        Ok(())
    }
    
    /// Generate mod.rs file for the entities module
    fn generate_entities_mod_file(&self, models: &[GeneratedModel], mod_path: &Path) -> Result<()> {
        let mut mod_content = String::new();
        mod_content.push_str("//! Generated SeaORM entities from HBF analysis\n\n");
        
        for model in models {
            let module_name = model.table_name.to_lowercase();
            mod_content.push_str(&format!("pub mod {};\n", module_name));
            mod_content.push_str(&format!("pub use {}::Entity as {};\n", module_name, model.model_name));
        }
        
        std::fs::write(mod_path, mod_content)?;
        Ok(())
    }
}

/// Generated SeaORM model information
#[derive(Debug, Clone)]
pub struct GeneratedModel {
    pub table_name: String,
    pub model_name: String,
    pub rust_code: String,
    pub file_path: String,
    pub relationships: Vec<TableRelationship>,
    pub html_patterns: Vec<HTMLPattern>,
}

/// Generated minijinja template information
#[derive(Debug, Clone)]
pub struct GeneratedTemplate {
    pub model_name: String,
    pub template_name: String,
    pub template_content: String,
    pub file_path: String,
    pub variables: Vec<String>,
}

/// HTML pattern identified by AI analysis
#[derive(Debug, Clone)]
pub struct HTMLPattern {
    pub pattern_type: String,
    pub description: String,
    pub frequency: usize,
    pub confidence: f64,
}

/// Relationship information for a table
#[derive(Debug, Clone)]
pub struct TableRelationship {
    pub from_field: String,
    pub to_field: String,
    pub relationship_type: String,
    pub confidence: f64,
}
