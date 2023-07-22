//! FirecrackerVersion : Describes the Firecracker version.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Firecracker build version.
    #[serde(rename = "firecracker_version")]
    firecracker_version: String,
}

impl FirecrackerVersion {
    /// Describes the Firecracker version.
    pub fn new(firecracker_version: String) -> FirecrackerVersion {
        FirecrackerVersion {
            firecracker_version,
        }
    }

    pub fn set_firecracker_version(&mut self, firecracker_version: String) {
        self.firecracker_version = firecracker_version;
    }

    pub fn with_firecracker_version(mut self, firecracker_version: String) -> FirecrackerVersion {
        self.firecracker_version = firecracker_version;
        self
    }

    pub fn firecracker_version(&self) -> &String {
        &self.firecracker_version
    }
}
