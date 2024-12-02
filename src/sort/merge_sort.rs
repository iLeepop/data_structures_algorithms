// merge_sort

fn merge_sort(nums: &mut [i32]) {
    if nums.len() > 1 {
        let mid = nums.len() >> 1;
        merge_sort(&mut nums[..mid]);
        merge_sort(&mut nums[mid..]);
        merge(nums, mid);
    }
}

fn merge(nums: &mut [i32], mid: usize) {
    let mut i = 0;
    let mut k = mid;
    let mut temp = Vec::new();

    for _j in 0..nums.len() {
        if k == nums.len() || i == mid {
            break;
        }

        if nums[i] < nums[k] {
            temp.push(nums[i]);
            i += 1;
        } else {
            temp.push(nums[k]);
            k += 1;
        }
    }

    if i < mid && k == nums.len() {
        for j in i..mid {
            temp.push(nums[j]);
        }
    } else if i == mid && k < nums.len() {
        for j in k..nums.len() {
            temp.push(nums[j]);
        }
    }

    for j in 0..nums.len() {
        nums[j] = temp[j];
    }
}

#[cfg(test)]
mod test {
    use crate::random_vec;

    use super::*;

    #[test]
    fn merge_test() {
        let mut nums = random_vec(10, 100, 0);
        merge_sort(&mut nums);
        println!("merge_sort after nums::{:?}", nums);
    }
}