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
