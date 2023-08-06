//! CpuConfig : The CPU configuration template defines a set of bit maps as modifiers of flags accessed by register to be disabled/enabled for the microvm.

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use super::{ApiClient, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CpuConfig {
    /// A collection of CPUIDs to be modified. (x86_64)
    #[serde(rename = "cpuid_modifiers")]
    cpuid_modifiers: Option<Value>,
    /// A collection of model specific registers to be modified. (x86_64)
    #[serde(rename = "msr_modifiers")]
    msr_modifiers: Option<Value>,
    /// A collection of registers to be modified. (aarch64)
    #[serde(rename = "reg_modifiers")]
    reg_modifiers: Option<Value>,
}

impl CpuConfig {
    /// The CPU configuration template defines a set of bit maps as modifiers of flags accessed by register to be disabled/enabled for the microvm.
    pub fn new() -> CpuConfig {
        CpuConfig {
            cpuid_modifiers: None,
            msr_modifiers: None,
            reg_modifiers: None,
        }
    }

    pub fn set_cpuid_modifiers(&mut self, cpuid_modifiers: Value) {
        self.cpuid_modifiers = Some(cpuid_modifiers);
    }

    pub fn with_cpuid_modifiers(mut self, cpuid_modifiers: Value) -> CpuConfig {
        self.cpuid_modifiers = Some(cpuid_modifiers);
        self
    }

    pub fn cpuid_modifiers(&self) -> Option<&Value> {
        self.cpuid_modifiers.as_ref()
    }

    pub fn reset_cpuid_modifiers(&mut self) {
        self.cpuid_modifiers = None;
    }

    pub fn set_msr_modifiers(&mut self, msr_modifiers: Value) {
        self.msr_modifiers = Some(msr_modifiers);
    }

    pub fn with_msr_modifiers(mut self, msr_modifiers: Value) -> CpuConfig {
        self.msr_modifiers = Some(msr_modifiers);
        self
    }

    pub fn msr_modifiers(&self) -> Option<&Value> {
        self.msr_modifiers.as_ref()
    }

    pub fn reset_msr_modifiers(&mut self) {
        self.msr_modifiers = None;
    }

    pub fn set_reg_modifiers(&mut self, reg_modifiers: Value) {
        self.reg_modifiers = Some(reg_modifiers);
    }

    pub fn with_reg_modifiers(mut self, reg_modifiers: Value) -> CpuConfig {
        self.reg_modifiers = Some(reg_modifiers);
        self
    }

    pub fn reg_modifiers(&self) -> Option<&Value> {
        self.reg_modifiers.as_ref()
    }

    pub fn reset_reg_modifiers(&mut self) {
        self.reg_modifiers = None;
    }
}

impl ApiClient {
    /// Apply a custom CPU template to the VM
    ///
    /// # Arguments
    ///
    /// * `config` - The custom CPU template to apply
    pub async fn apply_cpu_config(&self, config: &CpuConfig) -> Result<()> {
        self.put("/cpu-config", config).await
    }
}
