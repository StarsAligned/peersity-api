use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rust_log: String,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let dotenv_result = dotenv();
        if dotenv_result.is_err() {
            println!("No .env file found. Using environment variables.");
        } else {
            if let Ok(dotenv_path) = std::env::var("DOTENV_PATH") {
                if let Ok(content) = std::fs::read_to_string(dotenv_path) {
                    if content.trim().is_empty() {
                        return Err(config::ConfigError::Message("Empty .env file".into()));
                    } else if content
                        .lines()
                        .any(|line| !line.contains('=') || line.split('=').count() != 2)
                    {
                        return Err(config::ConfigError::Message("Invalid .env file".into()));
                    }
                }
            }
        }

        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        return settings.try_deserialize();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempdir::TempDir;

    #[test]
    fn test_config_new_with_env_file() {
        let temp_dir = TempDir::new("config_test").unwrap();
        let env_file_path = temp_dir.path().join(".env");
        fs::write(env_file_path.clone(), "TEST_KEY=TEST_VALUE").unwrap();

        env::set_var("DOTENV_PATH", env_file_path.to_str().unwrap());
        let config = Config::new();

        assert!(config.is_ok());
    }

    #[test]
    fn test_config_new_without_env_file() {
        env::remove_var("DOTENV_PATH");
        let config = Config::new();

        assert!(config.is_ok());
    }

    #[test]
    fn test_config_new_with_invalid_env_file() {
        let temp_dir = TempDir::new("config_test").unwrap();
        let env_file_path = temp_dir.path().join(".env");
        fs::write(env_file_path.clone(), "INVALID_KEY:INVALID_VALUE").unwrap();

        env::set_var("DOTENV_PATH", env_file_path.to_str().unwrap());
        let config = Config::new();

        assert!(config.is_err());
    }

    #[test]
    fn test_config_new_with_empty_env_file() {
        let temp_dir = TempDir::new("config_test").unwrap();
        let env_file_path = temp_dir.path().join(".env");
        fs::write(env_file_path.clone(), "").unwrap();

        env::set_var("DOTENV_PATH", env_file_path.to_str().unwrap());
        let config = Config::new();

        assert!(config.is_err());
    }
}
