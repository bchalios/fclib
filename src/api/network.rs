use super::client::ApiClient;
use super::{NetworkInterface, PartialNetworkInterface, Result};

impl ApiClient {
    /// Add a network interface to the VM.
    pub async fn add_network_interface(
        &self,
        iface_id: &str,
        iface: NetworkInterface,
    ) -> Result<()> {
        self.put(&format!("/network-interfaces/{iface_id}"), iface)
            .await
    }

    /// Update a network interface that is attached to the VM.
    pub async fn update_network_interface(
        &self,
        iface_id: &str,
        iface: PartialNetworkInterface,
    ) -> Result<()> {
        self.patch(&format!("/network-interfaces/{iface_id}"), iface)
            .await
    }
}
