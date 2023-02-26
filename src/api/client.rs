use std::path::{Path, PathBuf};

use hyper::body::Buf;
use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use hyperlocal::{UnixClientExt, UnixConnector, Uri};
use serde::de::DeserializeOwned;

use super::models::*;
use super::{ApiError, Error};

pub type Result<T> = std::result::Result<T, Error>;

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
    async fn put<T>(&self, endpoint: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;
        let uri = self.uri(endpoint);
        println!("Performing put to {uri:?}");

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
    async fn patch<T>(&self, endpoint: &str, body: T) -> Result<()>
    where
        T: serde::Serialize,
    {
        let serialized = serde_json::to_string(&body)?;
        let uri = self.uri(endpoint);
        println!("Performing put to {uri:?}");

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
    async fn get<'a, T>(&self, endpoint: &str) -> Result<T>
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

impl ApiClient {
    pub async fn create_snapshot(&self, body: SnapshotCreateParams) -> Result<()> {
        self.put("/snapshot/create", body).await
    }

    pub async fn create_sync_action(&self, info: InstanceActionInfo) -> Result<()> {
        self.put("/actions", info).await
    }

    pub async fn describe_balloon_config(&self) -> Result<Balloon> {
        self.get("/ballon").await
    }

    pub async fn describe_balloon_stats(&self) -> Result<BalloonStats> {
        self.get("/ballon/statistics").await
    }

    pub async fn describe_instance(&self) -> Result<InstanceInfo> {
        self.get("/").await
    }

    pub async fn get_export_vm_config(&self) -> Result<FullVmConfiguration> {
        self.get("/vm/config").await
    }

    pub async fn get_firecracker_version(&self) -> Result<FirecrackerVersion> {
        self.get("/version").await
    }

    pub async fn get_machine_configuration(&self) -> Result<MachineConfiguration> {
        self.get("/machine-config").await
    }

    pub async fn get_mmds(&self) -> Result<serde_json::Value> {
        self.get("/mmds").await
    }

    pub async fn load_snapshot(&self, body: SnapshotLoadParams) -> Result<()> {
        self.put("/snapshot/load", body).await
    }

    pub async fn patch_balloon(&self, body: BalloonUpdate) -> Result<()> {
        self.patch("/balloon", body).await
    }

    pub async fn patch_balloon_stats_interval(&self, body: BalloonStatsUpdate) -> Result<()> {
        self.patch("/balloon/statistics", body).await
    }

    pub async fn patch_guest_drive_by_id(&self, drive_id: &str, body: PartialDrive) -> Result<()> {
        self.patch(&format!("/drives/{drive_id}"), body).await
    }

    pub async fn patch_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: PartialNetworkInterface,
    ) -> Result<()> {
        self.patch(&format!("/network-interfaces/{iface_id}"), body)
            .await
    }

    pub async fn patch_machine_configuration(&self, body: MachineConfiguration) -> Result<()> {
        self.patch("/machine-config", body).await
    }

    pub async fn patch_mmds(&self, body: MmdsContentsObject) -> Result<()> {
        self.patch("/mmds", body).await
    }

    pub async fn patch_vm(&self, body: Vm) -> Result<()> {
        self.patch("/vm", body).await
    }

    pub async fn put_balloon(&self, body: Balloon) -> Result<()> {
        self.put("/balloon", body).await
    }

    pub async fn put_guest_boot_source(&self, body: BootSource) -> Result<()> {
        self.put("/boot-source", body).await
    }

    pub async fn put_guest_drive_by_id(&self, drive_id: &str, body: Drive) -> Result<()> {
        self.put(&format!("/drives/{drive_id}"), body).await
    }

    pub async fn put_guest_network_interface_by_id(
        &self,
        iface_id: &str,
        body: NetworkInterface,
    ) -> Result<()> {
        self.put(&format!("/network-interfaces/{iface_id}"), body)
            .await
    }

    pub async fn put_guest_vsock(&self, body: Vsock) -> Result<()> {
        self.put("/vsock", body).await
    }

    pub async fn put_logger(&self, body: Logger) -> Result<()> {
        self.put("/logger", body).await
    }

    pub async fn put_machine_configuration(&self, body: MachineConfiguration) -> Result<()> {
        self.put("/machine-config", body).await
    }

    pub async fn put_metrics(&self, body: Metrics) -> Result<()> {
        self.put("/metrics", body).await
    }

    pub async fn put_mmds(&self, body: MmdsContentsObject) -> Result<()> {
        self.put("/mmds", body).await
    }

    pub async fn put_mmds_config(&self, body: MmdsConfig) -> Result<()> {
        self.put("/mmds/config", body).await
    }
}
