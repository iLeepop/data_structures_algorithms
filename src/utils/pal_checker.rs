// pal_checker.rs
// 回文检查

use crate::Deque;

pub fn pal_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());
    for c in pal.chars() {
        let _r = d.add_rear(c);
    }
    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        let head = d.remove_front();
        let tail = d.remove_rear();
        if head != tail {
            is_pal = false;
        }
    }

    is_pal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pal_checker() {
        let pal = "lzhzl";
        let is_pal = pal_checker(pal);
        println!("{} is pal: {}", pal, is_pal)
    }
}