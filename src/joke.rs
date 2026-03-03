use rig::client::CompletionClient;
use rig::completion::Prompt;
use rig::providers::{gemini, openai};

pub async fn joke_agent(gemini_client: gemini::Client, openai_client: openai::Client) {
    let funny_agent = gemini_client
        .agent("gemini-2.5-flash")
        .preamble("You are a comedian.")
        .build();

    let teacher_agent = openai_client
        .agent("gpt-5-mini")
        .preamble("You are a teacher. Assume any specific subject as per needed.")
        .build();

    let prompt: String = String::from("Tell me a joke.");
    
    let gemini_response = funny_agent.prompt(&prompt)
        .await
        .expect("Failed to talk to Gemini");
    let openai_response = teacher_agent.prompt(&prompt)
        .await
        .expect("Failed to talk to GPT");

    println!("Gemini Response: {gemini_response} \n");
    println!("GPT Response: {openai_response}");
}
