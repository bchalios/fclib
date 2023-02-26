pub mod client;
pub mod models;

pub use models::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Hyper error: {0}")]
    Hyper(#[from] hyper::Error),
    #[error("(De)serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("API error: {0}")]
    Firecracker(#[from] ApiError),
}

#[derive(Debug, thiserror::Error)]
pub struct ApiError {
    pub code: hyper::StatusCode,
    pub content: models::Error,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {}: {}",
            self.code,
            self.content.fault_message().unwrap_or(&"".to_string())
        )
    }
}
