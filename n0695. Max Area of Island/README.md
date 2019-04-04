## Max Area of Island :star::star:
- 题目地址: [https://leetcode-cn.com/problems/max-area-of-island](https://leetcode-cn.com/problems/max-area-of-island)
- 执行时间: 8 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-11 15:35

### 题目内容
---
<p>给定一个包含了一些 0 和 1的非空二维数组 <code>grid</code> , 一个 <strong>岛屿</strong> 是由四个方向 (水平或垂直) 的 <code>1</code> (代表土地) 构成的组合。你可以假设二维矩阵的四个边缘都被水包围着。</p>

<p>找到给定的二维数组中最大的岛屿面积。(如果没有岛屿，则返回面积为0。)</p>

<p><strong>示例 1:</strong></p>

<pre>
[[0,0,1,0,0,0,0,1,0,0,0,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,1,1,0,1,0,0,0,0,0,0,0,0],
 [0,1,0,0,1,1,0,0,<strong>1</strong>,0,<strong>1</strong>,0,0],
 [0,1,0,0,1,1,0,0,<strong>1</strong>,<strong>1</strong>,<strong>1</strong>,0,0],
 [0,0,0,0,0,0,0,0,0,0,<strong>1</strong>,0,0],
 [0,0,0,0,0,0,0,1,1,1,0,0,0],
 [0,0,0,0,0,0,0,1,1,0,0,0,0]]
</pre>

<p>对于上面这个给定矩阵应返回 <code>6</code>。注意答案不应该是11，因为岛屿只能包含水平或垂直的四个方向的‘1’。</p>

<p><strong>示例 2:</strong></p>

<pre>
[[0,0,0,0,0,0,0,0]]</pre>

<p>对于上面这个给定的矩阵, 返回 <code>0</code>。</p>

<p><strong>注意: </strong>给定的矩阵<code>grid</code> 的长度和宽度都不超过 50。</p>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::cmp::max;
impl Solution {
    const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area: i32 = 0;
        for j in (0..grid.len()) {
            for i in (0..grid[0].len()) {
                if grid[j][i] == 1 {
                    max_area = max(max_area, Solution::island_area(&mut grid, (j as i32, i as i32)));
                }
            }
        }
        max_area
    }
    fn island_area(grid: &mut Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
        if grid[pos.0 as usize][pos.1 as usize] == 0 {
            0
        } else {
            let mut area = 0;
            let mut que = VecDeque::new();
            let (m, n) = (grid.len(), grid[0].len());
            grid[pos.0 as usize][pos.1 as usize] = 0;
            que.push_back(pos);
            while !que.is_empty() {
                let last_pos = que.pop_front().unwrap();
                area += 1;
                for &d in &Solution::dxy {
                    let next_pos:(i32, i32) = (last_pos.0 + d.0, last_pos.1 + d.1);
                    if next_pos.0 >= 0 && next_pos.0 < m as i32 &&
                        next_pos.1 >= 0 && next_pos.1 < n as i32 && 
                        grid[next_pos.0 as usize][next_pos.1 as usize] == 1 {
                            grid[next_pos.0 as usize][next_pos.1 as usize] = 0;
                            que.push_back(next_pos);
                        }
                }
            }

            area
        }
    }

}


```