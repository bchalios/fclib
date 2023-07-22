//! Metrics : Describes the configuration option for the metrics capability.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    /// Path to the named pipe or file where the JSON-formatted metrics are flushed.
    #[serde(rename = "metrics_path")]
    metrics_path: String,
}

impl Metrics {
    /// Describes the configuration option for the metrics capability.
    pub fn new(metrics_path: String) -> Metrics {
        Metrics { metrics_path }
    }

    pub fn set_metrics_path(&mut self, metrics_path: String) {
        self.metrics_path = metrics_path;
    }

    pub fn with_metrics_path(mut self, metrics_path: String) -> Metrics {
        self.metrics_path = metrics_path;
        self
    }

    pub fn metrics_path(&self) -> &String {
        &self.metrics_path
    }
}
