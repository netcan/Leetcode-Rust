### Image Smoother :star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/image-smoother](https://leetcode-cn.com/problems/image-smoother)
- 执行时间/Runtime: 28 ms 
- 内存消耗/Mem Usage: 3 MB
- 通过日期/Accept Datetime: 2019-03-25 20:46

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
