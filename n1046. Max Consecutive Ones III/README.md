### Max Consecutive Ones III :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/max-consecutive-ones-iii](https://leetcode-cn.com/problems/max-consecutive-ones-iii)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 3.3 MB
- 通过日期/Accept Datetime: 2019-03-18 11:35

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let mut zeroes = VecDeque::new();
        let mut ans = 0;
        let mut i = 0;
        for j in 0..a.len() {
            if a[j] == 0 {
                zeroes.push_back(j);
                if zeroes.len() > k as usize {
                    i = zeroes.pop_front().unwrap() + 1;
                }
            }

            ans = ans.max(j - i + 1);
        }

        ans as i32
    }
}


```
