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
   export CODESRAL_API_KEY=<your-api-key>
   ```
3. Run the application:
   ```bash
   cargo run
   ```

## Logging
To enable logging and view `info!` level logs, set the `RUST_LOG` environment variable:
```bash
RUST_LOG=info cargo run
```

## Dependencies
- `reqwest`
- `serde`
- `tokio`
- `serde_json`
- `log`
- `env_logger`
