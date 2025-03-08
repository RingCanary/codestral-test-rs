use crate::Config;
use crate::api::common::{ApiClient, get_api_key, make_api_request};
use crate::api::progress::with_progress;
use crate::models::CompletionRequest;
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use log::debug;

pub struct CodestralClient {
    model: String,
    temperature: f64,
    api_url: String,
    api_key: String,
    client: Client,
}

impl CodestralClient {
    pub fn new(config: Config) -> Result<Self> {
        let api_key = get_api_key("CODESTRAL_API_KEY")?;
        
        Ok(Self {
            model: config.code_model.clone(),
            temperature: config.code_temperature,
            api_url: config.code_api_url.clone(),
            api_key,
            client: Client::new(),
        })
    }
    
    pub async fn code_completion(&self, prompt: &str, suffix: &str, max_tokens: u32) -> Result<Value> {
        debug!("Sending request to Codestral API for code completion");
        
        let request = CompletionRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            suffix: suffix.to_string(),
            max_tokens,
            temperature: self.temperature,
        };
        
        let request_json = json!(request);
        
        // Use progress tracking for the API request
        with_progress(
            &format!("Generating code with model '{}'...", self.model),
            "Code completion received!",
            self.send_request(request_json)
        ).await
    }
}

#[async_trait]
impl ApiClient for CodestralClient {
    async fn send_request(&self, request_body: Value) -> Result<Value> {
        make_api_request(&self.client, &self.api_url, &self.api_key, request_body).await
    }
}
