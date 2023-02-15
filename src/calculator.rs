use egui::{text::LayoutJob, *};
use Item::*;

pub enum Item {
    Number(String),
    Factorial,
    OpeningParenthesis,
    ClosingParenthesis,
    Modulus,
    Sin,
    Ln,
    Divide,
    Pi,
    Cos,
    Log,
    Multiply,
    E,
    Tan,
    Sqrt,
    Minus,
    Ans,
    EXP,
    Power,
    Asin,
    Acos,
    Atan,
    Nroot,
}

impl Item {
    fn is_opening_parenthesis(&self) -> bool {
        matches!(
            self,
            OpeningParenthesis
                | Sin
                | Ln
                | Cos
                | Log
                | Tan
                | Sqrt
                | Power
                | Asin
                | Acos
                | Atan
                | Nroot
        )
    }

    fn can_put_end_parenthesis_after(&self) -> bool {
        match self {
            Number(num) if num.chars().last().unwrap() != '.' => true,
            Factorial | Pi | E | Ans => true,
            _ => false,
        }
    }
}

pub struct Equation {
    list: Vec<Item>,
}

impl Equation {
    pub fn new() -> Self {
        Self { list: vec![] }
    }

    pub fn try_push(&mut self, item: Item) {
        match item {
            _ if item.is_opening_parenthesis() => {
                self.list.push(item);
            }
            ClosingParenthesis => {
                if item.can_put_end_parenthesis_after() {
                    self.list.push(ClosingParenthesis);
                }
            }
            _ => {}
        }
    }

    fn open_parenthesis_count(&self) -> usize {
        self.list
            .iter()
            .filter(|item| item.is_opening_parenthesis())
            .count()
            - self
                .list
                .iter()
                .filter(|item| matches!(item, ClosingParenthesis))
                .count()
    }

    pub fn render(&self, size: f32, color: Color32) -> LayoutJob {
        let mut job = LayoutJob::default();
        if self.list.len() == 0 {
            job.append(
                "0",
                0.0,
                TextFormat {
                    font_id: FontId::new(size, FontFamily::Name("roboto".into())),
                    color,
                    ..Default::default()
                },
            );
            return job;
        }

        let mut default_layout = |text: &str| {
            job.append(
                text,
                0.0,
                TextFormat {
                    font_id: FontId::new(size, FontFamily::Name("roboto".into())),
                    color,
                    ..Default::default()
                },
            );
        };

        for item in &self.list {
            match item {
                Number(num) => default_layout(&num),
                Factorial => default_layout("!"),
                OpeningParenthesis => default_layout("("),
                ClosingParenthesis => default_layout(")"),
                Modulus => default_layout("%"),
                Sin => default_layout("sin("),
                _ => {}
            }
        }

        for _ in 0..self.open_parenthesis_count() {
            job.append(
                ")",
                0.0,
                TextFormat {
                    font_id: FontId::new(size, FontFamily::Name("roboto".into())),
                    color: Color32::from_rgb(204, 204, 204),
                    ..Default::default()
                },
            );
        }

        job
    }
}
