// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
impl Solution {
    pub fn longest_ones(a: Vec<i32>, mut k: i32) -> i32 {
        let mut zeroes = VecDeque::new();
        let mut ans = 0;
        let mut i = 0;
        for j in 0..a.len() {
            if a[j] == 0 {
                zeroes.push_back(j);
                if zeroes.len() > k as usize {
                    i = zeroes.pop_front().unwrap() + 1;
                }
            }

            ans = ans.max(j - i + 1);
        }

        ans as i32
    }
}

