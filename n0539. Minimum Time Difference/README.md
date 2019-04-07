# Minimum Time Difference :star::star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-time-difference](https://leetcode-cn.com/problems/minimum-time-difference)
- 执行时间: 4 ms 
- 内存消耗: 3.4 MB
- 通过日期: 2019-03-07 18:50

## 题目内容
<p>给定一个 24 小时制（小时:分钟）的时间列表，找出列表中任意两个时间的最小时间差并已分钟数表示。</p>

<p><br />
<strong>示例 1：</strong></p>

<pre>
<strong>输入:</strong> ["23:59","00:00"]
<strong>输出:</strong> 1
</pre>

<p><br />
<strong>备注:</strong></p>

<ol>
	<li>列表中时间数在 2~20000 之间。</li>
	<li>每个时间取值在 00:00~23:59 之间。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::min;
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        const FULL: i32 = 24 * 60;
        let mut sec:Vec<i32> = time_points.iter().map(|time| {
            time.split(":").map(|t| {
                t.parse::<i32>().unwrap()
            }).fold(0, |a, b| { a * 60 + b })
        }).collect::<Vec<i32>>();

        sec.sort();
        sec.push(sec[0]);

        match sec.iter().scan(-1, |a, &b| {
            let ret = if *a == -1 { FULL } else {
                min((b - *a + FULL) % FULL, (*a - b + FULL) % FULL)
            };
            *a = b;
            Some(ret)
        }).min() {
            Some(ans) => ans,
            None => 0
        }
    }
}


```