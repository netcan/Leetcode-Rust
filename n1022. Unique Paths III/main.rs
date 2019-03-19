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

