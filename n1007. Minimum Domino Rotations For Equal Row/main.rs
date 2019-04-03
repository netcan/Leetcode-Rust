// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::mem::swap;
impl Solution {
    pub fn min_domino_rotations(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        let mut ans = -1;
        for point in (1..=6) {
            for _ in 0..2 {
                let mut swap_times = 0;
                for (i, j) in a.iter().zip(b.iter()) {
                    if *i == point { continue; }
                    else if *j == point { swap_times += 1; }
                    else { swap_times = -1; break; }
                }
                if swap_times != -1 && (ans == -1 || ans > swap_times) {
                    ans = swap_times;
                }
                swap(&mut a, &mut b);
            }
        }
        ans
    }
}
