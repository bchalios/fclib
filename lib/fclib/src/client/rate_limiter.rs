use serde_derive::{Deserialize, Serialize};

/// TokenBucket : Defines a token bucket with a maximum capacity (size), an initial burst size
/// (one_time_burst) and an interval for refilling purposes (refill_time). The refill-rate is
/// derived from size and refill_time, and it is the constant rate at which the tokens
/// replenish. The refill process only starts happening after the initial burst budget is
/// consumed. Consumption from the token bucket is unbounded in speed which allows for bursts
/// bound in size by the amount of tokens available. Once the token bucket is empty,
/// consumption speed is bound by the refill_rate.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenBucket {
    /// The initial size of a token bucket.
    pub one_time_burst: Option<i64>,
    /// The amount of milliseconds it takes for the bucket to refill.
    pub refill_time: i64,
    /// The total number of tokens this bucket can hold.
    pub size: i64,
}

impl TokenBucket {
    /// Create a new [`TokenBucket`] without an one-time burst allowance
    pub fn new(size: i64, refill_time: i64) -> TokenBucket {
        TokenBucket {
            refill_time,
            size,
            one_time_burst: None,
        }
    }
}

/// RateLimiter : Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits
/// are defined by configuring each of the _bandwidth_ and _ops_ token buckets.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct RateLimiter {
    /// Token bucket with bytes as tokens
    pub bandwidth: Option<TokenBucket>,
    /// Token bucket with operations as tokens
    pub ops: Option<TokenBucket>,
}
