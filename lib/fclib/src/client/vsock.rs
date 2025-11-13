//! Vsock : Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side.
//! For host-initiated connections, Firecracker will be listening on the Unix socket identified
//! by the path `uds_path`. Firecracker will create this socket, bind and listen on it.
//! Host-initiated connections will be performed by connection to this socket and issuing a
//! connection forwarding request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`,
//! to connect to port 52). For guest-initiated connections, Firecracker will expect host
//! software to be bound and listening on Unix sockets at `uds_path_<PORT>`. E.g.
//! \"/path/to/host_vsock.sock_52\" for port number 52.
#[cfg(feature = "clap")]
use clap::Args;
use serde_derive::{Deserialize, Serialize};

use super::{ApiClient, Result};

#[cfg_attr(feature = "clap", derive(Args, Clone))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Vsock {
    /// Guest Vsock CID
    pub guest_cid: i32,
    /// Path to UNIX domain socket, used to proxy vsock connections.
    pub uds_path: String,
    /// This parameter has been deprecated since v1.0.0.
    pub vsock_id: Option<String>,
}

impl ApiClient {
    pub async fn config_vsock(&self, vsock: &Vsock) -> Result<()> {
        self.put("/vsock", vsock).await
    }
}
