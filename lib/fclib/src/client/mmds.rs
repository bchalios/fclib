//! MmdsConfig : Defines the MMDS configuration.

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct MmdsConfig {
    /// Enumeration indicating the MMDS version to be configured.
    pub version: Option<String>,
    /// List of the network interface IDs capable of forwarding packets to the MMDS. Network
    /// interface IDs mentioned must be valid at the time of this request. The net device model
    /// will reply to HTTP GET requests sent to the MMDS address via the interfaces mentioned. In
    /// this case, both ARP requests and TCP segments heading to `ipv4_address` are intercepted by
    /// the device model, and do not reach the associated TAP device.
    pub network_interfaces: Vec<String>,
    /// A valid IPv4 link-local address.
    pub ipv4_address: Option<String>,
}

impl MmdsConfig {
    /// Defines the MMDS configuration.
    pub fn new(network_interfaces: Vec<String>) -> MmdsConfig {
        MmdsConfig {
            version: None,
            network_interfaces,
            ipv4_address: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MmdsContentsObject {}

impl MmdsContentsObject {
    /// Describes the contents of MMDS in JSON format.
    pub fn new() -> MmdsContentsObject {
        MmdsContentsObject {}
    }
}

impl ApiClient {
    pub async fn mmds_contents(&self) -> Result<serde_json::Value> {
        self.get("/mmds").await
    }

    pub async fn update_mmds(&self, body: &MmdsContentsObject) -> Result<()> {
        self.patch("/mmds", body).await
    }

    pub async fn store_mmds(&self, body: &MmdsContentsObject) -> Result<()> {
        self.put("/mmds", body).await
    }

    pub async fn configure_mmds(&self, config: &MmdsConfig) -> Result<()> {
        self.put("/mmds/config", config).await
    }
}
