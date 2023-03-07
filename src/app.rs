use eframe::*;
use egui::{text::LayoutJob, *};
use egui_extras::RetainedImage;
use rand::Rng;

use crate::calculator::Equation;
use crate::calculator::Item::*;
use crate::solver;
pub struct Calculator {
    history_icon: RetainedImage,
    degrees: bool,
    inverse: bool,
    equation: Equation,
    history: Vec<(Equation, f64)>,
    previous_answer_state: PreviousAnswerState,
}

#[derive(PartialEq, Debug)]
enum PreviousAnswerState {
    Show,
    Hide,
    Error(Equation),
}

const FUNCTION_COLOR: Color32 = Color32::from_rgb(218, 220, 224);
const NUMBER_COLOR: Color32 = Color32::from_rgb(233, 235, 236);
const PREVIOUS_COLOR: Color32 = Color32::from_rgb(112, 117, 122);
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 45.0;
const FONT_SIZE: f32 = 23.0;
const GRID_SPACING: f32 = 7.5;
const EQUATION_SIZE: f32 = 39.0;
const PREVIOUS_SIZE: f32 = 22.0;
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
        self.handle_key_presses(ctx);
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
                        self.show_current(ui);
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
        self.show_previous(ctx);
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
            fonts.font_data.insert(
                "arial".into(),
                FontData::from_static(include_bytes!("..\\assets\\arial.ttf")),
            );
            fonts
                .families
                .insert(FontFamily::Name("roboto".into()), vec!["roboto".into()]);
            fonts
                .families
                .insert(FontFamily::Name("arial".into()), vec!["arial".into()]);
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
            history: vec![],
            previous_answer_state: PreviousAnswerState::Hide,
        }
    }

    fn handle_key_presses(&mut self, ctx: &Context) {
        use Key::*;
        if ctx.input(|i| i.key_pressed(Num1)) {
            self.equation.try_push(Number("1".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num2)) {
            self.equation.try_push(Number("2".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num3)) {
            self.equation.try_push(Number("3".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num4)) {
            self.equation.try_push(Number("4".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num5)) {
            self.equation.try_push(Number("5".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num6)) {
            self.equation.try_push(Number("6".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num7)) {
            self.equation.try_push(Number("7".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num8)) {
            self.equation.try_push(Number("8".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num9)) {
            self.equation.try_push(Number("9".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Num0)) {
            self.equation.try_push(Number("0".into()));
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Backspace)) {
            self.equation.backspace();
            self.previous_answer_state = PreviousAnswerState::Hide;
        }
        if ctx.input(|i| i.key_pressed(Enter)) {
            self.solve();
        }
    }

    fn buttons(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(GRID_SPACING, GRID_SPACING);
            ui.horizontal(|ui| {
                self.rad_deg_buttons(ui);
                if calculator_button("x!", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Factorial);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("(", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(OpeningParenthesis);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button(")", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(ClosingParenthesis);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("%", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Percent);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("AC", FUNCTION_COLOR).ui(ui).clicked() {
                    todo!();
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
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
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    };
                } else {
                    if calculator_button("sin", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Sin);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if self.inverse {
                    if Button::new(superscript(ui, "e", "x"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        if self.equation.try_push(E) {
                            self.equation.try_push(Power);
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("ln", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Ln);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if calculator_button("7", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("7".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("8", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("8".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("9", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("9".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("÷", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Divide);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if calculator_button("π", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Pi);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if self.inverse {
                    if Button::new(superscript(ui, "cos", "-1"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        self.equation.try_push(Acos);
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("cos", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Cos);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if self.inverse {
                    if Button::new(superscript(ui, "x", "10"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        if self.equation.try_push(Power) {
                            self.equation.try_push(Number("10".into()));
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("log", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Log);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if calculator_button("4", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("4".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("5", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("5".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("6", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("6".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("×", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Multiply);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if calculator_button("e", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(E);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if self.inverse {
                    if Button::new(superscript(ui, "tan", "-1"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        self.equation.try_push(Atan);
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("tan", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Tan);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if self.inverse {
                    if Button::new(superscript(ui, "x", "2"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        if self.equation.try_push(Power) {
                            self.equation.try_push(Number("2".into()));
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("√", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Sqrt);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if calculator_button("1", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("1".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("2", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("2".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("3", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("3".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button("–", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Subtract);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if self.inverse {
                    if calculator_button("Rnd", FUNCTION_COLOR).ui(ui).clicked() {
                        let random = rand::thread_rng().gen::<f64>().to_string();
                        self.equation.try_push(Rnd(format!("{random:.7}")));
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if calculator_button("Ans", FUNCTION_COLOR).ui(ui).clicked() {
                        self.equation.try_push(Ans);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if calculator_button("EXP", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(EXP);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if self.inverse {
                    if Button::new({
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
                    .ui(ui)
                    .clicked()
                    {
                        todo!();
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if Button::new(superscript(ui, "x", "y"))
                        .fill(FUNCTION_COLOR)
                        .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                        .rounding(ROUNDING)
                        .ui(ui)
                        .clicked()
                    {
                        self.equation.try_push(Power);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if calculator_button("0", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number("0".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if calculator_button(".", NUMBER_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Number(".".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if Button::new(RichText::new("=").size(FONT_SIZE).color(Color32::WHITE))
                    .fill(Color32::from_rgb(66, 133, 244))
                    .min_size(vec2(BUTTON_WIDTH, BUTTON_HEIGHT))
                    .rounding(ROUNDING)
                    .ui(ui)
                    .clicked()
                {
                    self.solve();
                }

                if calculator_button("+", FUNCTION_COLOR).ui(ui).clicked() {
                    self.equation.try_push(Add);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
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

    fn solve(&mut self) {
        if self.previous_answer_state != PreviousAnswerState::Show {
            let answer = solver::solve(
                &self.equation,
                self.degrees,
                self.history.last().map(|history| history.1).unwrap_or(0.0),
            );

            for _ in 0..self.equation.open_parentheses_count() {
                self.equation.try_push(ClosingParenthesis);
            }

            let equation = std::mem::replace(&mut self.equation, Equation::new());

            if let Some(answer) = answer {
                self.previous_answer_state = PreviousAnswerState::Show;
                if let Some(last) = self.history.last() {
                    if last.0 != equation {
                        self.history.push((equation, answer));
                    }
                } else {
                    self.history.push((equation, answer));
                }
            } else {
                self.previous_answer_state = PreviousAnswerState::Error(equation);
            }
        }
    }
    fn show_current(&self, ui: &mut Ui) {
        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
            match &self.previous_answer_state {
                PreviousAnswerState::Show => {
                    ui.label(
                        RichText::new(self.history.last().unwrap().1.to_string())
                            .size(EQUATION_SIZE),
                    );
                }
                PreviousAnswerState::Hide => {
                    ui.label(
                        self.equation
                            .render(EQUATION_SIZE, ui.visuals().text_color())
                            .clone(),
                    );
                }
                PreviousAnswerState::Error(equation) => {
                    ui.label(RichText::new("Error").size(EQUATION_SIZE));
                }
            }
        });
    }

    fn show_previous(&self, ctx: &Context) {
        Area::new("previous answer")
            .fixed_pos(pos2(0.0, 12.0))
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.add_space(22.0);
                    match &self.previous_answer_state {
                        PreviousAnswerState::Show => {
                            let mut render = self
                                .history
                                .last()
                                .unwrap()
                                .0
                                .render(PREVIOUS_SIZE, PREVIOUS_COLOR);
                            render.append(
                                " =",
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(
                                        PREVIOUS_SIZE,
                                        FontFamily::Name("roboto".into()),
                                    ),
                                    color: PREVIOUS_COLOR,
                                    ..Default::default()
                                },
                            );
                            ui.label(render);
                        }
                        PreviousAnswerState::Hide => {
                            if let Some(last) = self.history.last() {
                                ui.label(
                                    RichText::new(format!("Ans = {}", last.1.to_string()))
                                        .size(PREVIOUS_SIZE)
                                        .color(PREVIOUS_COLOR),
                                );
                            }
                        }
                        PreviousAnswerState::Error(equation) => {
                            let mut render = equation.render(PREVIOUS_SIZE, PREVIOUS_COLOR);
                            render.append(
                                " =",
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(
                                        PREVIOUS_SIZE,
                                        FontFamily::Name("roboto".into()),
                                    ),
                                    color: PREVIOUS_COLOR,
                                    ..Default::default()
                                },
                            );
                            ui.label(render);
                        }
                    }
                });
            });
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
