use std::ops::Mul;

use crate::calculator::{Equation, Item};
use Item::*;
//https://www.geeksforgeeks.org/expression-evaluation/
pub fn solve(equation: &Equation, degrees: bool, ans: f64) -> f64 {
    let items = equation.clean();

    let mut operation_stack = vec![];
    let mut value_stack = vec![];

    for item in items {
        match item {
            Number(num) => value_stack.push(num.parse().unwrap()),
            _ if item.is_opening_parenthesis() => operation_stack.push(item),
            ClosingParenthesis => {
                while let Some(false) = operation_stack
                    .last()
                    .map(|item| item.is_opening_parenthesis())
                {
                    let value2 = value_stack.pop().unwrap();
                    let value1 = value_stack.pop().unwrap();
                    let operation = operation_stack.pop().unwrap();
                    println!("1: {}, 2: {}, o: {:?}", value1, value2, operation);
                    value_stack.push(evaluate(operation, value1, value2));
                }
                if let Some(parenthesis) = operation_stack.pop() {
                    if parenthesis != OpeningParenthesis {
                        let last = value_stack.last_mut().unwrap();
                        match parenthesis {
                            Sin => *last = if degrees { last.to_radians() } else { *last }.sin(),
                            Cos => *last = if degrees { last.to_radians() } else { *last }.cos(),
                            Tan => *last = if degrees { last.to_radians() } else { *last }.tan(),
                            Ln => *last = last.ln(),
                            Log => *last = last.log10(),
                            Sqrt => *last = last.sqrt(),
                            Asin => {
                                *last = if degrees {
                                    last.asin().to_degrees()
                                } else {
                                    last.asin()
                                }
                            }
                            Acos => {
                                *last = if degrees {
                                    last.acos().to_degrees()
                                } else {
                                    last.acos()
                                }
                            }
                            Atan => {
                                *last = if degrees {
                                    last.atan().to_degrees()
                                } else {
                                    last.atan()
                                }
                            }
                            Nroot => todo!(),
                            _ => {} //OpeningParenthesis | Sin | Ln | Cos | Log | Tan | Sqrt | Asin | Acos | Atan | Nroot
                        }
                    }
                }
            }
            Add | Subtract | Multiply | Divide => {
                while let Some(last_item) = operation_stack.last() {
                    if last_item.has_precedence_over(&item) && value_stack.len() >= 2 {
                        dbg!(last_item);
                        dbg!(&item);
                        println!("{}", last_item.has_precedence_over(&item));
                        let value2 = value_stack.pop().unwrap();
                        let value1 = value_stack.pop().unwrap();
                        let operation = operation_stack.pop().unwrap();

                        println!("1: {}, 2: {}, o: {:?}", value1, value2, operation);

                        value_stack.push(evaluate(operation, value1, value2));
                    } else {
                        break;
                    }
                }
                operation_stack.push(item);
            }
            _ => {}
        }
        dbg!(&value_stack);
        dbg!(&operation_stack);
    }

    for operation in operation_stack.into_iter().rev() {
        let value2 = value_stack.pop().unwrap();
        let value1 = value_stack.pop().unwrap();
        value_stack.push(evaluate(operation, value1, value2));
    }
    println!("{}", value_stack[0]);
    value_stack[0]
}

fn evaluate(operation: Item, value1: f64, value2: f64) -> f64 {
    match operation {
        Add => value1 + value2,
        Subtract => value1 - value2,
        Multiply => value1 * value2,
        Divide => value1 / value2,
        _ => todo!(),
    }
}