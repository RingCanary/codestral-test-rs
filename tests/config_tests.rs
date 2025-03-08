use codestral_test_rs::config::Config;
use std::fs;
use std::path::Path;
use test_log::test;
use pretty_assertions::assert_eq;

#[test]
fn test_default_config() {
    let config = Config::default();
    
    // Verify default values
    assert_eq!(config.code_model, "codestral-latest");
    assert_eq!(config.chat_model, "mistral-large-latest");
    assert_eq!(config.code_api_url, "https://codestral.mistral.ai/v1/fim/completions");
    assert_eq!(config.chat_api_url, "https://api.mistral.ai/v1/chat/completions");
    assert_eq!(config.code_api_key_env, "CODESTRAL_API_KEY");
    assert_eq!(config.chat_api_key_env, "MISTRAL_API_KEY");
    assert_eq!(config.temperature_code, 0.0);
    assert_eq!(config.temperature_chat, 0.7);
    assert_eq!(config.log_file, "generations.log");
    assert_eq!(config.output_file, "generations.txt");
}

#[test]
fn test_save_and_load_config() {
    let test_file = "test_config.toml";
    let config = Config::default();
    
    // Save config to test file
    config.save_to_file(test_file).expect("Failed to save config");
    
    // Verify file exists
    assert!(Path::new(test_file).exists());
    
    // Load config from test file
    let loaded_config = Config::load_from_file(test_file).expect("Failed to load config");
    
    // Verify loaded values match original
    assert_eq!(config.code_model, loaded_config.code_model);
    assert_eq!(config.chat_model, loaded_config.chat_model);
    assert_eq!(config.code_api_url, loaded_config.code_api_url);
    assert_eq!(config.chat_api_url, loaded_config.chat_api_url);
    assert_eq!(config.code_api_key_env, loaded_config.code_api_key_env);
    assert_eq!(config.chat_api_key_env, loaded_config.chat_api_key_env);
    assert_eq!(config.temperature_code, loaded_config.temperature_code);
    assert_eq!(config.temperature_chat, loaded_config.temperature_chat);
    assert_eq!(config.log_file, loaded_config.log_file);
    assert_eq!(config.output_file, loaded_config.output_file);
    
    // Clean up
    fs::remove_file(test_file).expect("Failed to remove test file");
}

#[test]
fn test_generate_default_config() {
    let test_file = "test_default_config.toml";
    
    // Generate default config
    Config::generate_default_config(test_file).expect("Failed to generate default config");
    
    // Verify file exists
    assert!(Path::new(test_file).exists());
    
    // Load config from test file
    let loaded_config = Config::load_from_file(test_file).expect("Failed to load config");
    
    // Verify loaded values match default
    let default_config = Config::default();
    assert_eq!(default_config.code_model, loaded_config.code_model);
    assert_eq!(default_config.chat_model, loaded_config.chat_model);
    
    // Clean up
    fs::remove_file(test_file).expect("Failed to remove test file");
}
