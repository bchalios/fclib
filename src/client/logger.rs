//! Logger : Describes the configuration option for the logging capability.

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Logger {
    /// Set the level. The possible values are case-insensitive.
    #[serde(rename = "level")]
    level: Option<String>,
    /// Path to the named pipe or file for the human readable log output.
    #[serde(rename = "log_path")]
    log_path: String,
    /// Whether or not to output the level in the logs.
    #[serde(rename = "show_level")]
    show_level: Option<bool>,
    /// Whether or not to include the file path and line number of the log's origin.
    #[serde(rename = "show_log_origin")]
    show_log_origin: Option<bool>,
}

impl Logger {
    /// Describes the configuration option for the logging capability.
    pub fn new(log_path: String) -> Logger {
        Logger {
            level: None,
            log_path,
            show_level: None,
            show_log_origin: None,
        }
    }

    pub fn set_level(&mut self, level: String) {
        self.level = Some(level);
    }

    pub fn with_level(mut self, level: String) -> Logger {
        self.level = Some(level);
        self
    }

    pub fn level(&self) -> Option<&String> {
        self.level.as_ref()
    }

    pub fn reset_level(&mut self) {
        self.level = None;
    }

    pub fn set_log_path(&mut self, log_path: String) {
        self.log_path = log_path;
    }

    pub fn with_log_path(mut self, log_path: String) -> Logger {
        self.log_path = log_path;
        self
    }

    pub fn log_path(&self) -> &String {
        &self.log_path
    }

    pub fn set_show_level(&mut self, show_level: bool) {
        self.show_level = Some(show_level);
    }

    pub fn with_show_level(mut self, show_level: bool) -> Logger {
        self.show_level = Some(show_level);
        self
    }

    pub fn show_level(&self) -> Option<&bool> {
        self.show_level.as_ref()
    }

    pub fn reset_show_level(&mut self) {
        self.show_level = None;
    }

    pub fn set_show_log_origin(&mut self, show_log_origin: bool) {
        self.show_log_origin = Some(show_log_origin);
    }

    pub fn with_show_log_origin(mut self, show_log_origin: bool) -> Logger {
        self.show_log_origin = Some(show_log_origin);
        self
    }

    pub fn show_log_origin(&self) -> Option<&bool> {
        self.show_log_origin.as_ref()
    }

    pub fn reset_show_log_origin(&mut self) {
        self.show_log_origin = None;
    }
}

impl ApiClient {
    pub async fn config_logger(&self, logger: &Logger) -> Result<()> {
        self.put("/logger", logger).await
    }
}
