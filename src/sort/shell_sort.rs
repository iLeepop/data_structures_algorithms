// shell_sort.rs

fn shell_sort(nums: &mut [i32]) {
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize) {
        let mut i = start + gap;
        while i < nums.len() {
            let mut pos =  i;
            let curr = nums[pos];
            while pos >= gap && curr < nums[pos - gap] {
                nums[pos] = nums[pos - gap];
                pos -=  gap;
            }
            nums[pos] = curr;
            i += gap;
        }
    }

    let mut gap = nums.len() / 2;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

#[cfg(test)]
mod test {

    use super::shell_sort;

    #[test]
    fn test1() {
        let mut nums = [54, 32, 99, 18, 75, 31, 56, 21, 22];
        shell_sort(&mut nums);
        println!("sort result::{:?}", nums);
    }
}
