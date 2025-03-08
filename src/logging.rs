use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use log::{error, debug};
use chrono::Local;
use crate::models::ApiResponse;
use crate::config::Config;

// Helper function to log generation metrics
pub fn log_generation(api_type: &str, response: &ApiResponse, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_file)
        .map_err(|e| {
            error!("Failed to open {}: {}", config.log_file, e);
            format!("Failed to open {}: {}", config.log_file, e)
        })?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let log_data = serde_json::json!({
        "type": api_type,
        "timestamp": timestamp,
        "id": response.id,
        "model": response.model,
        "object": response.object,
        "finish_reason": response.finish_reason,
        "created": response.created,
        "completion_tokens": response.completion_tokens,
        "total_tokens": response.total_tokens,
    });

    writeln!(log_file, "{}", serde_json::to_string(&log_data).unwrap())?;
    Ok(())
}

// Helper function to write generation content to output file
pub fn write_generation_content(api_type: &str, response: &ApiResponse, user_input: Option<&str>, config: &Config) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.output_file)
        .map_err(|e| {
            error!("Failed to open {}: {}", config.output_file, e);
            format!("Failed to open {}: {}", config.output_file, e)
        })?;

    if let Some(content) = &response.content {
        writeln!(file, "Type: {}", api_type)?;
        writeln!(file, "ID: {}", response.id)?;
        
        if let Some(input) = user_input {
            writeln!(file, "User: {}", input)?;
        }
        
        writeln!(file, "{}{}", 
            if api_type == "Chat" { "Assistant: " } else { "" },
            content
        )?;
        writeln!(file, "---")?;
    } else {
        debug!("No content found in the response");
    }

    Ok(())
}
