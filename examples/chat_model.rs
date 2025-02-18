use std::error::Error as StdError;
use std::io::{ self, Write };
use log::{ info, error };
use pyano::{
    ModelManager,
    llm::{
        options::LLMHTTPCallOptions,
        llm_builder::LLM,
        stream_processing::llamacpp_process_stream,
    },
    agent::{ agent_builder::AgentBuilder, agent_trait::AgentTrait },
};
use std::sync::{ Arc, Mutex };
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // std::env::set_var("RUST_LOG", "debug");
    let system_prompt = "Answer the user questions";

    let model_manager = Arc::new(ModelManager::new());

    model_manager.show_models();

    println!("Enter a model from the list");
    print!("> ");
    io::stdout().flush()?;

    let mut model_input = String::new();
    io::stdin().read_line(&mut model_input)?;

    let model_name = model_input.trim();

    let llm = model_manager
        .clone()
        .get_llm(model_name, None).await
        .map_err(|e| {
            error!("Failed to Get {} model: {}", model_name, e);
            e
        })?;

    llm.clone().load().await;
    println!("{} {}", model_name, " loaded".bold().bright_yellow());
    println!("");

    println!("Welcome to the Pyano LLM CLI! Type 'exit' to quit.");
    // Define the user prompt

    // Execute the LLM call with the user prompt
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;

        let user_prompt = user_input.trim();

        if user_prompt.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            break;
        }

        // Create your chat agent
        let agent = AgentBuilder::new()
            .with_system_prompt(system_prompt.to_string())
            .with_user_prompt(user_prompt.to_string())
            .with_stream(true)
            .with_llm(llm.clone())
            .build();

        match agent.invoke().await {
            Ok(_) => println!("\n---"),
            Err(e) => eprintln!("Error during processing: {}", e),
        }
    }

    Ok(())
}
