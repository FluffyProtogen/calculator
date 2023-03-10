#[cfg(test)]
use crate::calculator::Equation;
use crate::calculator::Item::*;
use crate::equation;
use crate::solver::solve;
#[test]
fn add() {
    let equation = equation![Number("1".into()), Add, Number("2".into())];
    assert_eq!(solve(&equation, true, 0.0), Some(3.0));
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
    assert_eq!(solve(&equation, true, 0.0), Some(9.0));
}

#[test]
fn parentheses1() {
    let equation = equation![
        OpeningParenthesis,
        Number("1".into()),
        Add,
        Number("2".into())
    ];
    assert_eq!(solve(&equation, true, 0.0), Some(3.0));
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
    assert_eq!(solve(&equation, true, 0.0), Some(7.0));
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
    assert_eq!(solve(&equation, true, 0.0), Some(19.0));
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
    assert_eq!(solve(&equation, true, 0.0), Some(5.0));
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
