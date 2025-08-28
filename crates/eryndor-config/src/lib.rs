//! # Eryndor Config
//!
//! Configuration loading utilities for the Eryndor game engine.
//! Provides standardized JSON configuration loading with validation
//! and error handling for all Eryndor systems.

use eryndor_core::traits::ConfigLoader;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use thiserror::Error;
use tracing::{error, info, warn};

pub mod prelude {
    pub use crate::{ConfigError, JsonConfigLoader};
    pub use eryndor_core::traits::ConfigLoader;
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Validation error: {message}")]
    Validation { message: String },
    
    #[error("Directory error: {path} - {reason}")]
    Directory { path: String, reason: String },
}

/// Generic JSON configuration loader that implements the ConfigLoader trait
pub struct JsonConfigLoader;

impl<T> ConfigLoader<T> for JsonConfigLoader 
where
    T: DeserializeOwned + Clone,
{
    type Error = ConfigError;
    
    fn load_from_file(path: &str) -> Result<T, Self::Error> {
        info!("📁 Loading configuration from: {}", path);
        
        let file_path = Path::new(path);
        if !file_path.exists() {
            error!("Configuration file not found: {}", path);
            return Err(ConfigError::FileNotFound { 
                path: path.to_string() 
            });
        }
        
        let content = fs::read_to_string(file_path)?;
        let config: T = serde_json::from_str(&content)?;
        
        info!("✅ Successfully loaded configuration from: {}", path);
        Ok(config)
    }
    
    fn load_from_directory(dir: &str) -> Result<Vec<T>, Self::Error> {
        info!("📁 Loading configurations from directory: {}", dir);
        
        let dir_path = Path::new(dir);
        if !dir_path.exists() {
            error!("Configuration directory not found: {}", dir);
            return Err(ConfigError::Directory {
                path: dir.to_string(),
                reason: "Directory does not exist".to_string(),
            });
        }
        
        if !dir_path.is_dir() {
            error!("Path is not a directory: {}", dir);
            return Err(ConfigError::Directory {
                path: dir.to_string(),
                reason: "Path is not a directory".to_string(),
            });
        }
        
        let mut configs = Vec::new();
        let entries = fs::read_dir(dir_path)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match Self::load_from_file(path.to_str().unwrap()) {
                    Ok(config) => configs.push(config),
                    Err(e) => {
                        warn!("⚠️ Failed to load config from {:?}: {}", path, e);
                        // Continue loading other files instead of failing completely
                    }
                }
            }
        }
        
        info!("✅ Loaded {} configurations from directory: {}", configs.len(), dir);
        Ok(configs)
    }
    
    fn validate_config(config: &T) -> Result<(), Self::Error> {
        // Default implementation - systems can override for custom validation
        Ok(())
    }
}

/// Load a single JSON configuration file with error handling
pub fn load_json_config<T>(path: &str) -> Result<T, ConfigError>
where
    T: DeserializeOwned + Clone,
{
    JsonConfigLoader::load_from_file(path)
}

/// Load all JSON configuration files from a directory
pub fn load_json_configs_from_dir<T>(dir: &str) -> Result<Vec<T>, ConfigError>
where
    T: DeserializeOwned + Clone,
{
    JsonConfigLoader::load_from_directory(dir)
}

/// Utility function to create default config directories if they don't exist
pub fn ensure_config_directories(dirs: &[&str]) -> Result<(), ConfigError> {
    for dir in dirs {
        let path = Path::new(dir);
        if !path.exists() {
            info!("📁 Creating configuration directory: {}", dir);
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::fs;
    use tempfile::tempdir;
    
    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    struct TestConfig {
        name: String,
        value: i32,
    }
    
    #[test]
    fn test_load_json_config() {
        let temp_dir = tempdir().unwrap();
        let config_path = temp_dir.path().join("test.json");
        
        let test_config = TestConfig {
            name: "test".to_string(),
            value: 42,
        };
        
        let config_json = serde_json::to_string_pretty(&test_config).unwrap();
        fs::write(&config_path, config_json).unwrap();
        
        let loaded_config: TestConfig = load_json_config(config_path.to_str().unwrap()).unwrap();
        assert_eq!(loaded_config, test_config);
    }
    
    #[test]
    fn test_file_not_found() {
        let result: Result<TestConfig, _> = load_json_config("nonexistent.json");
        assert!(matches!(result, Err(ConfigError::FileNotFound { .. })));
    }
}