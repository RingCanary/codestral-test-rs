use crate::api::{MistralClient, CodestralClient, extract_response_fields};
use crate::logging::{log_generation, write_generation_content};
use crate::config::Config;

use std::error::Error;
use log::{info, error, debug};

pub async fn handle_code_completion(args: &[String], config: &Config) -> Result<(), Box<dyn Error>> {
    if args.len() < 5 {
        eprintln!("Usage for code completion: {} code <prompt> <suffix> <max_tokens>", args[0]);
        std::process::exit(1);
    }
    
    let prompt = &args[2];
    let suffix = &args[3];
    let max_tokens_str = &args[4];

    let max_tokens: u32 = match max_tokens_str.parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: <max_tokens> must be a valid positive integer.");
            std::process::exit(1);
        }
    };

    let codestral_client = CodestralClient::new(config.clone())?;

    debug!("Sending request to Codestral API for code completion");
    
    // Make the API request
    let completion_response = codestral_client.code_completion(prompt, suffix, max_tokens).await?;

    println!("Completion Response: {}", serde_json::to_string_pretty(&completion_response).unwrap());
    
    // Process the response
    if let Some(error) = completion_response.get("error") {
        error!("API returned an error: {}. Please check your API key or the request parameters.", error);
    } else {
        info!("Successfully received code completion response.");
    }

    let api_response = extract_response_fields(&completion_response);
    
    // Log the specified fields into generations.log
    log_generation("code_completion", &api_response, config)?;

    // Write the content to generations.txt
    write_generation_content("Code Completion", &api_response, None, config)?;

    Ok(())
}

pub async fn handle_chat(args: &[String], config: &Config) -> Result<(), Box<dyn Error>> {
    if args.len() < 3 {
        eprintln!("Usage for chat: {} chat <message> [max_tokens]", args[0]);
        std::process::exit(1);
    }
    
    let message = &args[2];
    let max_tokens = if args.len() >= 4 {
        match args[3].parse() {
            Ok(val) => Some(val),
            Err(_) => {
                eprintln!("Error: <max_tokens> must be a valid positive integer.");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    let mistral_client = MistralClient::new(config.clone())?;

    debug!("Sending request to Mistral API for chat completion");
    
    // Make the API request    
    let chat_response = mistral_client.chat(message, max_tokens).await?;

    println!("Chat Response: {}", serde_json::to_string_pretty(&chat_response).unwrap());
    
    // Process the response
    if let Some(error) = chat_response.get("error") {
        error!("Chat API returned an error: {}. Please check your API key or the request parameters.", error);
    } else {
        info!("Successfully received chat response.");
    }

    let api_response = extract_response_fields(&chat_response);
    
    // Log the specified fields into generations.log
    log_generation("chat", &api_response, config)?;

    // Write the content to generations.txt
    write_generation_content("Chat", &api_response, Some(message), config)?;

    Ok(())
}
