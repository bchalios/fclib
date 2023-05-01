// Firecracker API
//
// RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying
// JSON modeled data. The transport medium is a Unix Domain Socket.
//
// OpenAPI spec version: 1.2.0
// Contact: compute-capsule@amazon.com
// Generated by: https://github.com/swagger-api/swagger-codegen.git

/// Metrics : Describes the configuration option for the metrics capability.


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