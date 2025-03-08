use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use log::{info, error};
use std::env;
use chrono::Local;

#[derive(Serialize, Deserialize)]
struct CompletionRequest {
    model: String,
    prompt: String,
    suffix: String,
    max_tokens: u32,
    temperature: u32,
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
    let max_tokens = &args[3];

    let client = Client::new();
    let api_key = std::env::var("CODESRAL_API_KEY").expect("API key not set");
    // This is codestral API key by the name test-benchmark-code-mist

    // Query the completion endpoint
    let completion_response = client
        .post("https://codestral.mistral.ai/v1/fim/completions")
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&CompletionRequest {
            model: "codestral-latest".to_string(),
            prompt: prompt.to_string(),
            suffix: suffix.to_string(),
            max_tokens: max_tokens.to_string().parse().unwrap_or(64),
            temperature: 0,
        })
        .send()
        .await.map_err(|e| format!("Request failed: {}. Please check your network connection or the API endpoint.", e))?
        .json::<Value>()
        .await.map_err(|e| format!("Failed to parse response: {}. Please ensure the API returned valid JSON.", e))?;

    println!("Completion Response: {}", serde_json::to_string_pretty(&completion_response).unwrap());
    
    // User instructions based on the response
    if let Some(error) = completion_response.get("error") {
        error!("Error: {}. Please check your API key or the request parameters.", error);
    } else {
        info!("Successfully received response.");
    }

    // Log the specified fields into generations.log
    use std::fs::OpenOptions;
    use std::io::Write;

    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("generations.log")
        .map_err(|e| format!("Failed to open generations.log: {}", e))?;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    if let Some(choices) = completion_response.get("choices") {
        if let Some(first_choice) = choices.get(0) {
            let finish_reason = first_choice.get("finish_reason").and_then(|fr| fr.as_str()).unwrap_or("N/A");
            let tool_calls = first_choice.get("tool_calls").and_then(|tc| tc.as_array()).unwrap_or(&Vec::new()).len();

            writeln!(
                log_file,
                "[{}] ID: {}, Model: {}, Object: {}, Finish Reason: {}, Tool Calls: {}, Created: {}, Completion Tokens: {}, Total Tokens: {}",
                timestamp,
                completion_response.get("id").and_then(|id| id.as_str()).unwrap_or("N/A"),
                completion_response.get("model").and_then(|m| m.as_str()).unwrap_or("N/A"),
                completion_response.get("object").and_then(|o| o.as_str()).unwrap_or("N/A"),
                finish_reason,
                tool_calls,
                completion_response.get("created").and_then(|c| c.as_i64()).unwrap_or(0),
                completion_response.get("usage").and_then(|u| u.get("completion_tokens")).and_then(|ct| ct.as_i64()).unwrap_or(0),
                completion_response.get("usage").and_then(|u| u.get("total_tokens")).and_then(|tt| tt.as_i64()).unwrap_or(0),
            )?;
        }
    }

    if let Some(choices) = completion_response.get("choices") {
        if let Some(first_choice) = choices.get(0) {
            if let Some(content) = first_choice.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                use std::fs::OpenOptions;
                use std::io::Write;

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("generations.txt")
                    .map_err(|e| format!("Failed to open generations.txt: {}", e))?;

                if let Some(id) = completion_response.get("id").and_then(|id| id.as_str()) {
                    writeln!(file, "ID: {}", id)?;
                }
                writeln!(file, "{}", content)?;
                writeln!(file, "---")?;
            }
        }
    }

    Ok(())
}
