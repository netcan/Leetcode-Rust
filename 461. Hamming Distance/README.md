
### Hamming Distance :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/hamming-distance](https://leetcode-cn.com/problems/hamming-distance)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 提交日期/Datetime: 2019-03-08 14:38

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut v = x ^ y;
        let mut ret = 0;
        while v > 0 {
            if v & 1 == 1 {
                ret += 1;
            }
            v >>= 1;
        }
        ret
    }
}


```
