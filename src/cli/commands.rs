use clap::{Parser, Subcommand};

/// Codestral Test CLI - A command-line interface for interacting with Codestral and Mistral APIs
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// CLI Subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate code completions
    #[command(visible_alias = "code")]
    CodeCompletion {
        /// Prompt text before the cursor
        #[arg(required = true)]
        prompt: String,

        /// Text after the cursor (suffix)
        #[arg(required = true)]
        suffix: String,

        /// Maximum number of tokens to generate
        #[arg(short, long)]
        max_tokens: Option<u32>,
    },

    /// Chat with the model
    Chat {
        /// Message to send to the chat model
        #[arg(required = true)]
        message: String,

        /// Maximum number of tokens to generate
        #[arg(short, long)]
        max_tokens: Option<u32>,
    },

    /// Manage configuration
    Config {
        /// Config operation to perform
        #[command(subcommand)]
        action: ConfigCommands,
    },
}

/// Configuration subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Generate a default configuration file
    Generate {
        /// Path where to save the config file
        #[arg(default_value = "config.toml")]
        path: String,
    },
    
    /// View configuration from a file
    View {
        /// Path to the config file to view
        #[arg(default_value = "config.toml")]
        path: String,
    },
    
    /// Load configuration from a custom path
    Load {
        /// Path to the config file to load
        #[arg(required = true)]
        path: String,
    },
}
