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

