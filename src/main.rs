use codestral_test_rs::{
    cli::{Cli, Commands, handle_code_completion, handle_chat, handle_config}, 
    Config, 
    error::{Result}
};

use clap::Parser;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let cli = Cli::parse();
    
    // Set up logging based on verbosity
    match cli.debug {
        0 => std::env::set_var("RUST_LOG", "warn"),
        1 => std::env::set_var("RUST_LOG", "info"),
        2 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    }
    
    // Initialize logger
    env_logger::init();
    
    // Load configuration from default path or specified path
    let config_path = cli.config.as_deref().unwrap_or("config.toml");
    let config = Config::load_from_file(config_path)?;
    info!("Loaded configuration with code model '{}' and chat model '{}'", config.code_model, config.chat_model);
    
    // Handle subcommands
    match &cli.command {
        Commands::CodeCompletion { prompt, suffix, max_tokens } => {
            handle_code_completion(prompt, suffix, *max_tokens, &config).await?
        },
        Commands::Chat { message, max_tokens } => {
            handle_chat(message, *max_tokens, &config).await?
        },
        Commands::Config { action } => {
            handle_config(action, &config)?
        },
    }

    Ok(())
}
