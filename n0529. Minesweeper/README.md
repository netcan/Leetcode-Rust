# Minesweeper :star::star:
- 题目地址: [https://leetcode-cn.com/problems/minesweeper](https://leetcode-cn.com/problems/minesweeper)
- 执行时间: 28 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-04-07 22:35

## 题目内容
<p>让我们一起来玩扫雷游戏！</p>

<p>给定一个代表游戏板的二维字符矩阵。 <strong>'M'</strong> 代表一个<strong>未挖出的</strong>地雷，<strong>'E'</strong> 代表一个<strong>未挖出的</strong>空方块，<strong>'B' </strong>代表没有相邻（上，下，左，右，和所有4个对角线）地雷的<strong>已挖出的</strong>空白方块，<strong>数字</strong>（'1' 到 '8'）表示有多少地雷与这块<strong>已挖出的</strong>方块相邻，<strong>'X'</strong> 则表示一个<strong>已挖出的</strong>地雷。</p>

<p>现在给出在所有<strong>未挖出的</strong>方块中（'M'或者'E'）的下一个点击位置（行和列索引），根据以下规则，返回相应位置被点击后对应的面板：</p>

<ol>
	<li>如果一个地雷（'M'）被挖出，游戏就结束了- 把它改为 <strong>'X'</strong>。</li>
	<li>如果一个<strong>没有相邻地雷</strong>的空方块（'E'）被挖出，修改它为（'B'），并且所有和其相邻的方块都应该被递归地揭露。</li>
	<li>如果一个<strong>至少与一个地雷相邻</strong>的空方块（'E'）被挖出，修改它为数字（'1'到'8'），表示相邻地雷的数量。</li>
	<li>如果在此次点击中，若无更多方块可被揭露，则返回面板。</li>
</ol>



<p><strong>示例 1：</strong></p>

<pre><strong>输入:</strong> 

[['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'M', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E'],
 ['E', 'E', 'E', 'E', 'E']]

Click : [3,0]

<strong>输出:</strong> 

[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]

<strong>解释:</strong>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/minesweeper_example_1.png" style="width: 100%; max-width: 400px">
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入:</strong> 

[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'M', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]

Click : [1,2]

<strong>输出:</strong> 

[['B', '1', 'E', '1', 'B'],
 ['B', '1', 'X', '1', 'B'],
 ['B', '1', '1', '1', 'B'],
 ['B', 'B', 'B', 'B', 'B']]

<strong>解释:</strong>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/12/minesweeper_example_2.png" style="width: 100%; max-width: 400px">
</pre>



<p><strong>注意：</strong></p>

<ol>
	<li>输入矩阵的宽和高的范围为 [1,50]。</li>
	<li>点击的位置只能是未被挖出的方块 ('M' 或者 'E')，这也意味着面板至少包含一个可点击的方块。</li>
	<li>输入面板不会是游戏结束的状态（即有地雷已被挖出）。</li>
	<li>简单起见，未提及的规则在这个问题中可被忽略。例如，当游戏结束时你不需要挖出所有地雷，考虑所有你可能赢得游戏或标记方块的情况。</li>
</ol>

## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let click = (click[0] as usize, click[1] as usize);
        match board[click.0][click.1] {
            'M' => { // 挖到地雷
                board[click.0][click.1] = 'X';
            },
            'E' => { // 挖到空白
                let mut stack = Vec::new();
                stack.push(click);
                while !stack.is_empty() {
                    let cur_pos = stack.pop().unwrap();
                    let bomb_num = Solution::get_bomb_num(&board, (cur_pos.0 as i32, cur_pos.1 as i32));
                    if bomb_num > 0 {
                        board[cur_pos.0][cur_pos.1] = (bomb_num as u8 + '0' as u8) as char;
                    } else {
                        board[cur_pos.0][cur_pos.1] = 'B'; // 将B的四周方块加入栈上，递归揭露
                        let (m, n) = (board.len() as i32, board[0].len() as i32);
                        for j in (cur_pos.0 as i32 - 1).max(0)..=(cur_pos.0 as i32 + 1).min(m - 1) {
                            for i in (cur_pos.1 as i32 - 1).max(0)..=(cur_pos.1 as i32 + 1).min(n - 1) {
                                if board[j as usize][i as usize] == 'E' {
                                    stack.push((j as usize, i as usize));
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        };

        board
    }
    fn get_bomb_num(board: &Vec<Vec<char>>, pos: (i32, i32)) -> i32 {
        let mut num = 0;
        let (m, n) = (board.len() as i32, board[0].len() as i32);
        for j in (pos.0 - 1).max(0)..=(pos.0 + 1).min(m - 1) {
            for i in (pos.1 - 1).max(0)..=(pos.1 + 1).min(n - 1) {
                if (pos.0 != j || pos.1 != i) &&
                    board[j as usize][i as usize] == 'M' {
                        num += 1;
                }
            }
        }
        num
    }
}

```