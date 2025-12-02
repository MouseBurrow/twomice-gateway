use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Result<Config, &'static str> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL in .env must be set");
        Ok(Config {
            database_url,
            port: 8000,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;

    #[test]
    fn test_config() {
        dotenv().ok();
        let config = Config::init();
        assert!(config.is_ok(), "Config should be initialized successfully");
    }
}
