
### Island Perimeter :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/island-perimeter](https://leetcode-cn.com/problems/island-perimeter)
- 执行时间/Runtime: 32 ms 
- 内存消耗/Mem Usage: 3 MB
- 提交日期/Datetime: 2019-03-11 15:50

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
