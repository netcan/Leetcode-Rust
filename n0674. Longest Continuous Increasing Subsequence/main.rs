// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 { return 0; }
        let (mut length, mut max_length) = (1, 1);
        nums.iter().skip(1).fold(nums[0], |a, &b| {
            if a < b { length += 1; max_length = max(max_length, length) }
            else { length = 1; }
            b
        });
        max_length
    }
}

