
### Climbing Stairs :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/climbing-stairs](https://leetcode-cn.com/problems/climbing-stairs)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 提交日期/Datetime: 2019-03-05 19:27

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut vec = vec![0; (n + 2) as usize];
        vec[1] = 1;
        vec[2] = 2;
        for i in (3..=n as usize) {
            vec[i] = vec[i - 1] + vec[i - 2];
        }
        vec[n as usize]
    }
}


```
