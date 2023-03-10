use eframe::*;
use egui::*;

use crate::app::{Fluff, BUTTON_HEIGHT, BUTTON_WIDTH, ROUNDING};
pub struct CalculatorButton {
    text: WidgetText,
    fill: Color32,
    hover_fill: Color32,
    click_fill: Color32,
    stroke: Stroke,
    click_stroke: Stroke,
    rounding: Rounding,
    min_size: Vec2,
    padding: Option<Vec2>,
    max_text_width: Option<f32>,
}

impl CalculatorButton {
    pub fn min_size(self, min_size: Vec2) -> Self {
        Self { min_size, ..self }
    }

    pub fn click_fill(self, click_fill: Color32) -> Self {
        Self { click_fill, ..self }
    }

    pub fn hover_fill(self, hover_fill: Color32) -> Self {
        Self { hover_fill, ..self }
    }

    pub fn padding(self, padding: Vec2) -> Self {
        Self {
            padding: Some(padding),
            ..self
        }
    }

    pub fn max_text_width(self, max_text_width: f32) -> Self {
        Self {
            max_text_width: Some(max_text_width),
            ..self
        }
    }

    pub fn stroke(self, stroke: Stroke) -> Self {
        Self {
            click_stroke: stroke.clone(),
            stroke,
            ..self
        }
    }

    pub fn click_stroke(self, click_stroke: Stroke) -> Self {
        Self {
            click_stroke,
            ..self
        }
    }

    pub fn new<T>(text: T, color: Color32) -> Self
    where
        T: Into<WidgetText>,
    {
        Self {
            text: text.into(),
            fill: color,
            hover_fill: color,
            click_fill: color,
            stroke: Stroke::NONE,
            click_stroke: Stroke::new(1.0, Color32::from_rgb(32, 33, 36)),
            rounding: ROUNDING,
            min_size: vec2(BUTTON_WIDTH, BUTTON_HEIGHT),
            padding: None,
            max_text_width: None,
        }
    }
}

impl Widget for CalculatorButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let button_padding = self.padding.unwrap_or(ui.spacing().button_padding);

        let text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        let text = self
            .text
            .into_galley(ui, None, text_wrap_width, TextStyle::Button);

        let mut text_size = text.size();
        if let Some(max_text_width) = self.max_text_width {
            if text_size.x > max_text_width {
                text_size.x = max_text_width
            }
        }

        let mut desired_size = text_size;

        desired_size += 2.0 * button_padding;

        desired_size = desired_size.at_least(self.min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let fill = if response.is_pointer_button_down_on() {
                self.click_fill
            } else if response.hovered() {
                self.hover_fill
            } else {
                self.fill
            };
            let stroke = if response.is_pointer_button_down_on() {
                self.click_stroke
            } else {
                self.stroke
            };

            ui.painter().rect(rect, self.rounding, fill, stroke);

            let text_pos = ui
                .layout()
                .align_size_within_rect(text_size, rect.shrink2(button_padding))
                .min;

            let trim = if text_size.x < text.size().x {
                5.0
            } else {
                0.0
            };
            let clip_rect = Rect {
                min: rect.min,
                max: pos2(
                    desired_size.x + rect.min.x - button_padding.x - trim,
                    rect.max.y,
                ),
            };

            text.paint_with_fallback_color(
                &ui.painter().with_clip_rect(clip_rect),
                text_pos,
                ui.visuals().text_color(),
            );
        }

        response
    }
}
