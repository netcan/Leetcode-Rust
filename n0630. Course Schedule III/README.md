### Course Schedule III :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/course-schedule-iii](https://leetcode-cn.com/problems/course-schedule-iii)
- 执行时间/Runtime: 4108 ms 
- 内存消耗/Mem Usage: 105.8 MB
- 通过日期/Accept Datetime: 2019-03-13 16:14

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::{min, max};
use std::iter;
impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses: Vec<Vec<i32>> = courses.iter().filter(|&c| c[0] <= c[1]).cloned().collect();
        courses.sort_by(|a, b| { a[1].cmp(&b[1]) });

        let (max_t, max_d) = courses.iter().fold((courses[0][0], courses[0][1]), |mut maxc, c| {
            maxc.0 = max(maxc.0, c[0]);
            maxc.1 = max(maxc.1, c[1]);
            maxc
        });


        // dp[t]: 第t天最多完成的科目数
        let mut dp = vec![0; max_d as usize + 1];
        // 第n天学的科目
        let mut mark: Vec<Vec<bool>> = iter::repeat(vec![false; courses.len()]).take(max_t as usize + 1).collect();

        for (idx, c) in courses.iter().enumerate() {
            for t in (0..=max_d as usize).rev() {
                if t <= (c[1] as usize) &&
                    t >= (c[0] as usize) { // 可学
                        dp[t] = max(dp[t], dp[t - c[0] as usize] + 1);
                }
            }
        }

        *dp.iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::schedule_course(
                vec![vec![100, 200], 
                    vec![200, 1300], 
                    vec![1000, 1250], 
                    vec![2000, 3200]]
                ), 3);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::schedule_course(
                vec![vec![2,5],vec![2,19],vec![1,8],vec![1,3]]
                ), 4);
    }
}


```
