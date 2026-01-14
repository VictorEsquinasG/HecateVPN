mod app;
mod network;
mod packet;
mod ui;
mod config;

use local_ip_address::local_ip;
use ui::egui_ui::EguiApp;
use egui::IconData;


fn load_icon() -> IconData {
    let (icon_rgba, icon_width, icon_height) = {
        // Nota: include_bytes! es relativo al archivo actual (main.rs)
        // main.rs está en src/, así que subimos uno (..) y entramos a assets/
        let image = image::load_from_memory(include_bytes!("../assets/icon.png"))
            .expect("No se pudo cargar el icono (assets/icon.png)")
            .into_rgba8();
        let (width, height) = image.dimensions();
        (image.into_raw(), width, height)
    };

    IconData {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    }
}

fn main() -> eframe::Result<()> {
    let my_ip = local_ip().unwrap_or_else(|_| "0.0.0.0".parse().unwrap());

    let app = EguiApp {
        state: app::AppState::new(my_ip.to_string()),
        power_on_texture: None,
        power_off_texture: None,
    };

  let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("HecateVPN")
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Mini LAN Bridge",
        native_options,
        Box::new(|_| Box::new(app)),
    )
}
