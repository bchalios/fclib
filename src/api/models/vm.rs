//! Vm : Defines the microVM running state. It is especially useful in the snapshotting
//! context.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vm {
    #[serde(rename = "state")]
    state: String,
}

impl Vm {
    /// Defines the microVM running state. It is especially useful in the snapshotting context.
    pub fn new(state: String) -> Vm {
        Vm { state }
    }

    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }

    pub fn with_state(mut self, state: String) -> Vm {
        self.state = state;
        self
    }

    pub fn state(&self) -> &String {
        &self.state
    }
}
