// radix_sort

use rand::seq::index;

fn radix_sort(nums: &mut [usize]) {
    if nums.len() <= 1 {return;}

    let max_num = match nums.iter().max() {
        Some(&x) => x,
        None => return,
    };

    let radix = nums.len().next_power_of_two();

    let mut digit = 1;
    while digit <= max_num {
        let index_of = |x| x / digit % radix;
        let mut counter = vec![0; radix];
        for &x in nums.iter() {
            counter[index_of(x)] += 1;
        }
        for i in 1..radix {
            counter[i] += counter[i-1];
        }
        for &x in nums.to_owned().iter().rev() {
            counter[index_of(x)] -= 1;
            nums[counter[index_of(x)]] = x;
        }

        digit *= radix;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_sort() {
        let mut nums = vec![1, 3, 2, 4, 5, 7, 6, 8, 9, 0];
        radix_sort(&mut nums);
        println!("after sort:: {:?}", nums);
    }
}