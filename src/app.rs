use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub my_ip: String,
    pub peer_ip: String,
    pub peer_port: String, // optional
    pub logs: Arc<Mutex<Vec<String>>>,
    pub connected: bool,
}

impl AppState {
    pub fn new(my_ip: String) -> Self {
        Self {
            my_ip,
            peer_port: "9000".to_string(),
            ..Default::default()
        }
    }

    pub fn log(&self, message: String) {
        let mut logs = self.logs.lock().unwrap();
        logs.push(message);
        if logs.len() > 150 {
            logs.remove(0);
        }
    }
}
