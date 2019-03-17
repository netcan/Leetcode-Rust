### Bitwise AND of Numbers Range :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/bitwise-and-of-numbers-range](https://leetcode-cn.com/problems/bitwise-and-of-numbers-range)
- 执行时间/Runtime: 36 ms 
- 内存消耗/Mem Usage: 2.8 MB
- 通过日期/Accept Datetime: 2019-03-07 11:54

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let diff = n - m + 1;
        let mut bit_len = (diff as f64).log2() as u32;
        if (1 << bit_len) < diff {
            bit_len += 1;
        }

        m & !((1 << bit_len) - 1) & n 
    }
}


```
