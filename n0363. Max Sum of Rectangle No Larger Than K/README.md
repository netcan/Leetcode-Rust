## Max Sum of Rectangle No Larger Than K :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k)
- 执行时间: 496 ms 
- 内存消耗: 2.9 MB
- 通过日期: 2019-03-06 14:28

### 题目内容
<p>给定一个非空二维矩阵 <em>matrix </em>和一个整数<em> k</em>，找到这个矩阵内部不大于 <em>k</em> 的最大矩形和。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入: </strong>matrix = [[1,0,1],[0,-2,3]], k = 2
<strong>输出: </strong>2 
<strong>解释:</strong> 矩形区域 <code>[[0, 1], [-2, 3]]</code> 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。
</pre>

<p><strong>说明：</strong></p>

<ol>
	<li>矩阵内的矩形区域面积必须大于 0。</li>
	<li>如果行数远大于列数，你将如何解答呢？</li>
</ol>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    fn sum_submatrix(matrix_sum: &Vec<Vec<i32>>, i: usize, j: usize, k: usize, l: usize) -> i32 {
        if i as i32 - 1 >= 0 && j as i32 - 1 >= 0 {
            return matrix_sum[l][k] - matrix_sum[j-1][k] - matrix_sum[l][i-1] + matrix_sum[j-1][i-1];
        } else if i as i32 - 1 >= 0 {
            return matrix_sum[l][k] - matrix_sum[l][i-1];
        } else if j as i32 - 1 >= 0 {
            return matrix_sum[l][k] - matrix_sum[j-1][k];
        } else {
            return matrix_sum[l][k];
        }

    }

    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut matrix_sum = matrix.clone();
        let (M, N) = (matrix_sum.len(), matrix_sum[0].len());
        matrix_sum[0][0] = matrix[0][0];
        // 处理第一列
        for j in (1..M) {
            matrix_sum[j][0] = matrix_sum[j-1][0] + matrix[j][0];
        }
        // 处理第一行
        for i in (1..N) {
            matrix_sum[0][i] = matrix_sum[0][i-1] + matrix[0][i];
        }
        for j in (1..M) {
            for i in (1..N) {
                matrix_sum[j][i] = matrix_sum[j-1][i] + matrix_sum[j][i-1] - matrix_sum[j-1][i-1] + matrix[j][i];
            }
        }

        let mut maxk = None;
        for j in (0..M) {
            for i in (0..N) {
                for l in (j..M) {
                    for k in (i..N) {
                        let sum = Solution::sum_submatrix(&matrix_sum, i, j, k, l);
                        if sum == target {
                            return sum;
                        } else if sum < target {
                            maxk = match maxk {
                                None => Some(sum),
                                Some(maxk) if maxk < sum => Some(sum),
                                _ => maxk
                            }
                        }
                    }
                }
            }
        }
        if let Some(x) = maxk {
            return x;
        } else {
            return 0;
        }
    }
}


```
