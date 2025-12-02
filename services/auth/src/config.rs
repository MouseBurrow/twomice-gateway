use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init() -> Result<Config, &'static str> {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(Config {
            database_url,
            port: 8000,
        })
    }
}
