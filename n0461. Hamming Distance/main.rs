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

