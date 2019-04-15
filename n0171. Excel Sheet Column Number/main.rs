// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .fold(0, |acc, d| { acc * 26 + (d - 'A' as u8 + 1) as i32 })
    }
}

