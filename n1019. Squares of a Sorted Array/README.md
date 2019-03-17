### Squares of a Sorted Array :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/squares-of-a-sorted-array](https://leetcode-cn.com/problems/squares-of-a-sorted-array)
- 执行时间/Runtime: 16 ms 
- 内存消耗/Mem Usage: 3.1 MB
- 通过日期/Accept Datetime: 2019-03-11 22:29

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn sorted_squares(a: Vec<i32>) -> Vec<i32> {
        let mut a = a.iter().map(|&x| x * x).collect::<Vec<i32>>();
        a.sort();
        a
    }
}

```
