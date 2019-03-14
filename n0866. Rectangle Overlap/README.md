
### Rectangle Overlap :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/rectangle-overlap](https://leetcode-cn.com/problems/rectangle-overlap)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 通过日期/Accept Datetime: 2019-03-14 23:56

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::{max, min};
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let (a, b, c, d, e, f, g, h) =
            (rec1[0], rec1[1], rec1[2], rec1[3], rec2[0], rec2[1], rec2[2], rec2[3]);
         // 重叠部分
        min(h, d) - max(f, b) > 0 && min(c, g) - max(a, e) > 0
    }
}

```
