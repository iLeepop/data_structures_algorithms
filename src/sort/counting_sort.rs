// counting_sort

fn counting_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {
        return;
    }

    let max_bkt_num = 1 + nums.iter().max().unwrap();
    let mut counter = vec![0; max_bkt_num];
    for &v in nums.iter() {
        counter[v] += 1;
    }

    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::random_vec;

    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut nums = [12, 23, 43, 54, 67, 55, 34, 22, 45, 22, 22, 23, 54, 55, 34, 77, 89, 77, 23, 12, 0];
        counting_sort(&mut nums);
        println!("after sort:: {:?}", nums);    
    }
}