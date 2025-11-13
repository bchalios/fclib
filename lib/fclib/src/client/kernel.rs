#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

/// [`BootSource`] includes information about the kernel file and, potentially, initrd used to boot
/// the microVM
#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct BootSource {
    /// Host level path to the kernel image used to boot the guest
    #[serde(rename = "kernel_image_path")]
    pub kernel_image_path: String,
    /// Kernel boot arguments
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "boot_args")]
    pub boot_args: Option<String>,
    /// Host level path to the initrd image used to boot the guest
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "initrd_path")]
    pub initrd_path: Option<String>,
}

impl BootSource {
    /// Boot source descriptor.
    pub fn new(kernel_image_path: String) -> BootSource {
        BootSource {
            boot_args: None,
            initrd_path: None,
            kernel_image_path,
        }
    }
}

impl ApiClient {
    /// Setup the boot source of the VM.
    pub async fn set_boot_source(&mut self, boot_source: &BootSource) -> Result<()> {
        self.put("/boot-source", boot_source).await
    }
}
