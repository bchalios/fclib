use super::client::ApiClient;
use super::{BootSource, Result};

impl ApiClient {
    /// Setup the boot source of the VM.
    pub async fn set_boot_source(&mut self, boot_source: BootSource) -> Result<()> {
        self.put("/boot-source", boot_source).await
    }
}
