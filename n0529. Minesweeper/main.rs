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
