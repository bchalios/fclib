//! Configuration for virtio-block devices
//!
//! Firecracker uses virtio-block to emulate block devices for the guest. Currently, the devices
//! are backed by files in the host file system and can be rate limited for controlling the maximum
//! throughput in terms of bytes and operations per second allowed for each device.
//!
//! By default, it uses a synchronous IO engine for the files backing the drive on the host,
//! but it also has support for asynchronous IO for kernels that support it (>= 5.10.51), although
//! this feature is in dev-preview.
//!
//! Firecracker allows users to configure multiple drives per VM. It allows defining maximum one
//! root drive device (i.e. the device that holds the root filesystem). The devices need to be
//! configured before the microVM is booted, but parts of their configuration can be updated after
//! booting the microVM.

#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use super::rate_limiter::RateLimiter;
use super::{ApiClient, Result};

/// Caching strategy for a block device
#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum CacheType {
    #[default]
    Unsafe,
    WriteBack,
}

/// IO engine to use for the backing file on the host
#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum IoEngine {
    #[default]
    Sync,
    Async,
}

/// Configuration of a Firecracker drive
///
/// By default, the device will use [`Unsafe`] as caching strategy and the [`Sync`] IO engine.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Drive {
    /// Id of the drive.
    pub drive_id: String,

    /// Path to the drive in the host filesystem. If the path is not absolute it will be relative
    /// to the Firecracker process's current working directory.
    pub path_on_host: String,

    /// Type of the IO engine used by the device. "Async" is supported on host kernels newer than
    /// 5.10.51.
    /// Host level path for the guest drive
    #[cfg_attr(feature = "clap", arg(long, required = false, default_value = "sync"))]
    pub io_engine: IoEngine,

    /// Represents the caching strategy for the block device.
    #[cfg_attr(
        feature = "clap",
        arg(long, short, required = false, default_value = "unsafe")
    )]
    pub cache_type: CacheType,

    /// The drive is read-only.
    #[cfg_attr(feature = "clap", arg(long))]
    pub is_read_only: bool,

    /// The drive contains the root file system of the microVM.
    #[cfg_attr(feature = "clap", arg(long))]
    pub is_root_device: bool,

    #[cfg_attr(feature = "clap", clap(skip))]
    pub rate_limiter: Option<RateLimiter>,

    /// Represents the unique id of the boot partition of this device. It is optional and it will
    /// be taken into account only if the is_root_device field is true.
    #[cfg_attr(feature = "clap", arg(short, long))]
    pub partuuid: Option<String>,
}

impl Drive {
    /// Create a new [`Drive`]
    pub fn new(
        drive_id: String,
        path_on_host: String,
        is_root_device: bool,
        is_read_only: bool,
    ) -> Drive {
        Drive {
            drive_id,
            path_on_host,
            io_engine: IoEngine::default(),
            cache_type: CacheType::default(),
            is_read_only,
            is_root_device,
            rate_limiter: None,
            partuuid: None,
        }
    }
}

/// Helper for updating the configuration of a drive
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialDrive {
    /// Id of the drive to udate
    pub drive_id: String,

    /// Host level path for the file backing the guest drive
    #[cfg_attr(feature = "clap", arg(long, short))]
    pub path_on_host: Option<String>,

    /// Rate limiter configuration
    #[cfg_attr(feature = "clap", clap(skip))]
    pub rate_limiter: Option<RateLimiter>,
}

impl ApiClient {
    /// Add a new disk to the VM.
    ///
    /// This operation is only allowed before the VM has been booted.
    ///
    /// # Arguments
    ///
    /// * `drive_id` - The id of the new disk
    /// * `drive` - The [`Drive`] object to attach to the VM
    pub async fn add_drive(&mut self, drive_id: &str, drive: &Drive) -> Result<()> {
        self.put(&format!("/drives/{drive_id}"), &drive).await
    }

    /// Update a disk of the VM.
    ///
    /// # Arguments
    ///
    /// * `drive_id` - The id of the new disk
    /// * `drive` - The [`Drive`] object to attach to the VM
    pub async fn update_drive(&mut self, drive_id: &str, drive: &PartialDrive) -> Result<()> {
        self.patch(&format!("/drives/{drive_id}"), drive).await
    }
}
