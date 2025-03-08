use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

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
        .await.map_err(|e| format!("Request failed: {}", e))?
        .json::<Value>()
        .await.map_err(|e| format!("Failed to parse response: {}", e))?;

    println!("Completion Response: {:#?}", completion_response);

    Ok(())
}
