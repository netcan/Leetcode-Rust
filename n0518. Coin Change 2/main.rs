// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter::repeat;
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // dp[i][m]: 前i种硬币凑成m的个数
        // dp[i][m] = dp[i][m-coins[i]] + dp[i-1][m]
        // dp[i][0] = 1, dp[0][m] = 0

        let mut dp: Vec<Vec<i32>> =
            repeat(vec![0; amount as usize + 1]).take(coins.len() + 1).collect();
        for i in 0..=coins.len() { dp[i][0] = 1; }

        for i in 0..coins.len() {
            for m in 1..=amount {
                dp[i + 1][m as usize] = if m >= coins[i] {
                    dp[i + 1][(m - coins[i]) as usize]
                } else {
                    0
                } + dp[i][m as usize];
            }
        }
        dp[coins.len()][amount as usize]
    }
}

