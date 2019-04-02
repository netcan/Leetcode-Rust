### Best Sightseeing Pair :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/best-sightseeing-pair](https://leetcode-cn.com/problems/best-sightseeing-pair)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 3.5 MB
- 通过日期/Accept Datetime: 2019-04-02 17:10

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn max_score_sightseeing_pair(a: Vec<i32>) -> i32 {
        let (mut max_score, mut max_pair) = (0, 0);
        for i in 0..a.len() {
            max_score = (a[i] - i as i32 + max_pair).max(max_score);
            max_pair = max_pair.max(a[i] + i as i32);
        }

        max_score
    }
}


```
