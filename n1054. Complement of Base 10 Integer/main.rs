// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 { return 1; }
        let mut n_ = 1;
        while n_ < n { n_ <<= 1; }
        !n & (n_ - 1)
    }
}
