
### Longest Continuous Increasing Subsequence :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence](https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.7 MB
- 通过日期/Accept Datetime: 2019-03-15 12:09

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let (mut length, mut max_length) = (1, 1);
        nums.iter().skip(1).fold(nums[0], |a, b| {
            if a < *b { length += 1; max_length = max(max_length, length) }
            else { length = 1; }
            *b
        });
        max_length
    }
}


```
