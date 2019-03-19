// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut pair = [0; 60];
        for t in &time {
           cnt += pair[((60 - t % 60) % 60) as usize];
           pair[(t % 60) as usize] += 1;
        }
        cnt
    }
}
