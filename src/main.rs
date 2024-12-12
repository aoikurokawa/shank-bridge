use std::{fs::File, io::Read};

use rig::{
    completion::Prompt,
    providers::anthropic::{self, CLAUDE_3_5_SONNET},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Create OpenAI client
    let client = anthropic::ClientBuilder::new(
        &std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY not set"),
    )
    .build();

    let mut prompt_file = File::open(&std::env::var("PROMPT_FILE_PATH")?)?;
    let mut contents = String::new();
    prompt_file.read_to_string(&mut contents)?;

    // Create agent with a single context prompt
    let agent = client
        .agent(CLAUDE_3_5_SONNET)
        .preamble(&contents)
        .temperature(0.5)
        .max_tokens(8192)
        .build();

    // Prompt the agent and print the response
    let response = agent.prompt("What is a NCN?").await?;
    println!("{}", response);

    Ok(())
}
