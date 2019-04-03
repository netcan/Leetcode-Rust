## Find Smallest Letter Greater Than Target :star:
- 题目地址: [https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target](https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target)
- 执行时间: 8 ms 
- 内存消耗: 3.1 MB
- 通过日期: 2019-03-05 18:58

### 题目内容
<p>给定一个只包含小写字母的有序数组<code>letters</code> 和一个目标字母 <code>target</code>，寻找有序数组里面比目标字母大的最小字母。</p>

<p>数组里字母的顺序是循环的。举个例子，如果目标字母<code>target = 'z'</code> 并且有序数组为 <code>letters = ['a', 'b']</code>，则答案返回 <code>'a'</code>。</p>

<p><strong>示例:</strong></p>

<pre>
<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "a"
<strong>输出:</strong> "c"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "c"
<strong>输出:</strong> "f"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "d"
<strong>输出:</strong> "f"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "g"
<strong>输出:</strong> "j"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "j"
<strong>输出:</strong> "c"

<strong>输入:</strong>
letters = ["c", "f", "j"]
target = "k"
<strong>输出:</strong> "c"
</pre>

<p><strong>注:</strong></p>

<ol>
	<li><code>letters</code>长度范围在<code>[2, 10000]</code>区间内。</li>
	<li><code>letters</code> 仅由小写字母组成，最少包含两个不同的字母。</li>
	<li>目标字母<code>target</code> 是一个小写字母。</li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for c in letters.iter() {
            if *c > target {
                return *c;
            }
        }
        letters[0]
    }
}


```
