// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::Ordering;
impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        if envelopes.len() <= 0 { return 0; }
        envelopes.sort();

        println!("{:?}", envelopes);
        let mut dp = vec![1; envelopes.len()]; // 以envelopes[i]为结尾的最大数量

        for i in 0..envelopes.len() {
            for j in i+1..envelopes.len() {
                if envelopes[i][0] < envelopes[j][0] && 
                    envelopes[i][1] < envelopes[j][1] {
                        dp[j] = dp[j].max(dp[i] + 1);
                    }
            }
        }

        *dp.iter().max().unwrap()
    }
}

