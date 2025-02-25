/// quick sort

pub fn quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = partition(nums, low, high);
        if split > 1 {
            quick_sort(nums, low, split - 1);
        }
        quick_sort(nums, split + 1, high);
    }
}

pub fn partition(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut lm = low;
    let mut rm = high;
    loop {
        while lm <= rm && nums[lm] <= nums[low] {
            lm += 1;
        }

        while lm <= rm && nums[rm] >= nums[low] {
            rm -= 1;
        }

        if lm > rm {
            break;
        } else {
            nums.swap(lm, rm);
        }
    }

    nums.swap(low, rm);

    rm
}


#[cfg(test)]
mod test {
    use crate::random_vec;

    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = random_vec(3000, 1000, -1000);
        let len = v.len();
        quick_sort(&mut v, 0, len - 1);
        println!("{:?}", v);
    }
}
