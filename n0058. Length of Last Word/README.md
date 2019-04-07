# Length of Last Word :star:
- 题目地址: [https://leetcode-cn.com/problems/length-of-last-word](https://leetcode-cn.com/problems/length-of-last-word)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-05 19:11

## 题目内容
<p>给定一个仅包含大小写字母和空格 <code>' '</code> 的字符串，返回其最后一个单词的长度。</p>

<p>如果不存在最后一个单词，请返回 0 。</p>

<p><strong>说明：</strong>一个单词是指由字母组成，但不包含任何空格的字符串。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> "Hello World"
<strong>输出:</strong> 5
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() == 0 {
            return 0;
        }
        return words[words.len() - 1].len() as i32;

    }
}


```