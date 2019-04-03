// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut num: i32) -> i32 {
        let digits: Vec<i32> = digits.into_iter().map(|d| d.chars().nth(0).unwrap() as i32 - '0' as i32).collect();
        let num: Vec<usize> = num.to_string().as_bytes().into_iter().map(|b| *b as usize - '0' as usize).rev().collect();

        // 小于数字n的个数
        let num_times: Vec<i32> = (0..10).map(|d| {
            digits.iter().filter(|&n| *n < d).count() as i32
        }).collect();
        // 子集中是否存在数字d
        let has_digit: Vec<i32> = (0..10).map(|d| {
            digits.iter().find(|&d2| { *d2 == d }).is_some() as i32
        }).collect();

        // num_combination[n] = (digits.len()) ^ n
        let num_combination: Vec<i32> = (0..num.len() as u32).map(|n| digits.len().pow(n) as i32).collect();
        // 先从个位到最高位计算
        let mut cnt = num_combination[1..num.len()].iter().sum::<i32>();

        // 加上与num同长的个数
        let mut cnt2 = 1;
        for (idx, &n) in num[0..num.len()].iter().enumerate() {
            cnt2 = num_times[n] * num_combination[idx] + has_digit[n] * cnt2;
        }

        cnt + cnt2
    }
}

