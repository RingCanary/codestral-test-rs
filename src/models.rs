use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompletionRequest {
    pub model: String,
    pub prompt: String,
    pub suffix: String,
    pub max_tokens: u32,
    pub temperature: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: f64,
}

pub enum ApiMode {
    CodeCompletion,
    Chat,
}

pub struct ApiResponse {
    pub id: String,
    pub model: String,
    pub object: String,
    pub created: i64,
    pub finish_reason: String,
    pub content: Option<String>,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}
