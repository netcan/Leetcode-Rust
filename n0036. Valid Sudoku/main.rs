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
