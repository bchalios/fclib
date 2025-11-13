//! Metrics : Describes the configuration option for the metrics capability.

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    #[serde(rename = "metrics_path")]
    pub metrics_path: String,
}

impl ApiClient {
    pub async fn config_metrics(&self, logger: &Metrics) -> Result<()> {
        self.put("/metrics", logger).await
    }
}
