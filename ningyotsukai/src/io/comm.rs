//! Asynchronous I/O thread

use std::io;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::io::error::IoThreadError;

pub enum IoMessage {
    ConnectVTSTracker(SocketAddr),
    Exit
}

impl IoMessage {
    fn connect_vts_tracker(addr: impl ToSocketAddrs) -> Result<Self, io::Error> {
        Ok(Self::ConnectVTSTracker(addr.to_socket_addrs()?.next().ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "No address provided to connect to"))?))
    }
}

pub enum IoResponse {
    Error(IoThreadError)
}