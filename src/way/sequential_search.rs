// sequential_search.rs
// 顺序查找

fn sequential_search(nums: &[i32], num: i32) -> bool {
    let mut pos = 0;
    let mut found = false;

    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    found
}

fn sequetial_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;
    while pos < nums.len() && !found {
        if num == nums[pos] {
            found = true;
        } else {
            pos += 1;
        }
    }
    if found {
        Some(pos)
    } else {
        None
    }
}

fn ordered_sequential_search(nums: &[i32], num:i32) -> bool {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false;

    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true;
        } else {
            pos += 1;
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_search() {
        let num = 3;
        let nums = [9, 5, 7, 4, 1, 6, 2, 8, 5];
        println!("{} in nums is {}", num, sequential_search(&nums, num))
    }

    #[test]
    fn test_sequential_search_pos() {
        let num = 6;
        let nums = [9, 5, 7, 4, 1, 6, 2, 8, 5];
        match sequetial_search_pos(&nums, num) {
            Some(pos) => println!("index of {num} is {pos}"),
            None => println!("{num} not found")
        }
    }

    #[test]
    fn test_ordered_sequential_search() {
        let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 60, 62, 64];
        let num = 44;
        let found = ordered_sequential_search(&nums, num);
        println!("{num} in nums is {found}");

        let num = 49;
        let found = ordered_sequential_search(&nums, num);
        println!("{num} is in nums: {found}");
    }
}