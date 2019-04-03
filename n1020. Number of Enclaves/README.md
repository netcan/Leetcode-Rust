## Number of Enclaves :star::star:
- 题目地址: [https://leetcode-cn.com/problems/number-of-enclaves](https://leetcode-cn.com/problems/number-of-enclaves)
- 执行时间: 16 ms 
- 内存消耗: 3.4 MB
- 通过日期: 2019-03-31 13:07

### 题目内容
<p>给出一个二维数组 <code>A</code>，每个单元格为 0（代表海）或 1（代表陆地）。</p>

<p>移动是指在陆地上从一个地方走到另一个地方（朝四个方向之一）或离开网格的边界。</p>

<p>返回网格中<strong>无法</strong>在任意次数的移动中离开网格边界的陆地单元格的数量。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
<strong>输出：</strong>3
<strong>解释： </strong>
有三个 1 被 0 包围。一个 1 没有被包围，因为它在边界上。</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
<strong>输出：</strong>0
<strong>解释：</strong>
所有 1 都在边界上或可以到达边界。</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 500</code></li>
	<li><code>1 <= A[i].length <= 500</code></li>
	<li><code>0 <= A[i][j] <= 1</code></li>
	<li>所有行的大小都相同</li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::iter::repeat;

impl Solution {
    const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
    pub fn num_enclaves(mut a: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (a.len(), a[0].len());

        let mut mark: Vec<Vec<bool>> = repeat(vec![false; n]).take(m).collect();
        let mut que = VecDeque::new();

        for i in 0..m {
            for j in 0..n {
                if (i == 0 || j == 0 || i == m - 1 || j == n - 1) &&
                    a[i][j] == 1 {
                        mark[i][j] = true;
                        que.push_back((i as i32, j as i32));
                    }
            }
        }

        while !que.is_empty() {
            let last_pos = que.pop_front().unwrap();
            for &d in &Solution::dxy {
                let next_pos:(i32, i32) = (last_pos.0 + d.0, last_pos.1 + d.1);
                if next_pos.0 >= 0 && next_pos.0 < m as i32 &&
                    next_pos.1 >= 0 && next_pos.1 < n as i32 &&
                        a[next_pos.0 as usize][next_pos.1 as usize] == 1 &&
                        !mark[next_pos.0 as usize][next_pos.1 as usize] {
                            mark[next_pos.0 as usize][next_pos.1 as usize] = true;
                            que.push_back(next_pos);
                        }
            }
        }

        let mut cnt: i32 = 0;
        for i in 0..m {
            for j in 0..n {
                if a[i][j] == 1 && !mark[i][j] {
                    cnt += 1;
                }
            }
        }
        cnt
    }
}

```
