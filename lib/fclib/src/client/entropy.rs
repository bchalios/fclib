//! EntropyDevice : Defines an entropy device.

use serde_derive::{Deserialize, Serialize};

use super::rate_limiter::RateLimiter;
use super::{ApiClient, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct EntropyDevice {
    pub rate_limiter: Option<RateLimiter>,
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
