# Divisor Game :star:
- 题目地址: [https://leetcode-cn.com/problems/divisor-game](https://leetcode-cn.com/problems/divisor-game)
- 执行时间: 0 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-04-14 12:10

## 题目内容
<p>爱丽丝和鲍勃一起玩游戏，他们轮流行动。爱丽丝先手开局。</p>

<p>最初，黑板上有一个数字 <code>N</code> 。在每个玩家的回合，玩家需要执行以下操作：</p>

<ul>
	<li>选出任一 <code>x</code>，满足 <code>0 < x < N</code> 且 <code>N % x == 0</code> 。</li>
	<li>用 <code>N - x</code> 替换黑板上的数字 <code>N</code> 。</li>
</ul>

<p>如果玩家无法执行这些操作，就会输掉游戏。</p>

<p>只有在爱丽丝在游戏中取得胜利时才返回 <code>True</code>，否则返回 <code>false</code>。假设两个玩家都以最佳状态参与游戏。</p>



<ol>
</ol>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>2
<strong>输出：</strong>true
<strong>解释：</strong>爱丽丝选择 1，鲍勃无法进行操作。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>3
<strong>输出：</strong>false
<strong>解释：</strong>爱丽丝选择 1，鲍勃也选择 1，然后爱丽丝无法进行操作。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= N <= 1000</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        if n % 2 == 0 { true }
        else { false }
    }
}

```