//! InstanceActionInfo : Variant wrapper containing the real action.

#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

/// Actions to perform on a uVM
#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Serialize, Deserialize)]
pub enum InstanceAction {
    /// Flush VM metrics
    FlushMetrics,
    /// Start uVM
    InstanceStart,
    /// Send Ctrl+Alt+Del to VM (x86_64 only)
    SendCtrlAltDel,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct InstanceActionInfo {
    /// Enumeration indicating what type of action is contained in the payload
    #[serde(rename = "action_type")]
    action_type: InstanceAction,
}

impl InstanceActionInfo {
    /// Variant wrapper containing the real action.
    pub fn new(action_type: InstanceAction) -> InstanceActionInfo {
        InstanceActionInfo { action_type }
    }

    pub fn set_action_type(&mut self, action_type: InstanceAction) {
        self.action_type = action_type;
    }

    pub fn with_action_type(mut self, action_type: InstanceAction) -> InstanceActionInfo {
        self.action_type = action_type;
        self
    }

    pub fn action_type(&self) -> &InstanceAction {
        &self.action_type
    }
}
