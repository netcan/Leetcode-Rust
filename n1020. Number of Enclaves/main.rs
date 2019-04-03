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
