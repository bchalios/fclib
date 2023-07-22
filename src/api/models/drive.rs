#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use super::RateLimiter;

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum CacheType {
    #[default]
    Unsafe,
    WriteBack,
}

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum IoEngine {
    #[default]
    Sync,
    Async,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Drive {
    /// Id of the drive.
    #[serde(rename = "drive_id")]
    drive_id: String,

    /// Path to the drive in the host filesystem. If the path is not absolute it will be relative
    /// to the Firecracker process's current working directory.
    #[serde(rename = "path_on_host")]
    path_on_host: String,

    /// Type of the IO engine used by the device. "Async" is supported on host kernels newer than
    /// 5.10.51.
    /// Host level path for the guest drive
    #[cfg_attr(feature = "clap", arg(long, required = false, default_value = "sync"))]
    #[serde(rename = "io_engine")]
    io_engine: IoEngine,

    /// Represents the caching strategy for the block device.
    #[cfg_attr(
        feature = "clap",
        arg(long, short, required = false, default_value = "unsafe")
    )]
    #[serde(rename = "cache_type")]
    cache_type: CacheType,

    /// Determines whether the drive will be read-only or not.
    #[cfg_attr(feature = "clap", arg(long))]
    #[serde(rename = "is_read_only")]
    is_read_only: bool,

    /// Determines whether this drive contains the root file system of the microVM.
    #[cfg_attr(feature = "clap", arg(long))]
    #[serde(rename = "is_root_device")]
    is_root_device: bool,

    #[cfg_attr(feature = "clap", clap(skip))]
    #[serde(rename = "rate_limiter")]
    rate_limiter: Option<RateLimiter>,

    /// Represents the unique id of the boot partition of this device. It is optional and it will
    /// be taken into account only if the is_root_device field is true.
    #[cfg_attr(feature = "clap", arg(short, long))]
    #[serde(rename = "partuuid")]
    partuuid: Option<String>,
}

impl Drive {
    pub fn new(
        drive_id: String,
        is_read_only: bool,
        is_root_device: bool,
        path_on_host: String,
    ) -> Drive {
        Drive {
            drive_id,
            cache_type: CacheType::default(),
            is_read_only,
            is_root_device,
            partuuid: None,
            path_on_host,
            rate_limiter: None,
            io_engine: IoEngine::default(),
        }
    }

    pub fn set_drive_id(&mut self, drive_id: String) {
        self.drive_id = drive_id;
    }

    pub fn with_drive_id(mut self, drive_id: String) -> Drive {
        self.drive_id = drive_id;
        self
    }

    pub fn drive_id(&self) -> &String {
        &self.drive_id
    }

    pub fn set_cache_type(&mut self, cache_type: CacheType) {
        self.cache_type = cache_type;
    }

    pub fn with_cache_type(mut self, cache_type: CacheType) -> Drive {
        self.cache_type = cache_type;
        self
    }

    pub fn cache_type(&self) -> &CacheType {
        &self.cache_type
    }

    pub fn reset_cache_type(&mut self) {
        self.cache_type = CacheType::default();
    }

    pub fn set_is_read_only(&mut self, is_read_only: bool) {
        self.is_read_only = is_read_only;
    }

    pub fn with_is_read_only(mut self, is_read_only: bool) -> Drive {
        self.is_read_only = is_read_only;
        self
    }

    pub fn is_read_only(&self) -> &bool {
        &self.is_read_only
    }

    pub fn set_is_root_device(&mut self, is_root_device: bool) {
        self.is_root_device = is_root_device;
    }

    pub fn with_is_root_device(mut self, is_root_device: bool) -> Drive {
        self.is_root_device = is_root_device;
        self
    }

    pub fn is_root_device(&self) -> &bool {
        &self.is_root_device
    }

    pub fn set_partuuid(&mut self, partuuid: String) {
        self.partuuid = Some(partuuid);
    }

    pub fn with_partuuid(mut self, partuuid: String) -> Drive {
        self.partuuid = Some(partuuid);
        self
    }

    pub fn partuuid(&self) -> Option<&String> {
        self.partuuid.as_ref()
    }

    pub fn reset_partuuid(&mut self) {
        self.partuuid = None;
    }

    pub fn set_path_on_host(&mut self, path_on_host: String) {
        self.path_on_host = path_on_host;
    }

    pub fn with_path_on_host(mut self, path_on_host: String) -> Drive {
        self.path_on_host = path_on_host;
        self
    }

    pub fn path_on_host(&self) -> &String {
        &self.path_on_host
    }

    pub fn set_rate_limiter(&mut self, rate_limiter: RateLimiter) {
        self.rate_limiter = Some(rate_limiter);
    }

    pub fn with_rate_limiter(mut self, rate_limiter: RateLimiter) -> Drive {
        self.rate_limiter = Some(rate_limiter);
        self
    }

    pub fn rate_limiter(&self) -> Option<&RateLimiter> {
        self.rate_limiter.as_ref()
    }

    pub fn reset_rate_limiter(&mut self) {
        self.rate_limiter = None;
    }

    pub fn set_io_engine(&mut self, io_engine: IoEngine) {
        self.io_engine = io_engine;
    }

    pub fn with_io_engine(mut self, io_engine: IoEngine) -> Drive {
        self.io_engine = io_engine;
        self
    }

    pub fn io_engine(&self) -> &IoEngine {
        &self.io_engine
    }

    pub fn reset_io_engine(&mut self) {
        self.io_engine = IoEngine::default();
    }
}
