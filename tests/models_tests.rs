use codestral_test_rs::models::{CompletionRequest, ChatMessage, ChatCompletionRequest, ApiResponse};
use test_log::test;
use serde_json;

#[test]
fn test_completion_request_serialization() {
    let request = CompletionRequest {
        model: "codestral-latest".to_string(),
        prompt: "fn hello_world() {".to_string(),
        suffix: "}".to_string(),
        max_tokens: 100,
        temperature: 0.0,
    };
    
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    
    // Verify the JSON structure is correct
    let json: serde_json::Value = serde_json::from_str(&serialized).expect("Invalid JSON");
    assert_eq!(json["model"], "codestral-latest");
    assert_eq!(json["prompt"], "fn hello_world() {");
    assert_eq!(json["suffix"], "}");
    assert_eq!(json["max_tokens"], 100);
    assert_eq!(json["temperature"], 0.0);
}

#[test]
fn test_chat_message_serialization() {
    let message = ChatMessage {
        role: "user".to_string(),
        content: "Hello, world!".to_string(),
    };
    
    let serialized = serde_json::to_string(&message).expect("Failed to serialize");
    
    // Verify the JSON structure
    let json: serde_json::Value = serde_json::from_str(&serialized).expect("Invalid JSON");
    assert_eq!(json["role"], "user");
    assert_eq!(json["content"], "Hello, world!");
}

#[test]
fn test_chat_completion_request_serialization() {
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }
    ];
    
    let request = ChatCompletionRequest {
        model: "mistral-large-latest".to_string(),
        messages,
        max_tokens: Some(100),
        temperature: 0.7,
    };
    
    let serialized = serde_json::to_string(&request).expect("Failed to serialize");
    
    // Verify the JSON structure
    let json: serde_json::Value = serde_json::from_str(&serialized).expect("Invalid JSON");
    assert_eq!(json["model"], "mistral-large-latest");
    assert_eq!(json["messages"][0]["role"], "user");
    assert_eq!(json["messages"][0]["content"], "Hello, how are you?");
    assert_eq!(json["max_tokens"], 100);
    assert_eq!(json["temperature"], 0.7);
}

#[test]
fn test_api_response_creation() {
    let response = ApiResponse {
        id: "test-id".to_string(),
        model: "test-model".to_string(),
        object: "text_completion".to_string(),
        created: 1234567890,
        finish_reason: "stop".to_string(),
        content: Some("Hello, world!".to_string()),
        completion_tokens: 3,
        total_tokens: 10,
    };
    
    assert_eq!(response.id, "test-id");
    assert_eq!(response.model, "test-model");
    assert_eq!(response.object, "text_completion");
    assert_eq!(response.created, 1234567890);
    assert_eq!(response.finish_reason, "stop");
    assert_eq!(response.content, Some("Hello, world!".to_string()));
    assert_eq!(response.completion_tokens, 3);
    assert_eq!(response.total_tokens, 10);
}
