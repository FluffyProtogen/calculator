pub mod app;
pub mod calculator;
use eframe::*;
use egui::vec2;
//ONCE YOU HIT ENTER, HITTING ANY OPERATION WILL ADD TO THE ANSWER. TYPING ANY NUMBER OVERRIDES THE ANSWER
//FOR AUTOMATIC EXPONENTS LIKE E^2, MAKE TRY_PUSH RETURN A BOOL. IT ONLY WILL ADD ANOTHER NUMBER IF IT SUCCESSFULLY PUSHES.
//TRY DIFFERENT BASIC OPERATIONS SWAPPING OUT EACH OTHER. ALSO MINUS
//A POWER ENDS AFTER TYPING THE FINAL CLOSING PARENTHESIS OR TYPING ANY NON-NUMBER OPERATION
//END PARENTHESIS NEEDS TO FIND POWER LEVEL TOO - use open_parenthes
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
