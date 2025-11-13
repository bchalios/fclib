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
    pub iface_id: String,
    /// Host level path for the guest network interface
    #[serde(rename = "host_dev_name")]
    pub host_dev_name: String,
    /// MAC address to use for the interface inside the guest
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "guest_mac")]
    pub guest_mac: Option<String>,

    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "rx_rate_limiter")]
    pub rx_rate_limiter: Option<RateLimiter>,

    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "tx_rate_limiter")]
    pub tx_rate_limiter: Option<RateLimiter>,
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
}

/// Defines a partial network interface structure, used to update the
/// rate limiters for that interface, after microvm start.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    #[serde(rename = "iface_id")]
    pub iface_id: String,
    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "rx_rate_limiter")]
    pub rx_rate_limiter: Option<RateLimiter>,
    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "tx_rate_limiter")]
    pub tx_rate_limiter: Option<RateLimiter>,
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
