//! BootSource : Boot source descriptor.
use serde_derive::{Deserialize, Serialize};

#[cfg(feature = "clap")]
use clap::Args;

#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct BootSource {
    /// Host level path to the kernel image used to boot the guest
    #[serde(rename = "kernel_image_path")]
    kernel_image_path: String,
    /// Kernel boot arguments
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "boot_args")]
    boot_args: Option<String>,
    /// Host level path to the initrd image used to boot the guest
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "initrd_path")]
    initrd_path: Option<String>,
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

    pub fn set_boot_args(&mut self, boot_args: String) {
        self.boot_args = Some(boot_args);
    }

    pub fn with_boot_args(mut self, boot_args: String) -> BootSource {
        self.boot_args = Some(boot_args);
        self
    }

    pub fn boot_args(&self) -> Option<&String> {
        self.boot_args.as_ref()
    }

    pub fn reset_boot_args(&mut self) {
        self.boot_args = None;
    }

    pub fn set_initrd_path(&mut self, initrd_path: String) {
        self.initrd_path = Some(initrd_path);
    }

    pub fn with_initrd_path(mut self, initrd_path: String) -> BootSource {
        self.initrd_path = Some(initrd_path);
        self
    }

    pub fn initrd_path(&self) -> Option<&String> {
        self.initrd_path.as_ref()
    }

    pub fn reset_initrd_path(&mut self) {
        self.initrd_path = None;
    }

    pub fn set_kernel_image_path(&mut self, kernel_image_path: String) {
        self.kernel_image_path = kernel_image_path;
    }

    pub fn with_kernel_image_path(mut self, kernel_image_path: String) -> BootSource {
        self.kernel_image_path = kernel_image_path;
        self
    }

    pub fn kernel_image_path(&self) -> &String {
        &self.kernel_image_path
    }
}
