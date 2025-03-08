use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use log::{info, error};
use shellexpand;
use crate::error::{AppError, Result, ErrorExt};

fn default_code_model() -> String {
    "codestral-latest".to_string()
}

fn default_chat_model() -> String {
    "mistral-large-latest".to_string()
}

fn default_temperature_code() -> f64 {
    0.2
}

fn default_temperature_chat() -> f64 {
    0.7
}

fn default_code_api_key_env() -> String {
    "CODESTRAL_API_KEY".to_string()
}

fn default_chat_api_key_env() -> String {
    "MISTRAL_API_KEY".to_string()
}

fn default_code_api_url() -> String {
    "https://api.anthropic.com/v1/fim/completions".to_string()
}

fn default_chat_api_url() -> String {
    "https://api.mistral.ai/v1/chat/completions".to_string()
}

fn default_max_tokens_code() -> u32 {
    512
}

fn default_max_tokens_chat() -> u32 {
    2048
}

fn default_log_directory() -> String {
    "logs".to_string()
}

fn default_config_path() -> String {
    "config.toml".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_code_model")]
    pub code_model: String,
    
    #[serde(default = "default_chat_model")]
    pub chat_model: String,
    
    #[serde(default = "default_temperature_code")]
    pub code_temperature: f64,
    
    #[serde(default = "default_temperature_chat")]
    pub chat_temperature: f64,
    
    #[serde(default = "default_code_api_key_env")]
    pub code_api_key_env: String,
    
    #[serde(default = "default_chat_api_key_env")]
    pub chat_api_key_env: String,
    
    #[serde(default = "default_code_api_url")]
    pub code_api_url: String,
    
    #[serde(default = "default_chat_api_url")]
    pub chat_api_url: String,
    
    #[serde(default = "default_max_tokens_code")]
    pub max_tokens_code: u32,
    
    #[serde(default = "default_max_tokens_chat")]
    pub max_tokens_chat: u32,
    
    #[serde(default = "default_log_directory")]
    pub log_directory: String,
    
    #[serde(default = "default_config_path")]
    pub config_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            code_model: default_code_model(),
            chat_model: default_chat_model(),
            code_temperature: default_temperature_code(),
            chat_temperature: default_temperature_chat(),
            code_api_key_env: default_code_api_key_env(),
            chat_api_key_env: default_chat_api_key_env(),
            code_api_url: default_code_api_url(),
            chat_api_url: default_chat_api_url(),
            max_tokens_code: default_max_tokens_code(),
            max_tokens_chat: default_max_tokens_chat(),
            log_directory: default_log_directory(),
            config_path: default_config_path(),
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let expanded_path = shellexpand::tilde(path.as_ref().to_str().unwrap_or("~config.toml"));
        let path_str = expanded_path.to_string();
        
        info!("Loading configuration from {}", path_str);
        
        match fs::read_to_string(&path_str) {
            Ok(content) => {
                toml::from_str(&content)
                    .with_context(|| format!("Failed to parse config file at {}", path_str))
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
                    info!("Configuration file not found at {}. Generating default configuration.", path_str);
                    let config = Config::default();
                    match config.save_to_file(&path_str) {
                        Ok(_) => {
                            info!("Default configuration saved to {}", path_str);
                            Ok(config)
                        },
                        Err(e) => {
                            error!("Failed to save default configuration: {}", e);
                            Err(AppError::config(format!("Failed to save default configuration: {}", e)))
                        }
                    }
                } else {
                    error!("Failed to read configuration file: {}", e);
                    Err(AppError::Io(e))
                }
            }
        }
    }
    
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let expanded_path = shellexpand::tilde(path.as_ref().to_str().unwrap_or("~/.codestral-test-rs/config.toml"));
        let path_str = expanded_path.to_string();
        let path = Path::new(&path_str);
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
        
        // Serialize config to TOML
        let content = toml::to_string(self)
            .with_context(|| "Failed to serialize configuration to TOML")?;
        
        // Write to file
        let mut file = fs::File::create(path)
            .with_context(|| format!("Failed to create config file {}", path.display()))?;
            
        file.write_all(content.as_bytes())
            .with_context(|| format!("Failed to write config to {}", path.display()))?;
            
        info!("Configuration saved to {}", path.display());
        Ok(())
    }
    
    pub fn generate_default_config<P: AsRef<Path>>(path: P) -> Result<()> {
        let config = Config::default();
        config.save_to_file(path)?;
        Ok(())
    }
    
    pub fn get_log_directory_path(&self) -> PathBuf {
        let expanded_path = shellexpand::tilde(&self.log_directory);
        PathBuf::from(expanded_path.to_string())
    }
}
