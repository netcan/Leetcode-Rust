### Pairs of Songs With Total Durations Divisible by 60 :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/pairs-of-songs-with-total-durations-divisible-by-60](https://leetcode-cn.com/problems/pairs-of-songs-with-total-durations-divisible-by-60)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 3.4 MB
- 通过日期/Accept Datetime: 2019-03-17 16:16

```rust
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

```
