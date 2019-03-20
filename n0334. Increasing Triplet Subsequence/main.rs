// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        if nums.len() < 3 { return false; }
        let mut increasing = [i32::max_value(); 3]; // O(1) space

        for num in nums {
            match increasing.binary_search(&num) {
                Err(pos) => {
                    if pos >= 2 { return true; }
                    increasing[pos] = num;
                },
                _ => {}
            }
        }
        false
    }
}

