//! MmdsConfig : Defines the MMDS configuration.

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct MmdsConfig {
    /// Enumeration indicating the MMDS version to be configured.
    #[serde(rename = "version")]
    version: Option<String>,
    /// List of the network interface IDs capable of forwarding packets to the MMDS. Network
    /// interface IDs mentioned must be valid at the time of this request. The net device model
    /// will reply to HTTP GET requests sent to the MMDS address via the interfaces mentioned. In
    /// this case, both ARP requests and TCP segments heading to `ipv4_address` are intercepted by
    /// the device model, and do not reach the associated TAP device.
    #[serde(rename = "network_interfaces")]
    network_interfaces: Vec<String>,
    /// A valid IPv4 link-local address.
    #[serde(rename = "ipv4_address")]
    ipv4_address: Option<String>,
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

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: String) -> MmdsConfig {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }

    pub fn set_network_interfaces(&mut self, network_interfaces: Vec<String>) {
        self.network_interfaces = network_interfaces;
    }

    pub fn with_network_interfaces(mut self, network_interfaces: Vec<String>) -> MmdsConfig {
        self.network_interfaces = network_interfaces;
        self
    }

    pub fn network_interfaces(&self) -> &Vec<String> {
        &self.network_interfaces
    }

    pub fn set_ipv4_address(&mut self, ipv4_address: String) {
        self.ipv4_address = Some(ipv4_address);
    }

    pub fn with_ipv4_address(mut self, ipv4_address: String) -> MmdsConfig {
        self.ipv4_address = Some(ipv4_address);
        self
    }

    pub fn ipv4_address(&self) -> Option<&String> {
        self.ipv4_address.as_ref()
    }

    pub fn reset_ipv4_address(&mut self) {
        self.ipv4_address = None;
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
