use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use log::{info, error, debug};
use std::env;
use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct CompletionRequest {
    model: String,
    prompt: String,
    suffix: String,
    max_tokens: u32,
    temperature: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: {} <prompt> <suffix> <max_tokens>", args[0]);
        std::process::exit(1);
    }
    let prompt = &args[1];
    let suffix = &args[2];
    let max_tokens_str = &args[3];

    let max_tokens: u32 = match max_tokens_str.parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: <max_tokens> must be a valid positive integer.");
            std::process::exit(1);
        }
    };

    let client = Client::new();
    let api_key = match std::env::var("CODESTRAL_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            error!("CODESTRAL_API_KEY environment variable not set. Please set it before running the application.");
            eprintln!("Error: CODESTRAL_API_KEY environment variable not set.");
            std::process::exit(1);
        }
    };

    debug!("Sending request to Codestral API");
    // Query the completion endpoint
    let completion_response = match client
        .post("https://codestral.mistral.ai/v1/fim/completions")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&CompletionRequest {
            model: "codestral-latest".to_string(),
            prompt: prompt.to_string(),
            suffix: suffix.to_string(),
            max_tokens,
            temperature: 0.0,
        })
        .send()
        .await {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    error!("API request failed with status code: {}", status);
                    return Err(format!("API request failed with status code: {}. Please check your API key and request parameters.", status).into());
                }
                match response.json::<Value>().await {
                    Ok(json) => json,
                    Err(e) => {
                        error!("Failed to parse API response: {}", e);
                        return Err(format!("Failed to parse API response: {}. Please ensure the API returned valid JSON.", e).into());
                    }
                }
            },
            Err(e) => {
                error!("Request to API failed: {}", e);
                return Err(format!("Request to API failed: {}. Please check your network connection or the API endpoint.", e).into());
            }
        };

    println!("Completion Response: {}", serde_json::to_string_pretty(&completion_response).unwrap());
    
    // Process the response
    if let Some(error) = completion_response.get("error") {
        error!("API returned an error: {}. Please check your API key or the request parameters.", error);
    } else {
        info!("Successfully received response.");
    }

    // Extract commonly used fields from the response
    let id = completion_response.get("id").and_then(|id| id.as_str()).unwrap_or("N/A");
    let model = completion_response.get("model").and_then(|m| m.as_str()).unwrap_or("N/A");
    let object = completion_response.get("object").and_then(|o| o.as_str()).unwrap_or("N/A");
    let created = completion_response.get("created").and_then(|c| c.as_i64()).unwrap_or(0);
    
    let first_choice = completion_response
        .get("choices")
        .and_then(|choices| choices.get(0));
    
    let finish_reason = first_choice
        .and_then(|choice| choice.get("finish_reason"))
        .and_then(|fr| fr.as_str())
        .unwrap_or("N/A");
    
    let tool_calls = first_choice
        .and_then(|choice| choice.get("tool_calls"))
        .and_then(|tc| tc.as_array())
        .map_or(0, |v| v.len());
    
    let completion_tokens = completion_response
        .get("usage")
        .and_then(|u| u.get("completion_tokens"))
        .and_then(|ct| ct.as_i64())
        .unwrap_or(0);
    
    let total_tokens = completion_response
        .get("usage")
        .and_then(|u| u.get("total_tokens"))
        .and_then(|tt| tt.as_i64())
        .unwrap_or(0);

    // Log the specified fields into generations.log
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("generations.log")
        .map_err(|e| {
            error!("Failed to open generations.log: {}", e);
            format!("Failed to open generations.log: {}", e)
        })?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let log_data = serde_json::json!({
        "timestamp": timestamp,
        "id": id,
        "model": model,
        "object": object,
        "finish_reason": finish_reason,
        "tool_calls": tool_calls,
        "created": created,
        "completion_tokens": completion_tokens,
        "total_tokens": total_tokens,
    });

    writeln!(log_file, "{}", serde_json::to_string(&log_data).unwrap())?;

    // Write the content to generations.txt
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("generations.txt")
        .map_err(|e| {
            error!("Failed to open generations.txt: {}", e);
            format!("Failed to open generations.txt: {}", e)
        })?;

    if let Some(content) = first_choice
        .and_then(|choice| choice.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str()) {
        
        writeln!(file, "ID: {}", id)?;
        writeln!(file, "{}", content)?;
        writeln!(file, "---")?;
    } else {
        debug!("No content found in the response");
    }

    Ok(())
}
