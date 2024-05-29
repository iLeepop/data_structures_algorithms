// par_checker.rs
// 括号检查

use crate::data::Stack;

fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closes =")]}";
    opens.find(open) == closes.find(close)
}

pub fn par_checker(par: &str) -> bool {
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true;
    let mut stack = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        } else if c == ')' || c == ']' || c == '}' {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }

    balance && stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_par_checker() {
        assert_eq!(par_checker("()"), true);
        assert_eq!(par_checker("()()"), true);
        assert_eq!(par_checker("(()())"), true);
        assert_eq!(par_checker("(())()"), true);
        assert_eq!(par_checker("()(()())"), true);
        assert_eq!(par_checker("(()"), false);

        assert_eq!(par_checker("()()("), false);
        assert_eq!(par_checker("(()()"), false);
        assert_eq!(par_checker("(()())("), false);

        assert_eq!(par_checker("(23*((a+b)+(b+a)/45))%4"), true);
        assert_eq!(par_checker("(23*((a+b)+(b+a)/45))%4)"), false);
    }
}