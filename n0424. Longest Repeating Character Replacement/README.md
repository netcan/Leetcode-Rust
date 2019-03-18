### Longest Repeating Character Replacement :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/longest-repeating-character-replacement](https://leetcode-cn.com/problems/longest-repeating-character-replacement)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 3.3 MB
- 通过日期/Accept Datetime: 2019-03-18 15:50

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let chars: HashSet<u8> = HashSet::from_iter(s.iter().cloned());
        let mut ans = 0;
        for c in &chars {
            ans = ans.max(
                Solution::longest_character_replacement(&s, *c, k)
            );
        }
        ans
    }

    fn longest_character_replacement(s: &Vec<u8>, c: u8, k: i32) -> i32 {
        let mut replace_chars = VecDeque::new();
        let mut length = 0;
        let mut i = 0;
        for j in 0..s.len() {
            if s[j] != c {
                replace_chars.push_back(j);
                if replace_chars.len() > k as usize {
                    i = replace_chars.pop_front().unwrap() + 1;
                }
            }
            length = length.max(j - i + 1);
        }

        length as i32
    }
}


```
