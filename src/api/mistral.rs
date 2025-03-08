use crate::api::common::{ApiClient, get_api_key, make_api_request};
use crate::config::Config;
use crate::models::{ChatMessage, ChatCompletionRequest};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;
use log::debug;

pub struct MistralClient {
    client: Client,
    api_key: String,
    config: Config,
}

impl MistralClient {
    pub fn new(config: Config) -> Result<Self, Box<dyn Error>> {
        let api_key = get_api_key(&config.chat_api_key_env)?;
        
        Ok(Self {
            client: Client::new(),
            api_key,
            config,
        })
    }
    
    pub async fn chat(&self, message: &str, max_tokens: Option<u32>) -> Result<Value, Box<dyn Error>> {
        debug!("Sending request to Mistral API for chat completion");
        
        let chat_request = ChatCompletionRequest {
            model: self.config.chat_model.clone(),
            messages: vec![
                ChatMessage {
                    role: "user".to_string(),
                    content: message.to_string(),
                }
            ],
            max_tokens,
            temperature: self.config.temperature_chat,
        };
        
        let request_body = json!(chat_request);
        
        self.send_request(request_body).await
    }
}

#[async_trait]
impl ApiClient for MistralClient {
    async fn send_request(&self, request_body: Value) -> Result<Value, Box<dyn Error>> {
        make_api_request(
            &self.client, 
            &self.config.chat_api_url, 
            &self.api_key, 
            request_body
        ).await
    }
    
    fn get_model(&self) -> &str {
        &self.config.chat_model
    }
    
    fn get_temperature(&self) -> f64 {
        self.config.temperature_chat
    }
}
