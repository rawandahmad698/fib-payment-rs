use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Retry exhausted: {0}")]
    RetryExhausted(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error: {message} (Code: {code}, Status: {status_code}, Trace: {trace_id:?})")]
    ApiError {
        status_code: u16,
        code: String,
        message: String,
        trace_id: Option<String>,
    },
}

