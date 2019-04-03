## Course Schedule III :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/course-schedule-iii](https://leetcode-cn.com/problems/course-schedule-iii)
- 执行时间: 3528 ms 
- 内存消耗: 4.1 MB
- 通过日期: 2019-03-13 21:53

### 题目内容
<p>这里有 <code>n</code> 门不同的在线课程，他们按从 <code>1</code> 到 <code>n</code> 编号。每一门课程有一定的持续上课时间（课程时间）<code>t</code> 以及关闭时间第 d<sub> </sub>天。一门课要持续学习 <code>t</code> 天直到第 d<span style="font-size:10.5px"> </span>天时要完成，你将会从第 1 天开始。</p>

<p>给出 <code>n</code> 个在线课程用 <code>(t, d)</code> 对表示。你的任务是找出最多可以修几门课。</p>



<p><strong>示例：</strong></p>

<pre>
<strong>输入:</strong> [[100, 200], [200, 1300], [1000, 1250], [2000, 3200]]
<strong>输出:</strong> 3
<strong>解释:</strong> 
这里一共有 4 门课程, 但是你最多可以修 3 门:
首先, 修第一门课时, 它要耗费 100 天，你会在第 100 天完成, 在第 101 天准备下门课。
第二, 修第三门课时, 它会耗费 1000 天，所以你将在第 1100 天的时候完成它, 以及在第 1101 天开始准备下门课程。
第三, 修第二门课时, 它会耗时 200 天，所以你将会在第 1300 天时完成它。
第四门课现在不能修，因为你将会在第 3300 天完成它，这已经超出了关闭日期。</pre>



<p><strong>提示:</strong></p>

<ol>
	<li>整数 1 <= d, t, n <= 10,000 。</li>
	<li>你不能同时修两门课程。</li>
</ol>




### 解法
```rust
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


```
