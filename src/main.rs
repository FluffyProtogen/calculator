pub mod app;
pub mod calculator;
use eframe::*;
use egui::vec2;

//MAKE IT ONLY RENDER WHEN YOU DO SOMETHING NEW. IT SHOULD CACHE THE RENDERED LAYOUT JOB FOR PERFORMANCE

fn main() {
    let options = NativeOptions {
        initial_window_size: Some(vec2(760.0, 364.0)),
        resizable: false,
        transparent: true,
        ..Default::default()
    };

    run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(app::Calculator::new(cc))),
    )
    .unwrap();
}
