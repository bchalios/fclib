use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

use crate::api::client::ApiClient;

#[derive(Debug)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Warning => write!(f, "Warning"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Debug => write!(f, "Debug"),
        }
    }
}

#[derive(Debug)]
pub struct FcVmmBuilder {
    // Path to Firecracker binary
    fc_path: PathBuf,
    // Path to API socket
    api_sock: PathBuf,
    // Path to configuration file to start VM from
    config: Option<PathBuf>,
    // MicroVM id
    vm_id: Option<String>,
    // If `true` disable seccomp filters
    disable_seccomp: bool,
    // Path to log file
    log_path: Option<PathBuf>,
    // Log level
    log_level: LogLevel,
}

impl FcVmmBuilder {
    fn new<P: AsRef<Path>>(fc_path: PathBuf, api_socket: P) -> Self {
        FcVmmBuilder {
            fc_path,
            api_sock: api_socket.as_ref().to_path_buf(),
            config: None,
            vm_id: None,
            disable_seccomp: false,
            log_path: None,
            log_level: LogLevel::Error,
        }
    }

    pub fn with_config<P: AsRef<Path>>(&mut self, config: P) {
        self.config = Some(config.as_ref().to_path_buf());
    }

    pub fn with_vm_id<S: Into<String>>(&mut self, id: S) {
        self.vm_id = Some(id.into());
    }

    pub fn disable_seccomp(&mut self) {
        self.disable_seccomp = true;
    }

    pub fn with_log_path<P: AsRef<Path>>(&mut self, log_path: P) {
        self.log_path = Some(log_path.as_ref().to_path_buf());
    }

    pub fn with_log_level(&mut self, level: LogLevel) {
        self.log_level = level;
    }

    pub fn start_vmm(self) -> Result<FcVmm, std::io::Error> {
        let mut cmd = Command::new(self.fc_path);
        cmd.arg("--api-sock").arg(&self.api_sock);

        if let Some(path) = self.config {
            cmd.arg("--config-file").arg(path);
        }

        if let Some(id) = self.vm_id {
            cmd.arg("--id").arg(id);
        }

        if self.disable_seccomp {
            cmd.arg("--no-seccomp");
        }

        if let Some(path) = self.log_path {
            cmd.arg("--log-path").arg(path);
            cmd.arg("--level").arg(self.log_level.to_string());
        }

        let vmm = cmd.spawn()?;

        // TODO: sleep to wait for the socket to be created. This should
        // be fixed!
        thread::sleep(Duration::from_millis(100));

        Ok(FcVmm {
            vmm,
            client: ApiClient::new(&self.api_sock),
        })
    }
}

#[derive(Debug)]
pub struct FcVmm {
    // Firecracker VMM process
    vmm: Child,
    pub(crate) client: ApiClient,
}

impl FcVmm {
    pub fn builder<P: AsRef<Path>>(fc_path: P, api_socket: P) -> FcVmmBuilder {
        FcVmmBuilder::new(
            fc_path.as_ref().to_path_buf(),
            api_socket.as_ref().to_path_buf(),
        )
    }

    pub fn pid(&self) -> u32 {
        self.vmm.id()
    }
}

impl Drop for FcVmm {
    fn drop(&mut self) {
        let _ = self.vmm.kill();
    }
}
