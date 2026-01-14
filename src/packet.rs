use serde::{Serialize, Deserialize};

/// Packet represents the basic unit exchanged between peers.
///
/// In the future this can:
/// - carry encrypted payloads
/// - include routing information
/// - simulate LAN broadcast packets

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub id: u64,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn decode(data: &[u8]) -> anyhow::Result<Self> {
        Ok(bincode::deserialize(data)?)
    }

    pub fn ping() -> Self {
        Self {
            id: rand::random(),
            payload: b"ping".to_vec(),
        }
    }
}

