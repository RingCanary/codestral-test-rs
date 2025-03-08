use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use log::{info, error};

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
            prompt: "def f(".to_string(),
            suffix: "return a + b".to_string(),
            max_tokens: 64,
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

    Ok(())
}
