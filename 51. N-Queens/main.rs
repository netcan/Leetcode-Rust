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
            // å¯æ¾ç½®
            if Solution::check((depth, i), board) {
                board[depth][i] = true;
                Solution::total_n_queens_(depth + 1, board, solv);
                board[depth][i] = false;
            }
        }
    }
}