### House Robber :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/house-robber](https://leetcode-cn.com/problems/house-robber)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-23 11:50

```rust
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


```
