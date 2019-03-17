### Sort Array By Parity II :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/sort-array-by-parity-ii](https://leetcode-cn.com/problems/sort-array-by-parity-ii)
- 执行时间/Runtime: 20 ms 
- 内存消耗/Mem Usage: 3 MB
- 通过日期/Accept Datetime: 2019-03-06 11:17

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn sort_array_by_parity_ii(a: Vec<i32>) -> Vec<i32> {
        let mut ret = Vec::with_capacity(a.len());
        unsafe { ret.set_len(a.len()); }
        let (mut i, mut j) = (0, 1);
        for n in &a {
            if n & 1 == 1 {
                ret[j] = *n;
                j += 2;
            } else {
                ret[i] = *n;
                i += 2;
            }
        }
        ret
    }
}


```
