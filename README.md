# codestral-test-rs

A Rust application to interact with Codestral API for code completion and Mistral API for chat interactions.

## Setup Instructions
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd codestral-test-rs
   ```
2. Set the environment variables for the API keys:
   ```bash
   # For code completion functionality
   export CODESTRAL_API_KEY=<your-codestral-api-key>
   
   # For chat functionality
   export MISTRAL_API_KEY=<your-mistral-api-key>
   ```
3. Run the application:
   
   For code completion:
   ```bash
   cargo run code "<prompt>" "<suffix>" <max_tokens>
   ```
   
   For chat interactions:
   ```bash
   cargo run chat "<message>" [max_tokens]
   ```
   Note: `max_tokens` is optional for chat mode.

## Logging
To enable logging and view `info!` level logs, set the `RUST_LOG` environment variable:
```bash
RUST_LOG=info cargo run code "<prompt>" "<suffix>" <max_tokens>
```

or for chat:
```bash
RUST_LOG=info cargo run chat "<message>" [max_tokens]
```

Logs are written to `generations.log` and include the following fields:
- **Type**: The type of request (code_completion or chat).
- **ID**: The unique identifier for the API response.
- **Model**: The model used for the completion or chat.
- **Object**: The type of API response.
- **Finish Reason**: The reason the completion stopped.
- **Tool Calls**: The number of tool calls in the response (code completion only).
- **Created**: The timestamp of the response creation.
- **Completion Tokens**: The number of tokens used for the completion.
- **Total Tokens**: The total number of tokens used.

## Output Files
- **generations.txt**: Contains the generated content from the API responses, separated by `---`.
- **generations.log**: Contains detailed logs of each API response with timestamps.

## API Endpoints
- **Code Completion**: Uses the Codestral API at `https://codestral.mistral.ai/v1/fim/completions`
- **Chat**: Uses the Mistral API at `https://api.mistral.ai/v1/chat/completions`

## Dependencies
- `reqwest`
- `serde`
- `tokio`
- `serde_json`
- `log`
- `env_logger`
- `chrono`
