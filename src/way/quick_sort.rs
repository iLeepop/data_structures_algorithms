fn quick_sort(a: &mut Vec<i32>) -> Vec<i32> {
    if a.len() <= 1 {
        // println!("{:?}", a);
        return a.to_vec();
    } else if a.len() == 2 {
        if a[0] > a[1] {
            a.swap(0, 1);
        }
        return a.to_vec();
    }
    let sert = quick_sort_rec(a);
    let (r, l) = a.split_at(sert + 1);
    // println!("{:?}, {:?}", r, l);
    let rr = quick_sort(&mut r.to_vec());
    let ll = quick_sort(&mut l.to_vec());
    let t = rr.into_iter().chain(ll.into_iter()).collect::<Vec<_>>();
    t
}

fn quick_sort_rec(a: &mut Vec<i32>) -> usize {
    let mut sert = 0;
    let base = a[sert];
    let mut l_flag = true;
    let mut r_flag = true;

    while l_flag || r_flag {
        if sert == a.len() - 1 {
            break
        }
        for i in (sert + 1)..a.len() {
            // println!("left:{:?}", i);
            if a[i] < base {
                // println!("change!");
                a.swap(i, sert);
                sert = i;
                l_flag = true;
                break
            } else {
                // println!("left continue!");
                l_flag = false;
            }
        }
        if sert == 0 {
            break
        }
        for i in 0..sert {
            // println!("right:{:?}", i);
            if a[i] > base {
                // println!("change!");
                a.swap(i, sert);
                sert = i;
                r_flag = true;
                break
            } else {
                // println!("right continue!");
                r_flag = false;
            }
        }
    }
    
    sert
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_quick_sort() {
        let mut a_v = vec![3, 2, -1, 4, 5, 9, 6, 7, 10, -14, 21, 9, 1, 2, 18, -20, -3, 8, 7, 19, 31, 24, 25, -99, 17];
        let start_time = std::time::Instant::now();
        let r = quick_sort(&mut a_v);
        let duration = start_time.elapsed();
        println!("排序结果:{:?},总耗时:{:?}", r, duration);
    }

    #[test]
    fn test_quick_sort_rec() {
        let mut a_v = vec![3, 2, -1, 4, 5, 9, 6, 7, 10, -14, 21, 9, 1, 2, 18, -20, -3, 8, 7, 19, 31, 24, 25, -99, 17];
        let start_time = std::time::Instant::now();
        a_v.sort();
        let duration = start_time.elapsed();
        println!("排序结果:{:?},总耗时:{:?}", a_v, duration);
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
                    break
                }
            }
        }

        fn sl1() {
            let mut arr = vec![2,4,3];
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