pub mod app;
pub mod calculator;
pub mod solver;
use eframe::*;
use egui::vec2;
//ONCE YOU HIT ENTER, HITTING ANY OPERATION WILL ADD TO THE ANSWER. TYPING ANY NUMBER OVERRIDES THE ANSWER
//FOR AUTOMATIC EXPONENTS LIKE E^2, MAKE TRY_PUSH RETURN A BOOL. IT ONLY WILL ADD ANOTHER NUMBER IF IT SUCCESSFULLY PUSHES.

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

#[cfg(test)]
mod tests {
    use crate::calculator::Equation;
    use crate::calculator::Item::*;
    use crate::equation;
    use crate::solver::solve;
    #[test]
    fn add() {
        let equation = equation![Number("1".into()), Add, Number("2".into())];
        assert_eq!(solve(&equation, true, 0.0), 3.0);
    }

    #[test]
    fn order_of_operations1() {
        let equation = equation![
            Number("5".into()),
            Add,
            Number("2".into()),
            Multiply,
            Number("3".into()),
            Subtract,
            Number("2".into())
        ];
        assert_eq!(solve(&equation, true, 0.0), 9.0);
    }

    #[test]
    fn parentheses1() {
        let equation = equation![
            OpeningParenthesis,
            Number("1".into()),
            Add,
            Number("2".into())
        ];
        assert_eq!(solve(&equation, true, 0.0), 3.0);
    }

    #[test]
    fn parentheses2() {
        let equation = equation![
            OpeningParenthesis,
            Number("1".into()),
            Add,
            Number("2".into()),
            Multiply,
            Number("3".into())
        ];
        assert_eq!(solve(&equation, true, 0.0), 7.0);
    }

    #[test]
    fn parentheses3() {
        let equation = equation![
            OpeningParenthesis,
            Number("1".into()),
            Add,
            Number("2".into()),
            Multiply,
            Number("2".into()),
            Add,
            Number("1".into()),
            ClosingParenthesis,
            Multiply,
            Number("3".into()),
            Add,
            Number("1".into())
        ];
        assert_eq!(solve(&equation, true, 0.0), 19.0);
    }

    #[test]
    fn parentheses4() {
        let equation = equation![
            OpeningParenthesis,
            Number("1".into()),
            Add,
            OpeningParenthesis,
            Number("1".into()),
            Add,
            Number("1".into()),
            ClosingParenthesis,
            Multiply,
            Number("2".into()),
            ClosingParenthesis
        ];
        assert_eq!(solve(&equation, true, 0.0), 5.0);
    }

    #[macro_export]
    macro_rules! equation {
        ($($item:expr),*) => {
            {
                let mut equation = Equation::new();
                $(
                    equation.try_push($item);
                )*
                equation
            }
        }
    }
}
