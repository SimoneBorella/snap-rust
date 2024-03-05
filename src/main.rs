pub mod app;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size = Some(eframe::egui::Vec2::new(750., 500.));

    eframe::run_native(
        "SnapRust",
        native_options,
        Box::new(|cc| Box::new(app::SnapRustApp::new(cc))),
    )
    .unwrap();
}