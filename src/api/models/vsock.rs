//! Vsock : Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side.
//! For host-initiated connections, Firecracker will be listening on the Unix socket identified
//! by the path `uds_path`. Firecracker will create this socket, bind and listen on it.
//! Host-initiated connections will be performed by connection to this socket and issuing a
//! connection forwarding request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`,
//! to connect to port 52). For guest-initiated connections, Firecracker will expect host
//! software to be bound and listening on Unix sockets at `uds_path_<PORT>`. E.g.
//! \"/path/to/host_vsock.sock_52\" for port number 52.
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Vsock {
    /// Guest Vsock CID
    #[serde(rename = "guest_cid")]
    guest_cid: i32,
    /// Path to UNIX domain socket, used to proxy vsock connections.
    #[serde(rename = "uds_path")]
    uds_path: String,
    /// This parameter has been deprecated since v1.0.0.
    #[serde(rename = "vsock_id")]
    vsock_id: Option<String>,
}

impl Vsock {
    /// Defines a vsock device, backed by a set of Unix Domain Sockets, on the host side. For
    /// host-initiated connections, Firecracker will be listening on the Unix socket identified by
    /// the path `uds_path`. Firecracker will create this socket, bind and listen on it.
    /// Host-initiated connections will be performed by connection to this socket and issuing a
    /// connection forwarding request to the desired guest-side vsock port (i.e. `CONNECT 52\\n`, to
    /// connect to port 52). For guest-initiated connections, Firecracker will expect host software
    /// to be bound and listening on Unix sockets at `uds_path_<PORT>`. E.g.
    /// \"/path/to/host_vsock.sock_52\" for port number 52.
    pub fn new(guest_cid: i32, uds_path: String) -> Vsock {
        Vsock {
            guest_cid,
            uds_path,
            vsock_id: None,
        }
    }

    pub fn set_guest_cid(&mut self, guest_cid: i32) {
        self.guest_cid = guest_cid;
    }

    pub fn with_guest_cid(mut self, guest_cid: i32) -> Vsock {
        self.guest_cid = guest_cid;
        self
    }

    pub fn guest_cid(&self) -> &i32 {
        &self.guest_cid
    }

    pub fn set_uds_path(&mut self, uds_path: String) {
        self.uds_path = uds_path;
    }

    pub fn with_uds_path(mut self, uds_path: String) -> Vsock {
        self.uds_path = uds_path;
        self
    }

    pub fn uds_path(&self) -> &String {
        &self.uds_path
    }

    pub fn set_vsock_id(&mut self, vsock_id: String) {
        self.vsock_id = Some(vsock_id);
    }

    pub fn with_vsock_id(mut self, vsock_id: String) -> Vsock {
        self.vsock_id = Some(vsock_id);
        self
    }

    pub fn vsock_id(&self) -> Option<&String> {
        self.vsock_id.as_ref()
    }

    pub fn reset_vsock_id(&mut self) {
        self.vsock_id = None;
    }
}
