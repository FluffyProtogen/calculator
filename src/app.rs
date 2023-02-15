pub struct Calculator {
    degrees: bool,
    inverse: bool,
}

use eframe::{glow::TIME_ELAPSED, *};
use egui::{style::Spacing, text::LayoutJob, *};

const FUNCTION_COLOR: Color32 = Color32::from_rgb(218, 220, 224);
const NUMBER_COLOR: Color32 = Color32::from_rgb(233, 235, 236);
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 45.0;
const FONT_SIZE: f32 = 23.0;
const ROUNDING: Rounding = {
    let rounding = 6.5;
    Rounding {
        nw: rounding,
        ne: rounding,
        sw: rounding,
        se: rounding,
    }
};

impl App for Calculator {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top panel")
            .show_separator_line(false)
            .show(ctx, |ui| {
                egui::containers::Frame::none()
                    .stroke(Stroke::new(2.0, Color32::from_rgb(218, 220, 224)))
                    .rounding(ROUNDING)
                    .inner_margin(Margin {
                        left: 15.0,
                        right: 15.0,
                        top: 15.0,
                        bottom: 15.0,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            ui.label(RichText::new("(9 + 10) / 69 + 420").size(50.0));
                        });
                    });
            });
        CentralPanel::default().show(ctx, |ui| {
            self.buttons(ui);
        });
    }
}

impl Calculator {
    pub fn new(cc: &CreationContext) -> Self {
        let ctx = &cc.egui_ctx;
        ctx.set_visuals(Visuals::light());

        let fonts = {
            let mut fonts = FontDefinitions::default();
            fonts.font_data.insert(
                "roboto".into(),
                FontData::from_static(include_bytes!("..\\assets\\Roboto-Regular.ttf")),
            );
            fonts
                .families
                .insert(FontFamily::Name("roboto".into()), vec!["roboto".into()]);
            fonts
        };
        ctx.set_fonts(fonts);

        let style = {
            let mut style = (*ctx.style()).clone();
            let font_id = FontId::new(FONT_SIZE, FontFamily::Name("roboto".into()));
            style.text_styles = [
                (TextStyle::Button, font_id.clone()),
                (TextStyle::Body, font_id),
            ]
            .into();
            style
        };
        ctx.set_style(style);

        Self {
            degrees: true,
            inverse: false,
        }
    }

    fn buttons(&mut self, ui: &mut Ui) {
        Grid::new("buttons").spacing(vec2(7.5, 7.5)).show(ui, |ui| {
            self.rad_deg_buttons(ui);
            calculator_button("x!", FUNCTION_COLOR).ui(ui);
            calculator_button("(", FUNCTION_COLOR).ui(ui);
            calculator_button(")", FUNCTION_COLOR).ui(ui);
            calculator_button("%", FUNCTION_COLOR).ui(ui);
            calculator_button("AC", FUNCTION_COLOR).ui(ui);
            ui.end_row();

            if calculator_button(
                "Inv",
                if self.inverse {
                    NUMBER_COLOR
                } else {
                    FUNCTION_COLOR
                },
            )
            .ui(ui)
            .clicked()
            {
                self.inverse = !self.inverse;
            }

            if self.inverse {
                Button::new(superscript(ui, "sin", "-1"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("sin", FUNCTION_COLOR).ui(ui);
            }

            if self.inverse {
                Button::new(superscript(ui, "e", "x"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("ln", FUNCTION_COLOR).ui(ui);
            }
            calculator_button("7", NUMBER_COLOR).ui(ui);
            calculator_button("8", NUMBER_COLOR).ui(ui);
            calculator_button("9", NUMBER_COLOR).ui(ui);
            calculator_button("÷", FUNCTION_COLOR).ui(ui);
            ui.end_row();
            calculator_button("π", FUNCTION_COLOR).ui(ui);

            if self.inverse {
                Button::new(superscript(ui, "cos", "-1"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("cos", FUNCTION_COLOR).ui(ui);
            }
            if self.inverse {
                Button::new(superscript(ui, "x", "10"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("log", FUNCTION_COLOR).ui(ui);
            }
            calculator_button("4", NUMBER_COLOR).ui(ui);
            calculator_button("5", NUMBER_COLOR).ui(ui);
            calculator_button("6", NUMBER_COLOR).ui(ui);
            calculator_button("×", FUNCTION_COLOR).ui(ui);
            ui.end_row();
            calculator_button("e", FUNCTION_COLOR).ui(ui);
            if self.inverse {
                Button::new(superscript(ui, "tan", "-1"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("tan", FUNCTION_COLOR).ui(ui);
            }

            if self.inverse {
                Button::new(superscript(ui, "x", "2"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            } else {
                calculator_button("√", FUNCTION_COLOR).ui(ui);
            }
            calculator_button("1", NUMBER_COLOR).ui(ui);
            calculator_button("2", NUMBER_COLOR).ui(ui);
            calculator_button("3", NUMBER_COLOR).ui(ui);
            calculator_button("–", FUNCTION_COLOR).ui(ui);
            ui.end_row();
            if self.inverse {
                calculator_button("Rnd", FUNCTION_COLOR).ui(ui);
            } else {
                calculator_button("Ans", FUNCTION_COLOR).ui(ui);
            }
            calculator_button("EXP", FUNCTION_COLOR).ui(ui);

            if self.inverse {
                Button::new({
                    let mut job = LayoutJob::default();
                    job.append(
                        "y",
                        1.0,
                        TextFormat {
                            font_id: FontId::new(12.0, FontFamily::Name("roboto".into())),
                            valign: Align::TOP,
                            color: ui.visuals().text_color(),
                            ..Default::default()
                        },
                    );
                    job.append(
                        "√x",
                        0.0,
                        TextFormat {
                            font_id: FontId::new(FONT_SIZE, FontFamily::Name("roboto".into())),
                            valign: Align::TOP,
                            color: ui.visuals().text_color(),
                            ..Default::default()
                        },
                    );

                    job
                })
                .fill(FUNCTION_COLOR)
                .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                .rounding(ROUNDING)
                .ui(ui);
            } else {
                Button::new(superscript(ui, "x", "y"))
                    .fill(FUNCTION_COLOR)
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui);
            }

            calculator_button("0", NUMBER_COLOR).ui(ui);
            calculator_button(".", NUMBER_COLOR).ui(ui);

            Button::new(RichText::new("=").size(FONT_SIZE).color(Color32::WHITE))
                .fill(Color32::from_rgb(66, 133, 244))
                .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                .rounding(ROUNDING)
                .ui(ui);

            calculator_button("+", FUNCTION_COLOR).ui(ui);
        });
    }
    fn rad_deg_buttons(&mut self, ui: &mut Ui) {
        let disabled_color = Color32::from_rgb(135, 136, 140);
        if self.degrees {
            calculator_button("Deg", FUNCTION_COLOR).ui(ui);
            if Button::new(RichText::new("Rad").size(FONT_SIZE).color(disabled_color))
                .fill(FUNCTION_COLOR)
                .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                .rounding(ROUNDING)
                .ui(ui)
                .clicked()
            {
                self.degrees = false;
            }
        } else {
            if Button::new(RichText::new("Deg").size(FONT_SIZE).color(disabled_color))
                .fill(FUNCTION_COLOR)
                .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                .rounding(ROUNDING)
                .ui(ui)
                .clicked()
            {
                self.degrees = true;
            }
            calculator_button("Rad", FUNCTION_COLOR).ui(ui);
        }
    }
}

fn calculator_button(text: &str, color: Color32) -> Button {
    Button::new(RichText::new(text).size(FONT_SIZE))
        .fill(color)
        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
        .rounding(ROUNDING)
}

fn superscript(ui: &Ui, text: &str, superscript_text: &str) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.append(
        text,
        0.0,
        TextFormat {
            font_id: FontId::new(FONT_SIZE, FontFamily::Name("roboto".into())),
            valign: Align::TOP,
            color: ui.visuals().text_color(),
            ..Default::default()
        },
    );
    job.append(
        superscript_text,
        1.0,
        TextFormat {
            font_id: FontId::new(12.0, FontFamily::Name("roboto".into())),
            valign: Align::TOP,
            color: ui.visuals().text_color(),
            ..Default::default()
        },
    );
    job
}
