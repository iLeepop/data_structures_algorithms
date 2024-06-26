// exponential_search.rs
// 指数查找

use crate::way::binary_search::binary_search1;

fn exponential_search(nums: &[i32], target: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }

    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<= 1;
    }

    let low = high >> 1;

    binary_search1(&nums[low..size.min(high + 1)], target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_search() {
        let nums = [1, 3, 4, 7, 8, 9, 10, 13, 16, 20, 21, 23, 27, 30, 32, 35, 38, 40];
        let target = 28;
        let is_found = exponential_search(&nums, target);
        println!("{} is found: {}", target, is_found);
        assert_eq!(is_found, false);
    }
}