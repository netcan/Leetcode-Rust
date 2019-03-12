
### Find Smallest Letter Greater Than Target :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target](https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 3.1 MB
- 通过日期/Accept Datetime: 2019-03-05 18:58

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for c in letters.iter() {
            if *c > target {
                return *c;
            }
        }
        letters[0]
    }
}


```
