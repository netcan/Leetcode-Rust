// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn broken_calc(x: i32, mut y: i32) -> i32 {
        if (x >= y) { return x - y; }
        let mut step = 0;
        while y >= 0 {
            if y % 2 == 0 {
                y /= 2;
                step += 1;
            } else {
                y = y / 2 + 1;
                step += 2;
            }
            if x >= y { return step + x - y; }
        }
        step
    }
}

