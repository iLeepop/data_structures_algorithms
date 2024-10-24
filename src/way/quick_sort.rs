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
    let (r, l) = a.split_at(a.len() >> 1);
    // println!("{:?}, {:?}", r, l);
    let rr = quick_sort(&mut r.to_vec());
    let ll = quick_sort(&mut l.to_vec());
    let t = rr.into_iter().chain(ll.into_iter()).collect::<Vec<_>>();
    t
}

fn quick_sort_rec(a: &mut Vec<i32>) -> usize {
    // let mut sert = 0;
    let mut sert = a.len() >> 1;
    let base = a[sert];
    let mut l_flag = true;
    let mut r_flag = true;

    while l_flag || r_flag {
        if sert == a.len() - 1 {
            break;
        }
        for i in (sert + 1)..a.len() {
            // println!("left:{:?}", i);
            if a[i] < base {
                // println!("change!");
                a.swap(i, sert);
                sert = i;
                l_flag = true;
                break;
            } else {
                // println!("left continue!");
                l_flag = false;
            }
        }
        if sert == 0 {
            break;
        }
        for i in 0..sert {
            // println!("right:{:?}", i);
            if a[i] > base {
                // println!("change!");
                a.swap(i, sert);
                sert = i;
                r_flag = true;
                break;
            } else {
                // println!("right continue!");
                r_flag = false;
            }
        }
    }

    sert
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
    use super::*;
    use rand::Rng;
    fn random_vec(len: usize, max: i32, min: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        let v = (0..len)
            .map(|_| rng.gen_range(min..=max))
            .collect::<Vec<_>>();
        // let mut v = Vec::with_capacity(len);
        // for _ in 0..len {
        //     v.push(rng.gen_range(min..max));
        // }
        v
    }

    #[test]
    fn test_quick_sort() {
        let mut v1 = random_vec(300, 1000, -1000);
        let start_time1 = std::time::Instant::now();
        let r = quick_sort(&mut v1);
        let duration1 = start_time1.elapsed();
        println!("快速排序结果:,总耗时:{:?}", duration1);

        let mut v2 = random_vec(300, 1000, -1000);
        let start_time2 = std::time::Instant::now();
        v2.sort();
        let duration2 = start_time2.elapsed();
        println!("官方排序结果:,总耗时:{:?}", duration2);

        let mut v3 = random_vec(300, 1000, -1000);
        let start_time3 = std::time::Instant::now();
        maopao(&mut v3);
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
