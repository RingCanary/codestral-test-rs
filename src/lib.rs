pub mod models;
pub mod api;
pub mod logging;
pub mod cli;
pub mod config;

// Re-export the most commonly used items for convenience
pub use config::Config;
pub use cli::{handle_code_completion, handle_chat};
pub use api::{MistralClient, CodestralClient};
