mod config;
mod network;
mod packet;

use config::Config;
use network::NetworkNode;
use packet::Packet;

use rand::Rng;
use std::time::Duration;

/// Program entry point.
///
/// Responsibilities:
/// - Load configuration
/// - Initialize network node
/// - Start send/receive loops
///
/// IMPORTANT:
/// main.rs should NOT contain networking logic.
/// It only orchestrates high-level behavior.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load configuration (CLI for now, GUI later)
    let config = Config::from_args();

    // --- LOGS ---
    println!("Mini LAN Bridge starting...");
    println!("Binding to {}", config.bind_addr);
    println!("Peer address {}", config.peer_addr);

    // Create the network node
    let node: NetworkNode = match NetworkNode::new(config.bind_addr, config.peer_addr).await {
        Ok(n) => {
            println!("UDP node created successfully!");
            n
        }
        Err(e) => {
            eprintln!("Failed to bind UDP socket: {}", e);
            return Ok(()); // Detener programa seguro
        }
    };

    // Spawn receiver task
    let receiver = node.clone();
    tokio::spawn(async move {
        receiver.receive_loop().await;
    });

    // Main send loop (heartbeat / test packets)
    loop {
        let packet = Packet {
            id: rand::thread_rng().gen(),
            payload: b"Hello from Mini LAN Bridge".to_vec(),
        };

        if let Err(e) = node.send(&packet).await {
            eprintln!("Send error: {}", e);
        } else {
            println!("Sent packet id={}", packet.id);
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }

}
