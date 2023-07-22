//! InstanceInfo : Describes MicroVM instance information.

use serde_derive::{Deserialize, Serialize};

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

impl InstanceInfo {
    /// Describes MicroVM instance information.
    pub fn new(app_name: String, id: String, state: String, vmm_version: String) -> InstanceInfo {
        InstanceInfo {
            app_name,
            id,
            state,
            vmm_version,
        }
    }

    pub fn set_app_name(&mut self, app_name: String) {
        self.app_name = app_name;
    }

    pub fn with_app_name(mut self, app_name: String) -> InstanceInfo {
        self.app_name = app_name;
        self
    }

    pub fn app_name(&self) -> &String {
        &self.app_name
    }

    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub fn with_id(mut self, id: String) -> InstanceInfo {
        self.id = id;
        self
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    pub fn with_state(mut self, state: String) -> InstanceInfo {
        self.state = state;
        self
    }

    pub fn state(&self) -> &String {
        &self.state
    }

    pub fn set_vmm_version(&mut self, vmm_version: String) {
        self.vmm_version = vmm_version;
    }

    pub fn with_vmm_version(mut self, vmm_version: String) -> InstanceInfo {
        self.vmm_version = vmm_version;
        self
    }

    pub fn vmm_version(&self) -> &String {
        &self.vmm_version
    }
}
