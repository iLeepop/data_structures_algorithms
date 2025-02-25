/// 冒泡排序
/// O(n^2)

pub fn bubble_sort1(nums: &mut  [i32]) {
    if nums.len() < 2 {return;}
    for i in 1..nums.len()  {
        for j in 0..nums.len() - i {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1);
            }
        }  
    }
}

pub fn bubble_sort2(nums: &mut  [i32]) {
    let mut len = nums.len() - 1;
    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}

pub fn bubble_sort3(nums: &mut  [i32]) {
    let mut compare = true;
    let mut len = nums.len() - 1;

    while len > 0 && compare {
        compare = false;
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
                compare = true;
            }
        }        len -= 1;
    }
}
