## Image Smoother :star:
- 题目地址: [https://leetcode-cn.com/problems/image-smoother](https://leetcode-cn.com/problems/image-smoother)
- 执行时间: 28 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-03-25 20:46

### 题目内容
---
<p>包含整数的二维矩阵 M 表示一个图片的灰度。你需要设计一个平滑器来让每一个单元的灰度成为平均灰度 (向下舍入) ，平均灰度的计算是周围的8个单元和它本身的值求平均，如果周围的单元格不足八个，则尽可能多的利用它们。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong>
[[1,1,1],
 [1,0,1],
 [1,1,1]]
<strong>输出:</strong>
[[0, 0, 0],
 [0, 0, 0],
 [0, 0, 0]]
<strong>解释:</strong>
对于点 (0,0), (0,2), (2,0), (2,2): 平均(3/4) = 平均(0.75) = 0
对于点 (0,1), (1,0), (1,2), (2,1): 平均(5/6) = 平均(0.83333333) = 0
对于点 (1,1): 平均(8/9) = 平均(0.88888889) = 0
</pre>

<p><strong>注意:</strong></p>

<ol>
	<li>给定矩阵中的整数范围为 [0, 255]。</li>
	<li>矩阵的长和宽的范围均为 [1, 150]。</li>
</ol>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    const dx: [i32; 9] = [0, 1, 0, -1, 1, 1, -1, -1, 0];
    const dy: [i32; 9] = [-1, 0, 1, 0, -1, 1, 1, -1, 0];
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = m.clone();
        for i in 0..m.len() {
            for j in 0..m[0].len() {
                let (mut l, mut sum) = (0, 0);
                for k in 0..9 {
                    let (x, y) = (i as i32 + Solution::dx[k],
                                  j as i32 + Solution::dy[k]);
                    if x >= 0 && x < m.len() as i32 &&
                        y >= 0 && y < m[0].len() as i32 {
                            sum += m[x as usize][y as usize];
                            l += 1;
                        }
                }
                res[i][j] = sum / l;
            }
        }
        res
    }
}

```