## Valid Sudoku :star::star:
- 题目地址: [https://leetcode-cn.com/problems/valid-sudoku](https://leetcode-cn.com/problems/valid-sudoku)
- 执行时间: 4 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-17 09:56

### 题目内容
---
<p>判断一个 9x9 的数独是否有效。只需要<strong>根据以下规则</strong>，验证已经填入的数字是否有效即可。</p>

<ol>
	<li>数字 <code>1-9</code> 在每一行只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一列只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一个以粗实线分隔的 <code>3x3</code> 宫内只能出现一次。</li>
</ol>

<p><img src="https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png" style="height: 250px; width: 250px;"></p>

<p><small>上图是一个部分填充的有效的数独。</small></p>

<p>数独部分空格内已填入了数字，空白格用 <code>'.'</code> 表示。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong>
[
  ["5","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
<strong>输出:</strong> true
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong>
[
  ["8","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
<strong>输出:</strong> false
<strong>解释:</strong> 除了第一行的第一个数字从<strong> 5</strong> 改为 <strong>8 </strong>以外，空格内其他数字均与 示例1 相同。
     但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。</pre>

<p><strong>说明:</strong></p>

<ul>
	<li>一个有效的数独（部分已被填充）不一定是可解的。</li>
	<li>只需要根据以上规则，验证已经填入的数字是否有效即可。</li>
	<li>给定数独序列只包含数字 <code>1-9</code> 和字符 <code>'.'</code> 。</li>
	<li>给定数独永远是 <code>9x9</code> 形式的。</li>
</ul>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 快速检索第i, j列, 第k宫的数字是否被占用
        let mut line   = [[false; 9]; 9]; // j
        let mut column = [[false; 9]; 9]; // i
        let mut ceil   = [[false; 9]; 9]; // k
        // 初始化
        for j in 0..9 as usize {
            for i in 0..9 as usize {
                let num = match board[j][i].to_digit(10) {
                    Some(n) => (n - 1) as usize,
                    None => continue
                };
                if line[j][num] || column[i][num] || ceil[Solution::ceil_k((j, i))][num] {
                    return false;
                }
                line[j][num] = true;
                column[i][num] = true;
                ceil[Solution::ceil_k((j, i))][num] = true;
            }
        }
        return true;
    }

    // 求出pos属于第几个ceil
    fn ceil_k(pos: (usize, usize)) -> usize {
        return (pos.0 / 3) * 3 + pos.1 / 3;
    }
}

```