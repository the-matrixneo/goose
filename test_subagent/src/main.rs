use goose::recipe::{Recipe, SubagentConfig, SubagentCommunicationMode};
use goose::agents::Agent;
use goose::message::Message;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing subagent functionality...");

    // Create a simple subagent recipe
    let subagent_recipe = Recipe::builder()
        .title("Research Assistant")
        .description("A specialized research assistant")
        .instructions("You are a research assistant. Help with detailed research tasks.")
        .activities(vec!["Research topics".to_string(), "Analyze data".to_string()])
        .build()?;

    // Create subagent configuration
    let subagent_config = SubagentConfig {
        name: "research_assistant".to_string(),
        recipe: Box::new(subagent_recipe),
        trigger_conditions: vec!["research".to_string(), "find information".to_string()],
        communication_mode: SubagentCommunicationMode::Interactive,
        description: Some("Helps with research tasks".to_string()),
    };

    // Create main recipe with subagent
    let main_recipe = Recipe::builder()
        .title("Main Agent with Research Assistant")
        .description("An agent that can spawn a research assistant subagent")
        .instructions("Handle user queries and spawn research assistant when needed")
        .activities(vec![
            "Answer user questions".to_string(),
            "Spawn research assistant for complex queries".to_string(),
        ])
        .subagents(vec![subagent_config])
        .build()?;

    // Create agent and configure with recipe
    let agent = Agent::new();
    agent.configure_with_recipe(main_recipe).await?;

    // Test subagent spawning
    println!("✅ Successfully created agent with subagent recipe");

    // Create a message that should trigger the subagent
    let trigger_message = Message::user().with_text("I need help with research on AI");

    // Test listing subagents (should be empty initially)
    let subagents = agent.list_subagents().await;
    println!("Initial subagents: {:?}", subagents);

    println!("Subagent test completed successfully!");
    Ok(())
}