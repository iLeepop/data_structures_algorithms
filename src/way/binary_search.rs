// binary_search.rs
// 二分查找

pub fn binary_search1(nums: &[i32], num: i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid: usize = (low + high) >> 1;

        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    found
}

fn binary_search2(nums: &[i32], num: i32) -> bool {
    if 0 == nums.len() {
        return false;
    }
    let mid = nums.len() >> 1;
    if num == nums[mid] {
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid], num);
    } else {
        return binary_search2(&nums[mid + 1..], num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search1() {
        let nums = [
            1, 3, 4, 7, 8, 9, 10, 13, 16, 20, 21, 23, 27, 30, 32, 35, 38, 40,
        ];
        let num = 26;
        let found = binary_search1(&nums, num);
        let num1 = 38;
        let found1 = binary_search1(&nums, num1);
        println!("{} is in nums: {}", num, found);
        println!("{} is in nums: {}", num1, found1);
    }

    #[test]
    fn test_binary_search2() {
        let nums = [
            1, 3, 4, 7, 8, 9, 10, 13, 16, 20, 21, 23, 27, 30, 32, 35, 38, 40,
        ];
        let num = 26;
        let found = binary_search2(&nums, num);
        let num1 = 38;
        let found1 = binary_search2(&nums, num1);
        println!("{} is in nums: {}", num, found);
        println!("{} is in nums: {}", num1, found1);
    }
}
