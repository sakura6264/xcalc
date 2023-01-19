#![windows_subsystem = "windows"]
use eframe;
use eframe::IconData;
mod tools;

fn main() {
    let ico = image::load_from_memory(include_bytes!("../assets/icon.png")).unwrap().to_rgba8();
    
    let option = eframe::NativeOptions{
        initial_window_size: Some(eframe::egui::Vec2::new(1000.0, 650.0)),
        icon_data: Some(IconData{
            rgba: ico.into_raw(),
            width:24,
            height: 24,
        }),
        ..Default::default()
    };
    eframe::run_native("XCalc",option,Box::new(|_cc|Box::new(tools::app::App::new())));
}
