## Stone Game :star::star:
- 题目地址: [https://leetcode-cn.com/problems/stone-game](https://leetcode-cn.com/problems/stone-game)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-13 23:20

### 题目内容
<p>亚历克斯和李用几堆石子在做游戏。偶数堆石子<strong>排成一行</strong>，每堆都有正整数颗石子 <code>piles[i]</code> 。</p>

<p>游戏以谁手中的石子最多来决出胜负。石子的总数是奇数，所以没有平局。</p>

<p>亚历克斯和李轮流进行，亚历克斯先开始。 每回合，玩家从行的开始或结束处取走整堆石头。 这种情况一直持续到没有更多的石子堆为止，此时手中石子最多的玩家获胜。</p>

<p>假设亚历克斯和李都发挥出最佳水平，当亚历克斯赢得比赛时返回 <code>true</code> ，当李赢得比赛时返回 <code>false</code> 。</p>



<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[5,3,4,5]
<strong>输出：</strong>true
<strong>解释：</strong>
亚历克斯先开始，只能拿前 5 颗或后 5 颗石子 。
假设他取了前 5 颗，这一行就变成了 [3,4,5] 。
如果李拿走前 3 颗，那么剩下的是 [4,5]，亚历克斯拿走后 5 颗赢得 10 分。
如果李拿走后 5 颗，那么剩下的是 [3,4]，亚历克斯拿走后 4 颗赢得 9 分。
这表明，取前 5 颗石子对亚历克斯来说是一个胜利的举动，所以我们返回 true 。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>2 <= piles.length <= 500</code></li>
	<li><code>piles.length</code> 是偶数。</li>
	<li><code>1 <= piles[i] <= 500</code></li>
	<li><code>sum(piles)</code> 是奇数。</li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}

```
