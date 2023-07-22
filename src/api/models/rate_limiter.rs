//! RateLimiter : Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits
//! are defined by configuring each of the _bandwidth_ and _ops_ token buckets.
use crate::api::TokenBucket;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RateLimiter {
    /// Token bucket with bytes as tokens
    #[serde(rename = "bandwidth")]
    bandwidth: Option<TokenBucket>,
    /// Token bucket with operations as tokens
    #[serde(rename = "ops")]
    ops: Option<TokenBucket>,
}

impl RateLimiter {
    /// Defines an IO rate limiter with independent bytes/s and ops/s limits. Limits are defined by
    /// configuring each of the _bandwidth_ and _ops_ token buckets.
    pub fn new() -> RateLimiter {
        RateLimiter {
            bandwidth: None,
            ops: None,
        }
    }

    pub fn set_bandwidth(&mut self, bandwidth: TokenBucket) {
        self.bandwidth = Some(bandwidth);
    }

    pub fn with_bandwidth(mut self, bandwidth: TokenBucket) -> RateLimiter {
        self.bandwidth = Some(bandwidth);
        self
    }

    pub fn bandwidth(&self) -> Option<&TokenBucket> {
        self.bandwidth.as_ref()
    }

    pub fn reset_bandwidth(&mut self) {
        self.bandwidth = None;
    }

    pub fn set_ops(&mut self, ops: TokenBucket) {
        self.ops = Some(ops);
    }

    pub fn with_ops(mut self, ops: TokenBucket) -> RateLimiter {
        self.ops = Some(ops);
        self
    }

    pub fn ops(&self) -> Option<&TokenBucket> {
        self.ops.as_ref()
    }

    pub fn reset_ops(&mut self) {
        self.ops = None;
    }
}
