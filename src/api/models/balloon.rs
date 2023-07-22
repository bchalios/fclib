//! Balloon : Balloon device descriptor.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    amount_mib: i32,
    /// Whether the balloon should deflate when the guest has memory pressure.
    #[serde(rename = "deflate_on_oom")]
    deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the
    /// statistics. Defaults to 0.
    #[serde(rename = "stats_polling_interval_s")]
    stats_polling_interval_s: Option<i32>,
}

impl Balloon {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32, deflate_on_oom: bool) -> Balloon {
        Balloon {
            amount_mib,
            deflate_on_oom,
            stats_polling_interval_s: None,
        }
    }

    pub fn set_amount_mib(&mut self, amount_mib: i32) {
        self.amount_mib = amount_mib;
    }

    pub fn with_amount_mib(mut self, amount_mib: i32) -> Balloon {
        self.amount_mib = amount_mib;
        self
    }

    pub fn amount_mib(&self) -> &i32 {
        &self.amount_mib
    }

    pub fn set_deflate_on_oom(&mut self, deflate_on_oom: bool) {
        self.deflate_on_oom = deflate_on_oom;
    }

    pub fn with_deflate_on_oom(mut self, deflate_on_oom: bool) -> Balloon {
        self.deflate_on_oom = deflate_on_oom;
        self
    }

    pub fn deflate_on_oom(&self) -> &bool {
        &self.deflate_on_oom
    }

    pub fn set_stats_polling_interval_s(&mut self, stats_polling_interval_s: i32) {
        self.stats_polling_interval_s = Some(stats_polling_interval_s);
    }

    pub fn with_stats_polling_interval_s(mut self, stats_polling_interval_s: i32) -> Balloon {
        self.stats_polling_interval_s = Some(stats_polling_interval_s);
        self
    }

    pub fn stats_polling_interval_s(&self) -> Option<&i32> {
        self.stats_polling_interval_s.as_ref()
    }

    pub fn reset_stats_polling_interval_s(&mut self) {
        self.stats_polling_interval_s = None;
    }
}
