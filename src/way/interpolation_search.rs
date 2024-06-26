// interpolation_search.rs
// 插值查找

fn interpolation_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty() { return false; }

    let mut low = 0usize;
    let mut high = nums.len() - 1;

    let mut interpolant: usize = 0;
    loop {
        let low_val = nums[low];
        let high_val = nums[high];

        // 异常情况
        if high <= low || target < low_val || target > high_val {
            break;
        }

        let offset = (target - low_val)*(high - low) as i32 / (high_val - low_val);
        interpolant = low + offset as usize;

        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            break;
        }
    }
    
    if target == nums[interpolant] {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolation_search() {
        let nums = [1, 2, 3, 5, 6, 7, 8, 9, 10, 14, 16];
        let target = 8;
        let is_found = interpolation_search(&nums, target);
        println!("{} is found: {}", target, is_found);
    }
}
