use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct AppState {
    pub my_ip: String,
    pub peer_ip: String,
    pub peer_port: String,  // optional
    pub virtual_ip: String, // IP inside VPN
    pub connected: Arc<AtomicBool>,
    pub shutdown: Arc<AtomicBool>,
    pub connection_handle: Arc<Mutex<Option<std::thread::JoinHandle<()>>>>,
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    pub fn new(my_ip: String) -> Self {
        Self {
            my_ip,
            peer_ip: String::new(),
            peer_port: "9000".to_string(),
            virtual_ip: "10.0.0.1".to_string(),
            connected: Arc::new(AtomicBool::new(false)),
            shutdown: Arc::new(AtomicBool::new(false)),
            connection_handle: Arc::new(Mutex::new(None)),
            logs: Arc::new(Mutex::new(Vec::new())),
            ..Default::default()
        }
    }

    pub fn log(&self, message: String) {
        let mut logs = self.logs.lock().unwrap();

        logs.push(format!("> {}", message));
        if logs.len() > 150 {
            logs.remove(0);
        }
    }
}
