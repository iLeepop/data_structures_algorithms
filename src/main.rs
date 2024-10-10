// 插值查找
mod way;
use way::binary_search1;

fn main() {
    let nums = [1, 2, 3, 5, 6, 7, 8, 9, 10, 14, 16];
    let target = 11;
    println!("{} is found: {}", target, interpolation_search(&nums, target));
    println!("{} is found: {}", target, interpolation_search(&nums, target));
}

fn interpolation_search(nums: &[i32], num: i32) -> bool {
    if nums.is_empty() { return false; }

    let mut low = 0usize;
    let mut high = nums.len() - 1;

    let mut interpolant: usize = 0;
    loop {
        let low_val = nums[low];
        let high_val = nums[high];
        if high <= low || num < low_val || num > high_val {
            break;
        }

        let offset = (num - low_val)*(high - low) as i32 / (high_val - low_val);
        interpolant = low + offset as usize;

        if nums[interpolant] > num {
            high = interpolant - 1;
        } else if nums[interpolant] < num {
            low = interpolant + 1;
        } else {
            break;
        }
    }

    if num == nums[interpolant] {
        true
    } else {
        false
    }
}

fn exponential_search(nums: &[i32], num: i32) -> bool {
    let size = nums.len();
    if size == 0 { return false; }

    let mut high = 1usize;
    while high < size && nums[high] < num {
        high <<= 1;
    }

    let low = high >> 1;

    binary_search1(&nums[low..size.min(high + 1)], num)
}
