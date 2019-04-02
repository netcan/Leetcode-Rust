### Number of Enclaves :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/number-of-enclaves](https://leetcode-cn.com/problems/number-of-enclaves)
- 执行时间/Runtime: 584 ms 
- 内存消耗/Mem Usage: 3 MB
- 通过日期/Accept Datetime: 2019-03-31 11:33

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
impl Solution {
    const dxy: [(i32, i32); 4] = [(0 ,1), (0, -1), (-1, 0), (1, 0)];
    pub fn num_enclaves(mut a: Vec<Vec<i32>>) -> i32 {
        if a.len() == 0 {
            return 0;
        }
        let (m, n) = (a.len(), a[0].len());
        let mut cnt: i32 = 0;
        for j in 0..m {
            for i in 0..n {
                if a[j][i] == 1 && Solution::find_enclaves(&mut a, (j as i32, i as i32)) {
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn find_enclaves(grid: &mut Vec<Vec<i32>>, pos: (i32, i32)) -> bool {
        if grid[pos.0 as usize][pos.1 as usize] == 0 {
            false
        } else {
            let mut que = VecDeque::new();
            let (m, n) = (grid.len(), grid[0].len());
            let mut mark = vec![false; m * n];
            que.push_back(pos);
            mark[(n as i32 * pos.0 + pos.1) as usize] = true;

            let mut flag = true;

            while !que.is_empty() {
                let last_pos = que.pop_front().unwrap();
                for &d in &Solution::dxy {
                    let next_pos:(i32, i32) = (last_pos.0 + d.0, last_pos.1 + d.1);
                    if next_pos.0 >= 0 && next_pos.0 < m as i32 &&
                        next_pos.1 >= 0 && next_pos.1 < n as i32 {
                            if grid[next_pos.0 as usize][next_pos.1 as usize] == 1 && !mark[(n as i32 * next_pos.0 + next_pos.1) as usize] {
                                que.push_back(next_pos);
                                mark[(n as i32 * next_pos.0 + next_pos.1) as usize] = true;
                            }
                        } else {
                            flag = false;
                        }
                }
            }

            flag
        }
    }

}

```
