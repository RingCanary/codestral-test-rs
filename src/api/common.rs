use crate::error::{AppError, Result, ErrorExt};
use serde_json::Value;
use reqwest::{Client, header};
use std::env;
use async_trait::async_trait;

/// Get the API key from an environment variable
pub fn get_api_key(env_var_name: &str) -> Result<String> {
    env::var(env_var_name)
        .with_context(|| format!("Environment variable {} not found or not valid", env_var_name))
}

/// Make an API request to the specified URL with the provided JSON body
pub async fn make_api_request(
    client: &Client, 
    url: &str, 
    api_key: &str, 
    json_body: Value
) -> Result<Value> {
    // Create the request
    let response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json_body)
        .send()
        .await
        .with_context(|| format!("Failed to send request to {}", url))?;
    
    // Check if response is successful
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await
            .unwrap_or_else(|_| String::from("Could not extract error message from response"));
        
        return Err(AppError::api(format!("API request failed with status {}: {}", status, error_text)));
    }
    
    // Parse the response JSON
    match response.json::<Value>().await {
        Ok(json_response) => Ok(json_response),
        Err(e) => Err(AppError::api(format!("Failed to parse API response: {}", e)))
    }
}

// Extract common fields from API responses
pub fn extract_response_fields(response: &Value) -> crate::models::ApiResponse {
    let id = response.get("id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
    let object = response.get("object").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
    let model = response.get("model").and_then(|v| v.as_str()).unwrap_or("unknown").to_string();
    
    // Convert u64 to i64 for created timestamp
    let created = response
        .get("created")
        .and_then(|v| v.as_u64())
        .map(|v| v as i64) // Safe conversion as long as value is within i64 range
        .unwrap_or(0);
    
    // Content extraction differs between API responses
    let content = if let Some(choices) = response.get("choices") {
        if let Some(first_choice) = choices.as_array().and_then(|a| a.first()) {
            if let Some(text) = first_choice.get("text") {
                text.as_str().map(|s| s.to_string())
            } else if let Some(message) = first_choice.get("message") {
                message.get("content").and_then(|c| c.as_str()).map(|s| s.to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };
    
    // Finish reason extraction 
    let finish_reason = if let Some(choices) = response.get("choices") {
        if let Some(first_choice) = choices.as_array().and_then(|a| a.first()) {
            first_choice.get("finish_reason").and_then(|v| v.as_str()).unwrap_or("unknown").to_string()
        } else {
            "unknown".to_string()
        }
    } else {
        "unknown".to_string()
    };
    
    // Token counts (convert u32 to i64)
    let completion_tokens = response
        .get("usage")
        .and_then(|usage| usage.get("completion_tokens"))
        .and_then(|v| v.as_u64())
        .map(|v| v as i64) // Safe conversion from u64 to i64 for reasonable token counts
        .unwrap_or(0);
    
    let total_tokens = response
        .get("usage")
        .and_then(|usage| usage.get("total_tokens"))
        .and_then(|v| v.as_u64())
        .map(|v| v as i64) // Safe conversion from u64 to i64 for reasonable token counts
        .unwrap_or(0);
    
    crate::models::ApiResponse {
        id,
        object,
        model,
        created,
        content,
        finish_reason,
        completion_tokens,
        total_tokens,
    }
}

// API client trait
#[async_trait]
pub trait ApiClient {
    async fn send_request(&self, request_body: Value) -> Result<Value>;
    fn get_model(&self) -> &str;
    fn get_temperature(&self) -> f64;
}
