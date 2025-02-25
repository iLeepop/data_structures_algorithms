///

pub fn cbic_sort(nums: &mut [i32]) {
    for i in 0..nums.len() {
        for j in 0..nums.len() {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}

pub fn cbic_sort2(nums: &mut [i32]) {
    if nums.len() < 2 {return;}

    for i in 1..nums.len()  {
        for j in 0..i {
            if nums[i] < nums[j] {
                nums.swap(i, j);
            }
        }
    }
}
