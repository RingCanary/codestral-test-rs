use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use log::{info, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_code_model")]
    pub code_model: String,
    
    #[serde(default = "default_chat_model")]
    pub chat_model: String,
    
    #[serde(default = "default_code_api_url")]
    pub code_api_url: String,
    
    #[serde(default = "default_chat_api_url")]
    pub chat_api_url: String,
    
    #[serde(default = "default_code_api_key_env")]
    pub code_api_key_env: String,
    
    #[serde(default = "default_chat_api_key_env")]
    pub chat_api_key_env: String,
    
    #[serde(default = "default_temperature_code")]
    pub temperature_code: f64,
    
    #[serde(default = "default_temperature_chat")]
    pub temperature_chat: f64,
    
    #[serde(default = "default_log_file")]
    pub log_file: String,
    
    #[serde(default = "default_output_file")]
    pub output_file: String,
}

fn default_code_model() -> String {
    "codestral-latest".to_string()
}

fn default_chat_model() -> String {
    "mistral-large-latest".to_string()
}

fn default_code_api_url() -> String {
    "https://codestral.mistral.ai/v1/fim/completions".to_string()
}

fn default_chat_api_url() -> String {
    "https://api.mistral.ai/v1/chat/completions".to_string()
}

fn default_code_api_key_env() -> String {
    "CODESTRAL_API_KEY".to_string()
}

fn default_chat_api_key_env() -> String {
    "MISTRAL_API_KEY".to_string()
}

fn default_temperature_code() -> f64 {
    0.0
}

fn default_temperature_chat() -> f64 {
    0.7
}

fn default_log_file() -> String {
    "generations.log".to_string()
}

fn default_output_file() -> String {
    "generations.txt".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            code_model: default_code_model(),
            chat_model: default_chat_model(),
            code_api_url: default_code_api_url(),
            chat_api_url: default_chat_api_url(),
            code_api_key_env: default_code_api_key_env(),
            chat_api_key_env: default_chat_api_key_env(),
            temperature_code: default_temperature_code(),
            temperature_chat: default_temperature_chat(),
            log_file: default_log_file(),
            output_file: default_output_file(),
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                match toml::from_str(&contents) {
                    Ok(config) => {
                        info!("Successfully loaded configuration from {:?}", path.as_ref());
                        Ok(config)
                    },
                    Err(e) => {
                        error!("Failed to parse configuration file: {}", e);
                        Err(format!("Failed to parse configuration file: {}", e).into())
                    }
                }
            },
            Err(e) => {
                error!("Failed to read configuration file: {}", e);
                Err(format!("Failed to read configuration file: {}", e).into())
            }
        }
    }

    pub fn load() -> Self {
        let config_paths = ["config.toml", "~/.config/codestral-cli/config.toml"];
        
        for path in &config_paths {
            let path = shellexpand::tilde(path);
            let path_str = path.to_string();
            
            if Path::new(&path_str).exists() {
                match Self::load_from_file(&path_str) {
                    Ok(config) => return config,
                    Err(e) => {
                        error!("Error loading config from {}: {}", path_str, e);
                        // Continue to next path or default
                    }
                }
            }
        }
        
        info!("No configuration file found, using default settings.");
        Self::default()
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn Error>> {
        let toml_str = toml::to_string_pretty(self)?;
        fs::write(&path, toml_str)?;
        info!("Configuration saved to {:?}", path.as_ref());
        Ok(())
    }

    pub fn generate_default_config<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
        let config = Self::default();
        config.save_to_file(path)
    }
}
