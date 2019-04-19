# Video Stitching :star::star:
- 题目地址: [https://leetcode-cn.com/problems/video-stitching](https://leetcode-cn.com/problems/video-stitching)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-07 12:28

## 题目内容
<p>你将会获得一系列视频片段，这些片段来自于一项持续时长为 <code>T</code> 秒的体育赛事。这些片段可能有所重叠，也可能长度不一。</p>

<p>视频片段 <code>clips[i]</code> 都用区间进行表示：开始于 <code>clips[i][0]</code> 并于 <code>clips[i][1]</code> 结束。我们甚至可以对这些片段自由地再剪辑，例如片段 <code>[0, 7]</code> 可以剪切成 <code>[0, 1] + [1, 3] + [3, 7]</code> 三部分。</p>

<p>我们需要将这些片段进行再剪辑，并将剪辑后的内容拼接成覆盖整个运动过程的片段（<code>[0, T]</code>）。返回所需片段的最小数目，如果无法完成该任务，则返回 <code>-1</code> 。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], T = 10
<strong>输出：</strong>3
<strong>解释：</strong>
我们选中 [0,2], [8,10], [1,9] 这三个片段。
然后，按下面的方案重制比赛片段：
将 [1,9] 再剪辑为 [1,2] + [2,8] + [8,9] 。
现在我们手上有 [0,2] + [2,8] + [8,10]，而这些涵盖了整场比赛 [0, 10]。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>clips = [[0,1],[1,2]], T = 5
<strong>输出：</strong>-1
<strong>解释：</strong>
我们无法只用 [0,1] 和 [0,2] 覆盖 [0,5] 的整个过程。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], T = 9
<strong>输出：</strong>3
<strong>解释： </strong>
我们选取片段 [0,4], [4,7] 和 [6,9] 。
</pre>

<p><strong>示例 4：</strong></p>

<pre><strong>输入：</strong>clips = [[0,4],[2,8]], T = 5
<strong>输出：</strong>2
<strong>解释：</strong>
注意，你可能录制超过比赛结束时间的视频。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= clips.length <= 100</code></li>
	<li><code>0 <= clips[i][0], clips[i][1] <= 100</code></li>
	<li><code>0 <= T <= 100</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, t: i32) -> i32 {
        clips.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        // dp[t]: [0, t]所需要的最少片段数量
        // dp[t] = min(dp[s..=t] + 1)
        let mut dp = [clips.len() as i32 + 1; 101];
        dp[0] = 0;
        for i in 0..clips.len() {
            let (s, e) = (clips[i][0], clips[i][1]);
            for j in s..=e {
                dp[e as usize] = dp[e as usize].min(
                    dp[j as usize] + 1
                    );
            }
        }
        let mut ans = clips.len() as i32 + 1;
        for i in t..=100 {
            ans = ans.min(dp[i as usize]);
        }

        if ans > clips.len() as i32 { -1 }
        else { ans }
    }
}

```