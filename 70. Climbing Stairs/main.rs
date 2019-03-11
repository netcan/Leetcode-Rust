// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut vec = vec![0; (n + 2) as usize];
        vec[1] = 1;
        vec[2] = 2;
        for i in (3..=n as usize) {
            vec[i] = vec[i - 1] + vec[i - 2];
        }
        vec[n as usize]
    }
}

