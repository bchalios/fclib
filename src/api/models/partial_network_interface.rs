//! PartialNetworkInterface : Defines a partial network interface structure, used to update the
//! rate limiters for that interface, after microvm start.
use crate::api::RateLimiter;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialNetworkInterface {
    #[serde(rename = "iface_id")]
    iface_id: String,
    #[serde(rename = "rx_rate_limiter")]
    rx_rate_limiter: Option<RateLimiter>,
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
