### Binary String With Substrings Representing 1 To N :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n](https://leetcode-cn.com/problems/binary-string-with-substrings-representing-1-to-n)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-04-02 19:28

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        // 最多有s_len ^ 2种子串
        if s.len() * s.len() < n as usize { return false; }
        for i in 1..=n {
            if let None = s.find(&format!("{:b}", i)) {
                return false;
            }
        }
        true
    }
}


```
