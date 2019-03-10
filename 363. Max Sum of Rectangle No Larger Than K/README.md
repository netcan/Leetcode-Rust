
### Max Sum of Rectangle No Larger Than K
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k](https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k)
- 执行时间/Runtime: 496 ms 
- 内存消耗/Mem Usage: 2.9 MB
- 提交日期/Datime: 2019-03-06 14:28

```rust
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
