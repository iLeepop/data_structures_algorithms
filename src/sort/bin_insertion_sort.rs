/// binary insertion sort

pub fn bin_insertion_sort(nums: &mut [i32]) {
    let mut temp;
    let mut left; let mut mid; let mut right;

    for i in 1..nums.len() {
        left = 0;
        right = i - 1;
        temp = nums[i];

        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid] {
                if 0 == mid {break;}
                right = mid - 1;
            } else {
                left = mid +  1;
            }
        }

        for j in (left..=i-1).rev() {
            nums.swap(j, j+1);
        }

        if left != i {
            nums[left] = temp;
        }
    }
}
