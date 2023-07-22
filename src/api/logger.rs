use super::client::ApiClient;
use super::{Logger, Result};

impl ApiClient {
    pub async fn config_logger(&self, logger: Logger) -> Result<()> {
        self.put("/logger", logger).await
    }
}
