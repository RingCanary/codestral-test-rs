use crate::Config;
use crate::models::ApiResponse;
use crate::error::{AppError, Result, ErrorExt};
use std::fs::{self, OpenOptions};
use std::io::Write;
use chrono::Local;
use log::{info, error};
use serde_json::json;

// Log generation metrics to a log file
pub fn log_generation(api_type: &str, response: &ApiResponse, config: &Config) -> Result<()> {
    let log_dir = config.get_log_directory_path();
    
    // Ensure log directory exists
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)
            .with_context(|| format!("Failed to create log directory: {}", log_dir.display()))?;
    }
    
    let log_file_path = log_dir.join("generations.log");
    
    // Open log file in append mode or create if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)
        .with_context(|| format!("Failed to open log file: {}", log_file_path.display()))?;
    
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    // JSON formatted log entry with all fields
    let log_entry = json!({
        "completion_tokens": response.completion_tokens,
        "created": response.created,
        "finish_reason": response.finish_reason,
        "id": response.id,
        "model": response.model,
        "object": response.object,
        "timestamp": timestamp,
        "total_tokens": response.total_tokens,
        "type": api_type.to_lowercase()
    }).to_string();
    
    // Write to log file (append newline)
    file.write_all(format!("{}\n", log_entry).as_bytes())
        .with_context(|| format!("Failed to write to log file: {}", log_file_path.display()))?;
    
    info!("Logged generation: {}", response.id);
    Ok(())
}

// Write generated content to an output file
pub fn write_generation_content(api_type: &str, response: &ApiResponse, user_input: Option<&str>, config: &Config) -> Result<()> {
    let content = match &response.content {
        Some(content) => content,
        None => {
            error!("No content to write from the API response");
            return Err(AppError::logging("No content found in API response"));
        }
    };
    
    let log_dir = config.get_log_directory_path();
    
    // Ensure log directory exists
    if !log_dir.exists() {
        fs::create_dir_all(&log_dir)
            .with_context(|| format!("Failed to create log directory: {}", log_dir.display()))?;
    }
    
    let output_file_path = log_dir.join("generations.txt");
    
    // Open output file in append mode or create if it doesn't exist
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&output_file_path)
        .with_context(|| format!("Failed to open output file: {}", output_file_path.display()))?;
    
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
    
    // Format output entry
    let mut output_entry = format!("=== {} Generation [{}] ===\n", api_type, timestamp);
    
    if let Some(input) = user_input {
        output_entry.push_str(&format!("Input:\n{}\n\n", input));
    }
    
    output_entry.push_str(&format!("Output:\n{}\n\n", content));
    
    // Add metadata section to include all fields from the response
    output_entry.push_str(&format!("Metadata:\n"));
    output_entry.push_str(&format!("ID: {}\n", response.id));
    output_entry.push_str(&format!("Model: {}\n", response.model));
    output_entry.push_str(&format!("Object: {}\n", response.object));
    output_entry.push_str(&format!("Created: {}\n", response.created));
    output_entry.push_str(&format!("Finish Reason: {}\n", response.finish_reason));
    output_entry.push_str(&format!("Completion Tokens: {}\n", response.completion_tokens));
    output_entry.push_str(&format!("Total Tokens: {}\n", response.total_tokens));
    
    output_entry.push_str("==============================================\n\n");
    
    // Write to output file
    file.write_all(output_entry.as_bytes())
        .with_context(|| format!("Failed to write to output file: {}", output_file_path.display()))?;
    
    info!("Written generation content to {}", output_file_path.display());
    Ok(())
}
