use crate::api::RateLimiter;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PartialDrive {
    #[serde(rename = "drive_id")]
    drive_id: String,
    /// Host level path for the guest drive
    #[serde(rename = "path_on_host")]
    path_on_host: Option<String>,
    #[serde(rename = "rate_limiter")]
    rate_limiter: Option<RateLimiter>,
}

impl PartialDrive {
    pub fn new(drive_id: String) -> PartialDrive {
        PartialDrive {
            drive_id,
            path_on_host: None,
            rate_limiter: None,
        }
    }

    pub fn set_drive_id(&mut self, drive_id: String) {
        self.drive_id = drive_id;
    }

    pub fn with_drive_id(mut self, drive_id: String) -> PartialDrive {
        self.drive_id = drive_id;
        self
    }

    pub fn drive_id(&self) -> &String {
        &self.drive_id
    }

    pub fn set_path_on_host(&mut self, path_on_host: String) {
        self.path_on_host = Some(path_on_host);
    }

    pub fn with_path_on_host(mut self, path_on_host: String) -> PartialDrive {
        self.path_on_host = Some(path_on_host);
        self
    }

    pub fn path_on_host(&self) -> Option<&String> {
        self.path_on_host.as_ref()
    }

    pub fn reset_path_on_host(&mut self) {
        self.path_on_host = None;
    }

    pub fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.rate_limiter = Some(rate_limiter);
    }

    pub fn with_rate_limiter(mut self, rate_limiter: RateLimiter) -> PartialDrive {
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
