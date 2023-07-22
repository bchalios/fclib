#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    #[serde(rename = "mem_file_path")]
    mem_file_path: String,

    /// Path to the file that will contain the microVM state.
    #[serde(rename = "snapshot_path")]
    snapshot_path: String,

    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "snapshot_type")]
    snapshot_type: Option<String>,

    /// The microVM version for which we want to create the snapshot. It is optional and it
    /// defaults to the current version.
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "version")]
    version: Option<String>,
}

impl SnapshotCreateParams {
    pub fn new(mem_file_path: String, snapshot_path: String) -> SnapshotCreateParams {
        SnapshotCreateParams {
            mem_file_path,
            snapshot_path,
            snapshot_type: None,
            version: None,
        }
    }

    pub fn set_mem_file_path(&mut self, mem_file_path: String) {
        self.mem_file_path = mem_file_path;
    }

    pub fn with_mem_file_path(mut self, mem_file_path: String) -> SnapshotCreateParams {
        self.mem_file_path = mem_file_path;
        self
    }

    pub fn mem_file_path(&self) -> &String {
        &self.mem_file_path
    }

    pub fn set_snapshot_path(&mut self, snapshot_path: String) {
        self.snapshot_path = snapshot_path;
    }

    pub fn with_snapshot_path(mut self, snapshot_path: String) -> SnapshotCreateParams {
        self.snapshot_path = snapshot_path;
        self
    }

    pub fn snapshot_path(&self) -> &String {
        &self.snapshot_path
    }

    pub fn set_snapshot_type(&mut self, snapshot_type: String) {
        self.snapshot_type = Some(snapshot_type);
    }

    pub fn with_snapshot_type(mut self, snapshot_type: String) -> SnapshotCreateParams {
        self.snapshot_type = Some(snapshot_type);
        self
    }

    pub fn snapshot_type(&self) -> Option<&String> {
        self.snapshot_type.as_ref()
    }

    pub fn reset_snapshot_type(&mut self) {
        self.snapshot_type = None;
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn with_version(mut self, version: String) -> SnapshotCreateParams {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<&String> {
        self.version.as_ref()
    }

    pub fn reset_version(&mut self) {
        self.version = None;
    }
}
