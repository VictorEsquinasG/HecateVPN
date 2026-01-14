use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::app::AppState;
use crate::packet::Packet;

#[derive(Clone)]
pub struct NetworkNode {
    socket: Arc<UdpSocket>,
    peer: SocketAddr,
    state: Arc<AppState>,
}

impl NetworkNode {
    pub async fn new(
        bind_addr: SocketAddr,
        peer: SocketAddr,
        state: Arc<AppState>,
    ) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        socket.connect(peer).await?;

        state.log(format!("🔌 Socket bound on {}, peer={}", bind_addr, peer));

        Ok(Self {
            socket: Arc::new(socket),
            peer,
            state,
        })
    }

    pub async fn send(&self, packet: &Packet) -> anyhow::Result<()> {
        let bytes = packet.encode();
        self.socket.send(&bytes).await?;
        Ok(())
    }

    pub async fn receive_loop(&self) -> anyhow::Result<()> {
        let mut buf = [0u8; 2048];

        self.state.log("📡 Receive loop started".into());

        loop {
            if self.state.shutdown.load(Ordering::Relaxed) {
                self.state.log("🛑 Receive loop stopped".into());
                break;
            }

            let len = self.socket.recv(&mut buf).await?;

            let packet = Packet::decode(&buf[..len])?;

            // Primer paquete = conexión real
            if !self.state.connected.load(Ordering::Relaxed) {
                self.state.connected.store(true, Ordering::Relaxed);
                self.state.log("✅ Connected!".into());
            }

            self.state
                .log(format!("📥 Packet id={} ({} bytes)", packet.id, len));
        }

        Ok(())
    }
}
