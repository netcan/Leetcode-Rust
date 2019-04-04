## Pairs of Songs With Total Durations Divisible by 60 :star:
- 题目地址: [https://leetcode-cn.com/problems/pairs-of-songs-with-total-durations-divisible-by-60](https://leetcode-cn.com/problems/pairs-of-songs-with-total-durations-divisible-by-60)
- 执行时间: 8 ms 
- 内存消耗: 3.4 MB
- 通过日期: 2019-03-17 16:16

### 题目内容
---
<p>在歌曲列表中，第 <code>i</code> 首歌曲的持续时间为 <code>time[i]</code> 秒。</p>

<p>返回其总持续时间（以秒为单位）可被 <code>60</code> 整除的歌曲对的数量。形式上，我们希望索引的数字  <code>i < j</code> 且有 <code>(time[i] + time[j]) % 60 == 0</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[30,20,150,100,40]
<strong>输出：</strong>3
<strong>解释：</strong>这三对的总持续时间可被 60 整数：
(time[0] = 30, time[2] = 150): 总持续时间 180
(time[1] = 20, time[3] = 100): 总持续时间 120
(time[1] = 20, time[4] = 40): 总持续时间 60
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[60,60,60]
<strong>输出：</strong>3
<strong>解释：</strong>所有三对的总持续时间都是 120，可以被 60 整数。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= time.length <= 60000</code></li>
	<li><code>1 <= time[i] <= 500</code></li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut pair = [0; 60];
        for t in &time {
           cnt += pair[((60 - t % 60) % 60) as usize];
           pair[(t % 60) as usize] += 1;
        }
        cnt
    }
}

```