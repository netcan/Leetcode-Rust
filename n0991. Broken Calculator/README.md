## Broken Calculator :star::star:
- 题目地址: [https://leetcode-cn.com/problems/broken-calculator](https://leetcode-cn.com/problems/broken-calculator)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-15 09:31

### 题目内容
---
<p>在显示着数字的坏计算器上，我们可以执行以下两种操作：</p>

<ul>
	<li><strong>双倍（Double）：</strong>将显示屏上的数字乘 2；</li>
	<li><strong>递减（Decrement）：</strong>将显示屏上的数字减 1 。</li>
</ul>

<p>最初，计算器显示数字 <code>X</code>。</p>

<p>返回显示数字 <code>Y</code> 所需的最小操作数。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>X = 2, Y = 3
<strong>输出：</strong>2
<strong>解释：</strong>先进行双倍运算，然后再进行递减运算 {2 -> 4 -> 3}.
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>X = 5, Y = 8
<strong>输出：</strong>2
<strong>解释：</strong>先递减，再双倍 {5 -> 4 -> 8}.
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>X = 3, Y = 10
<strong>输出：</strong>3
<strong>解释：</strong>先双倍，然后递减，再双倍 {3 -> 6 -> 5 -> 10}.
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>X = 1024, Y = 1
<strong>输出：</strong>1023
<strong>解释：</strong>执行递减运算 1023 次
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= X <= 10^9</code></li>
	<li><code>1 <= Y <= 10^9</code></li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn broken_calc(x: i32, mut y: i32) -> i32 {
        if (x >= y) { return x - y; }
        let mut step = 0;
        while y >= 0 {
            if y % 2 == 0 {
                y /= 2;
                step += 1;
            } else {
                y = y / 2 + 1;
                step += 2;
            }
            if x >= y { return step + x - y; }
        }
        step
    }
}


```