use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::client::ApiClient;

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
pub struct VmmBuilder {
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

impl VmmBuilder {
    fn new<P: AsRef<Path>>(fc_path: PathBuf, api_socket: P) -> Self {
        VmmBuilder {
            fc_path,
            api_sock: api_socket.as_ref().to_path_buf(),
            config: None,
            vm_id: None,
            disable_seccomp: false,
            log_path: None,
            log_level: LogLevel::Error,
        }
    }

    pub fn with_config<P: AsRef<Path>>(mut self, config: P) -> Self {
        self.config = Some(config.as_ref().to_path_buf());
        self
    }

    pub fn with_vm_id<S: Into<String>>(mut self, id: S) -> Self {
        self.vm_id = Some(id.into());
        self
    }

    pub fn disable_seccomp(mut self) -> Self {
        self.disable_seccomp = true;
        self
    }

    pub fn with_log_path<P: AsRef<Path>>(mut self, log_path: P) -> Self {
        self.log_path = Some(log_path.as_ref().to_path_buf());
        self
    }

    pub fn with_log_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    pub fn start_vmm(self) -> std::result::Result<Vmm, std::io::Error> {
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

        let vmm = cmd
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        // TODO: sleep to wait for the socket to be created. This should
        // be fixed!
        thread::sleep(Duration::from_millis(100));

        Ok(Vmm {
            vmm,
            api_sock: self.api_sock,
        })
    }
}

#[derive(Debug)]
pub struct Vmm {
    // Firecracker VMM process
    vmm: Child,
    // Unix socket of the VMM
    api_sock: PathBuf,
}

impl Vmm {
    pub fn builder<F, A>(fc_path: F, api_socket: A) -> VmmBuilder
    where
        F: AsRef<Path>,
        A: AsRef<Path>,
    {
        VmmBuilder::new(fc_path.as_ref().to_path_buf(), api_socket)
    }

    pub fn pid(&self) -> u32 {
        self.vmm.id()
    }

    pub fn serial_in(&mut self, buf: &str) -> std::result::Result<(), std::io::Error> {
        self.vmm.stdin.as_ref().unwrap().write_all(buf.as_bytes())
    }

    pub fn serial_out(&mut self, buf: &mut String) -> std::result::Result<usize, std::io::Error> {
        self.vmm.stdout.as_mut().unwrap().read_to_string(buf)
    }

    pub fn api_client(&self) -> ApiClient {
        ApiClient::new(&self.api_sock)
    }
}

impl Drop for Vmm {
    fn drop(&mut self) {
        let _ = self.vmm.kill();
    }
}
