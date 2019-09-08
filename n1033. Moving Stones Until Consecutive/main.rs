// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let (a, b) = if a > b { (b, a) } else { (a, b) };
        let (a, c) = if a > c { (c, a) } else { (a, c) };
        let (b, c) = if b > c { (c, b) } else { (b, c) };
        vec![
            if b - a != 2 && c - b != 2 {
                (if b - a > 1 { 1 } else { 0 }) +
                    (if c - b > 1 { 1 } else { 0 })
            } else { 1 },
            c - a - 2
        ]
    }
}

