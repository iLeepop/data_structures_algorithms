/// insertion sort

pub fn insertion_sort(nums: &mut [i32]) {
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];

        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1];
            pos -= 1;
        }

        nums[pos] = curr;
    }
}
