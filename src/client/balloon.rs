//! Configuration for virtio-balloon device
//!
//! virtio-balloon is a device that allows the host OS to reclaim memory
//! from VMs. It does so by "inflating" the balloon inside the VM, i.e.
//! claim guest physical pages, and communicating to the host which pages
//! it can reclaim from the VM. It allows also giving back that memory to
//! the VM ("deflating").
//!
//! Firecracker allows configuring (optionally) a single balloon device

use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

/// Configuration of a Firecracker Balloon device
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

/// Statistics that the balloon device can report to host
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStats {
    /// Target number of pages the device aims to hold.
    #[serde(rename = "target_pages")]
    target_pages: i32,
    /// Actual number of pages the device is holding.
    #[serde(rename = "actual_pages")]
    actual_pages: i32,
    /// Target amount of memory (in MiB) the device aims to hold.
    #[serde(rename = "target_mib")]
    target_mib: i32,
    /// Actual amount of memory (in MiB) the device is holding.
    #[serde(rename = "actual_mib")]
    actual_mib: i32,
    /// The amount of memory that has been swapped in (in bytes).
    #[serde(rename = "swap_in")]
    swap_in: Option<i64>,
    /// The amount of memory that has been swapped out to disk (in bytes).
    #[serde(rename = "swap_out")]
    swap_out: Option<i64>,
    /// The number of major page faults that have occurred.
    #[serde(rename = "major_faults")]
    major_faults: Option<i64>,
    /// The number of minor page faults that have occurred.
    #[serde(rename = "minor_faults")]
    minor_faults: Option<i64>,
    /// The amount of memory not being used for any purpose (in bytes).
    #[serde(rename = "free_memory")]
    free_memory: Option<i64>,
    /// The total amount of memory available (in bytes).
    #[serde(rename = "total_memory")]
    total_memory: Option<i64>,
    /// An estimate of how much memory is available (in bytes) for starting new applications,
    /// without pushing the system to swap.
    #[serde(rename = "available_memory")]
    available_memory: Option<i64>,
    /// The amount of memory, in bytes, that can be quickly reclaimed without additional I/O.
    /// Typically these pages are used for caching files from disk.
    #[serde(rename = "disk_caches")]
    disk_caches: Option<i64>,
    /// The number of successful hugetlb page allocations in the guest.
    #[serde(rename = "hugetlb_allocations")]
    hugetlb_allocations: Option<i64>,
    /// The number of failed hugetlb page allocations in the guest.
    #[serde(rename = "hugetlb_failures")]
    hugetlb_failures: Option<i64>,
}

impl BalloonStats {
    pub fn target_pages(&self) -> i32 {
        self.target_pages
    }

    pub fn actual_pages(&self) -> i32 {
        self.actual_pages
    }

    pub fn target_mib(&self) -> i32 {
        self.target_mib
    }

    pub fn actual_mib(&self) -> i32 {
        self.actual_mib
    }

    pub fn swap_in(&self) -> Option<i64> {
        self.swap_in
    }

    pub fn swap_out(&self) -> Option<i64> {
        self.swap_out
    }

    pub fn major_faults(&self) -> Option<i64> {
        self.major_faults
    }

    pub fn minor_faults(&self) -> Option<i64> {
        self.minor_faults
    }

    pub fn free_memory(&self) -> Option<i64> {
        self.free_memory
    }

    pub fn total_memory(&self) -> Option<i64> {
        self.total_memory
    }

    pub fn available_memory(&self) -> Option<i64> {
        self.available_memory
    }

    pub fn disk_caches(&self) -> Option<i64> {
        self.disk_caches
    }

    pub fn hugetlb_allocations(&self) -> Option<i64> {
        self.hugetlb_allocations
    }

    pub fn hugetlb_failures(&self) -> Option<i64> {
        self.hugetlb_failures
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct BalloonStatsUpdate {
    /// Interval in seconds between refreshing statistics.
    #[serde(rename = "stats_polling_interval_s")]
    stats_polling_interval_s: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct BalloonUpdate {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    amount_mib: i32,
}

impl ApiClient {
    /// Get the currently applied balloon configuration
    pub async fn balloon_config(&self) -> Result<Balloon> {
        self.get("/balloon").await
    }

    /// Configure the Firecracker balloon device
    pub async fn configure_balloon(&self, balloon: &Balloon) -> Result<()> {
        self.put("/balloon", balloon).await
    }

    /// Update the maximum size of the balloon device
    pub async fn update_balloon_size(&self, size: i32) -> Result<()> {
        let balloon_update = BalloonUpdate { amount_mib: size };
        self.patch("/balloon", balloon_update).await
    }

    /// Update the time interval that the device will emit statistics
    pub async fn update_balloon_stats_interval(&self, interval: i32) -> Result<()> {
        let balloon_stats_update = BalloonStatsUpdate {
            stats_polling_interval_s: interval,
        };
        self.patch("/balloon/statistics", balloon_stats_update)
            .await
    }

    /// Get the balloon device statistics
    pub async fn balloon_stats(&self) -> Result<BalloonStats> {
        self.get("/ballon/statistics").await
    }
}
