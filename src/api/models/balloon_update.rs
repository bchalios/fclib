//! BalloonUpdate : Balloon device descriptor.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonUpdate {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    amount_mib: i32,
}

impl BalloonUpdate {
    /// Balloon device descriptor.
    pub fn new(amount_mib: i32) -> BalloonUpdate {
        BalloonUpdate { amount_mib }
    }

    pub fn set_amount_mib(&mut self, amount_mib: i32) {
        self.amount_mib = amount_mib;
    }

    pub fn with_amount_mib(mut self, amount_mib: i32) -> BalloonUpdate {
        self.amount_mib = amount_mib;
        self
    }

    pub fn amount_mib(&self) -> &i32 {
        &self.amount_mib
    }
}
