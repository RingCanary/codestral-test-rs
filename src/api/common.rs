use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use log::{error};
use crate::models::ApiResponse;
use async_trait::async_trait;

#[async_trait]
pub trait ApiClient {
    async fn send_request(&self, request_body: Value) -> Result<Value, Box<dyn Error>>;
    #[allow(dead_code)]
    fn get_model(&self) -> &str;
    #[allow(dead_code)]
    fn get_temperature(&self) -> f64;
}

// Helper function to get API key from environment variables
pub fn get_api_key(key_name: &str) -> Result<String, Box<dyn Error>> {
    match std::env::var(key_name) {
        Ok(key) => Ok(key),
        Err(_) => {
            let error_msg = format!("{} environment variable not set. Please set it before running the application.", key_name);
            error!("{}", error_msg);
            eprintln!("Error: {}", error_msg);
            std::process::exit(1);
        }
    }
}

// Helper function to handle API requests with proper error handling
pub async fn make_api_request(client: &Client, url: &str, api_key: &str, json_body: Value) -> Result<Value, Box<dyn Error>> {
    match client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json_body)
        .send()
        .await {
            Ok(response) => {
                if !response.status().is_success() {
                    let status = response.status();
                    let error_msg = format!("API request failed with status code: {}. Please check your API key and request parameters.", status);
                    error!("{}", error_msg);
                    return Err(error_msg.into());
                }
                
                match response.json::<Value>().await {
                    Ok(json) => Ok(json),
                    Err(e) => {
                        let error_msg = format!("Failed to parse API response: {}. Please ensure the API returned valid JSON.", e);
                        error!("{}", error_msg);
                        Err(error_msg.into())
                    }
                }
            },
            Err(e) => {
                let error_msg = format!("Request to API failed: {}. Please check your network connection or the API endpoint.", e);
                error!("{}", error_msg);
                Err(error_msg.into())
            }
        }
}

// Helper function to extract common fields from API response
pub fn extract_response_fields(response: &Value) -> ApiResponse {
    let id = response.get("id").and_then(|id| id.as_str()).unwrap_or("N/A").to_string();
    let model = response.get("model").and_then(|m| m.as_str()).unwrap_or("N/A").to_string();
    let object = response.get("object").and_then(|o| o.as_str()).unwrap_or("N/A").to_string();
    let created = response.get("created").and_then(|c| c.as_i64()).unwrap_or(0);
    
    let first_choice = response
        .get("choices")
        .and_then(|choices| choices.get(0));
    
    let finish_reason = first_choice
        .and_then(|choice| choice.get("finish_reason"))
        .and_then(|fr| fr.as_str())
        .unwrap_or("N/A")
        .to_string();
    
    let content = first_choice
        .and_then(|choice| choice.get("message"))
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_str())
        .map(String::from);
    
    let completion_tokens = response
        .get("usage")
        .and_then(|u| u.get("completion_tokens"))
        .and_then(|ct| ct.as_i64())
        .unwrap_or(0);
    
    let total_tokens = response
        .get("usage")
        .and_then(|u| u.get("total_tokens"))
        .and_then(|tt| tt.as_i64())
        .unwrap_or(0);

    ApiResponse {
        id,
        model,
        object,
        created,
        finish_reason,
        content,
        completion_tokens,
        total_tokens,
    }
}
