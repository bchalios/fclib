use super::client::ApiClient;
use super::{Result, Vm};

impl ApiClient {
    /// Pause microVM.
    pub async fn pause_microvm(&self) -> Result<()> {
        let vm_state = Vm::new("Paused".into());
        self.patch("/vm", vm_state).await
    }

    /// Resume microVM.
    pub async fn resume_microvm(&self) -> Result<()> {
        let vm_state = Vm::new("Resumed".into());
        self.patch("/vm", vm_state).await
    }
}
