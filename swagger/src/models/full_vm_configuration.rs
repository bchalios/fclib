// Firecracker API
//
// RESTful public-facing API. The API is accessible through HTTP calls on specific URLs carrying
// JSON modeled data. The transport medium is a Unix Domain Socket.
//
// OpenAPI spec version: 1.2.0
// Contact: compute-capsule@amazon.com
// Generated by: https://github.com/swagger-api/swagger-codegen.git

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FullVmConfiguration {
    #[serde(rename = "balloon")]
    balloon: Option<::models::Balloon>,
    /// Configurations for all block devices.
    #[serde(rename = "drives")]
    drives: Option<Vec<::models::Drive>>,
    #[serde(rename = "boot-source")]
    boot_source: Option<::models::BootSource>,
    #[serde(rename = "logger")]
    logger: Option<::models::Logger>,
    #[serde(rename = "machine-config")]
    machine_config: Option<::models::MachineConfiguration>,
    #[serde(rename = "metrics")]
    metrics: Option<::models::Metrics>,
    #[serde(rename = "mmds-config")]
    mmds_config: Option<::models::MmdsConfig>,
    /// Configurations for all net devices.
    #[serde(rename = "network-interfaces")]
    network_interfaces: Option<Vec<::models::NetworkInterface>>,
    #[serde(rename = "vsock")]
    vsock: Option<::models::Vsock>,
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

    pub fn set_balloon(&mut self, balloon: ::models::Balloon) {
        self.balloon = Some(balloon);
    }

    pub fn with_balloon(mut self, balloon: ::models::Balloon) -> FullVmConfiguration {
        self.balloon = Some(balloon);
        self
    }

    pub fn balloon(&self) -> Option<&::models::Balloon> {
        self.balloon.as_ref()
    }

    pub fn reset_balloon(&mut self) {
        self.balloon = None;
    }

    pub fn set_drives(&mut self, drives: Vec<::models::Drive>) {
        self.drives = Some(drives);
    }

    pub fn with_drives(mut self, drives: Vec<::models::Drive>) -> FullVmConfiguration {
        self.drives = Some(drives);
        self
    }

    pub fn drives(&self) -> Option<&Vec<::models::Drive>> {
        self.drives.as_ref()
    }

    pub fn reset_drives(&mut self) {
        self.drives = None;
    }

    pub fn set_boot_source(&mut self, boot_source: ::models::BootSource) {
        self.boot_source = Some(boot_source);
    }

    pub fn with_boot_source(mut self, boot_source: ::models::BootSource) -> FullVmConfiguration {
        self.boot_source = Some(boot_source);
        self
    }

    pub fn boot_source(&self) -> Option<&::models::BootSource> {
        self.boot_source.as_ref()
    }

    pub fn reset_boot_source(&mut self) {
        self.boot_source = None;
    }

    pub fn set_logger(&mut self, logger: ::models::Logger) {
        self.logger = Some(logger);
    }

    pub fn with_logger(mut self, logger: ::models::Logger) -> FullVmConfiguration {
        self.logger = Some(logger);
        self
    }

    pub fn logger(&self) -> Option<&::models::Logger> {
        self.logger.as_ref()
    }

    pub fn reset_logger(&mut self) {
        self.logger = None;
    }

    pub fn set_machine_config(&mut self, machine_config: ::models::MachineConfiguration) {
        self.machine_config = Some(machine_config);
    }

    pub fn with_machine_config(
        mut self,
        machine_config: ::models::MachineConfiguration,
    ) -> FullVmConfiguration {
        self.machine_config = Some(machine_config);
        self
    }

    pub fn machine_config(&self) -> Option<&::models::MachineConfiguration> {
        self.machine_config.as_ref()
    }

    pub fn reset_machine_config(&mut self) {
        self.machine_config = None;
    }

    pub fn set_metrics(&mut self, metrics: ::models::Metrics) {
        self.metrics = Some(metrics);
    }

    pub fn with_metrics(mut self, metrics: ::models::Metrics) -> FullVmConfiguration {
        self.metrics = Some(metrics);
        self
    }

    pub fn metrics(&self) -> Option<&::models::Metrics> {
        self.metrics.as_ref()
    }

    pub fn reset_metrics(&mut self) {
        self.metrics = None;
    }

    pub fn set_mmds_config(&mut self, mmds_config: ::models::MmdsConfig) {
        self.mmds_config = Some(mmds_config);
    }

    pub fn with_mmds_config(mut self, mmds_config: ::models::MmdsConfig) -> FullVmConfiguration {
        self.mmds_config = Some(mmds_config);
        self
    }

    pub fn mmds_config(&self) -> Option<&::models::MmdsConfig> {
        self.mmds_config.as_ref()
    }

    pub fn reset_mmds_config(&mut self) {
        self.mmds_config = None;
    }

    pub fn set_network_interfaces(&mut self, network_interfaces: Vec<::models::NetworkInterface>) {
        self.network_interfaces = Some(network_interfaces);
    }

    pub fn with_network_interfaces(
        mut self,
        network_interfaces: Vec<::models::NetworkInterface>,
    ) -> FullVmConfiguration {
        self.network_interfaces = Some(network_interfaces);
        self
    }

    pub fn network_interfaces(&self) -> Option<&Vec<::models::NetworkInterface>> {
        self.network_interfaces.as_ref()
    }

    pub fn reset_network_interfaces(&mut self) {
        self.network_interfaces = None;
    }

    pub fn set_vsock(&mut self, vsock: ::models::Vsock) {
        self.vsock = Some(vsock);
    }

    pub fn with_vsock(mut self, vsock: ::models::Vsock) -> FullVmConfiguration {
        self.vsock = Some(vsock);
        self
    }

    pub fn vsock(&self) -> Option<&::models::Vsock> {
        self.vsock.as_ref()
    }

    pub fn reset_vsock(&mut self) {
        self.vsock = None;
    }
}
