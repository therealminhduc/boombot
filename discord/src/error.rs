use thiserror::Error;

#[derive(Error, Debug)]
pub enum BotError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("URL cleaning error: {0}")]
    CleaningError(String),
}

pub type Result<T> = std::result::Result<T, BotError>;
