use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
}

impl Config {
    pub fn init(service: &str) -> Config {
        #[cfg(debug_assertions)]
        {
            // Only runs in local debug sessions!
            eprintln!("Code is running locally!");
            dotenv().expect("Environmental variables must be set correctly!");
        }

        let url_path = format!("{}_DATABASE_URL", service);
        let database_url = env::var(&url_path)
            .unwrap_or_else(|_| panic!("Missing environmental variable: {}", url_path));

        let port_path = format!("{}_SERVICE_PORT", service);
        let service_port = env::var(&port_path)
            .unwrap_or_else(|_| panic!("Missing environmental variable: {}", port_path))
            .parse::<u16>()
            .unwrap_or_else(|_| panic!("Port value must be 16bits integer"));

        Config {
            database_url,
            port: service_port,
        }
    }
}
