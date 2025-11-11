#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

use super::rate_limiter::RateLimiter;
use super::{ApiClient, Result};

/// Configuration of a microVM network interface
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// ID of the network interface
    #[serde(rename = "iface_id")]
    iface_id: String,
    /// Host level path for the guest network interface
    #[serde(rename = "host_dev_name")]
    host_dev_name: String,
    /// MAC address to use for the interface inside the guest
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "guest_mac")]
    guest_mac: Option<String>,

    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "rx_rate_limiter")]
    rx_rate_limiter: Option<RateLimiter>,

    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "tx_rate_limiter")]
    tx_rate_limiter: Option<RateLimiter>,
}

impl NetworkInterface {
    /// Defines a network interface.
    pub fn new(host_dev_name: String, iface_id: String) -> NetworkInterface {
        NetworkInterface {
            guest_mac: None,
            host_dev_name,
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }

    pub fn set_guest_mac(&mut self, guest_mac: String) {
        self.guest_mac = Some(guest_mac);
    }

    pub fn with_guest_mac(mut self, guest_mac: String) -> NetworkInterface {
        self.guest_mac = Some(guest_mac);
        self
    }

    pub fn guest_mac(&self) -> Option<&String> {
        self.guest_mac.as_ref()
    }

    pub fn reset_guest_mac(&mut self) {
        self.guest_mac = None;
    }

    pub fn set_host_dev_name(&mut self, host_dev_name: String) {
        self.host_dev_name = host_dev_name;
    }

    pub fn with_host_dev_name(mut self, host_dev_name: String) -> NetworkInterface {
        self.host_dev_name = host_dev_name;
        self
    }

    pub fn host_dev_name(&self) -> &String {
        &self.host_dev_name
    }

    pub fn set_iface_id(&mut self, iface_id: String) {
        self.iface_id = iface_id;
    }

    pub fn with_iface_id(mut self, iface_id: String) -> NetworkInterface {
        self.iface_id = iface_id;
        self
    }

    pub fn iface_id(&self) -> &String {
        &self.iface_id
    }

    pub fn set_rx_rate_limiter(&mut self, rx_rate_limiter: RateLimiter) {
        self.rx_rate_limiter = Some(rx_rate_limiter);
    }

    pub fn with_rx_rate_limiter(mut self, rx_rate_limiter: RateLimiter) -> NetworkInterface {
        self.rx_rate_limiter = Some(rx_rate_limiter);
        self
    }

    pub fn rx_rate_limiter(&self) -> Option<&RateLimiter> {
        self.rx_rate_limiter.as_ref()
    }

    pub fn reset_rx_rate_limiter(&mut self) {
        self.rx_rate_limiter = None;
    }

    pub fn set_tx_rate_limiter(&mut self, tx_rate_limiter: RateLimiter) {
        self.tx_rate_limiter = Some(tx_rate_limiter);
    }

    pub fn with_tx_rate_limiter(mut self, tx_rate_limiter: RateLimiter) -> NetworkInterface {
        self.tx_rate_limiter = Some(tx_rate_limiter);
        self
    }

    pub fn tx_rate_limiter(&self) -> Option<&RateLimiter> {
        self.tx_rate_limiter.as_ref()
    }

    pub fn reset_tx_rate_limiter(&mut self) {
        self.tx_rate_limiter = None;
    }
}

/// Defines a partial network interface structure, used to update the
/// rate limiters for that interface, after microvm start.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    #[serde(rename = "iface_id")]
    iface_id: String,
    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "rx_rate_limiter")]
    rx_rate_limiter: Option<RateLimiter>,
    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "tx_rate_limiter")]
    tx_rate_limiter: Option<RateLimiter>,
}

impl PartialNetworkInterface {
    /// Defines a partial network interface structure, used to update the rate limiters for that
    /// interface, after microvm start.
    pub fn new(iface_id: String) -> PartialNetworkInterface {
        PartialNetworkInterface {
            iface_id,
            rx_rate_limiter: None,
            tx_rate_limiter: None,
        }
    }

    pub fn set_iface_id(&mut self, iface_id: String) {
        self.iface_id = iface_id;
    }

    pub fn with_iface_id(mut self, iface_id: String) -> PartialNetworkInterface {
        self.iface_id = iface_id;
        self
    }

    pub fn iface_id(&self) -> &String {
        &self.iface_id
    }

    pub fn set_rx_rate_limiter(&mut self, rx_rate_limiter: RateLimiter) {
        self.rx_rate_limiter = Some(rx_rate_limiter);
    }

    pub fn with_rx_rate_limiter(mut self, rx_rate_limiter: RateLimiter) -> PartialNetworkInterface {
        self.rx_rate_limiter = Some(rx_rate_limiter);
        self
    }

    pub fn rx_rate_limiter(&self) -> Option<&RateLimiter> {
        self.rx_rate_limiter.as_ref()
    }

    pub fn reset_rx_rate_limiter(&mut self) {
        self.rx_rate_limiter = None;
    }

    pub fn set_tx_rate_limiter(&mut self, tx_rate_limiter: RateLimiter) {
        self.tx_rate_limiter = Some(tx_rate_limiter);
    }

    pub fn with_tx_rate_limiter(mut self, tx_rate_limiter: RateLimiter) -> PartialNetworkInterface {
        self.tx_rate_limiter = Some(tx_rate_limiter);
        self
    }

    pub fn tx_rate_limiter(&self) -> Option<&RateLimiter> {
        self.tx_rate_limiter.as_ref()
    }

    pub fn reset_tx_rate_limiter(&mut self) {
        self.tx_rate_limiter = None;
    }
}

impl ApiClient {
    /// Add a network interface to the VM.
    pub async fn add_network_interface(
        &self,
        iface_id: &str,
        iface: &NetworkInterface,
    ) -> Result<()> {
        self.put(&format!("/network-interfaces/{iface_id}"), iface)
            .await
    }

    /// Update a network interface that is attached to the VM.
    pub async fn update_network_interface(
        &self,
        iface_id: &str,
        iface: &PartialNetworkInterface,
    ) -> Result<()> {
        self.patch(&format!("/network-interfaces/{iface_id}"), iface)
            .await
    }
}
