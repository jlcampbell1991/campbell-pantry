use ::axum::{Router, routing::get};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid value for '{0}': {1}")]
    ParseError(String, String),
}

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|e| ConfigError::ParseError("PORT".to_string(), e.to_string()))?;

        Ok(Self { port })
    }
}
async fn ping() -> &'static str {
    "Hi, Kara, I'm the campbell-pantry!"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load config");

    let app = Router::new().route("/", get(ping));
    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
