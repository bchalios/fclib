//! CpuConfig : The CPU configuration template defines a set of bit maps as modifiers of flags accessed by register to be disabled/enabled for the microvm.

use serde_derive::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::Value;

use super::{ApiClient, Result};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CpuConfig {
    /// A collection of CPUIDs to be modified. (x86_64)
    pub cpuid_modifiers: Option<Value>,
    /// A collection of model specific registers to be modified. (x86_64)
    pub msr_modifiers: Option<Value>,
    /// A collection of registers to be modified. (aarch64)
    pub reg_modifiers: Option<Value>,
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
