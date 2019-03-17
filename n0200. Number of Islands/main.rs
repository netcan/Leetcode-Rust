// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
impl Solution {
    const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        if grid.len() == 0 {
            return 0;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut cnt: i32 = 0;
        for j in (0..m) {
            for i in (0..n) {
                if grid[j][i] == '1' && Solution::find_island(&mut grid, (j as i32, i as i32)) {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn find_island(grid: &mut Vec<Vec<char>>, pos: (i32, i32)) -> bool {
        if grid[pos.0 as usize][pos.1 as usize] == '0' {
            false
        } else {
            let mut que = VecDeque::new();
            let (m, n) = (grid.len(), grid[0].len());
            grid[pos.0 as usize][pos.1 as usize] = '0';
            que.push_back(pos);
            while !que.is_empty() {
                let last_pos = que.pop_front().unwrap();
                for &d in &Solution::dxy {
                    let next_pos:(i32, i32) = (last_pos.0 + d.0, last_pos.1 + d.1);
                    if next_pos.0 >= 0 && next_pos.0 < m as i32 &&
                        next_pos.1 >= 0 && next_pos.1 < n as i32 && 
                        grid[next_pos.0 as usize][next_pos.1 as usize] == '1' {
                            grid[next_pos.0 as usize][next_pos.1 as usize] = '0';
                            que.push_back(next_pos);
                        }
                }
            }

            true
        }
    }

}

