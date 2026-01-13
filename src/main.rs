mod app;
mod network;
mod packet;
mod ui;
mod config;

use local_ip_address::local_ip;
use ui::egui_ui::EguiApp;

fn main() -> eframe::Result<()> {
    let my_ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    let app = EguiApp {
        state: app::AppState::new(my_ip.to_string()),
    };

    eframe::run_native(
        "Mini LAN Bridge",
        eframe::NativeOptions::default(),
        Box::new(|_| Box::new(app)),
    )
}


// use crate::network::NetworkNode;
// use crate::packet::Packet;
// use eframe::App;
// use local_ip_address::local_ip;
// use std::net::SocketAddr;
// use std::sync::{Arc, Mutex};
// use tokio::runtime::Runtime;

// mod config;
// mod network;
// mod packet;

// #[derive(Default)]
// struct AppState {
//     my_ip: String,
//     peer_ip: String,
//     peer_port: String, // optional
//     logs: Arc<Mutex<Vec<String>>>,
//     connected: bool,
// }

// impl AppState {
//     fn log(&self, message: String) {
//         let mut logs = self.logs.lock().unwrap();
//         logs.push(message);
//         if logs.len() > 50 {
//             logs.remove(0);
//         }
//     }
// }

// impl App for AppState {
//     fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
//         use eframe::egui;

//         let peer_ip_for_thread = self.peer_ip.clone();

//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("Mini LAN Bridge");

//             /* CONTEXT INFO */
//             ui.horizontal(|ui| {
//                 ui.label("My IP:");
//                 ui.label(&self.my_ip);
//             });

//             /* DATA FORM */
//             ui.horizontal(|ui| {
//                 // IP
//                 ui.label("Peer IP:");
//                 ui.text_edit_singleline(&mut self.peer_ip);
//                 // PORT
//                 ui.label("Port:");
//                 ui.text_edit_singleline(&mut self.peer_port);
//             });

//             /* BUTTONS  [CONECT, EXIT] */
//             if ui.button("Connect").clicked() && !self.connected {
//                 self.connected = true;
//                 let logs_clone = self.logs.clone();
//                 let peer_ip = self.peer_ip.trim().to_string();
//                 let peer_port = if self.peer_port.trim().is_empty() {
//                     9000
//                 } else {
//                     self.peer_port.trim().parse::<u16>().unwrap_or(9000)
//                 };

//                 // All in a thread
//                 std::thread::spawn(move || {
//                     let rt = Runtime::new().unwrap();
//                     rt.block_on(async move {
//                         let my_ip = local_ip().unwrap();

//                         let bind_addr =
//                             format!("{}:{}", my_ip, 9000).parse::<SocketAddr>().unwrap();
//                         let peer_addr = format!("{}:{}", peer_ip, peer_port).parse::<SocketAddr>();

//                         let peer_addr = match peer_addr {
//                             Ok(addr) => addr,
//                             Err(e) => {
//                                 logs_clone
//                                     .lock()
//                                     .unwrap()
//                                     .push(format!("Invalid peer IP/port: {} ({})", peer_ip, e));
//                                 return;
//                             }
//                         };

//                         let node = match NetworkNode::new(bind_addr, peer_addr).await {
//                             Ok(n) => n,
//                             Err(e) => {
//                                 logs_clone
//                                     .lock()
//                                     .unwrap()
//                                     .push(format!("Failed to create node: {}", e));
//                                 return;
//                             }
//                         };

//                         logs_clone
//                             .lock()
//                             .unwrap()
//                             .push(format!("Connected to {}", peer_ip_for_thread));

//                         let node_clone = node.clone();
//                         let logs_clone2 = logs_clone.clone();
//                         tokio::spawn(async move {
//                             if let Err(e) = node_clone.receive_loop().await {
//                                 logs_clone2
//                                     .lock()
//                                     .unwrap()
//                                     .push(format!("Receive loop error: {}", e));
//                             } else {
//                                 logs_clone2
//                                     .lock()
//                                     .unwrap()
//                                     .push("Receive loop ended".to_string());
//                             }
//                         });

//                         loop {
//                             let packet = Packet {
//                                 id: rand::random(),
//                                 payload: b"Hello from Mini LAN Bridge".to_vec(),
//                             };
//                             if let Err(e) = node.send(&packet).await {
//                                 logs_clone
//                                     .lock()
//                                     .unwrap()
//                                     .push(format!("Send error: {}", e));
//                             } else {
//                                 logs_clone
//                                     .lock()
//                                     .unwrap()
//                                     .push(format!("Sent packet id={}", packet.id));
//                             }
//                             tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
//                         }
//                     });
//                 });
//             }

//             if ui.button("Exit").clicked() {
//                 std::process::exit(0);
//             }

//             /* LOGS */
//             ui.separator();
//             ui.label("Logs:");
//             // Aquí no movemos nada, self sigue disponible
//             eframe::egui::ScrollArea::vertical().show(ui, |ui| {
//                 for log in self.logs.lock().unwrap().iter() {
//                     ui.label(log);
//                 }
//             });
//         });
//     }
// }

// fn main() -> eframe::Result<()> {
//     let my_ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

//     let app = AppState {
//         my_ip: my_ip.to_string(),
//         ..Default::default()
//     };

//     let native_options = eframe::NativeOptions {
//         ..Default::default()
//     };

//     eframe::run_native(
//         "Mini LAN Bridge", // window title
//         native_options,
//         Box::new(|_cc| Box::new(app)),
//     )
// }
