//! TokenBucket : Defines a token bucket with a maximum capacity (size), an initial burst size
//! (one_time_burst) and an interval for refilling purposes (refill_time). The refill-rate is
//! derived from size and refill_time, and it is the constant rate at which the tokens
//! replenish. The refill process only starts happening after the initial burst budget is
//! consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
//! bound in size by the amount of tokens available. Once the token bucket is empty,
//! consumption speed is bound by the refill_rate.
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenBucket {
    /// The initial size of a token bucket.
    #[serde(rename = "one_time_burst")]
    one_time_burst: Option<i64>,
    /// The amount of milliseconds it takes for the bucket to refill.
    #[serde(rename = "refill_time")]
    refill_time: i64,
    /// The total number of tokens this bucket can hold.
    #[serde(rename = "size")]
    size: i64,
}

impl TokenBucket {
    /// Defines a token bucket with a maximum capacity (size), an initial burst size
    /// (one_time_burst) and an interval for refilling purposes (refill_time). The refill-rate is
    /// derived from size and refill_time, and it is the constant rate at which the tokens
    /// replenish. The refill process only starts happening after the initial burst budget is
    /// consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
    /// bound in size by the amount of tokens available. Once the token bucket is empty, consumption
    /// speed is bound by the refill_rate.
    pub fn new(refill_time: i64, size: i64) -> TokenBucket {
        TokenBucket {
            one_time_burst: None,
            refill_time,
            size,
        }
    }

    pub fn set_one_time_burst(&mut self, one_time_burst: i64) {
        self.one_time_burst = Some(one_time_burst);
    }

    pub fn with_one_time_burst(mut self, one_time_burst: i64) -> TokenBucket {
        self.one_time_burst = Some(one_time_burst);
        self
    }

    pub fn one_time_burst(&self) -> Option<&i64> {
        self.one_time_burst.as_ref()
    }

    pub fn reset_one_time_burst(&mut self) {
        self.one_time_burst = None;
    }

    pub fn set_refill_time(&mut self, refill_time: i64) {
        self.refill_time = refill_time;
    }

    pub fn with_refill_time(mut self, refill_time: i64) -> TokenBucket {
        self.refill_time = refill_time;
        self
    }

    pub fn refill_time(&self) -> &i64 {
        &self.refill_time
    }

    pub fn set_size(&mut self, size: i64) {
        self.size = size;
    }

    pub fn with_size(mut self, size: i64) -> TokenBucket {
        self.size = size;
        self
    }

    pub fn size(&self) -> &i64 {
        &self.size
    }
}
