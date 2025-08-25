use ai_bridge::agent_spec::{AgentSpecLoader, AgentSpec};
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize tracing for debug output
    tracing_subscriber::fmt::init();
    
    println!("Testing agent spec loading...");
    
    // Create loader
    let mut loader = AgentSpecLoader::new();
    
    // Load the agent spec
    let spec = loader.load_comprehensive_spec("agent.toml")?;
    
    println!("✓ Successfully loaded agent spec: {}", spec.metadata.name);
    println!("  Version: {}", spec.metadata.version);
    println!("  Domain: {}", spec.metadata.domain);
    println!("  Capabilities: {:?}", spec.metadata.capabilities);
    println!("  Prompts available: {:?}", spec.prompts.keys().collect::<Vec<_>>());
    
    // Verify all expected prompts are present
    assert!(spec.prompts.contains_key("semantic_analysis"));
    assert!(spec.prompts.contains_key("html_parsing"));
    assert!(spec.prompts.contains_key("validation_check"));
    println!("✓ All prompts loaded correctly");
    
    // Verify capabilities
    assert!(spec.metadata.capabilities.contains(&"hbf_analysis".to_string()));
    assert!(spec.metadata.capabilities.contains(&"semantic_relationship_discovery".to_string()));
    println!("✓ Capabilities loaded correctly");
    
    // Verify interface
    assert_eq!(spec.interface.inputs.len(), 3);
    assert_eq!(spec.interface.outputs.len(), 2);
    println!("✓ Interface specification loaded correctly");
    
    // Verify config
    if let Some(model) = spec.config.get("model") {
        println!("  Model: {:?}", model.as_string());
    }
    
    println!("\n✅ All tests passed! Agent spec is properly configured.");
    
    Ok(())
}
