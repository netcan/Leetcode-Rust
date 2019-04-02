// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let (mut num, mut len) = (1, 1); 
        while num % k != 0 && len < 1000000 {
            num = (num * 10 + 1) % k;
            len += 1;
        }

        if num % k == 0 { len } 
        else { -1 }
    }
}

