# Excel Sheet Column Title :star:
- 题目地址: [https://leetcode-cn.com/problems/excel-sheet-column-title](https://leetcode-cn.com/problems/excel-sheet-column-title)
- 执行时间: 0 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-04-10 19:58

## 题目内容
<p>给定一个正整数，返回它在 Excel 表中相对应的列名称。</p>

<p>例如，</p>

<pre>    1 -> A
    2 -> B
    3 -> C
    ...
    26 -> Z
    27 -> AA
    28 -> AB 
    ...
</pre>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> 1
<strong>输出:</strong> "A"
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> 28
<strong>输出:</strong> "AB"
</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> 701
<strong>输出:</strong> "ZY"
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn convert_to_title(mut n: i32) -> String {
        let mut title = String::new();
        while n != 0 {
            title = ((((n - 1) % 26) as u8 + 'A' as u8) as char).to_string() + &title;
            n = (n - 1) / 26;
        }
        title
    }
}


```