/// 鸡尾酒排序
/// O(n^2) O(n)

pub fn cocktail_sort(nums: &mut [i32]) {
    if nums.len() < 2 {return;}

    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >> 1) {
        if bubble {
            bubble = false;

            for j in i..(len - i - 1) {
                if nums[j] > nums[j + 1] {
                    nums.swap(j, j + 1);
                    bubble = true;
                }
            }

            for j in (i + 1..=(len - i - 1)).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j-1, j);
                    bubble = true;
                }
            }
        } else {
            break;
        }
    }
}
