# Moving Stones Until Consecutive :star:
- 题目地址: [https://leetcode-cn.com/problems/moving-stones-until-consecutive](https://leetcode-cn.com/problems/moving-stones-until-consecutive)
- 执行时间: 0 ms 
- 内存消耗: 1.9 MB
- 通过日期: 2019-05-25 11:52

## 题目内容
<p>三枚石子放置在数轴上，位置分别为 <code>a</code>，<code>b</code>，<code>c</code>。</p>

<p>每一回合，我们假设这三枚石子当前分别位于位置 <code>x, y, z</code> 且 <code>x < y < z</code>。从位置 <code>x</code> 或者是位置 <code>z</code> 拿起一枚石子，并将该石子移动到某一整数位置 <code>k</code> 处，其中 <code>x < k < z</code> 且 <code>k != y</code>。</p>

<p>当你无法进行任何移动时，即，这些石子的位置连续时，游戏结束。</p>

<p>要使游戏结束，你可以执行的最小和最大移动次数分别是多少？ 以长度为 2 的数组形式返回答案：<code>answer = [minimum_moves, maximum_moves]</code></p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>a = 1, b = 2, c = 5
<strong>输出：</strong>[1, 2]
<strong>解释：</strong>将石子从 5 移动到 4 再移动到 3，或者我们可以直接将石子移动到 3。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>a = 4, b = 3, c = 2
<strong>输出：</strong>[0, 0]
<strong>解释：</strong>我们无法进行任何移动。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= a <= 100</code></li>
	<li><code>1 <= b <= 100</code></li>
	<li><code>1 <= c <= 100</code></li>
	<li><code>a != b, b != c, c != a</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let (a, b) = if a > b { (b, a) } else { (a, b) };
        let (a, c) = if a > c { (c, a) } else { (a, c) };
        let (b, c) = if b > c { (c, b) } else { (b, c) };
        vec![
            if b - a != 2 && c - b != 2 {
                (if b - a > 1 { 1 } else { 0 }) +
                    (if c - b > 1 { 1 } else { 0 })
            } else { 1 },
            c - a - 2
        ]
    }
}


```