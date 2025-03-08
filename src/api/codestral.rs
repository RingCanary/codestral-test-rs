use crate::api::common::{ApiClient, get_api_key, make_api_request};
use crate::config::Config;
use crate::models::CompletionRequest;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;
use log::debug;

pub struct CodestralClient {
    client: Client,
    api_key: String,
    config: Config,
}

impl CodestralClient {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let api_key = get_api_key(&config.code_api_key_env)?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            config,
        })
    }
    
    pub async fn code_completion(&self, prompt: &str, suffix: &str, max_tokens: u32) -> Result<Value, Box<dyn Error>> {
        debug!("Sending request to Codestral API for code completion");
        
        let completion_request = CompletionRequest {
            model: self.config.code_model.clone(),
            prompt: prompt.to_string(),
            suffix: suffix.to_string(),
            max_tokens,
            temperature: self.config.temperature_code,
        };
        
        let request_body = json!(completion_request);
        
        self.send_request(request_body).await
    }
}

#[async_trait]
impl ApiClient for CodestralClient {
    async fn send_request(&self, request_body: Value) -> Result<Value, Box<dyn Error>> {
        make_api_request(
            &self.client, 
            &self.config.code_api_url, 
            &self.api_key, 
            request_body
        ).await
    }
    
    fn get_model(&self) -> &str {
        &self.config.code_model
    }
    
    fn get_temperature(&self) -> f64 {
        self.config.temperature_code
    }
}
