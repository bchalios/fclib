use super::client::ApiClient;
use super::{Result, SnapshotCreateParams, SnapshotLoadParams};

impl ApiClient {
    /// Take a snapshot of the VM.
    pub async fn snapshot_microvm(&self, body: &SnapshotCreateParams) -> Result<()> {
        self.put("/snapshot/create", body).await
    }

    /// Resume a VM from a snapshot.
    pub async fn load_microvm_snapshot(&self, body: &SnapshotLoadParams) -> Result<()> {
        self.put("/snapshot/load", body).await
    }
}
