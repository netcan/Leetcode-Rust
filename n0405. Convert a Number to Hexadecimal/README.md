## Convert a Number to Hexadecimal :star:
- 题目地址: [https://leetcode-cn.com/problems/convert-a-number-to-hexadecimal](https://leetcode-cn.com/problems/convert-a-number-to-hexadecimal)
- 执行时间: 0 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-06 09:52

### 题目内容
---
<p>给定一个整数，编写一个算法将这个数转换为十六进制数。对于负整数，我们通常使用 <a href="https://baike.baidu.com/item/%E8%A1%A5%E7%A0%81/6854613?fr=aladdin">补码运算</a> 方法。</p>

<p><strong>注意:</strong></p>

<ol>
	<li>十六进制中所有字母(<code>a-f</code>)都必须是小写。</li>
	<li>十六进制字符串中不能包含多余的前导零。如果要转化的数为0，那么以单个字符<code>'0'</code>来表示；对于其他情况，十六进制字符串中的第一个字符将不会是0字符。 </li>
	<li>给定的数确保在32位有符号整数范围内。</li>
	<li><strong>不能使用任何由库提供的将数字直接转换或格式化为十六进制的方法。</strong></li>
</ol>

<p><strong>示例 1：</strong></p>

<pre>
输入:
26

输出:
"1a"
</pre>

<p><strong>示例 2：</strong></p>

<pre>
输入:
-1

输出:
"ffffffff"
</pre>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    const H:[char;16] = [
        '0', '1', '2', '3', '4', '5',
        '6', '7', '8', '9', 'a', 'b',
        'c', 'd', 'e', 'f'
    ];
    pub fn to_hex(num: i32) -> String {
        let mut num = num as u32;
        if num == 0 {
            return "0".to_owned();
        }
        let mut ret = String::new();
        while num > 0 {
            ret.push(Solution::H[(num % 16) as usize]);
            num /= 16;
        }
        ret.chars().rev().collect()
    }
}


```