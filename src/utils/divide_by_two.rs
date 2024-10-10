// divide_by_two.rs
// 二分法求解

use crate::data::Stack;

pub fn divide_by_two(mut dec_num: u32) -> String {
    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}

pub fn base_convert(mut dec_num: u32, base: u32) -> String {
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();

    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    let mut base_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap();
        base_str += &digits[rem as usize].to_string();
    }

    base_str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_by_two() {
        let bin_str = divide_by_two(9);
        print!("17 is b{}", bin_str);
    }

    #[test]
    fn test_base_convert() {
        let bin_str = base_convert(17, 2);
        print!("10 is b{}\n", bin_str);

        let hex_str = base_convert(17, 16);
        print!("16 is b{}\n", hex_str);
    }
}
