// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn largest_sum_after_k_negations(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort();
        let b = a.clone();

        let mut j = 0;
        for _ in 0..k {
            a[j] = -a[j];
            if b[j] < 0 && j + 1 < a.len() {
                if b[j + 1] <= 0 || -b[j] > b[j + 1] {
                    j += 1;
                }
            }
        }
        a.iter().sum()
    }
}
