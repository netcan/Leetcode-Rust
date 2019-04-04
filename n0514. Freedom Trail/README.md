## Freedom Trail :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/freedom-trail](https://leetcode-cn.com/problems/freedom-trail)
- 执行时间: 24 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-05 00:18

### 题目内容
---
<p>视频游戏“辐射4”中，任务“通向自由”要求玩家到达名为“Freedom Trail Ring”的金属表盘，并使用表盘拼写特定关键词才能开门。</p>

<p>给定一个字符串 <strong>ring</strong>，表示刻在外环上的编码；给定另一个字符串 <strong>key</strong>，表示需要拼写的关键词。您需要算出能够拼写关键词中所有字符的<strong>最少</strong>步数。</p>

<p>最初，<strong>ring </strong>的第一个字符与12:00方向对齐。您需要顺时针或逆时针旋转 ring 以使 <strong>key </strong>的一个字符在 12:00 方向对齐，然后按下中心按钮，以此逐个拼写完 <strong>key </strong>中的所有字符。</p>

<p>旋转 <strong>ring </strong>拼出 key 字符 <strong>key[i] </strong>的阶段中：</p>

<ol>
	<li>您可以将 <strong>ring </strong>顺时针或逆时针旋转<strong>一个位置</strong>，计为1步。旋转的最终目的是将字符串 <strong>ring </strong>的一个字符与 12:00 方向对齐，并且这个字符必须等于字符 <strong>key[i] 。</strong></li>
	<li>如果字符 <strong>key[i] </strong>已经对齐到12:00方向，您需要按下中心按钮进行拼写，这也将算作 <strong>1 步</strong>。按完之后，您可以开始拼写 <strong>key </strong>的下一个字符（下一阶段）, 直至完成所有拼写。</li>
</ol>

<p><strong>示例：</strong></p>



<center><img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/ring.jpg" style="width: 26%;"></center>
 

<pre><strong>输入:</strong> ring = "godding", key = "gd"
<strong>输出:</strong> 4
<strong>解释:</strong>
 对于 key 的第一个字符 'g'，已经在正确的位置, 我们只需要1步来拼写这个字符。 
 对于 key 的第二个字符 'd'，我们需要逆时针旋转 ring "godding" 2步使它变成 "ddinggo"。
 当然, 我们还需要1步进行拼写。
 因此最终的输出是 4。
</pre>

<p><strong>提示：</strong></p>

<ol>
	<li><strong>ring</strong> 和 <strong>key</strong> 的字符串长度取值范围均为 1 至 100；</li>
	<li>两个字符串中都只有小写字符，并且均可能存在重复字符；</li>
	<li>字符串 <strong>key</strong> 一定可以由字符串 <strong>ring</strong> 旋转拼出。</li>
</ol>

### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.into_bytes();
        let key = key.into_bytes();
        // 记录各个字母的位置，快速检索用
        let mut alpha_pos = vec![Vec::new(); 26];
        for (idx, &alpha) in ring.iter().enumerate() {
            alpha_pos[(alpha - 'a' as u8) as usize].push(idx);
        }

        // dp[k_idx][ring_pos] = min(dp[k_idx - 1][ring_pos] + step), step是上一步到这一步的距离
        let mut dp: Vec<Vec<i32>> =
            vec![vec![(ring.len() * key.len()) as i32; ring.len() + 1]; key.len() + 1];

        dp[0][0] = 0; // 初始状态，转盘指向0点(ring_pos == 0)
        let ring_len = ring.len() as i32;

        for k_idx in 0..key.len() {
            for &k_pos in &alpha_pos[(key[k_idx] - 'a' as u8) as usize] {
                for ring_pos in 0..ring.len() {
                    let step = ((k_pos as i32 - ring_pos as i32 + ring_len) % ring_len).min(
                        (ring_pos as i32 - k_pos as i32 + ring_len) % ring_len
                    );

                    dp[k_idx + 1][k_pos] = dp[k_idx + 1][k_pos].min(
                        dp[k_idx][ring_pos] + step
                    );
                }
            }
        }

        dp[key.len()].iter().min().unwrap() + key.len() as i32
    }
}

```