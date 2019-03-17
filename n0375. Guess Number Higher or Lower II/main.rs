// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        // dp[i][j]: i到j的最小代价
        // dp[i][j] = min(k + max(dp[i][k-1], dp[k+1][j])) for k in i..j
        // dp[i][i+1] = i

        let n: usize = n as usize;
        let mut dp: Vec<Vec<i32>> =
            iter::repeat(vec![0; n as usize + 1])
            .take(n as usize + 1).collect();

        for i in 1..n { dp[i][i+1] = i as i32; }

        for step in 1..n {
            for i in 1..=n - step {
                let j = i + step;
                let mut min_money = -1;
                for k in i+1..j {
                    let money = k as i32 + dp[i][k-1].max(dp[k+1][j]);
                    if min_money == -1 || min_money > money { min_money = money; }
                }
                if min_money != -1 { dp[i][j] = min_money; }
            }
        }

        dp[1][n]
    }
}
