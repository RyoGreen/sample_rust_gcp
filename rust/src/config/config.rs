use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

#[derive(Deserialize)]
pub struct HttpConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub http: HttpConfig,
}

impl Config {
    pub fn from_file(file_path: &str) -> Self {
        let config_data = fs::read_to_string(file_path).expect("Failed to read config file");
        serde_json::from_str(&config_data).expect("Failed to parse config file")
    }
}
