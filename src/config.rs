use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        if let Err(_) = dotenv() {
            println!("No .env file found. Using environment variables.");
        }

        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        return settings.try_deserialize();
    }
}