/*
1 递 归 算 法 必 须 具 有 基 本 情 况
2 递 归 算 法 必 须 向 基 本 情 况 靠 近
3 递 归 算 法 必 须 以 递 归 方 式 调 用 自 身
*/

fn nums_sum1(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}

fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        nums_sum2(&nums[..nums.len() - 1]) + last
    }
}

// 尾递归版
fn nums_sum1_t(num: i32, nums: &[i32]) -> i32 {
    if 1 == nums.len() {
        num + nums[0]
    } else {
        nums_sum1_t(num + nums[0], &nums[1..])
    }
}

// 递归算进制
const BASESTR: [&str; 16] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F",
];

fn num2str_rec(num: i32, base: i32) -> String {
    if num < base {
        BASESTR[num as usize].to_string()
    } else {
        num2str_rec(num / base, base) + BASESTR[(num % base) as usize]
    }
}

// 汉诺塔
fn hanoi(height: usize, from: &str, to: &str, other: &str) {
    if height >= 1 {
        hanoi(height - 1, from, other, to);
        print!("{} -> {}", from, to);
        hanoi(height - 1, other, to, from);
    }
}

// 动态规划
fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    let mut min_cashes = amount;

    if cashes.contains(&amount) {
        return 1;
    } else {
        for c in cashes
            .iter()
            .filter(|&c| *c <= amount)
            .collect::<Vec<&u32>>()
        {
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }

    min_cashes
}

fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    let mut min_cashes_num = amount;

    if cashes.contains(&amount) {
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        return min_cashes[amount as usize];
    } else {
        for c in cashes
            .iter()
            .filter(|c| *(*c) <= amount)
            .collect::<Vec<&u32>>()
        {
            let cashe_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            if cashe_num < min_cashes_num {
                min_cashes_num = cashe_num;
                min_cashes[amount as usize] = min_cashes_num;
            }
        }
    }

    min_cashes_num
}

fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashes_num {
                min_cashes_num = cashe_num;
            }
        }
        min_cashes[denm as usize] = min_cashes_num;
    }
    min_cashes[amount as usize]
}

fn dp_rec_mc_show(
    cashes: &[u32],
    amount: u32,
    min_cashes: &mut [u32],
    cashes_used: &mut [u32],
) -> u32 {
    for denm in 1..=amount {
        let mut min_cashe_num = denm;
        let mut used_cashe = 1;
        for c in cashes.iter().filter(|&c| *c <= denm).collect::<Vec<&u32>>() {
            let index = (denm - c) as usize;
            let cashes_num = 1 + min_cashes[index];
            if cashes_num < min_cashe_num {
                min_cashe_num = cashes_num;
                used_cashe = *c;
            }
        }
        min_cashes[denm as usize] = min_cashe_num;
        cashes_used[denm as usize] = used_cashe;
    }
    min_cashes[amount as usize]
}

fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("{curr}");
        amount -= curr;
    }
}

fn fibnacci_rec(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } else {
        fibnacci_rec(n - 1) + fibnacci_rec(n - 2)
    }
}

fn fibnacci_dp(n: u32) -> u32 {
    let mut dp = [1, 1];
    for i in 2..=n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i - 1) % 2) as usize;
        let idx3 = ((i - 2) % 2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }

    dp[((n - 1) % 2) as usize]
}

#[cfg(test)]
mod tests {
    use core::num;

    use super::*;

    #[test]
    fn test_sums() {
        let nums = [1, 2, 3, 4, 5, 6];
        let num1 = nums_sum1(&nums);
        let num2 = nums_sum2(&nums);
        // 尾递归
        let num1_t = nums_sum1_t(0, &nums);
        println!("num1: {}, num2: {}, num1_t: {}\n", num1, num2, num1_t);
    }

    #[test]
    fn test_num2str_rec() {
        let num = 1024;
        let base = 2;
        println!("{}的{}进制为{}", num, base, num2str_rec(num, base));
    }

    #[test]
    fn test_hanoi() {
        hanoi(3, "右杆", "左杆", "中杆");
    }

    #[test]
    fn test_rec_mc1() {
        let cashes = [1, 5, 10, 25];
        let amount = 30;
        println!("{}找零货币数量最少为{}", amount, rec_mc1(&cashes, amount));
    }

    #[test]
    fn test_rec_mc2() {
        let cashes = [1, 5, 10, 25];
        let amount = 81u32;
        let mut min_cashes = [0; 82];
        let cashe_num = rec_mc2(&cashes, amount, &mut min_cashes);
        println!("{}找零货币数量最少为{}", amount, cashe_num);
    }

    #[test]
    fn test_dp_rec_mc2() {
        let amount = 81u32;
        let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes: [u32; 82] = [0; 82];
        let cash_num = dp_rec_mc(&cashes, amount, &mut min_cashes);
        println!("Refund for {amount} need {cash_num} cashes");
    }

    #[test]
    fn test_dp_rec_mc_show() {
        let amount = 45u32;
        let cashes = [1, 5, 10, 20, 50];
        let mut min_cashes: [u32; 82] = [0; 82];
        let mut cashes_used: [u32; 82] = [0; 82];
        let cs_num = dp_rec_mc_show(&cashes, amount, &mut min_cashes, &mut cashes_used);
        println!("Refund for {amount} need {cs_num} cashes");
        print_cashes(&cashes_used, amount);
    }

    #[test]
    fn test_fibnacci() {
        let n = 10;
        println!("fibnacci_rec({n}) = {}", fibnacci_rec(n));
        println!("fibnacci_dp({n}) = {}", fibnacci_dp(n));
    }
}
