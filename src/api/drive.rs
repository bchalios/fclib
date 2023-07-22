use super::client::ApiClient;
use super::{Drive, Result};

impl ApiClient {
    /// Add a new disk to the VM.
    ///
    /// This operation is only allowed before the VM has been booted.
    ///
    /// # Arguments
    ///
    /// * `drive_id` - The id of the new disk
    /// * `drive` - The [`Drive`] object to attach to the VM
    pub async fn add_drive(&mut self, drive_id: &str, drive: &Drive) -> Result<()> {
        self.put(&format!("/drives/{drive_id}"), &drive).await
    }

    /// Update a disk of the VM.
    ///
    /// # Arguments
    ///
    /// * `drive_id` - The id of the new disk
    /// * `drive` - The [`Drive`] object to attach to the VM
    pub async fn patch_drive(&mut self, drive_id: &str, drive: &Drive) -> Result<()> {
        self.patch(&format!("/drives/{drive_id}"), drive).await
    }
}
