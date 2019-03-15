
### Is Subsequence :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/is-subsequence](https://leetcode-cn.com/problems/is-subsequence)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 4.6 MB
- 通过日期/Accept Datetime: 2019-03-15 10:02

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.len() == 0 { return true; }
        let (s, t) = (s.into_bytes(), t.into_bytes());
        let (mut i, mut j) = (0, 0);
        while i < s.len() && j < t.len() {
            if s[i] == t[j] {
                i += 1;
                if i == s.len() { return true; }
            }
            j += 1;
        }
        false
    }
}


```
