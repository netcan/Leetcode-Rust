### Complement of Base 10 Integer :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/complement-of-base-10-integer](https://leetcode-cn.com/problems/complement-of-base-10-integer)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-17 16:04

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 { return 1; }
        let mut n_ = 1;
        while n_ < n { n_ <<= 1; }
        !n & (n_ - 1)
    }
}

```
