use core::num;

fn quick_sort(a: &mut [i32], low: usize, high: usize) {
    if low < high {
        let split = quick_sort_rec(a, low, high);
        if split > 1 {
            quick_sort(a, low, split - 1);
        }
        quick_sort(a, split + 1, high);
    }
}

fn quick_sort_rec(nums: &mut [i32], low: usize, high: usize) -> usize {
    let mut l = low;
    let mut r = high;
    loop {
        while l <= r && nums[l] <= nums[low] {
            l += 1;
        }

        while l <= r && nums[r] >= nums[low] {
            r -= 1;
        }

        if l > r {
            break;
        } else {
            nums.swap(l, r);
        }
    }

    nums.swap(r, low);

    r
}

fn maopao(a: &mut Vec<i32>) {
    for i in 0..a.len() {
        for j in 0..a.len() - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{bubble_sort3, quick_sort as qs};

    use super::*;
    use rand::Rng;
    fn random_vec(len: usize, max: i32, min: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let v = (0..len)
            .map(|_| rng.gen_range(min..=max))
            .collect::<Vec<_>>();
        v
    }

    #[test]
    fn test_quick_sort() {
        let len = 3000; let min = -1000; let max = 1000;
        let mut v1 = random_vec(len, max, min);
        let start_time1 = std::time::Instant::now();
        let l = v1.len() - 1;
        quick_sort(&mut v1, 0 , l);
        let duration1 = start_time1.elapsed();
        println!("快速排序结果:,总耗时:{:?}", duration1);

        let mut v4 = random_vec(len, max, min);
        let start_time4 = std::time::Instant::now();
        let  len = v4.len();
        qs(&mut v4, 0, len - 1);
        let duration4 = start_time4.elapsed();
        println!("new快速排序递归结果:,总耗时:{:?}", duration4);

        let mut v2 = random_vec(len, max, min);
        let start_time2 = std::time::Instant::now();
        v2.sort();
        let duration2 = start_time2.elapsed();
        println!("官方排序结果:,总耗时:{:?}", duration2);

        let mut v3 = random_vec(len, max, min);
        let start_time3 = std::time::Instant::now();
        bubble_sort3(&mut v3);
        let duration3 = start_time3.elapsed();
        println!("冒泡排序结果:,总耗时:{:?}", duration3);
    }

    #[test]
    fn test_quick_sort_rec() {
        let mut a_v = random_vec(300, 1000, -1000);
        let start_time = std::time::Instant::now();
        a_v.sort();
        let duration = start_time.elapsed();
        println!("官方排序结果:{:?},总耗时:{:?}", a_v, duration);
    }

    #[test]
    fn test_maopao() {
        let mut a_v = random_vec(300, 1000, -1000);
        let start_time = std::time::Instant::now();
        maopao(&mut a_v);
        let duration = start_time.elapsed();
        println!("冒泡排序结果:{:?},总耗时:{:?}", a_v, duration);
    }

    #[test]
    fn test_loop() {
        fn lw1() {
            let mut flag = true;
            while flag {
                for i in 0..=10 {
                    println!("{}", i);
                    if i == 5 {
                        flag = false;
                    }
                }
                for i in 10..=20 {
                    println!("{}", i);
                }
            }
        }
        fn lw2() {
            for i in 0..=10 {
                println!("{}", i);
                if i == 5 {
                    break;
                }
            }
        }

        fn sl1() {
            let mut arr = vec![2, 4, 3];
            let s_arr = arr.split_at(0);
            println!("{:?}", s_arr)
        }

        fn lw3() {
            for i in 0..0 {
                println!("{}", i);
            }
        }
        // lw1();
        // lw2();
        sl1();
        lw3();
    }
}
