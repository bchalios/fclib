use super::client::ApiClient;
use super::{MmdsConfig, MmdsContentsObject, Result};

impl ApiClient {
    pub async fn get_mmds(&self) -> Result<serde_json::Value> {
        self.get("/mmds").await
    }

    pub async fn patch_mmds(&self, body: MmdsContentsObject) -> Result<()> {
        self.patch("/mmds", body).await
    }

    pub async fn put_mmds(&self, body: MmdsContentsObject) -> Result<()> {
        self.put("/mmds", body).await
    }

    pub async fn configure_mmds(&self, config: MmdsConfig) -> Result<()> {
        self.put("/mmds/config", config).await
    }
}
