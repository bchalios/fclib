use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    /// A description of the error condition
    #[serde(rename = "fault_message")]
    fault_message: Option<String>,
}

impl Error {
    pub fn new() -> Error {
        Error {
            fault_message: None,
        }
    }

    pub fn set_fault_message(&mut self, fault_message: String) {
        self.fault_message = Some(fault_message);
    }

    pub fn with_fault_message(mut self, fault_message: String) -> Error {
        self.fault_message = Some(fault_message);
        self
    }

    pub fn fault_message(&self) -> Option<&String> {
        self.fault_message.as_ref()
    }

    pub fn reset_fault_message(&mut self) {
        self.fault_message = None;
    }
}
