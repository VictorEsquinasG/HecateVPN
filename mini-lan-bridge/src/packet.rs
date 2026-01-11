use serde::{Serialize, Deserialize};

/// Packet represents the basic unit exchanged between peers.
///
/// In the future this can:
/// - carry encrypted payloads
/// - include routing information
/// - simulate LAN broadcast packets
#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    /// Unique packet identifier
    pub id: u64,

    /// Raw payload bytes
    pub payload: Vec<u8>,
}

/*
TODO:
- Add packet type (Handshake, Data, Ping)
- Add optional encryption flag
- Add checksum or integrity verification
*/
