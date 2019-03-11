
### Single Number :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/single-number](https://leetcode-cn.com/problems/single-number)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.7 MB
- 通过日期/Accept Datetime: 2019-03-10 01:06

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |a, b| { a ^ b })
    }
}

```
