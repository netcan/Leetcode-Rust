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

