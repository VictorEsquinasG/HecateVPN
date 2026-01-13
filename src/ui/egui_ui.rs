use eframe::App;
use local_ip_address::local_ip;
use std::net::SocketAddr;
use tokio::runtime::Runtime;

use crate::app::AppState;
use crate::network::node::NetworkNode;
use crate::packet::Packet;

pub struct EguiApp {
    pub state: AppState,
}

impl App for EguiApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use eframe::egui;

        let peer_ip_for_thread = self.state.peer_ip.clone();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Mini LAN Bridge");

            /* CONTEXT INFO */
            ui.horizontal(|ui| {
                ui.label("My IP:");
                ui.label(&self.state.my_ip);
            });

            /* DATA FORM */
            ui.horizontal(|ui| {
                // IP
                ui.label("Peer IP:");
                ui.text_edit_singleline(&mut self.state.peer_ip);
                // PORT
                ui.label("Port:");
                ui.text_edit_singleline(&mut self.state.peer_port);
            });

            /* BUTTONS  [CONECT, EXIT] */
            if ui.button("Connect").clicked() && !self.state.connected {
                self.state.connected = true;
                let logs_clone = self.state.logs.clone();
                let peer_ip = self.state.peer_ip.trim().to_string();
                let peer_port = if self.state.peer_port.trim().is_empty() {
                    9000
                } else {
                    self.state.peer_port.trim().parse::<u16>().unwrap_or(9000)
                };

                // All in a thread
                std::thread::spawn(move || {
                    let rt = Runtime::new().unwrap();
                    rt.block_on(async move {
                        let my_ip = local_ip().unwrap();

                        let bind_addr =
                            format!("{}:{}", my_ip, 9000).parse::<SocketAddr>().unwrap();
                        let peer_addr = format!("{}:{}", peer_ip, peer_port).parse::<SocketAddr>();

                        let peer_addr = match peer_addr {
                            Ok(addr) => addr,
                            Err(e) => {
                                logs_clone
                                    .lock()
                                    .unwrap()
                                    .push(format!("Invalid peer IP/port: {} ({})", peer_ip, e));
                                return;
                            }
                        };

                        let node: NetworkNode = match NetworkNode::new(bind_addr, peer_addr).await {
                            Ok(n) => n,
                            Err(e) => {
                                logs_clone.lock().unwrap().push(format!("Network error: {e}"));
                                return;
                            }
                        };

                        logs_clone
                            .lock()
                            .unwrap()
                            .push(format!("Connected to {}", peer_ip_for_thread));

                        let node_clone = node.clone();
                        let logs_clone2 = logs_clone.clone();
                        tokio::spawn(async move {
                            if let Err(e) = node_clone.receive_loop().await {
                                logs_clone2
                                    .lock()
                                    .unwrap()
                                    .push(format!("Receive loop error: {}", e));
                            } else {
                                logs_clone2
                                    .lock()
                                    .unwrap()
                                    .push("Receive loop ended".to_string());
                            }
                        });

                        loop {
                            let packet = Packet {
                                id: rand::random(),
                                payload: b"Hello from Mini LAN Bridge".to_vec(),
                            };
                            if let Err(e) = node.send(&packet).await {
                                logs_clone
                                    .lock()
                                    .unwrap()
                                    .push(format!("Send error: {}", e));
                            } else {
                                logs_clone
                                    .lock()
                                    .unwrap()
                                    .push(format!("Sent packet id={}", packet.id));
                            }
                            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        }
                    });
                });
            }

            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }

            /* LOGS */
            ui.separator();
            ui.label("Logs:");
            // Aquí no movemos nada, self sigue disponible
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                for log in self.state.logs.lock().unwrap().iter() {
                    ui.label(log);
                }
            });
        });
    }
}
