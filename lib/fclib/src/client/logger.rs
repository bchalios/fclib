//! Logger : Describes the configuration option for the logging capability.

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logger {
    /// Set the level. The possible values are case-insensitive.
    pub level: Option<String>,
    /// Path to the named pipe or file for the human readable log output.
    pub log_path: String,
    /// Whether or not to output the level in the logs.
    pub show_level: bool,
    /// Whether or not to include the file path and line number of the log's origin.
    pub show_log_origin: bool,
}

impl Logger {
    /// Describes the configuration option for the logging capability.
    pub fn new(log_path: String) -> Logger {
        Logger {
            level: None,
            log_path,
            show_level: false,
            show_log_origin: false,
        }
    }
}

impl ApiClient {
    pub async fn config_logger(&self, logger: &Logger) -> Result<()> {
        self.put("/logger", logger).await
    }
}
