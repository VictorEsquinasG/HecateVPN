use std::net::SocketAddr;

/// Config holds all runtime configuration.
///
/// This struct is intentionally simple.
/// In the future it can be populated by:
/// - GUI
/// - config file (TOML)
/// - environment variables
#[derive(Debug, Clone)]
pub struct Config {
    /// Local address to bind the UDP socket
    pub bind_addr: SocketAddr,

    /// Remote peer address
    pub peer_addr: SocketAddr,
}

impl Config {
    /// Creates a Config from command-line arguments.
    ///
    /// Usage:
    /// mini-lan-bridge <bind_addr> <peer_addr>
    ///
    /// Example:
    /// mini-lan-bridge 0.0.0.0:9000 192.168.1.50:9000
    pub fn from_args() -> Self {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 3 {
            eprintln!("Usage:");
            eprintln!("  mini-lan-bridge <bind_addr> <peer_addr>");
            std::process::exit(1);
        }

        let bind_addr: SocketAddr = args[1]
            .parse()
            .expect("Invalid bind address");

        let peer_addr: SocketAddr = args[2]
            .parse()
            .expect("Invalid peer address");

        Self {
            bind_addr,
            peer_addr,
        }
    }
}

/*
TODO:
- Support config file (config.toml)
- Support GUI input
- Validate ports and IP ranges
*/
