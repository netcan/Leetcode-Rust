## Longest Repeating Character Replacement :star::star:
- 题目地址: [https://leetcode-cn.com/problems/longest-repeating-character-replacement](https://leetcode-cn.com/problems/longest-repeating-character-replacement)
- 执行时间: 12 ms 
- 内存消耗: 3.3 MB
- 通过日期: 2019-03-18 15:50

### 题目内容
<p>给你一个仅由大写英文字母组成的字符串，你可以将任意位置上的字符替换成另外的字符，总共可最多替换 <em>k </em>次。在执行上述操作后，找到包含重复字母的最长子串的长度。</p>

<p><strong>注意:</strong><br>
字符串长度 和 <em>k </em>不会超过 10<sup>4</sup>。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong>
s = "ABAB", k = 2

<strong>输出:</strong>
4

<strong>解释:</strong>
用两个'A'替换为两个'B',反之亦然。
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong>
s = "AABABBA", k = 1

<strong>输出:</strong>
4

<strong>解释:</strong>
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。
</pre>


### 解法
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
