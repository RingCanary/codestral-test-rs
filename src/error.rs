use std::fmt;
use std::io;
use std::env::VarError;
use thiserror::Error;

/// Application-specific error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("API error: {0}")]
    Api(String),
    
    #[error("Missing environment variable: {0}")]
    EnvVar(#[from] VarError),
    
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    
    #[error("JSON serialization/deserialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Network request error: {0}")]
    Request(#[from] reqwest::Error),
    
    #[error("Logging error: {0}")]
    Logging(String),
    
    #[error("Invalid CLI usage: {0}")]
    CliUsage(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl AppError {
    /// Create a new configuration error with a message
    pub fn config<S: Into<String>>(msg: S) -> Self {
        AppError::Config(msg.into())
    }
    
    /// Create a new API error with a message
    pub fn api<S: Into<String>>(msg: S) -> Self {
        AppError::Api(msg.into())
    }
    
    /// Create a new logging error with a message
    pub fn logging<S: Into<String>>(msg: S) -> Self {
        AppError::Logging(msg.into())
    }
    
    /// Create a new CLI usage error with a message
    pub fn cli_usage<S: Into<String>>(msg: S) -> Self {
        AppError::CliUsage(msg.into())
    }
    
    /// Create a new unknown error with a message
    pub fn unknown<S: Into<String>>(msg: S) -> Self {
        AppError::Unknown(msg.into())
    }
}

// Define a Result type alias for our app
pub type Result<T> = std::result::Result<T, AppError>;

// Extension trait to convert error types
pub trait ErrorExt<T> {
    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: fmt::Display;
}

// Implement our context trait for standard Result
impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: fmt::Display,
    {
        self.map_err(|err| {
            let context = context();
            AppError::Unknown(format!("{}: {}", context, err))
        })
    }
}
