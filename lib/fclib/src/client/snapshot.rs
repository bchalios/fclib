#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[cfg_attr(feature = "clap", derive(Clone, ValueEnum))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum SnapshotType {
    #[default]
    Full,
    Diff,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotCreateParams {
    /// Path to the file that will contain the guest memory.
    #[serde(rename = "mem_file_path")]
    pub mem_file_path: String,

    /// Path to the file that will contain the microVM state.
    #[serde(rename = "snapshot_path")]
    pub snapshot_path: String,

    /// Type of snapshot to create. It is optional and by default, a full snapshot is created.
    #[cfg_attr(feature = "clap", arg(long, short))]
    #[serde(rename = "snapshot_type")]
    pub snapshot_type: SnapshotType,
}

impl SnapshotCreateParams {
    pub fn new(mem_file_path: String, snapshot_path: String) -> SnapshotCreateParams {
        SnapshotCreateParams {
            mem_file_path,
            snapshot_path,
            snapshot_type: SnapshotType::default(),
        }
    }
}

#[cfg_attr(feature = "clap", derive(ValueEnum, Clone))]
#[derive(Debug, Default, Serialize, Deserialize)]
pub enum MemoryBackendType {
    #[default]
    File,
    Uffd,
}

#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryBackend {
    #[serde(rename = "backend_type")]
    pub backend_type: MemoryBackendType,
    /// Based on 'backend_type' it is either 1) Path to the file that contains the guest memory to
    /// be loaded 2) Path to the UDS where a process is listening for a UFFD initialization control
    /// payload and open file descriptor that it can use to serve this process's guest memory page
    /// faults
    #[serde(rename = "backend_path")]
    pub backend_path: String,
}

/// SnapshotLoadParams : Defines the configuration used for handling snapshot resume. Exactly
/// one of the two `mem_*` fields must be present in the body of the request.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Path to the file that contains the microVM state to be loaded.
    #[serde(rename = "snapshot_path")]
    pub snapshot_path: String,

    /// Configuration for the backend that handles memory load. If this field is specified,
    /// `mem_file_path` is forbidden. Either `mem_backend` or `mem_file_path` must be present at a
    /// time.
    #[cfg_attr(feature = "clap", clap(flatten))]
    #[serde(rename = "mem_backend")]
    pub mem_backend: MemoryBackend,

    /// When set to true, the vm is also resumed if the snapshot load is successful.
    #[cfg_attr(feature = "clap", arg(long, short, required = false))]
    #[serde(rename = "resume_vm")]
    pub resume_vm: bool,

    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    #[cfg_attr(feature = "clap", arg(long, short, required = false))]
    #[serde(rename = "enable_diff_snapshots")]
    pub enable_diff_snapshots: bool,
}

impl SnapshotLoadParams {
    /// Defines the configuration used for handling snapshot resume. Exactly one of the two `mem_*`
    /// fields must be present in the body of the request.
    pub fn new(snapshot_path: String, mem_backend: MemoryBackend) -> SnapshotLoadParams {
        SnapshotLoadParams {
            enable_diff_snapshots: false,
            mem_backend,
            snapshot_path,
            resume_vm: false,
        }
    }
}

impl ApiClient {
    /// Take a snapshot of the VM.
    pub async fn snapshot_microvm(&self, body: &SnapshotCreateParams) -> Result<()> {
        self.put("/snapshot/create", body).await
    }

    /// Resume a VM from a snapshot.
    pub async fn load_microvm_snapshot(&self, body: &SnapshotLoadParams) -> Result<()> {
        self.put("/snapshot/load", body).await
    }
}
