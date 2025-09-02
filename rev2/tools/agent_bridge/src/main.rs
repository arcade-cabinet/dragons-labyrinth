#[cfg(feature = "agent")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use bevy_agent::{BevyAIAgent, AIConfig, ModelType};
    let cfg = AIConfig::from_env()?;
    let agent = BevyAIAgent::new(cfg).await?;
    let resp = agent
        .request("Generate hex biome tiles and quests for a Bevy hex RPG.")
        .with_model(ModelType::GPT4)
        .execute()
        .await?;
    println!("{}", resp.content);
    Ok(())
}

#[cfg(not(feature = "agent"))]
fn main() {
    println!("agent_bridge compiled without `agent` feature; enable with `--features agent`.");
}
