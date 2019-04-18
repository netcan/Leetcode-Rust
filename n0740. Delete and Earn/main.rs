// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        // 打家劫舍问题
        let mut gold = [0; 10001];
        for &n in &nums {
            gold[n as usize] += n;
        }
        let mut dp = [0; 10001];
        let (mut take, mut max_earn) = (0, 0);
        for i in 1..=10000usize {
            dp[i] = if i > 1 {
                take = take.max(dp[i - 2]); // dp[0..=i-2].max()
                take + gold[i]
            } else {
                gold[i]
            };
            max_earn = max_earn.max(dp[i]);
        }

        max_earn
    }
}

