use crate::sockets::{SocketAddrCheck, SocketAddressFamily};
use std::net::SocketAddr;
use std::sync::Arc;

pub struct NetworkIncomingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,
}

pub struct NetworkOutgoingDatagramStream {
    pub(crate) inner: Arc<tokio::net::UdpSocket>,

    /// If this has a value, the stream is "connected".
    pub(crate) remote_address: Option<SocketAddr>,

    /// Socket address family.
    pub(crate) family: SocketAddressFamily,

    pub(crate) send_state: SendState,

    /// The check of allowed addresses
    pub(crate) socket_addr_check: Option<SocketAddrCheck>,
}

pub struct LoopbackIncomingDatagramStream {
    pub remote_address: Option<SocketAddr>,
    pub rx: Arc<
        tokio::sync::Mutex<
            tokio::sync::mpsc::UnboundedReceiver<(
                crate::sockets::loopback::UdpDatagram,
                tokio::sync::OwnedSemaphorePermit,
            )>,
        >,
    >,
    pub received: Option<(
        crate::sockets::loopback::UdpDatagram,
        tokio::sync::OwnedSemaphorePermit,
    )>,
}

pub struct LoopbackOutgoingDatagramStream {
    pub local_address: SocketAddr,
    pub remote_address: Option<SocketAddr>,
    pub(crate) family: SocketAddressFamily,
    pub(crate) socket_addr_check: Option<SocketAddrCheck>,
    pub permits: Arc<tokio::sync::Semaphore>,
    pub(crate) permit: Option<tokio::sync::OwnedSemaphorePermit>,
}

pub enum IncomingDatagramStream {
    Network(NetworkIncomingDatagramStream),
    Loopback(LoopbackIncomingDatagramStream),
}

pub enum OutgoingDatagramStream {
    Network(NetworkOutgoingDatagramStream),
    Loopback(LoopbackOutgoingDatagramStream),
}

pub(crate) enum SendState {
    /// Waiting for the API consumer to call `check-send`.
    Idle,

    /// Ready to send up to x datagrams.
    Permitted(usize),

    /// Waiting for the OS.
    Waiting,
}
