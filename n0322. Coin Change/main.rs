// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        // dp[m]: 凑到金额m所需要的最少金币数
        // dp[m] = min(dp[m - coins[i]] + 1, dp[m]), dp[coins[i]] = 1
        if amount == 0 { return 0; }
        let mut dp = vec![amount + 1; amount as usize + 1];
        for c in &coins {
            if *c <= amount { dp[*c as usize] = 1; }
        }

        for i in 0..coins.len() {
            for m in coins[i]..=amount {
                dp[m as usize] = dp[m as usize].min(dp[(m - coins[i]) as usize] + 1);
            }
        }

        if dp[amount as usize] > amount { -1 }
        else { dp[amount as usize] }
    }
}

