use crate::Config;
use crate::api::common::{ApiClient, get_api_key, make_api_request};
use crate::models::{ChatMessage, ChatCompletionRequest};
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::{json, Value};
use log::debug;

pub struct MistralClient {
    model: String,
    temperature: f64,
    api_url: String,
    api_key: String,
    client: Client,
}

impl MistralClient {
    pub fn new(config: Config) -> Result<Self> {
        let api_key = get_api_key("MISTRAL_API_KEY")?;
        
        Ok(Self {
            model: config.chat_model.clone(),
            temperature: config.chat_temperature,
            api_url: config.chat_api_url.clone(),
            api_key,
            client: Client::new(),
        })
    }
    
    pub async fn chat(&self, message: &str, max_tokens: Option<u32>) -> Result<Value> {
        debug!("Sending request to Mistral API for chat completion");
        
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: message.to_string(),
            }
        ];
        
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            max_tokens,
            temperature: self.temperature,
        };
        
        let request_json = json!(request);
        self.send_request(request_json).await
    }
}

#[async_trait]
impl ApiClient for MistralClient {
    async fn send_request(&self, request_body: Value) -> Result<Value> {
        make_api_request(&self.client, &self.api_url, &self.api_key, request_body).await
    }
    
    fn get_model(&self) -> &str {
        &self.model
    }
    
    fn get_temperature(&self) -> f64 {
        self.temperature
    }
}
