use std::{collections::HashMap, iter::Map};

use super::Stack;

pub fn postfix_expression(input: String) -> String {
    let mut exp_level = HashMap::new();
    exp_level.insert('(', 3);
    exp_level.insert(')', 3);
    exp_level.insert('*', 2);
    exp_level.insert('/', 2);
    exp_level.insert('+', 1);
    exp_level.insert('-', 1);

    let mut result = String::new();
    let mut stack = Stack::new();

    'label: for item in input.chars() {
        match item {
            '(' => stack.push(item),
            ')' => loop {
                match stack.pop() {
                    Some('(') => continue 'label,
                    Some(x) => result.push(x),
                    None => return "".to_string(),
                };
            },
            op @ ('+' | '-') => {
                while stack.peek().unwrap() != '(' {
                    match stack.pop() {
                        Some(x) => {
                            let now = exp_level.get(&x).unwrap();
                            let cur = exp_level.get(&op).unwrap();
                            if cur > now {
                                result.push(op);
                                break;
                            } else {
                                stack.push(x);
                            }
                        }
                        None => {
                            stack.push(op);
                            break;
                        }
                    }
                }
            }
            op @ ('*' | '/') => stack.push(op),
            a @ _ => result.push(a),
        };
    }

    while !stack.is_empty() {
        result.push(stack.pop().unwrap())
    }

    result
}
