# Number of Matching Subsequences :star::star:
- 题目地址: [https://leetcode-cn.com/problems/number-of-matching-subsequences](https://leetcode-cn.com/problems/number-of-matching-subsequences)
- 执行时间: 164 ms 
- 内存消耗: 4.7 MB
- 通过日期: 2019-03-15 11:17

## 题目内容
<p>给定字符串 <code>S</code> 和单词字典 <code>words</code>, 求 <code>words[i]</code> 中是 <code>S</code> 的子序列的单词个数。</p>

<pre>
<strong>示例:</strong>
<strong>输入:</strong> 
S = "abcde"
words = ["a", "bb", "acd", "ace"]
<strong>输出:</strong> 3
<strong>解释:</strong> 有三个是 S 的子序列的单词: "a", "acd", "ace"。
</pre>

<p><strong>注意:</strong></p>

<ul>
	<li>所有在<code>words</code>和 <code>S</code> 里的单词都只由小写字母组成。</li>
	<li><code>S</code> 的长度在 <code>[1, 50000]</code>。</li>
	<li><code>words</code> 的长度在 <code>[1, 5000]</code>。</li>
	<li><code>words[i]</code>的长度在<code>[1, 50]</code>。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashMap;
impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let s = s.into_bytes();
        let words: Vec<Vec<u8>> = words.into_iter().map(|w| { w.into_bytes() }).collect();

        // 预处理s
        let mut s_char_pos = HashMap::new();
        for (pos, c) in s.iter().enumerate() {
            let c_pos_list = s_char_pos.entry(c).or_insert(Vec::new());
            c_pos_list.push(pos);
        }

        let mut cnt = 0;
        for word in &words {
            let mut w_pos = 0;
            let mut is_seq = true;
            for w in word {
                if !s_char_pos.contains_key(&w) {
                    is_seq = false; break;
                }
                let pos_list = s_char_pos.get(&w).unwrap();
                match pos_list.binary_search(&w_pos) {
                    Ok(idx) => { w_pos = pos_list[idx] + 1; },
                    Err(idx) => {
                        if(idx >= 0 && idx < pos_list.len()) { w_pos = pos_list[idx] + 1; }
                        else { is_seq = false; break; }
                    }
                }
            }
            if is_seq { cnt += 1; }
        }

        cnt
    }
}


```