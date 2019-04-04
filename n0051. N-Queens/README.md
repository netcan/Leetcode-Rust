## N-Queens :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/n-queens](https://leetcode-cn.com/problems/n-queens)
- 执行时间: 8 ms 
- 内存消耗: 2.6 MB
- 通过日期: 2019-03-10 12:57

### 题目内容
---
<p><em>n </em>皇后问题研究的是如何将 <em>n</em> 个皇后放置在 <em>n</em>×<em>n</em> 的棋盘上，并且使皇后彼此之间不能相互攻击。</p>

<p><img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/8-queens.png"></p>

<p><small>上图为 8 皇后问题的一种解法。</small></p>

<p>给定一个整数 <em>n</em>，返回所有不同的 <em>n </em>皇后问题的解决方案。</p>

<p>每一种解法包含一个明确的 <em>n</em> 皇后问题的棋子放置方案，该方案中 <code>'Q'</code> 和 <code>'.'</code> 分别代表了皇后和空位。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> 4
<strong>输出:</strong> [
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
<strong>解释:</strong> 4 皇后问题存在两个不同的解法。
</pre>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut board: Vec<Vec<bool>> = iter::repeat(
                iter::repeat(false).take(n as usize).collect()
            ).take(n as usize).collect();
        let mut solv: Vec<Vec<String>> = Vec::new();
        Solution::total_n_queens_(0, &mut board, &mut solv);
        solv
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

    fn total_n_queens_(depth: usize, board: &mut Vec<Vec<bool>>, solv: &mut Vec<Vec<String>>) {
        let n = board.len();
        if depth == n {
            solv.push(
                board.iter().map(|v| {
                    v.iter().map(|x| {
                        if *x { "Q".to_string() } else { ".".to_string() }
                    }).collect::<Vec<String>>().join("")
                }).collect()
            );
            return;
        }
        for i in 0..n {
            // 可放置
            if Solution::check((depth, i), board) {
                board[depth][i] = true;
                Solution::total_n_queens_(depth + 1, board, solv);
                board[depth][i] = false;
            }
        }
    }
}

```