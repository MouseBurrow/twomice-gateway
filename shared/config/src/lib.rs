use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    /// Load the .env file in the root
    ///
    /// # Errors
    /// * `dotenvy::Error` - If the .env file cannot be loaded
    pub fn load_env() -> dotenvy::Result<PathBuf> {
        dotenv()
    }

    pub fn init() -> Result<Config, &'static str> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL in .env must be set");
        Ok(Config {
            database_url,
            port: 8000,
        })
    }
}
