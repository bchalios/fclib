pub mod balloon;
pub mod cpu_config;
pub mod drive;
pub mod entropy;
pub mod kernel;
pub mod logger;
pub mod metrics;
pub mod mmds;
pub mod network;
pub mod rate_limiter;
pub mod snapshot;
pub mod vm;
pub mod vsock;

use std::path::Path;

use log::debug;
use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::{Client, ClientBuilder};
use serde::de::DeserializeOwned;
use serde::Deserialize;

/// Errors returned by Firecracker
#[derive(Debug, thiserror::Error, Deserialize, Default)]
pub struct FcError {
    /// A description of the error condition
    #[serde(rename = "fault_message")]
    fault_message: Option<String>,
}

impl std::fmt::Display for FcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fault_message.as_ref().unwrap_or(&"".to_owned())
        )
    }
}

#[derive(Debug, thiserror::Error, displaydoc::Display)]
pub enum FcClientError {
    /// (De)serialization error: {0}
    Serde(#[from] serde_json::Error),
    /// Client error: {0}
    Client(#[from] reqwest::Error),
    /// Firecracker error: {0}
    Firecracker(#[from] FcError),
}

pub type Result<T> = std::result::Result<T, FcClientError>;

/// An HTTP client that can speak the Firecracker API on top of a Unix socket.
#[derive(Debug)]
pub struct ApiClient {
    /// A reqwest [Client] that can speak on top of a UDS
    client: Client,
}

impl ApiClient {
    /// Create a new [ApiClient]
    ///
    /// # Arguments
    ///
    /// * `path` - Filesystem path to the Unix socket.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let client = ClientBuilder::new()
            .unix_socket(path.as_ref())
            .build()
            .unwrap();
        Self { client }
    }

    fn url(&self, path: &str) -> String {
        format!("http://localhost{path}")
    }

    // Performs a PUT request on the specified path
    pub(crate) async fn put<T>(&self, path: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;

        let response = self
            .client
            .put(self.url(path))
            .header(CONTENT_TYPE, "json")
            .header(CONTENT_LENGTH, serialized.len())
            .body(serialized)
            .send()
            .await?;

        let code = response.status();
        if code.is_success() {
            Ok(())
        } else {
            let err: FcError = response.json().await?;
            Err(FcClientError::Firecracker(err))
        }
    }

    // Performs a PATCH request on the specified path
    pub(crate) async fn patch<T>(&self, path: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;
        debug!("PATCH @ {path}");

        let response = self
            .client
            .patch(path)
            .header(CONTENT_TYPE, "json")
            .header(CONTENT_LENGTH, serialized.len())
            .body(serialized)
            .send()
            .await?;

        let code = response.status();
        if code.is_success() {
            Ok(())
        } else {
            let err: FcError = response.json().await?;
            Err(FcClientError::Firecracker(err))
        }
    }

    // Performs a GET request on the specified path
    pub(crate) async fn get<'a, T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let resp = self.client.get(path).send().await?.json().await?;
        Ok(resp)
    }
}
