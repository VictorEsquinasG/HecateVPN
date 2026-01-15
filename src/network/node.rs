use std::net::SocketAddr;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::app::AppState;
use crate::packet::Packet;

/// NetworkNode manages the bridge between the physical network (UDP)
/// and the virtual network (TAP interface)
#[derive(Clone)]
pub struct NetworkNode {
    socket: Arc<UdpSocket>,
    peer: SocketAddr,
    state: Arc<AppState>,
    tun_device: Arc<tokio::sync::Mutex<tun::AsyncDevice>>,
}

impl NetworkNode {
    /// Initialize the VPN node
    /// `virtual_ip`: The IP inside the VPN (e.g. 10.0.0.2)
    pub async fn new(
        bind_addr: SocketAddr,
        peer: SocketAddr,
        state: Arc<AppState>,
        virtual_ip: &str
    ) -> anyhow::Result<Self> {
        // Bind UDP socket (Physical layer)
        let socket = UdpSocket::bind(bind_addr).await?;
        let socket = Arc::new(socket);

        state.log(format!("🔌 Socket bound on {}, peer={}", bind_addr, peer));

        // Configure TAP device (Virtual layer)
        let mut config = tun::Configuration::default();
        config.address(virtual_ip).netmask("255.255.255.0").destination(virtual_ip).up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true); // Linux specific header handling
        });
        let tun_device = tun::create_as_async(&config)?;
        let tun_device = Arc::new(tokio::sync::Mutex::new(tun_device));

        Ok(Self {
            socket,
            peer,
            state,
            tun_device,
        })
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let mut buf_tun = [0u8; 4096]; // Buffer for reading from Visual Interface
        let mut buf_udp = [0u8; 4096]; // Buffer for reading from Physical Interface
        
        // Use tokio select to handle whichever comes first:
        // 1. The game sends a packet (Read from TUN)
        // 2. The peer sends a packet (Read from UDP)
        tokio::select! {
        // A. - Outbound: TAP -> UDP
        // Lock TUN device to read
        res = async {
            let mut locket_dev = self.tun_device.lock().await;
            locket_dev.read(&mut buf_tun).await
        } => {
        match res {
            Ok(n) => {
                // Wrap the raw frame in our Packet structure
                let packet = Packet::new(buf_tun[0..n].to_vec());
                // Serialize (and encrypt in the future)
                let encoded = bincode::serialize(&packet)?;
                // Send over UDP
                self.socket.send_to(&encoded, self.peer).await?;
            },
            Err(e) => eprint!("Error reading from TUN device: {}", e),
        }

                // B. - Inbound: UDP -> TAP
                res = self.socket.recv_from(&mut buf_udp) => {
                    match res {
                        Ok((n,src_addr)) => {
                            // Security check: only accept packets from the known peer
                            if src_addr == self.peer{
                                // Deserialize (and decrypt in the future)
                               if let Ok(packet) = bincode::deserialize::<Packet>(&buf_udp[..n]) {
                                // Write raw frame to the Virtual Interface
                                // The OS will now "think" this packet arrived from local network
                                let mut locket_dev = self.tun_device.lock().await;
                                locket_dev.write_all(&packet.payload).await?;
                               }
                            }
                        }
                    },
                    Err(e) => {
                        eprint!("Error reading from UDP socket: {}", e),
                    }
                }

            }
        }
    }

    /// The main loop that bridges the traffic
    /// This function spawns two tasks:
    /// - One for reading from the TUN device and sending over UDP
    /// - Another for reading from UDP and writing to the TUN device
    /// Returns when shutdown is requested or an error occurs
    pub async fn send(&self, packet: &Packet) -> anyhow::Result<()> {
        let bytes = packet.encode();
        self.socket.send(&bytes).await?;
        Ok(())
    }

    pub async fn receive_loop(&self) -> anyhow::Result<()> {
        let mut buf_tun = [0u8; 4096]; // Buffer for reading from Visual Interface
        let mut buf_udp = [0u8; 4096]; // Buffer for reading from Physical Interface

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

            self.state.log(format!("📥 Packet id={} ({} bytes)", packet.id, len));
        }

        Ok(())
    }
}
