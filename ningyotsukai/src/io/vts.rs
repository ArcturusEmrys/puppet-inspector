use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::rc::Rc;

use smol::net::UdpSocket;
use smol::LocalExecutor;
use smol::future::FutureExt;
use json::object;

use crate::io::error::IoThreadError;

async fn send_heartbeat_packet(socket: UdpSocket, addr: SocketAddr) -> Result<(), IoThreadError> {
    loop {
        socket.send_to(object! {
            "messageType": "iOSTrackingDataRequest",
            "time": 10,
            "sentBy": "ningyotsukai",
            "ports": [socket.local_addr()?.port()]
        }.to_string().as_bytes(), addr).await?;
    }
}

async fn recv_tracking_packet(socket: UdpSocket) -> Result<(), IoThreadError> {
    let mut buf = vec![0; 65507];
    loop {
        let (size, _) = socket.recv_from(&mut buf).await?;
        if size > buf.len() {
            //TODO: We lost data!
            buf.resize(size, 0);
        }

        let json = json::parse(str::from_utf8(&buf[0..size])?)?;

        eprintln!("{:?}", json);
    }
}

pub async fn connect_vts_tracker(ex: Rc<LocalExecutor<'_>>, addr: SocketAddr) -> Result<(), IoThreadError> {
    let socket = match addr {
        SocketAddr::V4(_addr) => UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0)).await?,
        SocketAddr::V6(_addr) => UdpSocket::bind((Ipv6Addr::UNSPECIFIED, 0)).await?
    };

    let send = ex.spawn(send_heartbeat_packet(socket.clone(), addr));
    let recv = ex.spawn(recv_tracking_packet(socket.clone()));

    send.or(recv).await
}
