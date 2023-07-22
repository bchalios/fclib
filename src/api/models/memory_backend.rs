#[cfg(feature = "clap")]
use clap::{Args, ValueEnum};
use serde_derive::{Deserialize, Serialize};

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
