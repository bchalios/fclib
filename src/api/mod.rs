pub mod actions;
pub mod balloon;
pub mod client;
pub mod drive;
pub mod kernel;
pub mod logger;
pub mod machine_config;
pub mod metrics;
pub mod mmds;
pub mod models;
pub mod network;
pub mod snapshot;
pub mod vm;
pub mod vm_state;
pub mod vsock;

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

pub type Result<T> = std::result::Result<T, Error>;
