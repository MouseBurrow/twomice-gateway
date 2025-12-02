use std::env;
use std::path::PathBuf;

#[macro_export]
macro_rules! env_dir {
    () => {{ concat!(env!("CARGO_MANIFEST_DIR"), "/.env") }};
}

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    /// Load the .env file in the given directory
    ///
    /// # Arguments
    /// * `dir` - The result of env_dir!()
    ///
    /// # Errors
    /// * `dotenvy::Error` - If the .env file cannot be loaded
    pub fn load_env(dir: &str) -> dotenvy::Result<PathBuf> {
        dotenvy::from_filename(dir)
    }

    pub fn init() -> Result<Config, &'static str> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL in .env must be set");
        Ok(Config {
            database_url,
            port: 8000,
        })
    }
}
