use thiserror::Error;
use serde::Deserialize;

#[derive(Error, Debug)]
pub enum SdkError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error (Status {status}): {message}")]
    Api {
        status: u16,
        message: String,
    },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub message: Option<String>,
    pub error: Option<String>,
}
