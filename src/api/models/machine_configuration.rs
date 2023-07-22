//! MachineConfiguration : Describes the number of vCPUs, memory size, SMT capabilities and the
//! CPU template.

use crate::api::CpuTemplate;

#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineConfiguration {
    /// Memory size of VM
    #[serde(rename = "mem_size_mib")]
    mem_size_mib: i32,

    /// Number of vCPUs (either 1 or an even number)
    #[serde(rename = "vcpu_count")]
    vcpu_count: i32,

    /// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "false"))]
    #[serde(rename = "smt")]
    smt: Option<bool>,

    /// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots can
    /// be created. These belong to diff snapshots, which contain, besides the microVM state, only
    /// the memory dirtied since a previous snapshot. Full snapshots each contain a full copy of
    /// the guest memory.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "false"))]
    #[serde(rename = "track_dirty_pages")]
    track_dirty_pages: Option<bool>,

    /// CPU template to use.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "none"))]
    #[serde(rename = "cpu_template")]
    cpu_template: Option<CpuTemplate>,
}

impl MachineConfiguration {
    /// Describes the number of vCPUs, memory size, SMT capabilities and the CPU template.
    pub fn new(mem_size_mib: i32, vcpu_count: i32) -> MachineConfiguration {
        MachineConfiguration {
            cpu_template: None,
            smt: None,
            mem_size_mib,
            track_dirty_pages: None,
            vcpu_count,
        }
    }

    pub fn set_cpu_template(&mut self, cpu_template: CpuTemplate) {
        self.cpu_template = Some(cpu_template);
    }

    pub fn with_cpu_template(mut self, cpu_template: CpuTemplate) -> MachineConfiguration {
        self.cpu_template = Some(cpu_template);
        self
    }

    pub fn cpu_template(&self) -> Option<&CpuTemplate> {
        self.cpu_template.as_ref()
    }

    pub fn reset_cpu_template(&mut self) {
        self.cpu_template = None;
    }

    pub fn set_smt(&mut self, smt: bool) {
        self.smt = Some(smt);
    }

    pub fn with_smt(mut self, smt: bool) -> MachineConfiguration {
        self.smt = Some(smt);
        self
    }

    pub fn smt(&self) -> Option<&bool> {
        self.smt.as_ref()
    }

    pub fn reset_smt(&mut self) {
        self.smt = None;
    }

    pub fn set_mem_size_mib(&mut self, mem_size_mib: i32) {
        self.mem_size_mib = mem_size_mib;
    }

    pub fn with_mem_size_mib(mut self, mem_size_mib: i32) -> MachineConfiguration {
        self.mem_size_mib = mem_size_mib;
        self
    }

    pub fn mem_size_mib(&self) -> &i32 {
        &self.mem_size_mib
    }

    pub fn set_track_dirty_pages(&mut self, track_dirty_pages: bool) {
        self.track_dirty_pages = Some(track_dirty_pages);
    }

    pub fn with_track_dirty_pages(mut self, track_dirty_pages: bool) -> MachineConfiguration {
        self.track_dirty_pages = Some(track_dirty_pages);
        self
    }

    pub fn track_dirty_pages(&self) -> Option<&bool> {
        self.track_dirty_pages.as_ref()
    }

    pub fn reset_track_dirty_pages(&mut self) {
        self.track_dirty_pages = None;
    }

    pub fn set_vcpu_count(&mut self, vcpu_count: i32) {
        self.vcpu_count = vcpu_count;
    }

    pub fn with_vcpu_count(mut self, vcpu_count: i32) -> MachineConfiguration {
        self.vcpu_count = vcpu_count;
        self
    }

    pub fn vcpu_count(&self) -> &i32 {
        &self.vcpu_count
    }
}
