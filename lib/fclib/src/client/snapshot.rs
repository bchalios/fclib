#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

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
}

impl SnapshotCreateParams {
    pub fn new(mem_file_path: String, snapshot_path: String) -> SnapshotCreateParams {
        SnapshotCreateParams {
            mem_file_path,
            snapshot_path,
            snapshot_type: None,
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
    backend_type: MemoryBackendType,
    /// Based on 'backend_type' it is either 1) Path to the file that contains the guest memory to
    /// be loaded 2) Path to the UDS where a process is listening for a UFFD initialization control
    /// payload and open file descriptor that it can use to serve this process's guest memory page
    /// faults
    #[serde(rename = "backend_path")]
    backend_path: String,
}

impl MemoryBackend {
    pub fn new(backend_type: MemoryBackendType, backend_path: String) -> MemoryBackend {
        MemoryBackend {
            backend_type,
            backend_path,
        }
    }

    pub fn set_backend_type(&mut self, backend_type: MemoryBackendType) {
        self.backend_type = backend_type;
    }

    pub fn with_backend_type(mut self, backend_type: MemoryBackendType) -> MemoryBackend {
        self.backend_type = backend_type;
        self
    }

    pub fn backend_type(&self) -> &MemoryBackendType {
        &self.backend_type
    }

    pub fn set_backend_path(&mut self, backend_path: String) {
        self.backend_path = backend_path;
    }

    pub fn with_backend_path(mut self, backend_path: String) -> MemoryBackend {
        self.backend_path = backend_path;
        self
    }

    pub fn backend_path(&self) -> &String {
        &self.backend_path
    }
}

/// SnapshotLoadParams : Defines the configuration used for handling snapshot resume. Exactly
/// one of the two `mem_*` fields must be present in the body of the request.
#[cfg_attr(feature = "clap", derive(Args))]
#[derive(Debug, Serialize, Deserialize)]
pub struct SnapshotLoadParams {
    /// Path to the file that contains the microVM state to be loaded.
    #[serde(rename = "snapshot_path")]
    snapshot_path: String,

    /// Configuration for the backend that handles memory load. If this field is specified,
    /// `mem_file_path` is forbidden. Either `mem_backend` or `mem_file_path` must be present at a
    /// time.
    #[cfg_attr(feature = "clap", clap(flatten))]
    #[serde(rename = "mem_backend")]
    mem_backend: MemoryBackend,

    /// When set to true, the vm is also resumed if the snapshot load is successful.
    #[cfg_attr(feature = "clap", arg(long, short, required = false))]
    #[serde(rename = "resume_vm")]
    resume_vm: bool,

    /// Enable support for incremental (diff) snapshots by tracking dirty guest pages.
    #[cfg_attr(feature = "clap", arg(long, short, required = false))]
    #[serde(rename = "enable_diff_snapshots")]
    enable_diff_snapshots: bool,
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

    pub fn enable_diff_snapshots(&mut self) {
        self.enable_diff_snapshots = true;
    }

    pub fn disable_diff_snapshots(&mut self) {
        self.enable_diff_snapshots = false;
    }

    pub fn with_enable_diff_snapshots(mut self, enable_diff_snapshots: bool) -> SnapshotLoadParams {
        self.enable_diff_snapshots = enable_diff_snapshots;
        self
    }

    pub fn enabled_diff_snapshots(&self) -> bool {
        self.enable_diff_snapshots
    }

    pub fn set_mem_backend(&mut self, mem_backend: MemoryBackend) {
        self.mem_backend = mem_backend;
    }

    pub fn with_mem_backend(mut self, mem_backend: MemoryBackend) -> SnapshotLoadParams {
        self.mem_backend = mem_backend;
        self
    }

    pub fn mem_backend(&self) -> &MemoryBackend {
        &self.mem_backend
    }

    pub fn set_snapshot_path(&mut self, snapshot_path: String) {
        self.snapshot_path = snapshot_path;
    }

    pub fn with_snapshot_path(mut self, snapshot_path: String) -> SnapshotLoadParams {
        self.snapshot_path = snapshot_path;
        self
    }

    pub fn snapshot_path(&self) -> &String {
        &self.snapshot_path
    }

    pub fn enable_resume_vm(&mut self) {
        self.resume_vm = true;
    }

    pub fn disable_resume_vm(&mut self) {
        self.resume_vm = false;
    }

    pub fn with_resume_vm(mut self, resume_vm: bool) -> SnapshotLoadParams {
        self.resume_vm = resume_vm;
        self
    }

    pub fn enabled_resume_vm(&self) -> bool {
        self.resume_vm
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
