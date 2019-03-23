// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len() + 1]; // 偷第i家时最大收益
        for i in 0..nums.len() {
            dp[i + 1] =  
                if i > 0 { dp[0..=i-1].iter().max().unwrap() + nums[i] }
                else { nums[i] }
        }
        *dp.iter().max().unwrap()
    }
}

