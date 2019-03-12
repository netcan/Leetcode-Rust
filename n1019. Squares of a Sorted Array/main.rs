// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut a = a.iter().map(|&x| x * x).collect::<Vec<i32>>();
        a.sort();
        a
    }
}
