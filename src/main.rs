use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::{gemini, openai};

// mod joke;
mod tools;

use tools::Multiplier;

#[tokio::main]
async fn main() -> Result<(), Box<dyn  std::error::Error>>{
    let gemini_client = gemini::Client::from_env();
    let openai_client = openai::Client::from_env();

    // joke::joke_agent(gemini_client, openai_client).await;
    let prompt: String = String::from("Multiply 2 and 3");
    let gemini_agent = gemini_client
        .agent("gemini-2.5-flash")
        .tool(Multiplier)
        .build();

    let openai_agent = openai_client
        .agent("gpt-5-mini")
        .tool(Multiplier)
        .build();

    let gemini_response = gemini_agent.prompt(&prompt)
        .await
        .expect("Gemini Failed");

    let openai_response = openai_agent.prompt(&prompt)
        .await
        .expect("OpenAI Failed");

    println!("Gemini Response: {gemini_response} \n");
    println!("GPT Response: {openai_response}");

    Ok(())
}
