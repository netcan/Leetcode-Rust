## Minimum Domino Rotations For Equal Row :star::star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-domino-rotations-for-equal-row](https://leetcode-cn.com/problems/minimum-domino-rotations-for-equal-row)
- 执行时间: 12 ms 
- 内存消耗: 3.2 MB
- 通过日期: 2019-03-13 23:19

### 题目内容
<p>在一排多米诺骨牌中，<code>A[i]</code> 和 <code>B[i]</code> 分别代表第 i 个多米诺骨牌的上半部分和下半部分。（一个多米诺是两个从 1 到 6 的数字同列平铺形成的 —— 该平铺的每一半上都有一个数字。）</p>

<p>我们可以旋转第 <code>i</code> 张多米诺，使得 <code>A[i]</code> 和 <code>B[i]</code> 的值交换。</p>

<p>返回能使 <code>A</code> 中所有值或者 <code>B</code> 中所有值都相同的最小旋转次数。</p>

<p>如果无法做到，返回 <code>-1</code>.</p>



<p><strong>示例 1：</strong></p>

<p><img alt="" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/03/08/domino.png" style="height: 161px; width: 200px;"></p>

<pre><strong>输入：</strong>A = [2,1,2,4,2,2], B = [5,2,6,2,3,2]
<strong>输出：</strong>2
<strong>解释：</strong>
图一表示：在我们旋转之前， A 和 B 给出的多米诺牌。
如果我们旋转第二个和第四个多米诺骨牌，我们可以使上面一行中的每个值都等于 2，如图二所示。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>A = [3,5,1,2,3], B = [3,6,3,3,4]
<strong>输出：</strong>-1
<strong>解释：</strong>
在这种情况下，不可能旋转多米诺牌使一行的值相等。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A[i], B[i] <= 6</code></li>
	<li><code>2 <= A.length == B.length <= 20000</code></li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::mem::swap;
impl Solution {
    pub fn min_domino_rotations(mut a: Vec<i32>, mut b: Vec<i32>) -> i32 {
        let mut ans = -1;
        for point in (1..=6) {
            for _ in 0..2 {
                let mut swap_times = 0;
                for (i, j) in a.iter().zip(b.iter()) {
                    if *i == point { continue; }
                    else if *j == point { swap_times += 1; }
                    else { swap_times = -1; break; }
                }
                if swap_times != -1 && (ans == -1 || ans > swap_times) {
                    ans = swap_times;
                }
                swap(&mut a, &mut b);
            }
        }
        ans
    }
}

```
