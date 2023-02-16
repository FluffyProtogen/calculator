use egui::{text::LayoutJob, *};
use Item::*;

const POWER_SCALE: f32 = 0.65;
const POWER_MAX: usize = 4;

#[derive(Debug, PartialEq)]
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
    Add,
}

impl Item {
    fn is_opening_parenthesis(&self) -> bool {
        matches!(
            self,
            OpeningParenthesis | Sin | Ln | Cos | Log | Tan | Sqrt | Asin | Acos | Atan | Nroot
        )
    }

    fn can_put_end_parenthesis_after(&self) -> bool {
        match self {
            Number(num) => {
                if num.len() == 1 {
                    if num.chars().nth(0).unwrap() == '-' {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            Factorial | Pi | E | Ans | ClosingParenthesis => true,
            _ => false,
        }
    }

    fn can_put_operation_after(&self) -> bool {
        match self {
            Number(num) => {
                if num.len() == 1 {
                    if num.chars().nth(0).unwrap() == '-' {
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
            Factorial | Pi | E | Ans | ClosingParenthesis => true,
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

    pub fn backspace(&mut self) {
        if let Some(Number(num)) = self.list.last_mut() {
            if num == "0." {
                self.list.pop();
            } else if num == "-0." {
                num.pop();
                num.pop();
            } else if num.len() > 1 {
                num.pop();
            } else {
                self.list.pop();
            }
        } else {
            self.list.pop();
        }
    }

    pub fn try_push(&mut self, item: Item) {
        match item {
            _ if item.is_opening_parenthesis() => {
                self.list.push(item);
            }
            ClosingParenthesis => {
                if let Some(last) = self.list.last() {
                    if last.can_put_end_parenthesis_after() && self.open_parentheses_count_at(0) > 0
                    {
                        self.list.push(ClosingParenthesis);
                    }
                }
            }
            Number(num) => {
                if let Some(Number(current_num)) = self.list.last_mut() {
                    if num == "." {
                        if current_num == "-" {
                            current_num.push_str("0.")
                        } else if !current_num.contains('.') {
                            current_num.push('.');
                        }
                    } else {
                        if let Some('0') = current_num.chars().nth(0) {
                            if current_num.len() == 1 {
                                *current_num = num;
                            } else {
                                current_num.push_str(&num);
                            }
                        } else {
                            current_num.push_str(&num);
                        }
                    }
                } else {
                    if let Some(ClosingParenthesis) = self.list.last() {
                        self.list.push(Multiply);
                    }
                    if num == "." {
                        self.list.push(Number("0.".into()));
                    } else {
                        self.list.push(Number(num));
                    }
                }
            }
            Factorial | Add | Multiply | Divide => {
                if let Some(last) = self.list.last_mut() {
                    if last.can_put_operation_after() {
                        self.list.push(item);
                    } else if matches!(last, Add | Multiply | Divide | Minus) {
                        *last = item;
                    }
                } else {
                    self.list.push(Number("0".into()));
                    self.list.push(item);
                }
            }
            Minus => {
                if let Some(last) = self.list.last_mut() {
                    match last {
                        Number(num) => {
                            if num.len() == 1 {
                                if num.chars().nth(0).unwrap() == '-' {
                                } else {
                                    self.list.push(Minus);
                                }
                            } else {
                                self.list.push(Minus);
                            }
                        }
                        Modulus | Divide | Multiply | Power | EXP => {
                            self.list.push(Number("-".into()))
                        }
                        Add => *last = Minus,
                        _ if last.is_opening_parenthesis() => {
                            self.list.push(Number("-".into()));
                        }
                        _ if last.can_put_end_parenthesis_after() => self.list.push(Minus),
                        _ => {}
                    }
                } else {
                    self.list.push(Number("-".into()))
                }
            }
            Power => {
                if let Some(last) = self.list.last() {
                    if last.can_put_operation_after() {
                        self.list.push(Power);
                    }
                }
            }
            _ => {}
        }
    }

    fn open_parentheses_count_at(&self, position: usize) -> usize {
        self.list
            .iter()
            .skip(position)
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

        let mut default_layout = |text, power_level| {
            job.append(
                text,
                0.0,
                TextFormat {
                    font_id: FontId::new(
                        size * POWER_SCALE.powf(power_level as f32),
                        FontFamily::Name("roboto".into()),
                    ),
                    color,
                    valign: Align::TOP,
                    ..Default::default()
                },
            );
        };

        let mut level_open_parentheses_counts = vec![];
        let mut parentheses_counts = vec![];
        for (index, item) in self.list.iter().enumerate() {
            let power_level = parentheses_counts.len();
            if power_level + 1 > level_open_parentheses_counts.len() {
                level_open_parentheses_counts.push(0);
            }
            if item.is_opening_parenthesis() {
                level_open_parentheses_counts[power_level] += 1;
            }
            if *item == ClosingParenthesis {
                level_open_parentheses_counts[power_level] -= 1;
            }
            match item {
                Number(num) => default_layout(&num, power_level),
                Factorial => default_layout("!", power_level),
                OpeningParenthesis => default_layout("(", power_level),
                ClosingParenthesis => default_layout(")", power_level),
                Modulus => default_layout(" % ", power_level),
                Sin => default_layout("sin(", power_level),
                Ln => default_layout("ln(", power_level),
                Divide => default_layout(" ÷ ", power_level),
                Pi => default_layout("π", power_level),
                Cos => default_layout("cos(", power_level),
                Log => default_layout("log(", power_level),
                Multiply => default_layout(" × ", power_level),
                E => default_layout(" e ", power_level),
                Tan => default_layout("tan(", power_level),
                Sqrt => default_layout("√(", power_level),
                Minus => default_layout(" – ", power_level),
                EXP => default_layout("E", power_level),
                Add => default_layout(" + ", power_level),
                Power => {
                    parentheses_counts.push(0);
                    if index == self.list.len() - 1 {
                        default_layout("□", power_level + 1);
                        if power_level + 2 > level_open_parentheses_counts.len() {
                            level_open_parentheses_counts.push(0);
                        }
                    }
                }
                _ => {}
            }
            if let Some(parentheses_count) = parentheses_counts.last_mut() {
                if item.is_opening_parenthesis() {
                    *parentheses_count += 1;
                }
                if *item == ClosingParenthesis {
                    *parentheses_count -= 1;
                }
                if *parentheses_count == 0
                    && item.can_put_operation_after()
                    && self.list.get(index + 1) != Some(&Power)
                {
                    while parentheses_counts.last() == Some(&0) {
                        parentheses_counts.pop();
                    }
                }
            }
        }

        for (level, open_parens) in level_open_parentheses_counts.iter().rev().enumerate() {
            let level =
                (level_open_parentheses_counts.len() as i32 - level as i32 - 1).clamp(0, i32::MAX);
            for i in 0..*open_parens {
                job.append(
                    ")",
                    0.0,
                    TextFormat {
                        font_id: FontId::new(
                            size * POWER_SCALE.powf(level as f32),
                            FontFamily::Name("roboto".into()),
                        ),
                        color: Color32::from_rgb(204, 204, 204),
                        valign: Align::TOP,
                        ..Default::default()
                    },
                );
            }
        }
        job
    }
}
