# Score After Flipping Matrix :star::star:
- 题目地址: [https://leetcode-cn.com/problems/score-after-flipping-matrix](https://leetcode-cn.com/problems/score-after-flipping-matrix)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-13 22:33

## 题目内容
<p>有一个二维矩阵 <code>A</code> 其中每个元素的值为 <code>0</code> 或 <code>1</code> 。</p>

<p>移动是指选择任一行或列，并转换该行或列中的每一个值：将所有 <code>0</code> 都更改为 <code>1</code>，将所有 <code>1</code> 都更改为 <code>0</code>。</p>

<p>在做出任意次数的移动后，将该矩阵的每一行都按照二进制数来解释，矩阵的得分就是这些数字的总和。</p>

<p>返回尽可能高的分数。</p>



<ol>
</ol>

<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>[[0,0,1,1],[1,0,1,0],[1,1,0,0]]
<strong>输出：</strong>39
<strong>解释：
</strong>转换为 [[1,1,1,1],[1,0,0,1],[1,1,1,1]]
0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 20</code></li>
	<li><code>1 <= A[0].length <= 20</code></li>
	<li><code>A[i][j]</code> 是 <code>0</code> 或 <code>1</code></li>
</ol>


## 解法
```rust
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

```