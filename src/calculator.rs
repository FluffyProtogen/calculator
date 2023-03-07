use egui::{text::LayoutJob, *};
use Item::*;

const POWER_SCALE: f32 = 0.65;

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    Number(String),
    Rnd(String),
    Factorial,
    OpeningParenthesis,
    ClosingParenthesis,
    Percent,
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
    Subtract,
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
    pub fn is_opening_parenthesis(&self) -> bool {
        matches!(
            self,
            OpeningParenthesis | Sin | Ln | Cos | Log | Tan | Sqrt | Asin | Acos | Atan | Nroot
        )
    }

    pub fn can_put_end_parenthesis_after(&self) -> bool {
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
            Percent | Factorial | Pi | E | Ans | ClosingParenthesis | Rnd(..) => true,
            _ => false,
        }
    }

    pub fn can_put_operation_after(&self) -> bool {
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
            Percent | Factorial | Pi | E | Ans | ClosingParenthesis | Rnd(..) => true,
            _ => false,
        }
    }

    pub fn has_precedence_over(&self, other: &Item) -> bool {
        if self == other {
            true
        } else if self.is_opening_parenthesis() {
            false
        } else {
            match other {
                _ if other.is_opening_parenthesis() => false,
                ClosingParenthesis => false,
                Power => match self {
                    _ if self.is_opening_parenthesis() => true,
                    _ => false,
                },
                Multiply | Divide => match self {
                    _ if self.is_opening_parenthesis() => true,
                    Power => true,
                    _ => false,
                },
                _ => true,
            }
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

    pub fn clean(&self, ans: f64) -> Vec<Item> {
        let mut cleaned = vec![];

        for item in &self.list {
            match item {
                Rnd(num) => cleaned.push(Number(num.clone())),
                Pi => cleaned.push(Number("3.141592653589793238462643383279502884197".into())),
                E => cleaned.push(Number("2.7182818284590452353602874713527".into())),
                Ans => cleaned.push(Number(ans.to_string())),
                Percent => {
                    let last = cleaned.pop().unwrap();
                    cleaned.push(OpeningParenthesis);
                    cleaned.push(last);
                    cleaned.push(Divide);
                    cleaned.push(Number("100".into()));
                    cleaned.push(ClosingParenthesis);
                }
                Factorial => {
                    let last = cleaned.pop().unwrap();
                    cleaned.push(OpeningParenthesis);
                    cleaned.push(last);
                    cleaned.push(Factorial);
                    cleaned.push(ClosingParenthesis);
                }
                EXP => {
                    cleaned.push(Multiply);
                    cleaned.push(Number("10".into()));
                    cleaned.push(Power);
                }
                _ if item.is_opening_parenthesis() => {
                    if let Some(last) = cleaned.last() {
                        if last.can_put_operation_after() {
                            cleaned.push(Multiply);
                        }
                    }
                    cleaned.push(item.clone());
                }
                _ => cleaned.push(item.clone()),
            }
        }

        for _ in 0..self.open_parentheses_count() {
            cleaned.push(ClosingParenthesis);
        }

        cleaned
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
        } else if matches!(
            self.list.last(),
            Some(Power) | Some(Factorial) | Some(Percent)
        ) {
            if let Some(Number(num)) = self.list.iter_mut().nth_back(1) {
                if num == "0." {
                    self.list.pop();
                    self.list.pop();
                } else if num == "0" {
                    self.list.pop();
                    self.list.pop();
                } else {
                    self.list.pop();
                }
            } else {
                self.list.pop();
            }
        } else {
            self.list.pop();
        }
    }

    pub fn try_push(&mut self, item: Item) -> bool {
        match item {
            _ if item.is_opening_parenthesis() => {
                if matches!(
                    self.list.last(),
                    Some(ClosingParenthesis) | Some(Pi) | Some(E) | Some(Ans)
                ) {
                    self.list.push(Multiply);
                }
                self.list.push(item);
                true
            }
            ClosingParenthesis => {
                if let Some(last) = self.list.last() {
                    if last.can_put_end_parenthesis_after() && self.open_parentheses_count() > 0 {
                        self.list.push(ClosingParenthesis);
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Number(num) => {
                if let Some(Number(current_num)) = self.list.last_mut() {
                    if num == "." {
                        if current_num == "-" {
                            current_num.push_str("0.");
                            true
                        } else if !current_num.contains('.') {
                            current_num.push('.');
                            true
                        } else {
                            false
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
                        true
                    }
                } else {
                    if matches!(
                        self.list.last(),
                        Some(ClosingParenthesis)
                            | Some(Pi)
                            | Some(E)
                            | Some(Ans)
                            | Some(Rnd(..))
                            | Some(Percent)
                            | Some(Factorial)
                    ) {
                        self.list.push(Multiply);
                    }
                    if num == "." {
                        self.list.push(Number("0.".into()));
                    } else {
                        self.list.push(Number(num));
                    }
                    true
                }
            }
            Add | Multiply | Divide => {
                if let Some(last) = self.list.last_mut() {
                    if last.can_put_operation_after() {
                        self.list.push(item);
                        true
                    } else if matches!(last, Add | Multiply | Divide | Subtract) {
                        *last = item;
                        true
                    } else {
                        false
                    }
                } else {
                    self.list.push(Number("0".into()));
                    self.list.push(item);
                    true
                }
            }
            Factorial | Percent => {
                if let Some(Number(num)) = self.list.last() {
                    if num == "-" {
                        false
                    } else {
                        self.list.push(item);
                        true
                    }
                } else {
                    if let Some(last_item) = self.list.last() {
                        if last_item.can_put_operation_after() {
                            self.list.push(item);
                            true
                        } else {
                            false
                        }
                    } else {
                        self.list.push(Number("0".into()));
                        self.list.push(item);
                        true
                    }
                }
            }
            Subtract => {
                if let Some(last) = self.list.last_mut() {
                    match last {
                        Number(num) => {
                            if num.len() == 1 {
                                if num.chars().nth(0).unwrap() == '-' {
                                    false
                                } else {
                                    self.list.push(Subtract);
                                    true
                                }
                            } else {
                                self.list.push(Subtract);
                                true
                            }
                        }
                        Percent | Divide | Multiply | Power | EXP => {
                            self.list.push(Number("-".into()));
                            true
                        }
                        Add => {
                            *last = Subtract;
                            true
                        }
                        _ if last.is_opening_parenthesis() => {
                            self.list.push(Number("-".into()));
                            true
                        }
                        _ if last.can_put_end_parenthesis_after() => {
                            self.list.push(Subtract);
                            true
                        }
                        _ => false,
                    }
                } else {
                    self.list.push(Number("-".into()));
                    true
                }
            }
            Power => {
                if let Some(last) = self.list.last() {
                    if last.can_put_operation_after() {
                        self.list.push(Power);
                        true
                    } else {
                        false
                    }
                } else {
                    self.list.push(Number("0".into()));
                    self.list.push(Power);
                    true
                }
            }
            Pi | E | Ans => {
                if matches!(
                    self.list.last(),
                    Some(ClosingParenthesis)
                        | Some(Pi)
                        | Some(E)
                        | Some(Ans)
                        | Some(Rnd(..))
                        | Some(Percent)
                        | Some(Factorial)
                ) {
                    self.list.push(Multiply);
                }
                self.list.push(item);
                true
            }
            Rnd(num) => {
                if let Some(item) = self.list.last() {
                    if item.can_put_operation_after() {
                        self.list.push(Multiply);
                    }
                    self.list.push(Rnd(num));
                } else {
                    self.list.push(Rnd(num));
                }
                true
            }
            EXP => {
                if let Some(Number(num)) = self.list.last().as_ref() {
                    if num == "." {
                        false
                    } else if num == "-" {
                        false
                    } else {
                        self.list.push(EXP);
                        true
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn open_parentheses_count(&self) -> usize {
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

        let mut default_layout = |text, power_level, font: &str| {
            job.append(
                text,
                0.0,
                TextFormat {
                    font_id: FontId::new(
                        size * POWER_SCALE.powf(power_level as f32),
                        FontFamily::Name(font.into()),
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
                Number(num) => default_layout(&num, power_level, "roboto"),
                Factorial => default_layout("!", power_level, "roboto"),
                OpeningParenthesis => default_layout("(", power_level, "roboto"),
                ClosingParenthesis => default_layout(")", power_level, "roboto"),
                Percent => default_layout("%", power_level, "roboto"),
                Sin => default_layout("sin(", power_level, "roboto"),
                Ln => default_layout("ln(", power_level, "roboto"),
                Divide => default_layout(" ÷ ", power_level, "roboto"),
                Pi => default_layout("π", power_level, "roboto"),
                Cos => default_layout("cos(", power_level, "roboto"),
                Log => default_layout("log(", power_level, "roboto"),
                Multiply => default_layout(" × ", power_level, "roboto"),
                E => default_layout("e", power_level, "roboto"),
                Tan => default_layout("tan(", power_level, "roboto"),
                Sqrt => default_layout("√(", power_level, "roboto"),
                Subtract => default_layout(" – ", power_level, "roboto"),
                EXP => default_layout("E", power_level, "roboto"),
                Add => default_layout(" + ", power_level, "roboto"),
                Ans => default_layout("Ans", power_level, "roboto"),
                Asin => default_layout("arcsin(", power_level, "roboto"),
                Acos => default_layout("arccos(", power_level, "roboto"),
                Atan => default_layout("arctan(", power_level, "roboto"),
                Rnd(num) => default_layout(&num, power_level, "roboto"),
                Power => {
                    parentheses_counts.push(0);
                    if index == self.list.len() - 1 {
                        default_layout("□", power_level + 1, "arial");
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
