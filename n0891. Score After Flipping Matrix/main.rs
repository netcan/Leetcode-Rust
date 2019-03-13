// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::max;
impl Solution {
    pub fn matrix_score(mut a: Vec<Vec<i32>>) -> i32 {
        // 第一列全部翻转为1
        a.iter_mut().for_each(|v| {
            if v[0] == 0 {
                v.iter_mut().for_each(|b| { *b = 1 - *b; })
            }
        });
        let mut ans = a.len();
        // 从第二列开始
        for i in 1..a[0].len() {
            let mut number_of_one = 0;
            for j in 0..a.len() {
                if a[j][i] == 1 {
                    number_of_one += 1;
                }
            }
            number_of_one = max(number_of_one, a.len() - number_of_one);
            ans = ans * 2 + number_of_one;
        }
        ans as i32
    }
}
