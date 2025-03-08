use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use codestral_test_rs::{Config, models::{CompletionRequest, ChatMessage, ChatCompletionRequest}};
use serde_json::json;
use codestral_test_rs::api::extract_response_fields;

fn bench_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("API Request Serialization");
    
    // Benchmark CompletionRequest serialization
    let completion_request = CompletionRequest {
        model: "codestral-latest".to_string(),
        prompt: "fn main() {".to_string(),
        suffix: "}".to_string(),
        max_tokens: 100,
        temperature: 0.0,
    };
    
    group.bench_function("serialize_completion_request", |b| {
        b.iter(|| serde_json::to_string(&completion_request))
    });
    
    // Benchmark ChatCompletionRequest serialization with varying message counts
    for msg_count in [1, 5, 10, 20] {
        let messages = vec![
            ChatMessage {
                role: "user".to_string(),
                content: "Hello, how are you?".to_string(),
            };
            msg_count
        ];
        
        let chat_request = ChatCompletionRequest {
            model: "mistral-large-latest".to_string(),
            messages,
            max_tokens: Some(100),
            temperature: 0.7,
        };
        
        group.bench_with_input(
            BenchmarkId::new("serialize_chat_request", msg_count), 
            &chat_request, 
            |b, request| {
                b.iter(|| serde_json::to_string(request))
            }
        );
    }
    
    group.finish();
}

fn bench_response_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("API Response Parsing");
    
    // Create sample responses of different sizes
    let sizes = [
        ("small", 1, 10),
        ("medium", 50, 100),
        ("large", 200, 500),
        ("extra_large", 500, 1000)
    ];
    
    for (size_name, completion_tokens, total_tokens) in sizes {
        // Generate a code completion response
        let content = "fn hello_world() {\n".to_string() + &"    println!(\"Hello, world!\");\n".repeat(completion_tokens / 10) + "}";
        
        let code_response = json!({
            "id": format!("test-id-{}", size_name),
            "model": "codestral-latest",
            "object": "text_completion",
            "created": 1234567890,
            "choices": [
                {
                    "finish_reason": "stop",
                    "text": content
                }
            ],
            "usage": {
                "completion_tokens": completion_tokens,
                "total_tokens": total_tokens
            }
        });
        
        group.bench_with_input(
            BenchmarkId::new("parse_code_response", size_name), 
            &code_response, 
            |b, response| {
                b.iter(|| extract_response_fields(response))
            }
        );
        
        // Generate a chat completion response
        let chat_content = "I'm an AI assistant and I'm here to help you with your coding questions. ".repeat(completion_tokens / 20);
        
        let chat_response = json!({
            "id": format!("chat-id-{}", size_name),
            "model": "mistral-large-latest",
            "object": "chat.completion",
            "created": 1234567890,
            "choices": [
                {
                    "index": 0,
                    "message": {
                        "role": "assistant",
                        "content": chat_content
                    },
                    "finish_reason": "stop"
                }
            ],
            "usage": {
                "prompt_tokens": total_tokens - completion_tokens,
                "completion_tokens": completion_tokens,
                "total_tokens": total_tokens
            }
        });
        
        group.bench_with_input(
            BenchmarkId::new("parse_chat_response", size_name), 
            &chat_response, 
            |b, response| {
                b.iter(|| extract_response_fields(response))
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_serialization, bench_response_parsing);
criterion_main!(benches);
