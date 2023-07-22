use super::client::ApiClient;
use super::{Result, SnapshotCreateParams, SnapshotLoadParams};

impl ApiClient {
    /// Take a snapshot of the VM.
    pub async fn snapshot_vm(&self, body: SnapshotCreateParams) -> Result<()> {
        self.put("/snapshot/create", body).await
    }

    /// Resume a VM from a snapshot.
    pub async fn resume_vm_from_snapshot(&self, body: SnapshotLoadParams) -> Result<()> {
        self.put("/snapshot/load", body).await
    }
}
