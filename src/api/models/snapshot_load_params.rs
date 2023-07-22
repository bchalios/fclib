//! SnapshotLoadParams : Defines the configuration used for handling snapshot resume. Exactly
//! one of the two `mem_*` fields must be present in the body of the request.
use super::MemoryBackend;

#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

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
