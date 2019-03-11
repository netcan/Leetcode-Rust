// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let a: HashSet<i32> = nums1.iter().cloned().collect();
        let b: HashSet<i32> = nums2.iter().cloned().collect();
        
        a.intersection(&b).cloned().collect()
    }
}
