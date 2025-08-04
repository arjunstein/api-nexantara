use dotenv::dotenv;
use std::env;

pub struct Config {
    pub database_url: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            api_key: env::var("API_KEY").expect("API_KEY must be set"),
        }
    }
}