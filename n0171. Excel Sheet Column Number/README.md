# Excel Sheet Column Number :star:
- 题目地址: [https://leetcode-cn.com/problems/excel-sheet-column-number](https://leetcode-cn.com/problems/excel-sheet-column-number)
- 执行时间: 4 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-10 19:15

## 题目内容
<p>给定一个Excel表格中的列名称，返回其相应的列序号。</p>

<p>例如，</p>

<pre>    A -> 1
    B -> 2
    C -> 3
    ...
    Z -> 26
    AA -> 27
    AB -> 28 
    ...
</pre>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> "A"
<strong>输出:</strong> 1
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong>"AB"
<strong>输出:</strong> 28
</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入: </strong>"ZY"
<strong>输出:</strong> 701</pre>

<p><strong>致谢：</strong><br>
特别感谢 <a href="http://leetcode.com/discuss/user/ts">@ts</a> 添加此问题并创建所有测试用例。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        s.into_bytes()
            .into_iter()
            .fold(0, |acc, d| { acc * 26 + (d - 'A' as u8 + 1) as i32 })
    }
}


```