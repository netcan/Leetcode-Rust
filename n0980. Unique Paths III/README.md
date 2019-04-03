## Unique Paths III :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/unique-paths-iii](https://leetcode-cn.com/problems/unique-paths-iii)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-19 20:38

### 题目内容
<p>在二维网格 <code>grid</code> 上，有 4 种类型的方格：</p>

<ul>
	<li><code>1</code> 表示起始方格。且只有一个起始方格。</li>
	<li><code>2</code> 表示结束方格，且只有一个结束方格。</li>
	<li><code>0</code> 表示我们可以走过的空方格。</li>
	<li><code>-1</code> 表示我们无法跨越的障碍。</li>
</ul>

<p>返回在四个方向（上、下、左、右）上行走时，从起始方格到结束方格的不同路径的数目，<strong>每一个无障碍方格都要通过一次</strong>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>[[1,0,0,0],[0,0,0,0],[0,0,2,-1]]
<strong>输出：</strong>2
<strong>解释：</strong>我们有以下两条路径：
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2)
2. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2)</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>[[1,0,0,0],[0,0,0,0],[0,0,0,2]]
<strong>输出：</strong>4
<strong>解释：</strong>我们有以下四条路径： 
1. (0,0),(0,1),(0,2),(0,3),(1,3),(1,2),(1,1),(1,0),(2,0),(2,1),(2,2),(2,3)
2. (0,0),(0,1),(1,1),(1,0),(2,0),(2,1),(2,2),(1,2),(0,2),(0,3),(1,3),(2,3)
3. (0,0),(1,0),(2,0),(2,1),(2,2),(1,2),(1,1),(0,1),(0,2),(0,3),(1,3),(2,3)
4. (0,0),(1,0),(2,0),(2,1),(1,1),(0,1),(0,2),(0,3),(1,3),(1,2),(2,2),(2,3)</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>[[0,1],[2,0]]
<strong>输出：</strong>0
<strong>解释：</strong>
没有一条路能完全穿过每一个空的方格一次。
请注意，起始和结束方格可以位于网格中的任意位置。
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= grid.length * grid[0].length <= 20</code></li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    const dx: [i32; 4] = [0, 1, -1, 0];
    const dy: [i32; 4] = [1, 0, 0, -1];

    pub fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &Vec<Vec<i32>>, vis: &mut Vec<Vec<bool>>, (i, j): (usize, usize), path_lenght: i32) -> i32 {
            // 到达终点
            if grid[i][j] == 2 {
                if path_lenght == 0 { return 1; }
                else { return 0; }
            }
            let (m, n) = (grid.len() as i32, grid[0].len() as i32);

            let mut path_num = 0;
            for k in 0..4 {
                let (next_i, next_j) = (
                    i as i32 + Solution::dx[k], j as i32 + Solution::dy[k]
                    );
                if next_i >= 0 && next_i < m &&
                    next_j >= 0 && next_j < n &&
                        !vis[next_i as usize][next_j as usize] &&
                        grid[next_i as usize][next_j as usize] != -1 {
                            vis[next_i as usize][next_j as usize] = true;

                            path_num += dfs(grid, vis,
                                (next_i as usize, next_j as usize),
                                path_lenght - 1);

                            vis[next_i as usize][next_j as usize] = false;
                        }
            }
            path_num
        }

        let (m, n) = (grid.len(), grid[0].len());
        let mut vis: Vec<Vec<bool>> =
            iter::repeat(vec![false; n]).take(m).collect();
        let path_lenght = grid.iter().flatten().filter(|&x| *x == 0).count() as i32;
        let start_pos = grid.iter().flatten().position(|&x| x == 1).unwrap();
        vis[start_pos / n][start_pos % n] = true;

        dfs(&grid, &mut vis, (start_pos / n, start_pos % n), path_lenght + 1)
    }
}


```
