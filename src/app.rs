use eframe::*;
use egui::{text::LayoutJob, *};
use egui_extras::RetainedImage;

use crate::calculator::Equation;
use crate::calculator::Item::*;
pub struct Calculator {
    history_icon: RetainedImage,
    degrees: bool,
    inverse: bool,
    equation: Equation,
}

const FUNCTION_COLOR: Color32 = Color32::from_rgb(218, 220, 224);
const NUMBER_COLOR: Color32 = Color32::from_rgb(233, 235, 236);
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 45.0;
const FONT_SIZE: f32 = 23.0;
const GRID_SPACING: f32 = 7.5;
const EQUATION_SIZE: f32 = 33.0;
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
                        top: 40.0,
                        bottom: 10.0,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            ui.label(
                                self.equation
                                    .render(EQUATION_SIZE, ui.visuals().text_color()),
                            );
                        });
                    });
            });
        CentralPanel::default().show(ctx, |ui| {
            self.buttons(ui);
        });
        Area::new("history")
            .fixed_pos(pos2(17.0, 12.0))
            .show(ctx, |ui| {
                if ImageButton::new(
                    self.history_icon.texture_id(ctx),
                    self.history_icon.size_vec2(),
                )
                .frame(false)
                .ui(ui)
                .clicked()
                {
                    println!("F");
                }
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
            history_icon: RetainedImage::from_svg_bytes(
                "history icon",
                include_bytes!("..\\assets\\History Icon.svg"),
            )
            .unwrap(),
            equation: Equation::new(),
        }
    }

    fn buttons(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(GRID_SPACING, GRID_SPACING);
            ui.horizontal(|ui| {
                self.rad_deg_buttons(ui);
                if calculator_button("x!", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Factorial);
                }
                if calculator_button("(", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(OpeningParenthesis);
                }
                if calculator_button(")", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(ClosingParenthesis);
                }
                if calculator_button("%", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Modulus);
                }
                calculator_button("AC", FUNCTION_COLOR).ui(ui);
            });

            ui.horizontal(|ui| {
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
                    if Button::new(superscript(ui, "sin", "-1"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        self.equation.try_push(Asin);
                    };
                } else {
                    if calculator_button("sin", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Sin);
                    }
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
            });
            ui.horizontal(|ui| {
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
            });
            ui.horizontal(|ui| {
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
            });
            ui.horizontal(|ui| {
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
        });
    }
    fn rad_deg_buttons(&mut self, ui: &mut Ui) {
        let disabled_color = Color32::from_rgb(135, 136, 140);
        let job = {
            let mut job = LayoutJob::default();
            job.append(
                "Rad",
                0.0,
                TextFormat {
                    font_id: FontId::new(FONT_SIZE, FontFamily::Name("roboto".into())),
                    valign: Align::TOP,
                    color: if self.degrees {
                        disabled_color
                    } else {
                        ui.visuals().text_color()
                    },
                    ..Default::default()
                },
            );
            job.append(
                "Deg",
                60.0,
                TextFormat {
                    font_id: FontId::new(FONT_SIZE, FontFamily::Name("roboto".into())),
                    valign: Align::TOP,
                    color: if self.degrees {
                        ui.visuals().text_color()
                    } else {
                        disabled_color
                    },
                    ..Default::default()
                },
            );
            job
        };
        let response = Button::new(job)
            .fill(FUNCTION_COLOR)
            .min_size(vec2(BUTTON_WIDTH * 2.0 + GRID_SPACING, BUTTON_HEIGHT))
            .rounding(ROUNDING)
            .ui(ui);
        let painter = ui.painter_at(response.rect);
        painter.rect(
            Rect {
                min: response.rect.min + vec2(BUTTON_WIDTH + 2.5, 10.0),
                max: response.rect.max - vec2(BUTTON_WIDTH + 2.5, 10.0),
            },
            Rounding::none(),
            Color32::from_rgb(165, 168, 173),
            Stroke::NONE,
        );

        if response.clicked() {
            self.degrees = !self.degrees;
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
