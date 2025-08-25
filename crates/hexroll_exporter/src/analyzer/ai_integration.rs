//! AI integration for semantic HBF analysis using spec-driven agents

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use ai_bridge::{
    agent_spec::{AgentSpec, AgentSpecLoader, AgentConfigValue},
    agent_executor::{AgentExecutor, AgentExecutionRequest},
    context::BuildContext,
    openai_client::OpenAIClient,
};

use super::types::{AnalysisReport, TableInfo, AIInsights, DiscoveredRelationship, AIAnalysisResult};

/// AI-powered semantic analysis using spec-driven agents
pub struct AIAnalysisEngine {
    agent_spec: AgentSpec,
    executor: AgentExecutor,
    build_context: BuildContext,
}

impl AIAnalysisEngine {
    /// Create new AI analysis engine by loading the hbf-analyzer agent spec
    pub async fn new<P: AsRef<Path>>(agent_spec_path: P) -> Result<Self> {
        println!("🤖 Loading hbf-analyzer agent specification...");
        
        // Load the agent specification using direct comprehensive spec loading
        let mut loader = AgentSpecLoader::new();
        let agent_spec = loader.load_comprehensive_spec(agent_spec_path)
            .map_err(|e| anyhow::anyhow!("Failed to load agent spec: {}", e))?;
        
        // Create OpenAI client and build context
        let openai_client = OpenAIClient::new()?;
        let executor = AgentExecutor::new(openai_client);
        let build_context = BuildContext::new("./tmp")?;
        
        println!("   ✅ Loaded agent: {} v{}", agent_spec.metadata.name, agent_spec.metadata.version);
        
        Ok(Self { agent_spec, executor, build_context })
    }
    
    /// Perform comprehensive AI analysis on the HBF data
    pub async fn analyze_hbf_with_ai(&mut self, report: &mut AnalysisReport) -> Result<()> {
        println!("🤖 Starting AI-enhanced HBF analysis...");
        
        let mut ai_insights = AIInsights {
            relationship_coverage: 0.0,
            overall_confidence: 0.0,
            discovered_relationships: Vec::new(),
            validation_warnings: Vec::new(),
            extraction_suggestions: Vec::new(),
            accuracy_prediction: 0.0,
        };
        
        // Analyze each table with significant content
        for (table_name, table_info) in &report.table_info {
            if table_info.record_count > 10 && table_info.sample_data.is_some() {
                println!("   🤖 AI analyzing table: {}", table_name);
                
                match self.analyze_table_semantically(table_name, table_info, report).await {
                    Ok(table_result) => {
                        ai_insights.discovered_relationships.extend(table_result.discovered_relationships);
                        ai_insights.validation_warnings.extend(table_result.validation_concerns);
                        ai_insights.extraction_suggestions.extend(table_result.extraction_suggestions);
                        
                        // Update overall confidence
                        ai_insights.overall_confidence = 
                            (ai_insights.overall_confidence + table_result.confidence_score) / 2.0;
                    }
                    Err(e) => {
                        println!("   ⚠️  AI analysis failed for {}: {}", table_name, e);
                        ai_insights.validation_warnings.push(format!(
                            "AI analysis failed for {}: {}",
                            table_name, e
                        ));
                    }
                }
            }
        }
        
        // Perform cross-table relationship validation
        self.validate_cross_table_relationships(report, &mut ai_insights).await?;
        
        // Calculate final metrics
        self.calculate_final_metrics(report, &mut ai_insights);
        
        report.ai_insights = Some(ai_insights);
        
        println!("🤖 AI analysis complete - {} relationships discovered", 
                 report.ai_insights.as_ref().unwrap().discovered_relationships.len());
        
        Ok(())
    }
    
    /// Analyze a single table using AI semantic analysis
    async fn analyze_table_semantically(&mut self, table_name: &str, table_info: &TableInfo,
                                       report: &AnalysisReport) -> Result<AIAnalysisResult> {
        // Prepare analysis context
        let analysis_context = self.prepare_analysis_context(table_name, table_info, report)?;
        
        // Determine table type
        let table_type = if table_name.to_lowercase().contains("entities") || 
                           table_name.to_lowercase().contains("entity") {
            "entities"
        } else if table_name.to_lowercase().contains("refs") || 
                  table_name.to_lowercase().contains("ref") {
            "refs" 
        } else {
            "unknown"
        };
        
        // Extract HTML content for analysis
        let html_content = self.extract_html_content_from_context(&analysis_context)?;
        
        // Create execution request for the agent
        let mut inputs = HashMap::new();
        inputs.insert("hbf_content".to_string(), AgentConfigValue::String(html_content));
        inputs.insert("table_type".to_string(), AgentConfigValue::String(table_type.to_string()));
        
        let mut context = HashMap::new();
        context.insert("relationship_hints".to_string(), 
            AgentConfigValue::String("Focus on Entity-Ref relationships via HTML links and semantic connections".to_string()));
        
        let exec_request = AgentExecutionRequest {
            agent_name: self.agent_spec.metadata.name.clone(),
            inputs,
            context,
            config_overrides: HashMap::new(),
            prompt_template: Some("semantic_analysis".to_string()),
        };
        
        // Execute the semantic_analysis prompt
        let result = self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await
            .map_err(|e| anyhow::anyhow!("AI analysis failed: {}", e))?;
        
        // Parse the AI response into structured data
        if let Some(response_text) = result.outputs.get("result").and_then(|v| v.as_string()) {
            self.parse_ai_analysis_result(response_text)
        } else {
            Ok(AIAnalysisResult {
                discovered_relationships: Vec::new(),
                confidence_score: 0.5,
                validation_concerns: vec!["No response from AI agent".to_string()],
                extraction_suggestions: Vec::new(),
            })
        }
    }
    
    /// Prepare analysis context for AI processing
    fn prepare_analysis_context(&self, table_name: &str, table_info: &TableInfo, 
                               report: &AnalysisReport) -> Result<HashMap<String, serde_json::Value>> {
        let mut context = HashMap::new();
        
        // Table metadata
        context.insert("table_name".to_string(), serde_json::Value::String(table_name.to_string()));
        context.insert("record_count".to_string(), serde_json::Value::Number(table_info.record_count.into()));
        context.insert("columns".to_string(), serde_json::to_value(&table_info.columns)?);
        
        // Sample data for semantic analysis
        if let Some(sample_data) = &table_info.sample_data {
            context.insert("sample_data".to_string(), sample_data.clone());
        }
        
        // HTML patterns found
        context.insert("html_patterns".to_string(), serde_json::to_value(&table_info.html_patterns)?);
        
        // Known relationships
        context.insert("implicit_relationships".to_string(), serde_json::to_value(&report.implicit_relationships)?);
        context.insert("embedded_references".to_string(), serde_json::to_value(&report.embedded_references)?);
        
        Ok(context)
    }
    
    /// Extract HTML content from analysis context
    fn extract_html_content_from_context(&self, context: &HashMap<String, serde_json::Value>) -> Result<String> {
        let mut html_content = String::new();
        
        if let Some(sample_data) = context.get("sample_data") {
            if let Some(rows) = sample_data.as_array() {
                for (i, row) in rows.iter().enumerate() {
                    if let Some(row_obj) = row.as_object() {
                        html_content.push_str(&format!("Record {}:\n", i + 1));
                        for (col_name, value) in row_obj {
                            if let Some(text_value) = value.as_str() {
                                if text_value.contains("<") && text_value.contains(">") {
                                    html_content.push_str(&format!("  {}: {}\n", col_name, text_value));
                                } else if !text_value.is_empty() && text_value.len() > 20 {
                                    // Include non-HTML text content for context
                                    let preview = text_value.chars().take(200).collect::<String>();
                                    html_content.push_str(&format!("  {}: {}\n", col_name, preview));
                                }
                            }
                        }
                        html_content.push('\n');
                    }
                }
            }
        }
        
        if html_content.is_empty() {
            html_content = "No HTML content found in sample data".to_string();
        }
        
        Ok(html_content)
    }
    
    /// Parse AI analysis result into structured format
    fn parse_ai_analysis_result(&self, ai_response: &str) -> Result<AIAnalysisResult> {
        // Extract relationship information from AI response
        let discovered_relationships = self.extract_relationships_from_response(ai_response);
        let confidence_score = self.extract_confidence_from_response(ai_response);
        let validation_concerns = self.extract_concerns_from_response(ai_response);
        let extraction_suggestions = self.extract_suggestions_from_response(ai_response);
        
        Ok(AIAnalysisResult {
            discovered_relationships,
            confidence_score,
            validation_concerns,
            extraction_suggestions,
        })
    }
    
    /// Extract relationship information from AI response
    fn extract_relationships_from_response(&self, response: &str) -> Vec<DiscoveredRelationship> {
        let mut relationships = Vec::new();
        
        // Look for relationship patterns in the response
        for line in response.lines() {
            if (line.contains("relationship") || line.contains("connection")) && 
               (line.contains("confidence") || line.contains("entity") || line.contains("ref")) {
                // Try to extract relationship info from response text
                if let Some(rel) = self.parse_relationship_line(line) {
                    relationships.push(rel);
                }
            }
        }
        
        relationships
    }
    
    /// Parse a single relationship line from AI response
    fn parse_relationship_line(&self, line: &str) -> Option<DiscoveredRelationship> {
        // Enhanced parsing for semantic relationships
        let line_lower = line.to_lowercase();
        
        if line_lower.contains("entity") && (line_lower.contains("ref") || line_lower.contains("link")) {
            // Extract confidence if present
            let confidence = if let Some(captures) = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*%").unwrap().captures(line) {
                captures[1].parse::<f64>().unwrap_or(80.0) / 100.0
            } else {
                0.8 // Default confidence
            };
            
            Some(DiscoveredRelationship {
                source_entity: "entity_from_ai".to_string(),
                target_ref: "ref_from_ai".to_string(),
                relationship_type: "semantic_connection".to_string(),
                confidence,
                reasoning: line.trim().to_string(),
            })
        } else {
            None
        }
    }
    
    /// Extract confidence score from AI response
    fn extract_confidence_from_response(&self, response: &str) -> f64 {
        // Look for confidence percentages in the response
        for line in response.lines() {
            if line.contains("confidence") || line.contains("Confidence") {
                // Try to extract percentage
                if let Some(captures) = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*%").unwrap().captures(line) {
                    if let Ok(percentage) = captures[1].parse::<f64>() {
                        return percentage / 100.0;
                    }
                }
            }
        }
        0.7 // Default confidence
    }
    
    /// Extract validation concerns from AI response
    fn extract_concerns_from_response(&self, response: &str) -> Vec<String> {
        let mut concerns = Vec::new();
        
        for line in response.lines() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("concern") || line_lower.contains("warning") || 
               line_lower.contains("issue") || line_lower.contains("problem") ||
               line_lower.contains("ambiguous") || line_lower.contains("uncertain") {
                concerns.push(line.trim().to_string());
            }
        }
        
        concerns
    }
    
    /// Extract improvement suggestions from AI response
    fn extract_suggestions_from_response(&self, response: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        
        for line in response.lines() {
            let line_lower = line.to_lowercase();
            if line_lower.contains("suggest") || line_lower.contains("recommend") || 
               line_lower.contains("improve") || line_lower.contains("consider") ||
               line_lower.contains("should") || line_lower.contains("could") {
                suggestions.push(line.trim().to_string());
            }
        }
        
        suggestions
    }
    
    /// Validate cross-table relationships using AI
    async fn validate_cross_table_relationships(&mut self, report: &AnalysisReport, 
                                               ai_insights: &mut AIInsights) -> Result<()> {
        println!("🤖 AI validating cross-table relationships...");
        
        // Focus on high-confidence implicit relationships
        for relationship in &report.implicit_relationships {
            if relationship.confidence > 0.5 && relationship.match_count > 5 {
                println!("   🔍 AI validating: {}.{} ↔ {}.{}", 
                         relationship.from_table, relationship.from_column,
                         relationship.to_table, relationship.to_column);
                
                // Create execution request for validation analysis
                let mut inputs = HashMap::new();
                inputs.insert("entity_count".to_string(), 
                    AgentConfigValue::String(report.table_info.values().map(|t| t.record_count).sum::<usize>().to_string()));
                inputs.insert("refs_count".to_string(), 
                    AgentConfigValue::String(report.table_info.values().map(|t| t.record_count).sum::<usize>().to_string()));
                inputs.insert("relationship_count".to_string(), 
                    AgentConfigValue::String(report.implicit_relationships.len().to_string()));
                inputs.insert("data_sample".to_string(), AgentConfigValue::String(format!(
                    "Analyzing {}.{} ↔ {}.{} with {} matches (confidence: {:.1}%)", 
                    relationship.from_table, relationship.from_column,
                    relationship.to_table, relationship.to_column,
                    relationship.match_count, relationship.confidence * 100.0
                )));
                
                let exec_request = AgentExecutionRequest {
                    agent_name: self.agent_spec.metadata.name.clone(),
                    inputs,
                    context: HashMap::new(),
                    config_overrides: HashMap::new(),
                    prompt_template: Some("validation_check".to_string()),
                };
                
                // Execute validation analysis
                match self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await {
                    Ok(result) => {
                        if let Some(validation_text) = result.outputs.get("result").and_then(|v| v.as_string()) {
                            ai_insights.validation_warnings.push(format!(
                                "AI VALIDATION - {}.{} ↔ {}.{}: {}", 
                                relationship.from_table, relationship.from_column,
                                relationship.to_table, relationship.to_column,
                                validation_text.trim()
                            ));
                        }
                    }
                    Err(e) => {
                        println!("   ⚠️  AI validation failed: {}", e);
                        ai_insights.validation_warnings.push(format!(
                            "AI validation failed for {}.{} ↔ {}.{}: {}",
                            relationship.from_table, relationship.from_column,
                            relationship.to_table, relationship.to_column, e
                        ));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Perform HTML parsing analysis on entity descriptions
    pub async fn analyze_html_content(&mut self, entity_name: &str, html_description: &str) -> Result<Vec<DiscoveredRelationship>> {
        println!("   🤖 AI parsing HTML for: {}", entity_name);
        
        // Create execution request for HTML parsing
        let mut inputs = HashMap::new();
        inputs.insert("entity_name".to_string(), AgentConfigValue::String(entity_name.to_string()));
        inputs.insert("html_content".to_string(), AgentConfigValue::String(html_description.to_string()));
        
        let exec_request = AgentExecutionRequest {
            agent_name: self.agent_spec.metadata.name.clone(),
            inputs,
            context: HashMap::new(),
            config_overrides: HashMap::new(),
            prompt_template: Some("html_parsing".to_string()),
        };
        
        // Execute the html_parsing prompt
        match self.executor.execute_agent(&self.agent_spec, exec_request, &self.build_context).await {
            Ok(result) => {
                if let Some(parsing_text) = result.outputs.get("result").and_then(|v| v.as_string()) {
                    // Parse HTML analysis results
                    self.parse_html_analysis_result(parsing_text, entity_name)
                } else {
                    Ok(Vec::new())
                }
            }
            Err(e) => {
                println!("   ⚠️  HTML parsing failed for {}: {}", entity_name, e);
                Ok(Vec::new())
            }
        }
    }
    
    /// Parse HTML analysis result from AI
    fn parse_html_analysis_result(&self, ai_response: &str, entity_name: &str) -> Result<Vec<DiscoveredRelationship>> {
        let mut relationships = Vec::new();
        
        // Look for extracted links and references in the AI response
        for line in ai_response.lines() {
            let line_lower = line.to_lowercase();
            if (line_lower.contains("link") || line_lower.contains("reference") || 
                line_lower.contains("href")) && line_lower.contains("entity") {
                
                // Extract confidence if present
                let confidence = if let Some(captures) = regex::Regex::new(r"(\d+(?:\.\d+)?)\s*%").unwrap().captures(line) {
                    captures[1].parse::<f64>().unwrap_or(85.0) / 100.0
                } else {
                    0.85 // Default confidence for HTML parsing
                };
                
                relationships.push(DiscoveredRelationship {
                    source_entity: entity_name.to_string(),
                    target_ref: "parsed_from_html".to_string(),
                    relationship_type: "html_link".to_string(),
                    confidence,
                    reasoning: line.trim().to_string(),
                });
            }
        }
        
        Ok(relationships)
    }
    
    /// Calculate final accuracy metrics
    fn calculate_final_metrics(&self, report: &AnalysisReport, ai_insights: &mut AIInsights) {
        // Calculate relationship coverage
        let total_entities = report.table_info.values()
            .filter(|t| t.name.to_lowercase().contains("entities") || t.name.to_lowercase().contains("entity"))
            .map(|t| t.record_count)
            .sum::<usize>();
            
        let total_refs = report.table_info.values()
            .filter(|t| t.name.to_lowercase().contains("refs") || t.name.to_lowercase().contains("ref"))
            .map(|t| t.record_count)
            .sum::<usize>();
        
        // Calculate coverage based on discovered relationships
        let total_discovered = report.implicit_relationships.len() + ai_insights.discovered_relationships.len();
        ai_insights.relationship_coverage = if total_refs > 0 {
            (total_discovered as f64 / total_refs as f64).min(1.0)
        } else {
            1.0
        };
        
        // Calculate accuracy prediction based on confidence scores
        let avg_confidence = if !ai_insights.discovered_relationships.is_empty() {
            ai_insights.discovered_relationships.iter()
                .map(|r| r.confidence)
                .sum::<f64>() / ai_insights.discovered_relationships.len() as f64
        } else {
            ai_insights.overall_confidence
        };
        
        ai_insights.accuracy_prediction = (ai_insights.relationship_coverage + avg_confidence) / 2.0;
        
        println!("   📊 AI Metrics: {:.1}% coverage, {:.1}% confidence, {:.1}% predicted accuracy",
                 ai_insights.relationship_coverage * 100.0,
                 ai_insights.overall_confidence * 100.0,
                 ai_insights.accuracy_prediction * 100.0);
        
        // Generate final recommendations
        if ai_insights.accuracy_prediction > 0.95 {
            ai_insights.extraction_suggestions.push("HIGH CONFIDENCE: Extraction should achieve target 100% accuracy".to_string());
        } else if ai_insights.accuracy_prediction > 0.85 {
            ai_insights.extraction_suggestions.push("MEDIUM CONFIDENCE: Manual validation recommended for critical relationships".to_string());
        } else {
            ai_insights.extraction_suggestions.push("LOW CONFIDENCE: Significant manual review required before extraction".to_string());
        }
        
        ai_insights.extraction_suggestions.push(format!(
            "TARGET VALIDATION: {} entities + {} refs = {} total gaming records for extraction",
            total_entities, total_refs, total_entities + total_refs
        ));
    }
}

/// Enhanced semantic analysis for Entity-Ref relationship discovery
pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    /// Analyze Entity HTML descriptions for embedded Ref links
    pub async fn analyze_entity_ref_connections(ai_engine: &mut AIAnalysisEngine, 
                                               report: &mut AnalysisReport) -> Result<()> {
        println!("🤖 Analyzing Entity-Ref semantic connections...");
        
        // Focus on Entities table with HTML descriptions
        for (table_name, table_info) in &report.table_info {
            if table_name.to_lowercase().contains("entities") || table_name.to_lowercase().contains("entity") {
                // Look for description columns
                for column in &table_info.columns {
                    if column.name.to_lowercase().contains("description") || 
                       column.name.to_lowercase().contains("content") {
                        
                        let text_samples = super::core::CoreAnalyzer::extract_text_samples_from_table(table_info, &column.name);
                        
                        // Analyze each entity description with AI
                        for (i, sample) in text_samples.iter().enumerate() {
                            if sample.contains("<") && sample.contains(">") {
                                let entity_id = format!("entity_{}", i);
                                
                                match ai_engine.analyze_html_content(&entity_id, sample).await {
                                    Ok(relationships) => {
                                        if let Some(ai_insights) = &mut report.ai_insights {
                                            ai_insights.discovered_relationships.extend(relationships);
                                        }
                                    }
                                    Err(e) => {
                                        println!("   ⚠️  HTML analysis failed for {}: {}", entity_id, e);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
