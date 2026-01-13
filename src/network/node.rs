use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;

use crate::packet::Packet;

/// NetworkNode handles all UDP communication.
///
/// This struct abstracts low-level networking details.
#[derive(Clone)]
pub struct NetworkNode {
    socket: Arc<UdpSocket>,
    peer: SocketAddr,
}

impl NetworkNode {
    /// Creates a new UDP node bound to a local address
    pub async fn new(bind: SocketAddr, peer: SocketAddr) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(bind).await?;

        Ok(Self {
            socket: Arc::new(socket),
            peer,
        })
    }

    /// Sends a packet to the remote peer
    pub async fn send(&self, packet: &Packet) -> anyhow::Result<()> {
        let data = bincode::serialize(packet)?;
        self.socket.send_to(&data, self.peer).await?;
        Ok(())
    }

    /// Receives packets indefinitely
    pub async fn receive_loop(&self) -> anyhow::Result<()> {
        let mut buffer = [0u8; 1024];

        loop {
            let (len, addr) = self.socket.recv_from(&mut buffer).await?;
            if let Ok(packet) = bincode::deserialize::<Packet>(&buffer[..len]) {
                println!(
                    "Received from {} | id={} | payload={}",
                    addr,
                    packet.id,
                    String::from_utf8_lossy(&packet.payload)
                );
            }
        }
    }
}
