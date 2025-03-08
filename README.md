# codestral-test-rs

A Rust application to interact with Codestral API for code completion and Mistral API for chat interactions, with a modular and configurable design.

## Features
- Modular API clients for Codestral (code completion) and Mistral (chat)
- Configuration system with TOML file support
- Command-line interface for code completion, chat, and configuration management
- Detailed logging and output generation

## Setup Instructions
1. Clone the repository:
   ```bash
   git clone [codestral-test-rs](https://github.com/RingCanary/codestral-test-rs.git)
   cd codestral-test-rs
   ```

2. Set the environment variables for the API keys:
   ```bash
   # For code completion functionality
   export CODESTRAL_API_KEY=<your-codestral-api-key>
   
   # For chat functionality
   export MISTRAL_API_KEY=<your-mistral-api-key>
   ```

3. Generate a default configuration file (optional):
   ```bash
   cargo run config generate [config_path]
   ```
   If no path is provided, it will create `config.toml` in the current directory.

4. Run the application:
   
   For code completion:
   ```bash
   cargo run code "<prompt>" "<suffix>" <max_tokens>
   ```
   
   For chat interactions:
   ```bash
   cargo run chat "<message>" [max_tokens]
   ```
   Note: `max_tokens` is optional for chat mode.
   
   To view or generate configuration:
   ```bash
   # Generate default configuration
   cargo run config generate [path]
   
   # View configuration from file
   cargo run config <config_path>
   ```

## Configuration
The application uses a configuration file (`config.toml` by default) to manage settings. Here's an example configuration:

```toml
# Model settings
code_model = "codestral-latest"
chat_model = "mistral-large-latest"

# API URLs
code_api_url = "https://codestral.mistral.ai/v1/fim/completions"
chat_api_url = "https://api.mistral.ai/v1/chat/completions"

# Environment variables for API keys
code_api_key_env = "CODESTRAL_API_KEY"
chat_api_key_env = "MISTRAL_API_KEY"

# Temperature settings (0.0 - 1.0)
temperature_code = 0.0  # Lower temperature for more deterministic code generation
temperature_chat = 0.7  # Higher temperature for more creative chat responses

# File paths
log_file = "generations.log"
output_file = "generations.txt"
```

You can create or modify this file manually or use the `config generate` command.

## Logging
To enable logging and view `info!` level logs, set the `RUST_LOG` environment variable:
```bash
RUST_LOG=info cargo run code "<prompt>" "<suffix>" <max_tokens>
```

or for chat:
```bash
RUST_LOG=info cargo run chat "<message>" [max_tokens]
```

Logs are written to the file specified in your configuration (default: `generations.log`) and include the following fields:
- **Type**: The type of request (code_completion or chat).
- **ID**: The unique identifier for the API response.
- **Model**: The model used for the completion or chat.
- **Object**: The type of API response.
- **Finish Reason**: The reason the completion stopped.
- **Created**: The timestamp of the response creation.
- **Completion Tokens**: The number of tokens used for the completion.
- **Total Tokens**: The total number of tokens used.

## Output Files
- **generations.txt** (default): Contains the generated content from the API responses, separated by `---`.
- **generations.log** (default): Contains detailed logs of each API response with timestamps.

These file paths can be customized in the configuration file.

## Project Structure
- **src/main.rs**: Main entry point and command-line argument handling
- **src/models.rs**: Data structures for API requests and responses
- **src/config.rs**: Configuration management
- **src/logging.rs**: Logging and output generation
- **src/cli.rs**: Command-line interface functionality
- **src/api/**: API client modules
  - **mod.rs**: Module definitions and exports
  - **common.rs**: Common API functionality and traits
  - **mistral.rs**: Mistral API client for chat completions
  - **codestral.rs**: Codestral API client for code completions

## Dependencies
- `reqwest`: HTTP client for API requests
- `serde`, `serde_json`: Serialization/deserialization
- `tokio`: Async runtime
- `log`, `env_logger`: Logging
- `chrono`: Date/time functionality
- `toml`: Configuration file parsing
- `shellexpand`: Path expansion
- `async-trait`: Async traits for API clients
