// infix_to_postfix.rs

use std::collections::HashMap;

use crate::{par_checker, Stack};

pub fn infix_to_postfix(infix: &str) -> Option<String> {
    if !par_checker(infix) {
        return None;
    }

    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    let mut op_stack = Stack::new();
    let mut postfix = Vec::new();
    for token in infix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            op_stack.push(token);
        } else if ")" == token {
            let mut top = op_stack.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = op_stack.pop().unwrap();
            }
        } else {
            while (!op_stack.is_empty()) && (prec[op_stack.peek().unwrap()] >= prec[token]) {
                postfix.push(op_stack.pop().unwrap());
            }
            op_stack.push(token);
        }
    }
    while !op_stack.is_empty() {
        postfix.push(op_stack.pop().unwrap());
    }

    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }
    Some(postfix_str)
}

pub fn postfix_eval(postfix: &str) -> Option<i32> {
    if postfix.len() < 5 {
        return None;
    }
    let mut op_stack = Stack::new();
    for token in postfix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            op_stack.push(token.parse::<i32>().unwrap());
        } else {
            let op2 = op_stack.pop().unwrap();
            let op1 = op_stack.pop().unwrap();
            match token {
                "+" => op_stack.push(op1 + op2),
                "-" => op_stack.push(op1 - op2),
                "*" => op_stack.push(op1 * op2),
                "/" => op_stack.push(op1 / op2),
                _ => continue,
            }
        }
    }

    Some(op_stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix() {
        let postfix = infix_to_postfix("( A + B ) * C - D / E");
        assert_eq!(postfix, Some("A B + C * D E / - ".to_string()));
    }

    #[test]
    fn test_postfix_eval() {
        let eval = postfix_eval("3 4 * 5 6 * +");
        assert_eq!(eval, Some(42));
    }

    #[test]
    fn test_complex_postfix_eval() {
        let postfix = infix_to_postfix("( 3 * 4 ) + ( 5 * 6 ) - ( 8 * 6 )");
        let eval = postfix_eval(&postfix.unwrap()[..]);
        assert_eq!(eval, Some(-6));
        assert_ne!(eval, Some(42));
    }
}
