### Number of Longest Increasing Subsequence :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/number-of-longest-increasing-subsequence](https://leetcode-cn.com/problems/number-of-longest-increasing-subsequence)
- 执行时间/Runtime: 24 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-21 12:17

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return nums.len() as i32; }
        let mut dp = vec![1; nums.len()]; // dp[i]以nums[i]结尾的LIS
        let mut dp_len = vec![1; nums.len()];
        let (mut max_len, mut cnt) = (1, 0);
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[j] > nums[i] {
                    if dp[i] + 1 > dp[j] {
                        dp[j] = dp[i] + 1;
                        dp_len[j] = dp_len[i];
                    } else if dp[i]+ 1 == dp[j]  {
                        dp_len[j] += dp_len[i];
                    }
                    max_len = max_len.max(dp[j]);
                } 
            }
        }
        let mut cnt = 0;
        for (i, &len) in dp.iter().enumerate() {
            if len == max_len { cnt += dp_len[i]; }
        }
        cnt
    }
}


```
