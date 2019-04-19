// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        let max_diff = *a.iter().max().unwrap() - *a.iter().min().unwrap();
        if max_diff == 0 { return a.len() as i32; }
        // dp[i][diff]: 以第i个数为结尾，[0..i]这个子序列以diff为等差的最大长度
        // dp[i][diff] = max(dp[i][diff], dp[j][diff] + 1) for j in 0..i
        // 考虑到diff可能为负数，所以加个偏移，使其下标diff_idx >= 0
        let mut dp = vec![vec![1; (max_diff * 2) as usize + 1]; a.len()];
        let mut ans = 1;
        for i in 1..a.len() {
            for j in 0..i {
                let diff = a[i] - a[j] + max_diff; // 令diff >= 0
                dp[i][diff as usize] = dp[i][diff as usize].max(dp[j][diff as usize] + 1);
                ans=ans.max(dp[i][diff as usize]);
            }
        }

        ans
    }
}
