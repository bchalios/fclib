use std::path::{Path, PathBuf};

use super::{ApiError, Error, Result};
use hyper::body::Buf;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use log::debug;
use serde::de::DeserializeOwned;

/// Describes a Client that can speak the Firecracker API on top of a Unix socket.
#[derive(Debug)]
pub struct ApiClient {
    /// Path to the Unix socket
    path: PathBuf,
    /// Hyper [Client] that can speak on top of a [UnixConnector]
    client: Client<UnixConnector>,
}

impl ApiClient {
    /// Create a new [ApiClient]
    ///
    /// # Arguments
    ///
    /// * `path` - Filesystem path to the Unix socket.
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let client = Client::unix();
        Self {
            path: path.as_ref().to_path_buf(),
            client,
        }
    }

    // Returns the URI for a particular endpoint
    fn uri(&self, endpoint: &str) -> Uri {
        Uri::new(&self.path, endpoint)
    }

    // Performs a PUT request on a specific endpoint
    pub(crate) async fn put<T>(&self, endpoint: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;
        let uri = self.uri(endpoint);
        debug!("PUT @ {endpoint}");

        let request = Request::put(uri)
            .header(CONTENT_TYPE, "json")
            .header(CONTENT_LENGTH, serialized.len())
            .body(Body::from(serialized))
            .unwrap();

        let resp = self.client.request(request).await?;
        let code = resp.status();
        if code.is_success() {
            Ok(())
        } else {
            let body = hyper::body::aggregate(resp).await?;
            let content: super::models::Error = serde_json::from_reader(body.reader())?;
            Err(Error::from(ApiError { code, content }))
        }
    }

    // Performs a PATCH request on a specific endpoint
    pub(crate) async fn patch<T>(&self, endpoint: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;
        let uri = self.uri(endpoint);
        debug!("PATCH @ {endpoint}");

        let request = Request::patch(uri)
            .header(CONTENT_TYPE, "json")
            .header(CONTENT_LENGTH, serialized.len())
            .body(Body::from(serialized))
            .unwrap();

        let resp = self.client.request(request).await?;
        let code = resp.status();
        if code.is_success() {
            Ok(())
        } else {
            let body = hyper::body::aggregate(resp).await?;
            let content: super::models::Error = serde_json::from_reader(body.reader())?;
            Err(Error::from(ApiError { code, content }))
        }
    }

    // Performs a GET request on a specific endpoint
    pub(crate) async fn get<'a, T>(&self, endpoint: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = self.uri(endpoint);
        let request = Request::get(uri).body(Body::default()).unwrap();

        let resp = self.client.request(request).await?;
        let body = hyper::body::aggregate(resp).await?;

        let blah: T = serde_json::from_reader(body.reader())?;
        Ok(blah)
    }
}
