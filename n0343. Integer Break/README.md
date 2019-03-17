### Integer Break :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/integer-break](https://leetcode-cn.com/problems/integer-break)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-07 15:06

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut dp = [0; 60];
        for x in (2..=n) {
            for i in (1..=x/2) {
                let j = x - i;
                dp[x] = max(dp[x], max(max(max(i * j, dp[i] * dp[j]), dp[i] * j), i * dp[j]));
            }
        }
        dp[n] as i32
    }
}


```
