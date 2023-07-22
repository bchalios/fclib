//! MmdsContentsObject : Describes the contents of MMDS in JSON format.

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MmdsContentsObject {}

impl MmdsContentsObject {
    /// Describes the contents of MMDS in JSON format.
    pub fn new() -> MmdsContentsObject {
        MmdsContentsObject {}
    }
}
