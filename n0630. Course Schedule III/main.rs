// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::{min, max};
use std::iter;
impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by(|a, b| { a[1].cmp(&b[1]) });
        let max_d = min(courses.last().unwrap()[1], courses.iter().fold(0, |sum_d, c| {
            sum_d + c[0]
        }));

        // dp[t]: 第t天最多完成的科目数
        let mut dp = vec![0; max_d as usize + 1];
        let mut ans: i32 = 0;

        for (idx, c) in courses.iter().enumerate() {
            for t in (c[0] as usize ..= max_d as usize).rev() {
                if t <= (c[1] as usize) { // 可学
                    dp[t] = max(dp[t], dp[t - c[0] as usize] + 1);
                    ans = max(ans, dp[t]);
                }
            }
        }

        ans
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

