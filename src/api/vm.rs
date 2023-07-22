use super::client::ApiClient;
use super::{FirecrackerVersion, FullVmConfiguration, InstanceInfo, Result};

impl ApiClient {
    /// Get information about the microVM instance.
    pub async fn instance_info(&self) -> Result<InstanceInfo> {
        self.get("/").await
    }

    /// Get a full JSON object with the microVM's configuration.
    pub async fn vm_config(&self) -> Result<FullVmConfiguration> {
        self.get("/vm/config").await
    }

    /// Get the version of Firecracker used to launch this microVM.
    pub async fn get_firecracker_version(&self) -> Result<FirecrackerVersion> {
        self.get("/version").await
    }
}
