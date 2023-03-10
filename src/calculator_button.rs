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
}

impl CalculatorButton {
    pub fn min_size(self, min_size: Vec2) -> Self {
        Self { min_size, ..self }
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
        }
    }
}

impl Widget for CalculatorButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let button_padding = ui.spacing().button_padding;

        let text_wrap_width = ui.available_width() - 2.0 * button_padding.x;
        let text = self
            .text
            .into_galley(ui, Some(true), text_wrap_width, TextStyle::Button);

        let mut desired_size = text.size();

        desired_size += 2.0 * button_padding;

        desired_size = desired_size.at_least(self.min_size);

        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

        if ui.is_rect_visible(rect) {
            let fill = if response.clicked_or_drag_ended() {
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
                .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                .min;

            text.paint_with_fallback_color(ui.painter(), text_pos, ui.visuals().text_color())
        }

        response
    }
}
