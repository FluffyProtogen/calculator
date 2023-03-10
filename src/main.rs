pub mod app;
pub mod calculator;
pub mod calculator_button;
pub mod solver;
use eframe::*;
use egui::vec2;

pub mod tests;
fn main() {
    let options = NativeOptions {
        initial_window_size: Some(vec2(760.5, 399.0)),
        transparent: true,
        decorated: false,
        ..Default::default()
    };

    run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(app::Calculator::new(cc))),
    )
    .unwrap();
}
