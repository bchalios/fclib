use super::client::ApiClient;
use super::{MachineConfiguration, Result};

impl ApiClient {
    /// Get the configuration of the VM.
    pub async fn get_machine_configuration(&self) -> Result<MachineConfiguration> {
        self.get("/machine-config").await
    }

    /// Update the configuration of the VM.
    pub async fn update_machine_configuration(&self, body: MachineConfiguration) -> Result<()> {
        self.patch("/machine-config", body).await
    }

    /// Configure the VM.
    pub async fn configure_machine(&self, body: MachineConfiguration) -> Result<()> {
        self.put("/machine-config", body).await
    }
}
