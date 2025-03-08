mod commands;

pub use commands::{Cli, Commands, ConfigCommands};

use crate::Config;
use crate::api::{CodestralClient, MistralClient};
use crate::logging::{log_generation, write_generation_content};
use crate::error::Result;
use log::info;

/// Handle code completion command
pub async fn handle_code_completion(prompt: &str, suffix: &str, max_tokens: Option<u32>, config: &Config) -> Result<()> {
    // Use provided max_tokens or default from config
    let max_tokens = max_tokens.unwrap_or(config.max_tokens_code);
    
    info!("Initializing Codestral client");
    let client = CodestralClient::new(config.clone())?;
    
    info!("Sending code completion request with max_tokens: {}", max_tokens);
    
    // Make API request
    let response_json = client.code_completion(prompt, suffix, max_tokens).await?;
    
    // Extract and process the response
    use crate::api::extract_response_fields;
    let api_response = extract_response_fields(&response_json);
    
    // Log generation metrics
    log_generation("Code", &api_response, config)?;
    
    // Write generation content
    let user_input = format!("{}{}", prompt, suffix);
    write_generation_content("Code", &api_response, Some(&user_input), config)?;
    
    // Print generated code to stdout
    if let Some(content) = &api_response.content {
        println!("{}", content);
    } else {
        println!("No content was generated.");
    }
    
    Ok(())
}

/// Handle chat command
pub async fn handle_chat(message: &str, max_tokens: Option<u32>, config: &Config) -> Result<()> {
    // Use provided max_tokens or default from config
    let max_tokens = max_tokens.or(Some(config.max_tokens_chat));
    
    info!("Initializing Mistral client");
    let client = MistralClient::new(config.clone())?;
    
    info!("Sending chat request with message: {}", message);
    
    // Make API request
    let response_json = client.chat(message, max_tokens).await?;
    
    // Extract and process the response
    use crate::api::extract_response_fields;
    let api_response = extract_response_fields(&response_json);
    
    // Log generation metrics
    log_generation("Chat", &api_response, config)?;
    
    // Write generation content
    write_generation_content("Chat", &api_response, Some(message), config)?;
    
    // Print response to stdout
    if let Some(content) = &api_response.content {
        println!("Assistant: {}", content);
    } else {
        println!("No response was generated.");
    }
    
    Ok(())
}

/// Handle configuration commands
pub fn handle_config(action: &ConfigCommands, _config: &Config) -> Result<()> {
    match action {
        ConfigCommands::Generate { path } => {
            Config::generate_default_config(path)?;
            println!("Default configuration generated at {}", path);
        },
        ConfigCommands::View { path } => {
            match Config::load_from_file(path) {
                Ok(loaded_config) => {
                    println!("Configuration from {}:", path);
                    println!("Code model: {}", loaded_config.code_model);
                    println!("Chat model: {}", loaded_config.chat_model);
                    println!("Code temperature: {}", loaded_config.code_temperature);
                    println!("Chat temperature: {}", loaded_config.chat_temperature);
                    println!("Code API URL: {}", loaded_config.code_api_url);
                    println!("Chat API URL: {}", loaded_config.chat_api_url);
                    println!("Logs directory: {}", loaded_config.log_directory);
                },
                Err(e) => {
                    eprintln!("Error loading configuration: {}", e);
                    return Err(e);
                }
            }
        },
        ConfigCommands::Load { path } => {
            match Config::load_from_file(path) {
                Ok(loaded_config) => {
                    println!("Configuration loaded from {}:", path);
                    println!("Code model: {}", loaded_config.code_model);
                    println!("Chat model: {}", loaded_config.chat_model);
                    println!("Code temperature: {}", loaded_config.code_temperature);
                    println!("Chat temperature: {}", loaded_config.chat_temperature);
                    println!("Code API URL: {}", loaded_config.code_api_url);
                    println!("Chat API URL: {}", loaded_config.chat_api_url);
                    println!("Logs directory: {}", loaded_config.log_directory);
                },
                Err(e) => {
                    eprintln!("Error loading configuration: {}", e);
                    return Err(e);
                }
            }
        }
    }
    
    Ok(())
}
