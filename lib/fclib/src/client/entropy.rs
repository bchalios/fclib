//! EntropyDevice : Defines an entropy device.

use serde_derive::{Deserialize, Serialize};

use super::rate_limiter::RateLimiter;
use super::{ApiClient, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EntropyDevice {
    #[serde(rename = "rate_limiter")]
    rate_limiter: Option<RateLimiter>,
}

impl EntropyDevice {
    /// Defines an entropy device.
    pub fn new() -> EntropyDevice {
        EntropyDevice { rate_limiter: None }
    }

    pub fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.rate_limiter = Some(rate_limiter);
    }

    pub fn with_rate_limiter(mut self, rate_limiter: RateLimiter) -> EntropyDevice {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn rate_limiter(&self) -> Option<&RateLimiter> {
        self.rate_limiter.as_ref()
    }

    pub fn reset_rate_limiter(&mut self) {
        self.rate_limiter = None;
    }
}

impl ApiClient {
    /// Enable entropy device for the microVM.
    ///
    /// The [`Entropy`] configuration allows enabling a rate limiter for the device. This operation
    /// is only allowed before the VM has been booted.
    ///
    /// # Arguments
    ///
    /// * `entropy` - The configuration of the [`Entropy`] device
    pub async fn configure_entropy_device(&mut self, entropy: &EntropyDevice) -> Result<()> {
        self.put("/entropy", entropy).await
    }
}
