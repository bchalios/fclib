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
    app_name: String,
    /// MicroVM / instance ID.
    #[serde(rename = "id")]
    id: String,
    /// The current detailed state (Not started, Running, Paused) of the Firecracker instance. This
    /// value is read-only for the control-plane.
    #[serde(rename = "state")]
    state: String,
    /// MicroVM hypervisor build version.
    #[serde(rename = "vmm_version")]
    vmm_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FirecrackerVersion {
    /// Firecracker build version.
    #[serde(rename = "firecracker_version")]
    firecracker_version: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(rename = "balloon")]
    balloon: Option<Balloon>,
    /// Configurations for all block devices.
    #[serde(rename = "drives")]
    drives: Option<Vec<Drive>>,
    #[serde(rename = "boot-source")]
    boot_source: Option<BootSource>,
    #[serde(rename = "logger")]
    logger: Option<Logger>,
    #[serde(rename = "machine-config")]
    machine_config: Option<MachineConfiguration>,
    #[serde(rename = "metrics")]
    metrics: Option<Metrics>,
    #[serde(rename = "mmds-config")]
    mmds_config: Option<MmdsConfig>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces")]
    network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(rename = "vsock")]
    vsock: Option<Vsock>,
}

impl FullVmConfiguration {
    pub fn new() -> FullVmConfiguration {
        FullVmConfiguration {
            balloon: None,
            drives: None,
            boot_source: None,
            logger: None,
            machine_config: None,
            metrics: None,
            mmds_config: None,
            network_interfaces: None,
            vsock: None,
        }
    }

    pub fn set_balloon(&mut self, balloon: Balloon) {
        self.balloon = Some(balloon);
    }

    pub fn with_balloon(mut self, balloon: Balloon) -> FullVmConfiguration {
        self.balloon = Some(balloon);
        self
    }

    pub fn balloon(&self) -> Option<&Balloon> {
        self.balloon.as_ref()
    }

    pub fn reset_balloon(&mut self) {
        self.balloon = None;
    }

    pub fn set_drives(&mut self, drives: Vec<Drive>) {
        self.drives = Some(drives);
    }

    pub fn with_drives(mut self, drives: Vec<Drive>) -> FullVmConfiguration {
        self.drives = Some(drives);
        self
    }

    pub fn drives(&self) -> Option<&Vec<Drive>> {
        self.drives.as_ref()
    }

    pub fn reset_drives(&mut self) {
        self.drives = None;
    }

    pub fn set_boot_source(&mut self, boot_source: BootSource) {
        self.boot_source = Some(boot_source);
    }

    pub fn with_boot_source(mut self, boot_source: BootSource) -> FullVmConfiguration {
        self.boot_source = Some(boot_source);
        self
    }

    pub fn boot_source(&self) -> Option<&BootSource> {
        self.boot_source.as_ref()
    }

    pub fn reset_boot_source(&mut self) {
        self.boot_source = None;
    }

    pub fn set_logger(&mut self, logger: Logger) {
        self.logger = Some(logger);
    }

    pub fn with_logger(mut self, logger: Logger) -> FullVmConfiguration {
        self.logger = Some(logger);
        self
    }

    pub fn logger(&self) -> Option<&Logger> {
        self.logger.as_ref()
    }

    pub fn reset_logger(&mut self) {
        self.logger = None;
    }

    pub fn set_machine_config(&mut self, machine_config: MachineConfiguration) {
        self.machine_config = Some(machine_config);
    }

    pub fn with_machine_config(
        mut self,
        machine_config: MachineConfiguration,
    ) -> FullVmConfiguration {
        self.machine_config = Some(machine_config);
        self
    }

    pub fn machine_config(&self) -> Option<&MachineConfiguration> {
        self.machine_config.as_ref()
    }

    pub fn reset_machine_config(&mut self) {
        self.machine_config = None;
    }

    pub fn set_metrics(&mut self, metrics: Metrics) {
        self.metrics = Some(metrics);
    }

    pub fn with_metrics(mut self, metrics: Metrics) -> FullVmConfiguration {
        self.metrics = Some(metrics);
        self
    }

    pub fn metrics(&self) -> Option<&Metrics> {
        self.metrics.as_ref()
    }

    pub fn reset_metrics(&mut self) {
        self.metrics = None;
    }

    pub fn set_mmds_config(&mut self, mmds_config: MmdsConfig) {
        self.mmds_config = Some(mmds_config);
    }

    pub fn with_mmds_config(mut self, mmds_config: MmdsConfig) -> FullVmConfiguration {
        self.mmds_config = Some(mmds_config);
        self
    }

    pub fn mmds_config(&self) -> Option<&MmdsConfig> {
        self.mmds_config.as_ref()
    }

    pub fn reset_mmds_config(&mut self) {
        self.mmds_config = None;
    }

    pub fn set_network_interfaces(&mut self, network_interfaces: Vec<NetworkInterface>) {
        self.network_interfaces = Some(network_interfaces);
    }

    pub fn with_network_interfaces(
        mut self,
        network_interfaces: Vec<NetworkInterface>,
    ) -> FullVmConfiguration {
        self.network_interfaces = Some(network_interfaces);
        self
    }

    pub fn network_interfaces(&self) -> Option<&Vec<NetworkInterface>> {
        self.network_interfaces.as_ref()
    }

    pub fn reset_network_interfaces(&mut self) {
        self.network_interfaces = None;
    }

    pub fn set_vsock(&mut self, vsock: Vsock) {
        self.vsock = Some(vsock);
    }

    pub fn with_vsock(mut self, vsock: Vsock) -> FullVmConfiguration {
        self.vsock = Some(vsock);
        self
    }

    pub fn vsock(&self) -> Option<&Vsock> {
        self.vsock.as_ref()
    }

    pub fn reset_vsock(&mut self) {
        self.vsock = None;
    }
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
