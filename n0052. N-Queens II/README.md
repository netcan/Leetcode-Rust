## N-Queens II :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/n-queens-ii](https://leetcode-cn.com/problems/n-queens-ii)
- 执行时间: 12 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-10 12:44

### 题目内容
<p><em>n </em>皇后问题研究的是如何将 <em>n</em> 个皇后放置在 <em>n</em>×<em>n</em> 的棋盘上，并且使皇后彼此之间不能相互攻击。</p>

<p><img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/8-queens.png" style="height: 276px; width: 258px;"></p>

<p><small>上图为 8 皇后问题的一种解法。</small></p>

<p>给定一个整数 <em>n</em>，返回 <em>n</em> 皇后不同的解决方案的数量。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> 4
<strong>输出:</strong> 2
<strong>解释:</strong> 4 皇后问题存在如下两个不同的解法。
[
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

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
