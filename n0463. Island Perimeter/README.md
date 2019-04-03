## Island Perimeter :star:
- 题目地址: [https://leetcode-cn.com/problems/island-perimeter](https://leetcode-cn.com/problems/island-perimeter)
- 执行时间: 32 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-03-11 15:50

### 题目内容
<p>给定一个包含 0 和 1 的二维网格地图，其中 1 表示陆地 0 表示水域。</p>

<p>网格中的格子水平和垂直方向相连（对角线方向不相连）。整个网格被水完全包围，但其中恰好有一个岛屿（或者说，一个或多个表示陆地的格子相连组成的岛屿）。</p>

<p>岛屿中没有“湖”（“湖” 指水域在岛屿内部且不和岛屿周围的水相连）。格子是边长为 1 的正方形。网格为长方形，且宽度和高度均不超过 100 。计算这个岛屿的周长。</p>



<p><strong>示例 :</strong></p>

<pre><strong>输入:</strong>
[[0,1,0,0],
 [1,1,1,0],
 [0,1,0,0],
 [1,1,0,0]]

<strong>输出:</strong> 16

<strong>解释:</strong> 它的周长是下面图片中的 16 个黄色的边：

<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/island.png">
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut p = 0;

        for j in 0..grid.len() {
            for i in 0..grid[0].len() {
                if grid[j][i] == 1 {
                    p += 4 - Solution::grid_num(&grid, (j as i32, i as i32));
                }
            }
        }
        p
    }

    fn grid_num(grid: &Vec<Vec<i32>>, pos: (i32, i32)) -> i32 {
        const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
        let (m, n) = (grid.len(), grid[0].len());
        let mut cnt: i32 = 0;
        for &d in &Solution::dxy {
            let next_pos:(i32, i32) = (pos.0 + d.0, pos.1 + d.1);
            if next_pos.0 >= 0 && next_pos.0 < m as i32 &&
                next_pos.1 >= 0 && next_pos.1 < n as i32 &&
                    grid[next_pos.0 as usize][next_pos.1 as usize] == 1 {
                        cnt += 1;
                    }
        }
        cnt
    }
}


```
