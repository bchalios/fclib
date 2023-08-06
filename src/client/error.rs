use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct FcError {
    /// A description of the error condition
    #[serde(rename = "fault_message")]
    fault_message: Option<String>,
}

impl std::fmt::Display for FcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fault_message.as_ref().unwrap_or(&"".to_owned())
        )
    }
}
