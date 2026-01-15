use serde::{Serialize, Deserialize};

/// Packet represents the basic unit exchanged between peers.
///
/// In the future this can:
/// - carry encrypted payloads
/// - include routing information
/// - simulate LAN broadcast packets

/// Packet structure to encapsulate network frames.
/// We use bincode for serialize this structure before sending it over UDP.
#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub id: u64,
    /// Use protocol ID or magic bytes to filter out garbage traffic from internet
    pub protocol_id: u32,
    /// The raw ethernet frame captured from TAP interface
    pub payload: Vec<u8>,
}

impl Packet {

    pub fn new(payload: Vec<u8>) -> Self {
         Self {
            id: rand::random(),
            protocol_id: 0xDEADBEEF,
            payload: payload,
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
        Ok(bincode::deserialize(data)?)
    }

    pub fn ping() -> Self {
        Self {
            id: rand::random(),
            protocol_id: 0xDEADBEEF,
            payload: b"ping".to_vec(),
        }
    }
}

