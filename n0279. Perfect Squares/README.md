## Perfect Squares :star::star:
- 题目地址: [https://leetcode-cn.com/problems/perfect-squares](https://leetcode-cn.com/problems/perfect-squares)
- 执行时间: 404 ms 
- 内存消耗: 2.9 MB
- 通过日期: 2019-03-18 18:51

### 题目内容
---
<p>给定正整数 <em>n</em>，找到若干个完全平方数（比如 <code>1, 4, 9, 16, ...</code>）使得它们的和等于<em> n</em>。你需要让组成和的完全平方数的个数最少。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> <em>n</em> = <code>12</code>
<strong>输出:</strong> 3 
<strong>解释: </strong><code>12 = 4 + 4 + 4.</code></pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> <em>n</em> = <code>13</code>
<strong>输出:</strong> 2
<strong>解释: </strong><code>13 = 4 + 9.</code></pre>


### 解法
---
```rust
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


```