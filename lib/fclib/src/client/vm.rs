#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use super::balloon::Balloon;
use super::drive::Drive;
use super::kernel::BootSource;
use super::logger::Logger;
use super::metrics::Metrics;
use super::mmds::MmdsConfig;
use super::network::NetworkInterface;
use super::vsock::Vsock;
use super::{ApiClient, Result};

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Serialize, Deserialize)]
enum InstanceAction {
    /// Flush VM metrics
    FlushMetrics,
    /// Start uVM
    InstanceStart,
    /// Send Ctrl+Alt+Del to VM (x86_64 only)
    SendCtrlAltDel,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
struct InstanceActionInfo {
    /// Enumeration indicating what type of action is contained in the payload
    #[serde(rename = "action_type")]
    action_type: InstanceAction,
}

impl InstanceActionInfo {
    fn new(action_type: InstanceAction) -> Self {
        Self { action_type }
    }
}

impl ApiClient {
    async fn do_action(&self, action: InstanceAction) -> Result<()> {
        let action = InstanceActionInfo::new(action);
        self.put("/actions", action).await
    }

    /// Start the microVM.
    pub async fn start_microvm(&self) -> Result<()> {
        self.do_action(InstanceAction::InstanceStart).await
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    /// Stop the microVM.
    pub async fn stop_microvm(&self) -> Result<()> {
        self.do_action(InstanceAction::SendCtrlAltDel).await
    }

    /// Flush microVM's metrics.
    pub async fn flush_metrics(&self) -> Result<()> {
        self.do_action(InstanceAction::FlushMetrics).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum VmState {
    Resumed,
    Paused,
}

#[derive(Debug, Serialize, Deserialize)]
struct Vm {
    #[serde(rename = "state")]
    state: VmState,
}

impl Vm {
    fn new(state: VmState) -> Self {
        Self { state }
    }
}

impl ApiClient {
    /// Pause microVM.
    pub async fn pause_microvm(&self) -> Result<()> {
        let vm_state = Vm::new(VmState::Paused);
        self.patch("/vm", vm_state).await
    }

    /// Resume microVM.
    pub async fn resume_microvm(&self) -> Result<()> {
        let vm_state = Vm::new(VmState::Resumed);
        self.patch("/vm", vm_state).await
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceInfo {
    /// Application name.
    #[serde(rename = "app_name")]
    pub app_name: String,
    /// MicroVM / instance ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The current detailed state (Not started, Running, Paused) of the Firecracker instance. This
    /// value is read-only for the control-plane.
    #[serde(rename = "state")]
    pub state: String,
    /// MicroVM hypervisor build version.
    #[serde(rename = "vmm_version")]
    pub vmm_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Firecracker build version.
    #[serde(rename = "firecracker_version")]
    pub firecracker_version: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(rename = "balloon")]
    pub balloon: Option<Balloon>,
    /// Configurations for all block devices.
    #[serde(rename = "drives")]
    pub drives: Option<Vec<Drive>>,
    #[serde(rename = "boot-source")]
    pub boot_source: Option<BootSource>,
    #[serde(rename = "logger")]
    pub logger: Option<Logger>,
    #[serde(rename = "machine-config")]
    pub machine_config: Option<MachineConfiguration>,
    #[serde(rename = "metrics")]
    pub metrics: Option<Metrics>,
    #[serde(rename = "mmds-config")]
    pub mmds_config: Option<MmdsConfig>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "vsock")]
    pub vsock: Option<Vsock>,
}

impl ApiClient {
    /// Get information about the microVM instance.
    pub async fn instance_info(&self) -> Result<InstanceInfo> {
        self.get("/").await
    }

    /// Get a full JSON object with the microVM's configuration.
    pub async fn vm_config(&self) -> Result<FullVmConfiguration> {
        self.get("/vm/config").await
    }

    /// Get the version of Firecracker used to launch this microVM.
    pub async fn firecracker_version(&self) -> Result<FirecrackerVersion> {
        self.get("/version").await
    }
}

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Serialize, Deserialize, Default)]
pub enum CpuTemplate {
    C3,
    T2,
    T2S,
    T2CL,
    T2A,
    #[default]
    None,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct MachineConfiguration {
    /// Memory size of VM
    #[serde(rename = "mem_size_mib")]
    pub mem_size_mib: i32,

    /// Number of vCPUs (either 1 or an even number)
    #[serde(rename = "vcpu_count")]
    pub vcpu_count: i32,

    /// Flag for enabling/disabling simultaneous multithreading. Can be enabled only on x86.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "false"))]
    #[serde(rename = "smt")]
    pub smt: Option<bool>,

    /// Enable dirty page tracking. If this is enabled, then incremental guest memory snapshots can
    /// be created. These belong to diff snapshots, which contain, besides the microVM state, only
    /// the memory dirtied since a previous snapshot. Full snapshots each contain a full copy of
    /// the guest memory.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "false"))]
    #[serde(rename = "track_dirty_pages")]
    pub track_dirty_pages: Option<bool>,

    /// CPU template to use.
    #[cfg_attr(feature = "clap", arg(long, short, default_value = "none"))]
    #[serde(rename = "cpu_template")]
    pub cpu_template: Option<CpuTemplate>,
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
}

impl ApiClient {
    /// Get the configuration of the VM.
    pub async fn get_machine_configuration(&self) -> Result<MachineConfiguration> {
        self.get("/machine-config").await
    }

    /// Update the configuration of the VM.
    pub async fn update_machine_configuration(&self, body: &MachineConfiguration) -> Result<()> {
        self.patch("/machine-config", body).await
    }

    /// Configure the VM.
    pub async fn configure_machine(&self, body: &MachineConfiguration) -> Result<()> {
        self.put("/machine-config", body).await
    }
}
