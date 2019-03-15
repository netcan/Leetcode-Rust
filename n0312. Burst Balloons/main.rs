// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn max_coins(mut nums: Vec<i32>) -> i32 {
        // dp[i][j]: 从i到j获得的最大硬币数量
        // coin[k]: 戳爆第k个气球得分
        // dp[i][j] = max(dp[i][j], dp[i][k - 1] + coin[i-1]*coin[k]*coin[j+1] + dp[k+1][j])
        let n = nums.len();
        if n == 0 { return 0; }
        else if n == 1 { return nums[0]; }

        let mut coin = vec![1]; coin.append(&mut nums); coin.push(1);

        let mut dp:Vec<Vec<i32>> =
            iter::repeat(vec![0; n + 1]).take(n + 1).collect();
        for i in 1..=n { dp[i][i] = coin[i-1] * coin[i] * coin[i+1]; }

        for step in 1..n {
            for i in 1..=n-step {
                let j = i + step;
                for k in i..=j {
                    let left = if k - 1 < i { 0 } else { dp[i][k - 1] };
                    let right = if k + 1 > j { 0 } else { dp[k + 1][j] };
                    dp[i][j] = dp[i][j].max(left + right + coin[i-1] * coin[k] * coin[j+1]);
                }
            }
        }
        dp[1][n]
    }
}
