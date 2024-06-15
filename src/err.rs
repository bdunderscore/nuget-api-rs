use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Invalid version range: {0}")]
    InvalidVersionRange(String),
}

pub type Result<T> = std::result::Result<T, Error>;