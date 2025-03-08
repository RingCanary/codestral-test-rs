# codestral-test-rs

A simple Rust application to experiment with the Codestral API.

## Setup Instructions
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd codestral-test-rs
   ```
2. Set the environment variable for the API key:
   ```bash
   export CODESTRAL_API_KEY=<your-api-key>
   ```
3. Run the application:
   ```bash
   cargo run "<prompt>" "<suffix>" <max_tokens>
   ```

## Logging
To enable logging and view `info!` level logs, set the `RUST_LOG` environment variable:
```bash
RUST_LOG=info cargo run "<prompt>" "<suffix>" <max_tokens>
```

Logs are written to `generations.log` and include the following fields:
- **ID**: The unique identifier for the API response.
- **Model**: The model used for the completion.
- **Object**: The type of API response.
- **Finish Reason**: The reason the completion stopped.
- **Tool Calls**: The number of tool calls in the response.
- **Created**: The timestamp of the response creation.
- **Completion Tokens**: The number of tokens used for the completion.
- **Total Tokens**: The total number of tokens used.

## Output Files
- **generations.txt**: Contains the generated content from the API responses, separated by `---`.
- **generations.log**: Contains detailed logs of each API response with timestamps.

## Ignored Files
The following files are ignored by Git:
- `*.log`: All log files.
- `generations.txt`: The file containing generated content.

## Dependencies
- `reqwest`
- `serde`
- `tokio`
- `serde_json`
- `log`
- `env_logger`
- `chrono`
