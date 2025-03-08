use codestral_test_rs::{
    cli::{handle_code_completion, handle_chat}, 
    Config, 
    error::{AppError, Result}
};

use std::env;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();
    
    // Load configuration from default path
    let config_path = "~/.codestral-test-rs/config.toml";
    let config = Config::load_from_file(config_path)?;
    info!("Loaded configuration with code model '{}' and chat model '{}'", config.code_model, config.chat_model);
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        let usage = format!(
            "Usage:\n  \
            For code completion: {} code-completion <prompt> <suffix> [max_tokens]\n  \
            For chat: {} chat <message> [max_tokens]\n  \
            For configuration: {} config [generate | view | path/to/config.toml]",
            args[0], args[0], args[0]
        );
        eprintln!("{}", usage);
        return Err(AppError::cli_usage(&usage));
    }

    match args[1].as_str() {
        "code-completion" => {
            let cmd_args = args[2..].to_vec();
            handle_code_completion(&cmd_args, &config).await?
        },
        "chat" => {
            let cmd_args = args[2..].to_vec();
            handle_chat(&cmd_args, &config).await?
        },
        "config" => {
            if args.len() >= 3 {
                match args[2].as_str() {
                    "generate" => {
                        let path = if args.len() >= 4 {
                            &args[3]
                        } else {
                            "~/.codestral-test-rs/config.toml"
                        };
                        Config::generate_default_config(path)?;
                        println!("Default configuration generated at {}", path);
                    },
                    "view" => {
                        let path = if args.len() >= 4 {
                            &args[3]
                        } else {
                            "~/.codestral-test-rs/config.toml"
                        };
                        match Config::load_from_file(path) {
                            Ok(loaded_config) => {
                                println!("Configuration from {}:", path);
                                println!("Code model: {}", loaded_config.code_model);
                                println!("Chat model: {}", loaded_config.chat_model);
                                println!("Code temperature: {}", loaded_config.code_temperature);
                                println!("Chat temperature: {}", loaded_config.chat_temperature);
                                println!("Code API URL: {}", loaded_config.code_api_url);
                                println!("Chat API URL: {}", loaded_config.chat_api_url);
                                println!("Logs directory: {}", loaded_config.log_directory);
                            },
                            Err(e) => {
                                eprintln!("Error loading configuration: {}", e);
                                return Err(e);
                            }
                        }
                    }
                    path => {
                        match Config::load_from_file(path) {
                            Ok(loaded_config) => {
                                println!("Configuration loaded from {}:", path);
                                println!("Code model: {}", loaded_config.code_model);
                                println!("Chat model: {}", loaded_config.chat_model);
                                println!("Code temperature: {}", loaded_config.code_temperature);
                                println!("Chat temperature: {}", loaded_config.chat_temperature);
                                println!("Code API URL: {}", loaded_config.code_api_url);
                                println!("Chat API URL: {}", loaded_config.chat_api_url);
                                println!("Logs directory: {}", loaded_config.log_directory);
                            },
                            Err(e) => {
                                eprintln!("Error loading configuration: {}", e);
                                return Err(e);
                            }
                        }
                    }
                }
            } else {
                let error_msg = format!("Usage for config: {} config [generate | view | path/to/config.toml]", args[0]);
                eprintln!("{}", error_msg);
                return Err(AppError::cli_usage(&error_msg));
            }
        },
        _ => {
            let error_msg = "Invalid mode. Use 'code-completion', 'chat', or 'config'.";
            eprintln!("{}", error_msg);
            return Err(AppError::cli_usage(error_msg));
        }
    }

    Ok(())
}
