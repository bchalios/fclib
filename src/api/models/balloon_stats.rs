//! BalloonStats : Describes the balloon device statistics.

use serde_derive::{Deserialize, Serialize};

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
    /// Describes the balloon device statistics.
    pub fn new(
        target_pages: i32,
        actual_pages: i32,
        target_mib: i32,
        actual_mib: i32,
    ) -> BalloonStats {
        BalloonStats {
            target_pages,
            actual_pages,
            target_mib,
            actual_mib,
            swap_in: None,
            swap_out: None,
            major_faults: None,
            minor_faults: None,
            free_memory: None,
            total_memory: None,
            available_memory: None,
            disk_caches: None,
            hugetlb_allocations: None,
            hugetlb_failures: None,
        }
    }

    pub fn set_target_pages(&mut self, target_pages: i32) {
        self.target_pages = target_pages;
    }

    pub fn with_target_pages(mut self, target_pages: i32) -> BalloonStats {
        self.target_pages = target_pages;
        self
    }

    pub fn target_pages(&self) -> &i32 {
        &self.target_pages
    }

    pub fn set_actual_pages(&mut self, actual_pages: i32) {
        self.actual_pages = actual_pages;
    }

    pub fn with_actual_pages(mut self, actual_pages: i32) -> BalloonStats {
        self.actual_pages = actual_pages;
        self
    }

    pub fn actual_pages(&self) -> &i32 {
        &self.actual_pages
    }

    pub fn set_target_mib(&mut self, target_mib: i32) {
        self.target_mib = target_mib;
    }

    pub fn with_target_mib(mut self, target_mib: i32) -> BalloonStats {
        self.target_mib = target_mib;
        self
    }

    pub fn target_mib(&self) -> &i32 {
        &self.target_mib
    }

    pub fn set_actual_mib(&mut self, actual_mib: i32) {
        self.actual_mib = actual_mib;
    }

    pub fn with_actual_mib(mut self, actual_mib: i32) -> BalloonStats {
        self.actual_mib = actual_mib;
        self
    }

    pub fn actual_mib(&self) -> &i32 {
        &self.actual_mib
    }

    pub fn set_swap_in(&mut self, swap_in: i64) {
        self.swap_in = Some(swap_in);
    }

    pub fn with_swap_in(mut self, swap_in: i64) -> BalloonStats {
        self.swap_in = Some(swap_in);
        self
    }

    pub fn swap_in(&self) -> Option<&i64> {
        self.swap_in.as_ref()
    }

    pub fn reset_swap_in(&mut self) {
        self.swap_in = None;
    }

    pub fn set_swap_out(&mut self, swap_out: i64) {
        self.swap_out = Some(swap_out);
    }

    pub fn with_swap_out(mut self, swap_out: i64) -> BalloonStats {
        self.swap_out = Some(swap_out);
        self
    }

    pub fn swap_out(&self) -> Option<&i64> {
        self.swap_out.as_ref()
    }

    pub fn reset_swap_out(&mut self) {
        self.swap_out = None;
    }

    pub fn set_major_faults(&mut self, major_faults: i64) {
        self.major_faults = Some(major_faults);
    }

    pub fn with_major_faults(mut self, major_faults: i64) -> BalloonStats {
        self.major_faults = Some(major_faults);
        self
    }

    pub fn major_faults(&self) -> Option<&i64> {
        self.major_faults.as_ref()
    }

    pub fn reset_major_faults(&mut self) {
        self.major_faults = None;
    }

    pub fn set_minor_faults(&mut self, minor_faults: i64) {
        self.minor_faults = Some(minor_faults);
    }

    pub fn with_minor_faults(mut self, minor_faults: i64) -> BalloonStats {
        self.minor_faults = Some(minor_faults);
        self
    }

    pub fn minor_faults(&self) -> Option<&i64> {
        self.minor_faults.as_ref()
    }

    pub fn reset_minor_faults(&mut self) {
        self.minor_faults = None;
    }

    pub fn set_free_memory(&mut self, free_memory: i64) {
        self.free_memory = Some(free_memory);
    }

    pub fn with_free_memory(mut self, free_memory: i64) -> BalloonStats {
        self.free_memory = Some(free_memory);
        self
    }

    pub fn free_memory(&self) -> Option<&i64> {
        self.free_memory.as_ref()
    }

    pub fn reset_free_memory(&mut self) {
        self.free_memory = None;
    }

    pub fn set_total_memory(&mut self, total_memory: i64) {
        self.total_memory = Some(total_memory);
    }

    pub fn with_total_memory(mut self, total_memory: i64) -> BalloonStats {
        self.total_memory = Some(total_memory);
        self
    }

    pub fn total_memory(&self) -> Option<&i64> {
        self.total_memory.as_ref()
    }

    pub fn reset_total_memory(&mut self) {
        self.total_memory = None;
    }

    pub fn set_available_memory(&mut self, available_memory: i64) {
        self.available_memory = Some(available_memory);
    }

    pub fn with_available_memory(mut self, available_memory: i64) -> BalloonStats {
        self.available_memory = Some(available_memory);
        self
    }

    pub fn available_memory(&self) -> Option<&i64> {
        self.available_memory.as_ref()
    }

    pub fn reset_available_memory(&mut self) {
        self.available_memory = None;
    }

    pub fn set_disk_caches(&mut self, disk_caches: i64) {
        self.disk_caches = Some(disk_caches);
    }

    pub fn with_disk_caches(mut self, disk_caches: i64) -> BalloonStats {
        self.disk_caches = Some(disk_caches);
        self
    }

    pub fn disk_caches(&self) -> Option<&i64> {
        self.disk_caches.as_ref()
    }

    pub fn reset_disk_caches(&mut self) {
        self.disk_caches = None;
    }

    pub fn set_hugetlb_allocations(&mut self, hugetlb_allocations: i64) {
        self.hugetlb_allocations = Some(hugetlb_allocations);
    }

    pub fn with_hugetlb_allocations(mut self, hugetlb_allocations: i64) -> BalloonStats {
        self.hugetlb_allocations = Some(hugetlb_allocations);
        self
    }

    pub fn hugetlb_allocations(&self) -> Option<&i64> {
        self.hugetlb_allocations.as_ref()
    }

    pub fn reset_hugetlb_allocations(&mut self) {
        self.hugetlb_allocations = None;
    }

    pub fn set_hugetlb_failures(&mut self, hugetlb_failures: i64) {
        self.hugetlb_failures = Some(hugetlb_failures);
    }

    pub fn with_hugetlb_failures(mut self, hugetlb_failures: i64) -> BalloonStats {
        self.hugetlb_failures = Some(hugetlb_failures);
        self
    }

    pub fn hugetlb_failures(&self) -> Option<&i64> {
        self.hugetlb_failures.as_ref()
    }

    pub fn reset_hugetlb_failures(&mut self) {
        self.hugetlb_failures = None;
    }
}
