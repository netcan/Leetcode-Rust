### Max Area of Island :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/max-area-of-island](https://leetcode-cn.com/problems/max-area-of-island)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-11 15:35

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
