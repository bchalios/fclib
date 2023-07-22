//! BalloonStatsUpdate : Update the statistics polling interval, with the first statistics
//! update scheduled immediately. Statistics cannot be turned on/off after boot.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStatsUpdate {
    /// Interval in seconds between refreshing statistics.
    #[serde(rename = "stats_polling_interval_s")]
    stats_polling_interval_s: i32,
}

impl BalloonStatsUpdate {
    /// Update the statistics polling interval, with the first statistics update scheduled
    /// immediately. Statistics cannot be turned on/off after boot.
    pub fn new(stats_polling_interval_s: i32) -> BalloonStatsUpdate {
        BalloonStatsUpdate {
            stats_polling_interval_s,
        }
    }

    pub fn set_stats_polling_interval_s(&mut self, stats_polling_interval_s: i32) {
        self.stats_polling_interval_s = stats_polling_interval_s;
    }

    pub fn with_stats_polling_interval_s(
        mut self,
        stats_polling_interval_s: i32,
    ) -> BalloonStatsUpdate {
        self.stats_polling_interval_s = stats_polling_interval_s;
        self
    }

    pub fn stats_polling_interval_s(&self) -> &i32 {
        &self.stats_polling_interval_s
    }
}
