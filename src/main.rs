mod models;
mod api;
mod logging;
mod cli;
mod config;

use cli::{handle_code_completion, handle_chat};
use config::Config;

use std::error::Error;
use std::env;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    
    // Load configuration
    let config = Config::load();
    info!("Loaded configuration with code model '{}' and chat model '{}'", config.code_model, config.chat_model);
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  For code completion: {} code <prompt> <suffix> <max_tokens>", args[0]);
        eprintln!("  For chat: {} chat <message> [max_tokens]", args[0]);
        eprintln!("  For configuration: {} config [generate | path/to/config.toml]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "code" => handle_code_completion(&args, &config).await?,
        "chat" => handle_chat(&args, &config).await?,
        "config" => {
            if args.len() >= 3 {
                match args[2].as_str() {
                    "generate" => {
                        let path = if args.len() >= 4 {
                            &args[3]
                        } else {
                            "config.toml"
                        };
                        Config::generate_default_config(path)?;
                        println!("Default configuration generated at {}", path);
                    },
                    path => {
                        match Config::load_from_file(path) {
                            Ok(loaded_config) => {
                                println!("Configuration loaded from {}:", path);
                                println!("Code model: {}", loaded_config.code_model);
                                println!("Chat model: {}", loaded_config.chat_model);
                                println!("Code API URL: {}", loaded_config.code_api_url);
                                println!("Chat API URL: {}", loaded_config.chat_api_url);
                                println!("Log file: {}", loaded_config.log_file);
                                println!("Output file: {}", loaded_config.output_file);
                            },
                            Err(e) => {
                                eprintln!("Error loading configuration: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                }
            } else {
                eprintln!("Usage for config: {} config [generate | path/to/config.toml]", args[0]);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Invalid mode. Use 'code', 'chat', or 'config'.");
            std::process::exit(1);
        }
    }

    Ok(())
}
