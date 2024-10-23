/// 梳排序
/// O(nlogn)
/// O(1)

pub fn comb_sort(nums: &mut [i32]) {
    if  nums.len() <= 1 {return;}

    let mut i;
    let mut gap = nums.len();

    while gap > 0 {
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len() {
            if nums[i-gap] > nums[i] {
                nums.swap(i-gap, i);
            }
            i += 1;
        }
    }

    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;
        while i < nums.len() - 1 {
            if nums[i] > nums[i+1] {
                nums.swap(i, i+1);
                exchange = true;
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_sort() {
        let mut nums = vec![6, 7, 8, 3, 4, 5, 9, 1, 2];
        comb_sort(&mut nums);
        assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_comb_sort2() {}
}
