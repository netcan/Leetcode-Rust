
### N-Queens II :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/n-queens-ii](https://leetcode-cn.com/problems/n-queens-ii)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 提交日期/Datetime: 2019-03-10 12:44

```rust
use std::iter;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut board: Vec<Vec<bool>> = iter::repeat(
                iter::repeat(false).take(n as usize).collect()
            ).take(n as usize).collect();
        Solution::total_n_queens_(0, &mut board)
    }

    fn check(pos: (usize, usize), board: &mut Vec<Vec<bool>>) -> bool {
        let n = board.len();
        for j in 0..n {
            for i in 0..n {
                if j == pos.0 || i == pos.1 ||
                    j + i == pos.1 + pos.0 || i + pos.0 == pos.1 + j {
                        if board[j][i] {
                            return false;
                        }
                }
            }
        }
        true
    }

    fn total_n_queens_(depth: usize, board: &mut Vec<Vec<bool>>) -> i32 {
        let n = board.len();
        if depth == n {
            return 1;
        }
        let mut ret = 0;
        for i in 0..n {
            // 可放置
            if Solution::check((depth, i), board) {
                board[depth][i] = true;
                ret += Solution::total_n_queens_(depth + 1, board);
                board[depth][i] = false;
            }
        }
        ret
    }
}
```
