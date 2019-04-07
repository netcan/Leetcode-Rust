# String Compression :star:
- 题目地址: [https://leetcode-cn.com/problems/string-compression](https://leetcode-cn.com/problems/string-compression)
- 执行时间: 8 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-06 16:03

## 题目内容
<p>给定一组字符，使用<a href="https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95">原地算法</a>将其压缩。</p>

<p>压缩后的长度必须始终小于或等于原数组长度。</p>

<p>数组的每个元素应该是长度为1 的<strong>字符</strong>（不是 int 整数类型）。</p>

<p>在完成<a href="https://baike.baidu.com/item/%E5%8E%9F%E5%9C%B0%E7%AE%97%E6%B3%95">原地</a><strong>修改输入数组</strong>后，返回数组的新长度。</p>



<p><strong>进阶：</strong><br />
你能否仅使用O(1) 空间解决问题？</p>



<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>
["a","a","b","b","c","c","c"]

<strong>输出：</strong>
返回6，输入数组的前6个字符应该是：["a","2","b","2","c","3"]

<strong>说明：</strong>
"aa"被"a2"替代。"bb"被"b2"替代。"ccc"被"c3"替代。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>
["a"]

<strong>输出：</strong>
返回1，输入数组的前1个字符应该是：["a"]

<strong>说明：</strong>
没有任何字符串被替代。
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>
["a","b","b","b","b","b","b","b","b","b","b","b","b"]

<strong>输出：</strong>
返回4，输入数组的前4个字符应该是：["a","b","1","2"]。

<strong>说明：</strong>
由于字符"a"不重复，所以不会被压缩。"bbbbbbbbbbbb"被“b12”替代。
注意每个数字在数组中都有它自己的位置。
</pre>

<p><strong>注意：</strong></p>

<ol>
	<li>所有字符都有一个ASCII值在<code>[35, 126]</code>区间内。</li>
	<li><code>1 <= len(chars) <= 1000</code>。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut repeat_num = 1;
        let mut i = 1;
        while i < chars.len() {
            if chars[i - 1] == chars[i] {
                repeat_num += 1;
            } else {
                if repeat_num >= 2 {
                    let repeat_num_char = repeat_num.to_string();
                    chars.splice((i - repeat_num + 1)..i, repeat_num_char.chars());
                    i = i - repeat_num + 1 + repeat_num_char.len();
                }
                repeat_num = 1;
            }
            i += 1;
        }

        if repeat_num >= 2 {
            chars.splice((chars.len() - repeat_num + 1)..chars.len(), repeat_num.to_string().chars());
        }

        chars.len() as i32
    }
}


```