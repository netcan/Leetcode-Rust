// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |a, b| { a ^ b })
    }
}
