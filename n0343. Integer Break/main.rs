// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut dp = [0; 60];
        for x in (2..=n) {
            for i in (1..=x/2) {
                let j = x - i;
                dp[x] = max(dp[x], max(max(max(i * j, dp[i] * dp[j]), dp[i] * j), i * dp[j]));
            }
        }
        dp[n] as i32
    }
}

