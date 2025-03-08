use crate::Config;
use crate::api::{CodestralClient, MistralClient};
use crate::logging::{log_generation, write_generation_content};
use crate::error::{AppError, Result, ErrorExt};
use log::{info, error};

/// Handle code completion command
pub async fn handle_code_completion(args: &[String], config: &Config) -> Result<()> {
    if args.len() < 2 {
        let error_msg = "Usage: code-completion <prefix> <suffix> [max_tokens]";
        error!("{}", error_msg);
        return Err(AppError::cli_usage(error_msg));
    }
    
    let prompt = &args[0];
    let suffix = &args[1];
    
    let max_tokens = if args.len() > 2 {
        args[2].parse::<u32>().with_context(|| "Invalid max_tokens value. Please provide a valid number")?
    } else {
        config.max_tokens_code
    };
    
    info!("Initializing Codestral client");
    let client = CodestralClient::new(config.clone())?;
    
    info!("Sending code completion request with max_tokens: {}", max_tokens);
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
pub async fn handle_chat(args: &[String], config: &Config) -> Result<()> {
    if args.is_empty() {
        let error_msg = "Usage: chat <message> [max_tokens]";
        error!("{}", error_msg);
        return Err(AppError::cli_usage(error_msg));
    }
    
    let message = &args[0];
    
    let max_tokens = if args.len() > 1 {
        Some(args[1].parse::<u32>().with_context(|| "Invalid max_tokens value. Please provide a valid number")?)
    } else {
        Some(config.max_tokens_chat)
    };
    
    info!("Initializing Mistral client");
    let client = MistralClient::new(config.clone())?;
    
    info!("Sending chat request with message: {}", message);
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
