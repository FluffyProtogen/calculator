use calculator::*;
use eframe::*;
use egui::vec2;

fn main() {
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(vec2(300.0, 425.0));
    options.transparent = true;
    run_native(
        "Calculator",
        options,
        Box::new(|cc| Box::new(app::Calculator::new(cc))),
    )
    .unwrap();
}
