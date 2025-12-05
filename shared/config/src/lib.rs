use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    #[cfg(debug_assertions)]
    pub fn load_local_env() {
        eprintln!("Code is checking local env!!!");
        dotenv().expect("Environmental variables must be set correctly!");
    }

    pub fn init(service: &str) -> Config {
        let url_path = format!("{}_DATABASE_URL", service);
        let database_url = env::var(&url_path)
            .unwrap_or_else(|_| panic!("Missing environmental variable: {}", url_path));

        let service_port = env::var("PORT")
            .unwrap_or_else(|_| panic!("Missing environmental variable PORT"))
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Port value must be 16bits integer"));

        eprintln!("Successfully loaded configs");

        Config {
            database_url,
            port: service_port,
        }
    }
}
