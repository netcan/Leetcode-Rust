## Sudoku Solver :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/sudoku-solver](https://leetcode-cn.com/problems/sudoku-solver)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-12 11:22

### 题目内容
<p>编写一个程序，通过已填充的空格来解决数独问题。</p>

<p>一个数独的解法需<strong>遵循如下规则</strong>：</p>

<ol>
	<li>数字 <code>1-9</code> 在每一行只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一列只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一个以粗实线分隔的 <code>3x3</code> 宫内只能出现一次。</li>
</ol>

<p>空白格用 <code>'.'</code> 表示。</p>

<p><img src="http://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png"></p>

<p><small>一个数独。</small></p>

<p><img src="http://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png"></p>

<p><small>答案被标成红色。</small></p>

<p><strong>Note:</strong></p>

<ul>
	<li>给定的数独序列只包含数字 <code>1-9</code> 和字符 <code>'.'</code> 。</li>
	<li>你可以假设给定的数独只有唯一解。</li>
	<li>给定数独永远是 <code>9x9</code> 形式的。</li>
</ul>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::char;
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // 快速检索第i, j列, 第k宫的数字是否被占用
        let mut line   = [[false; 9]; 9]; // j
        let mut column = [[false; 9]; 9]; // i
        let mut ceil   = [[false; 9]; 9]; // k
        let mut origin = [[false; 9]; 9]; // 原始数字位置
        // 初始化
        for j in 0..9 as usize {
            for i in 0..9 as usize {
                let num = match board[j][i].to_digit(10) {
                    Some(n) => (n - 1) as usize,
                    None => continue
                };
                line[j][num] = true;
                column[i][num] = true;
                origin[j][i] = true;
                ceil[Solution::ceil_k((j, i))][num] = true;
            }
        }
        Solution::solve_sudoku_(board, (0, 0), &mut line, &mut column, &mut ceil, &origin);
    }

    fn solve_sudoku_(board:  &mut Vec<Vec<char>>,
                     (j, i): (usize, usize),
                     line:   &mut [[bool; 9]; 9],
                     column: &mut [[bool; 9]; 9],
                     ceil:   &mut [[bool; 9]; 9],
                     origin: &[[bool; 9]; 9]) -> bool {
        if j >= 9 { return true; }
        let next_pos = (j + (i + 1) / 9, (i + 1) % 9);
        if origin[j][i] { return Solution::solve_sudoku_(board, next_pos, line, column, ceil, origin); }

        let mut flag = false;
        for num in 0..9 as usize {
            let k = Solution::ceil_k((j, i));
            if ! line[j][num] && ! column[i][num] && ! ceil[k][num] { // 数字num + 1没用过
                line[j][num] = true;
                column[i][num] = true;
                ceil[k][num] = true;

                board[j][i] = char::from_digit(num as u32 + 1, 10).unwrap();

                flag |= Solution::solve_sudoku_(board, next_pos, line, column, ceil, origin);
                if flag { // 填数ok
                    break;
                } else {
                    line[j][num] = false;
                    column[i][num] = false;
                    ceil[k][num] = false;
                }
            }
        }
        flag
    }

    // 求出pos属于第几个ceil
    fn ceil_k(pos: (usize, usize)) -> usize {
        return (pos.0 / 3) * 3 + pos.1 / 3;
    }
}


```
