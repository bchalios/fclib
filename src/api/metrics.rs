use super::client::ApiClient;
use super::{Metrics, Result};

impl ApiClient {
    pub async fn config_metrics(&self, logger: Metrics) -> Result<()> {
        self.put("/metrics", logger).await
    }
}
