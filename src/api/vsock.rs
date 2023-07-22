use super::client::ApiClient;
use super::{Result, Vsock};

impl ApiClient {
    pub async fn config_vsock(&self, vsock: Vsock) -> Result<()> {
        self.put("/vsock", vsock).await
    }
}
