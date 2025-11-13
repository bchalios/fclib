//! Configuration for virtio-balloon device
//!
//! virtio-balloon is a device that allows the host OS to reclaim memory
//! from VMs. It does so by "inflating" the balloon inside the VM, i.e.
//! claim guest physical pages, and communicating to the host which pages
//! it can reclaim from the VM. It allows also giving back that memory to
//! the VM ("deflating").
//!
//! Firecracker allows configuring (optionally) a single balloon device

#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

/// Configuration of a Firecracker Balloon device
#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Balloon {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    pub amount_mib: i32,
    /// Whether the balloon should deflate when the guest has memory pressure.
    #[serde(rename = "deflate_on_oom")]
    pub deflate_on_oom: bool,
    /// Interval in seconds between refreshing statistics. A non-zero value will enable the
    /// statistics. Defaults to 0.
    #[serde(rename = "stats_polling_interval_s")]
    pub stats_polling_interval_s: Option<i32>,
}

/// Statistics that the balloon device can report to host
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStats {
    /// Target number of pages the device aims to hold.
    #[serde(rename = "target_pages")]
    pub target_pages: i32,
    /// Actual number of pages the device is holding.
    #[serde(rename = "actual_pages")]
    pub actual_pages: i32,
    /// Target amount of memory (in MiB) the device aims to hold.
    #[serde(rename = "target_mib")]
    pub target_mib: i32,
    /// Actual amount of memory (in MiB) the device is holding.
    #[serde(rename = "actual_mib")]
    pub actual_mib: i32,
    /// The amount of memory that has been swapped in (in bytes).
    #[serde(rename = "swap_in")]
    pub swap_in: Option<i64>,
    /// The amount of memory that has been swapped out to disk (in bytes).
    #[serde(rename = "swap_out")]
    pub swap_out: Option<i64>,
    /// The number of major page faults that have occurred.
    #[serde(rename = "major_faults")]
    pub major_faults: Option<i64>,
    /// The number of minor page faults that have occurred.
    #[serde(rename = "minor_faults")]
    pub minor_faults: Option<i64>,
    /// The amount of memory not being used for any purpose (in bytes).
    #[serde(rename = "free_memory")]
    pub free_memory: Option<i64>,
    /// The total amount of memory available (in bytes).
    #[serde(rename = "total_memory")]
    pub total_memory: Option<i64>,
    /// An estimate of how much memory is available (in bytes) for starting new applications,
    /// without pushing the system to swap.
    #[serde(rename = "available_memory")]
    pub available_memory: Option<i64>,
    /// The amount of memory, in bytes, that can be quickly reclaimed without additional I/O.
    /// Typically these pages are used for caching files from disk.
    #[serde(rename = "disk_caches")]
    pub disk_caches: Option<i64>,
    /// The number of successful hugetlb page allocations in the guest.
    #[serde(rename = "hugetlb_allocations")]
    pub hugetlb_allocations: Option<i64>,
    /// The number of failed hugetlb page allocations in the guest.
    #[serde(rename = "hugetlb_failures")]
    pub hugetlb_failures: Option<i64>,
}

#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonStatsUpdate {
    /// Interval in seconds between refreshing statistics.
    #[serde(rename = "stats_polling_interval_s")]
    pub stats_polling_interval_s: i32,
}

#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct BalloonUpdate {
    /// Target balloon size in MiB.
    #[serde(rename = "amount_mib")]
    pub amount_mib: i32,
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
