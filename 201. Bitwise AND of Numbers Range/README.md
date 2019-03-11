
### Bitwise AND of Numbers Range :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/bitwise-and-of-numbers-range](https://leetcode-cn.com/problems/bitwise-and-of-numbers-range)
- 执行时间/Runtime: 24 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 提交日期/Datetime: 2019-03-07 12:05

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let m: u32 = m as u32;
        let n: u32 = n as u32;
        let diff = n - m + 1;
        let mut bit_and = 1;
        while bit_and < diff {
            bit_and <<= 1;
        }
        (m & !(bit_and - 1) & n) as i32
    }
}


```
