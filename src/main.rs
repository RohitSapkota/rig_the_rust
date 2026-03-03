use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::{gemini, openai};

#[tokio::main]
async fn main() -> Result<(), Box<dyn  std::error::Error>>{
    let gemini_client = gemini::Client::from_env();
    let openai_client = openai::Client::from_env();

    let funny_agent = gemini_client
        .agent("gemini-2.5-flash")
        .preamble("You are a comedian.")
        .build();

    let teacher_agent = openai_client
        .agent("gpt-5-mini")
        .preamble("You are a teacher. Assume any specific subject as per needed.")
        .build();

    let prompt: String = String::from("Tell me a joke.");
    
    let gemini_response = funny_agent.prompt(&prompt).await?;
    let openai_response = teacher_agent.prompt(&prompt).await?;

    println!("Gemini Response: {gemini_response} \n");
    println!("GPT Response: {openai_response}");

    Ok(())
}
