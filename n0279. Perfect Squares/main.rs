// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut square = vec![];
        // 队列的层数即答案，每次入队完全平方数
        let mut que  = VecDeque::new();
        let mut mark = HashMap::<i32, bool>::new();

        for i in 1..=n {
            let i2 = i * i;
            if i2 <= n {
                square.push(i2);
                mark.insert(i2, true);
                que.push_back((i2, 1));
            }
            else { break; }
        }

        while !que.is_empty() {
            let (num, step) = que.pop_front().unwrap();
            if num == n { return step; }

            for sq in &square {
                let next_num = sq + num;
                if next_num > n { break; }
                // 根据4平方数定理，剪枝
                if !mark.contains_key(&next_num) && step < 4 {
                    que.push_back((next_num, step + 1));
                    mark.insert(next_num, true);
                }
            }
        }

        0
    }
}

