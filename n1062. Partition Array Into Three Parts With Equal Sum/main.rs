// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        if a.len() < 3 { return false; }
        let sum = a.iter().sum::<i32>() / 3;

        let mut p = [0; 3]; // equal part
        let mut i = 0;
        for k in 0..3 {
            while i < a.len() {
                p[k] += a[i];
                if p[k] == sum { i += 1; break; }
                i += 1;
            }
        }
        return p[0] == p[1] && p[1] == p[2] && p[2] == sum;
    }
}
