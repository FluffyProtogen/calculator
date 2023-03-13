use eframe::epaint::Shadow;
use eframe::*;
use egui::{text::LayoutJob, *};
use egui_extras::RetainedImage;
use rand::Rng;

use crate::calculator::Equation;
use crate::calculator::Item::*;
use crate::calculator_button::CalculatorButton;
use crate::solver;
pub struct Calculator {
    history_icon: RetainedImage,
    degrees: bool,
    inverse: bool,
    equation: Equation,
    history: Vec<(Equation, f64)>,
    previous_answer_state: PreviousAnswerState,
    animation_time: Option<f32>,
    show_history_menu: bool,
}

#[derive(PartialEq, Debug)]
enum PreviousAnswerState {
    Show,
    Hide,
    Error(Equation),
}

pub const FUNCTION_COLOR: Color32 = Color32::from_rgb(218, 220, 224);
pub const NUMBER_COLOR: Color32 = Color32::from_rgb(233, 235, 236);
pub const PREVIOUS_COLOR: Color32 = Color32::from_rgb(112, 117, 122);
pub const BUTTON_WIDTH: f32 = 100.0;
pub const BUTTON_HEIGHT: f32 = 45.0;
pub const FONT_SIZE: f32 = 23.0;
pub const GRID_SPACING: f32 = 7.5;
pub const EQUATION_SIZE: f32 = 39.0;
pub const PREVIOUS_SIZE: f32 = 22.0;
pub const TITLE_BAR_HEIGHT: f32 = 60.0;

pub const ANIMATION_DURATION: f32 = 0.14;

pub const ROUNDING: Rounding = {
    let rounding = 6.5;
    Rounding {
        nw: rounding,
        ne: rounding,
        sw: rounding,
        se: rounding,
    }
};

impl App for Calculator {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }

    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.handle_key_presses(ctx);
        TopBottomPanel::top("top panel")
            .frame(
                egui::containers::Frame::none()
                    .fill(ctx.style().visuals.window_fill)
                    .rounding(Rounding {
                        nw: 6.5,
                        ne: 6.5,
                        sw: 0.0,
                        se: 0.0,
                    })
                    .inner_margin(Margin {
                        left: 10.0,
                        right: 10.0,
                        top: 0.0,
                        bottom: 5.0,
                    }),
            )
            .show_separator_line(false)
            .show(ctx, |ui| {
                let app_rect = ui.max_rect();

                let title_bar_rect = {
                    let mut rect = app_rect;
                    rect.max.y = rect.min.y + 20.0;
                    rect
                };
                Self::title_bar_ui(ctx, ui, frame, title_bar_rect);

                egui::containers::Frame::none()
                    .stroke(Stroke::new(2.0, FUNCTION_COLOR))
                    .rounding(ROUNDING)
                    .inner_margin(Margin {
                        left: 15.0,
                        right: 15.0,
                        top: 40.0,
                        bottom: 10.0,
                    })
                    .show(ui, |ui| {
                        ui.with_layout(Layout::right_to_left(Align::RIGHT), |ui| {
                            ui.label(RichText::new("").size(EQUATION_SIZE));
                        });
                    });
            });
        CentralPanel::default()
            .frame(
                egui::containers::Frame::none()
                    .fill(ctx.style().visuals.window_fill)
                    .rounding(Rounding {
                        nw: 0.0,
                        ne: 0.0,
                        sw: 6.5,
                        se: 6.5,
                    })
                    .inner_margin(Margin {
                        left: 7.8,
                        right: 0.0,
                        top: 5.0,
                        bottom: 0.0,
                    }),
            )
            .show(ctx, |ui| {
                self.buttons(ui);
            });
        self.show_history(ctx);
        self.show_previous(ctx);
        self.show_current(ctx);

        if let Some(time) = &mut self.animation_time {
            if *time < ANIMATION_DURATION {
                *time += ctx.input(|i| i.stable_dt);
                ctx.request_repaint();
            } else {
                self.animation_time = None;
            }
        }
    }
}

impl Calculator {
    fn title_bar_ui(
        ctx: &Context,
        ui: &mut egui::Ui,
        frame: &mut eframe::Frame,
        title_bar_rect: eframe::epaint::Rect,
    ) {
        let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());

        if title_bar_response.is_pointer_button_down_on() {
            frame.drag_window();
        }
        ui.label("");

        Area::new("close")
            .fixed_pos(pos2(726.0, 0.0))
            .show(ctx, |ui| {
                if Button::new(RichText::new("❌").font(FontId::proportional(25.0)))
                    .frame(false)
                    .ui(ui)
                    .clicked()
                {
                    frame.close();
                }
            });

        Area::new("minimize")
            .fixed_pos(pos2(701.0, -2.0))
            .show(ctx, |ui| {
                if Button::new(RichText::new("–").font(FontId::proportional(30.0)))
                    .frame(false)
                    .ui(ui)
                    .clicked()
                {
                    frame.set_minimized(true);
                }
            });
    }

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
            animation_time: None,
            show_history_menu: false,
        }
    }

    fn handle_key_presses(&mut self, ctx: &Context) {
        let keys = ctx.input(|i| {
            i.raw
                .events
                .iter()
                .filter_map(|item| {
                    if let Event::Text(text) = item {
                        Some(text.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        });

        for key in keys {
            let item = match key.as_str() {
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" | "." => {
                    Number(key.into())
                }
                "+" => {
                    self.try_continue_answer();
                    Add
                }
                "-" => {
                    self.try_continue_answer();
                    Subtract
                }
                "*" => {
                    self.try_continue_answer();
                    Multiply
                }
                "/" => {
                    self.try_continue_answer();
                    Divide
                }
                "!" => Factorial,
                "%" => Percent,
                "^" => Power,
                "(" => OpeningParenthesis,
                ")" => ClosingParenthesis,
                "q" => Sqrt,
                "e" => E,
                "r" => Nroot,
                "t" => {
                    if self.inverse {
                        self.inverse = false;
                        Atan
                    } else {
                        Tan
                    }
                }
                "p" => Pi,
                "a" => Ans,
                "s" => {
                    if self.inverse {
                        self.inverse = false;
                        Asin
                    } else {
                        Sin
                    }
                }
                "g" => Log,
                "l" => Ln,
                "c" => {
                    if self.inverse {
                        self.inverse = false;
                        Acos
                    } else {
                        Cos
                    }
                }
                "i" => {
                    self.inverse = !self.inverse;
                    continue;
                }
                _ => continue,
            };
            self.equation.try_push(item);
            self.previous_answer_state = PreviousAnswerState::Hide;
            self.show_history_menu = false;
        }

        if ctx.input(|i| i.key_pressed(Key::Backspace)) {
            self.equation.backspace();
            self.previous_answer_state = PreviousAnswerState::Hide;
            self.show_history_menu = false;
        }
        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.solve();
            self.show_history_menu = false;
        }
    }

    fn buttons(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = vec2(GRID_SPACING, GRID_SPACING);
            ui.horizontal(|ui| {
                self.rad_deg_buttons(ui);
                if CalculatorButton::new("x!", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Factorial);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("(", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(OpeningParenthesis);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new(")", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(ClosingParenthesis);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("%", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Percent);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if self.previous_answer_state == PreviousAnswerState::Hide {
                    if CalculatorButton::new("CE", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.backspace();
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("AC", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.clear();
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
            });

            ui.horizontal(|ui| {
                if CalculatorButton::new(
                    "Inv",
                    if self.inverse {
                        NUMBER_COLOR
                    } else {
                        FUNCTION_COLOR
                    },
                )
                .ui(ui)
                .clicked_or_drag_ended()
                {
                    self.inverse = !self.inverse;
                }

                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "sin", "-1"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Asin);
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    };
                } else {
                    if CalculatorButton::new("sin", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Sin);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "e", "x"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        if self.equation.try_push(E) {
                            self.equation.try_push(Power);
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("ln", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Ln);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if CalculatorButton::new("7", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("7".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("8", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("8".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("9", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("9".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("÷", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.try_continue_answer();
                    self.equation.try_push(Divide);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if CalculatorButton::new("π", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Pi);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "cos", "-1"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Acos);
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("cos", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Cos);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "x", "10"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        if self.equation.try_push(Power) {
                            self.equation.try_push(Number("10".into()));
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("log", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Log);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if CalculatorButton::new("4", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("4".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("5", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("5".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("6", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("6".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("×", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.try_continue_answer();
                    self.equation.try_push(Multiply);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if CalculatorButton::new("e", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(E);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "tan", "-1"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Atan);
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("tan", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Tan);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if self.inverse {
                    if CalculatorButton::new(superscript(ui, "x", "2"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        if self.equation.try_push(Power) {
                            self.equation.try_push(Number("2".into()));
                        }
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("√", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Sqrt);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if CalculatorButton::new("1", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("1".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("2", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("2".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("3", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("3".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new("–", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.try_continue_answer();
                    self.equation.try_push(Subtract);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
            });
            ui.horizontal(|ui| {
                if self.inverse {
                    if CalculatorButton::new("Rnd", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        let random = rand::thread_rng().gen::<f64>().to_string();
                        self.equation.try_push(Rnd(format!("{random:.7}")));
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new("Ans", FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Ans);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }
                if CalculatorButton::new("EXP", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(EXP);
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if self.inverse {
                    if CalculatorButton::new(
                        {
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
                                    font_id: FontId::new(
                                        FONT_SIZE,
                                        FontFamily::Name("roboto".into()),
                                    ),
                                    valign: Align::TOP,
                                    color: ui.visuals().text_color(),
                                    ..Default::default()
                                },
                            );

                            job
                        },
                        FUNCTION_COLOR,
                    )
                    .ui(ui)
                    .clicked_or_drag_ended()
                    {
                        todo!();
                        self.inverse = false;
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                } else {
                    if CalculatorButton::new(superscript(ui, "x", "y"), FUNCTION_COLOR)
                        .ui(ui)
                        .clicked_or_drag_ended()
                    {
                        self.equation.try_push(Power);
                        self.previous_answer_state = PreviousAnswerState::Hide;
                    }
                }

                if CalculatorButton::new("0", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number("0".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }
                if CalculatorButton::new(".", NUMBER_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.equation.try_push(Number(".".into()));
                    self.previous_answer_state = PreviousAnswerState::Hide;
                }

                if CalculatorButton::new(
                    RichText::new("=").size(FONT_SIZE).color(Color32::WHITE),
                    Color32::from_rgb(66, 133, 244),
                )
                .ui(ui)
                .clicked_or_drag_ended()
                {
                    self.solve();
                }

                if CalculatorButton::new("+", FUNCTION_COLOR)
                    .ui(ui)
                    .clicked_or_drag_ended()
                {
                    self.try_continue_answer();
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
        let response = CalculatorButton::new(job, FUNCTION_COLOR)
            .min_size(vec2(BUTTON_WIDTH * 2.0 + GRID_SPACING, BUTTON_HEIGHT))
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

        if response.clicked_or_drag_ended() {
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
                    if last.0 != equation || last.0.contains_ans() {
                        self.history.push((equation, answer));
                    }
                } else {
                    self.history.push((equation, answer));
                }
            } else {
                self.previous_answer_state = PreviousAnswerState::Error(equation);
            }
        }
        self.animation_time = Some(0.0);
    }
    fn show_current(&self, ctx: &Context) {
        let t = self.animation_time.unwrap_or(ANIMATION_DURATION) / ANIMATION_DURATION;
        let y_position = smoothstep(95.0, 43.0, t) + TITLE_BAR_HEIGHT / 2.0;
        Area::new("current answer")
            .fixed_pos(pos2(0.0, y_position))
            .show(ctx, |ui| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.set_clip_rect(Rect {
                        min: pos2(12.0, TITLE_BAR_HEIGHT / 2.0),
                        max: pos2(ui.max_rect().max.x, 94.5 + TITLE_BAR_HEIGHT / 2.0),
                    });

                    ui.add_space(22.0);
                    match &self.previous_answer_state {
                        PreviousAnswerState::Show => {
                            ui.label(
                                RichText::new(format_number(self.history.last().unwrap().1))
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
            });
    }

    fn show_previous(&self, ctx: &Context) {
        let t = self.animation_time.unwrap_or(ANIMATION_DURATION) / ANIMATION_DURATION;
        let size = smoothstep(EQUATION_SIZE, PREVIOUS_SIZE, t);

        let color = {
            let color = ctx.style().visuals.text_color();
            let r = smoothstep(color.r() as f32, PREVIOUS_COLOR.r() as f32, t) as u8;
            let g = smoothstep(color.g() as f32, PREVIOUS_COLOR.g() as f32, t) as u8;
            let b = smoothstep(color.b() as f32, PREVIOUS_COLOR.b() as f32, t) as u8;
            Color32::from_rgb(r, g, b)
        };

        Area::new("previous answer")
            .fixed_pos(pos2(0.0, 12.0 + TITLE_BAR_HEIGHT / 2.0))
            .show(ctx, |ui| {
                ui.set_clip_rect(Rect {
                    min: pos2(55.0, 0.0 + TITLE_BAR_HEIGHT / 2.0),
                    max: ui.max_rect().max,
                });
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    ui.add_space(22.0);
                    match &self.previous_answer_state {
                        PreviousAnswerState::Show => {
                            let mut render = self.history.last().unwrap().0.render(size, color);
                            render.append(
                                " =",
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(size, FontFamily::Name("roboto".into())),
                                    color,
                                    ..Default::default()
                                },
                            );
                            ui.label(render);
                        }
                        PreviousAnswerState::Hide => {
                            if let Some(last) = self.history.last() {
                                ui.label(
                                    RichText::new(format!("Ans = {}", format_number(last.1)))
                                        .size(size)
                                        .color(color),
                                );
                            }
                        }
                        PreviousAnswerState::Error(equation) => {
                            let mut render = equation.render(size, color);
                            render.append(
                                " =",
                                0.0,
                                TextFormat {
                                    font_id: FontId::new(size, FontFamily::Name("roboto".into())),
                                    color,
                                    ..Default::default()
                                },
                            );
                            ui.label(render);
                        }
                    }
                });
            });
    }

    fn show_history(&mut self, ctx: &Context) {
        let mut just_opened = false;

        Area::new("history button")
            .fixed_pos(pos2(17.0, 12.0 + TITLE_BAR_HEIGHT / 2.0))
            .order(Order::Foreground)
            .interactable(!self.show_history_menu)
            .show(ctx, |ui| {
                let response = ImageButton::new(
                    self.history_icon.texture_id(ctx),
                    self.history_icon.size_vec2(),
                )
                .frame(false)
                .ui(ui);
                if response.clicked_or_drag_ended() {
                    self.show_history_menu = true;
                    just_opened = true;
                }
            });
        if self.show_history_menu {
            Area::new("history")
                .fixed_pos(pos2(7.5, 3.5 + TITLE_BAR_HEIGHT / 2.0))
                .show(ctx, |ui| {
                    egui::containers::Frame::none()
                        .fill(Color32::WHITE)
                        .shadow(Shadow {
                            extrusion: 5.5,
                            color: Color32::from_rgba_premultiplied(0, 0, 0, 40),
                        })
                        .rounding(ROUNDING)
                        .inner_margin(Margin::from(10.0))
                        .show(ui, |ui| {
                            ui.set_max_width(450.0);
                            ui.set_min_height(180.0);
                            if self.history.len() == 0 {
                                ui.allocate_space(vec2(450.0, 30.0));
                                ui.separator();
                            } else {
                                ui.allocate_space(vec2(0.0, 30.0));
                                ui.separator();

                                ScrollArea::vertical().max_width(450.0).show(ui, |ui| {
                                    ui.allocate_space(vec2(ui.available_width(), 14.0));
                                    for (equation, answer) in &self.history {
                                        ui.horizontal(|ui| {
                                            if CalculatorButton::new(
                                                equation.render(
                                                    FONT_SIZE,
                                                    Color32::from_rgb(66, 133, 244),
                                                ),
                                                Color32::TRANSPARENT,
                                            )
                                            .stroke(Stroke::new(1.2, FUNCTION_COLOR))
                                            .min_size(vec2(0.0, BUTTON_HEIGHT))
                                            .padding(vec2(9.0, 0.0))
                                            .max_text_width(200.0)
                                            .hover_fill(Color32::from_rgb(247, 248, 248))
                                            .click_fill(Color32::from_rgb(232, 240, 254))
                                            .click_stroke(Stroke::new(
                                                1.2,
                                                Color32::from_rgb(210, 227, 252),
                                            ))
                                            .ui(ui)
                                            .clicked()
                                            {
                                                self.previous_answer_state =
                                                    PreviousAnswerState::Hide;
                                                self.show_history_menu = false;
                                                self.equation = equation.clone();
                                            }

                                            ui.add_space(2.5);

                                            ui.label(
                                                RichText::new("=").color(PREVIOUS_COLOR).size(28.0),
                                            );

                                            ui.add_space(2.5);

                                            let response = CalculatorButton::new(
                                                RichText::new(format_number(*answer))
                                                    .color(Color32::from_rgb(66, 133, 244)),
                                                Color32::TRANSPARENT,
                                            )
                                            .stroke(Stroke::new(1.2, FUNCTION_COLOR))
                                            .min_size(vec2(0.0, BUTTON_HEIGHT))
                                            .padding(vec2(9.0, 0.0))
                                            .hover_fill(Color32::from_rgb(247, 248, 248))
                                            .click_fill(Color32::from_rgb(232, 240, 254))
                                            .click_stroke(Stroke::new(
                                                1.2,
                                                Color32::from_rgb(210, 227, 252),
                                            ))
                                            .ui(ui);
                                            if response.clicked() {
                                                self.previous_answer_state =
                                                    PreviousAnswerState::Hide;
                                                self.show_history_menu = false;
                                                self.equation = Equation::new();
                                                self.equation.try_push(Rnd(format_number(*answer)));
                                            }

                                            if just_opened {
                                                response.scroll_to_me(None);
                                            }
                                        });

                                        ui.add_space(18.0);
                                    }
                                });
                            }

                            let rect = ui.max_rect().expand(25.0);
                            if !just_opened && !ui.rect_contains_pointer(rect) {
                                self.show_history_menu = false;
                            }
                        });
                });
        }
    }

    fn try_continue_answer(&mut self) {
        if self.equation.is_empty() {
            if self.previous_answer_state == PreviousAnswerState::Show {
                if let Some((_, num)) = self.history.last() {
                    self.equation.try_push(Rnd(format_number(*num)));
                }
            }
        }
    }
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

fn smoothstep(start: f32, end: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    let t = -2.0 * t * t * t + 3.0 * t * t;
    end * t + start * (1.0 - t)
}

fn format_number(num: f64) -> String {
    let integer_digits = num.abs().trunc().to_string().len();

    if integer_digits < 13 {
        num.to_string()
    } else {
        let e = integer_digits - 1;
        let num = num / 10.0f64.powf((integer_digits - 1) as f64);
        format!("{num:.7}e+{e}")
    }
}

pub trait Fluff {
    fn clicked_or_drag_ended(&self) -> bool;
}

impl Fluff for Response {
    fn clicked_or_drag_ended(&self) -> bool {
        self.clicked() || (self.drag_released() && self.hovered())
    }
}
