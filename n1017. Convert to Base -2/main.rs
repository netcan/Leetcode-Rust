// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n == 0 { return 0.to_string(); }
        let mut ans = String::new();
        while n != 0 {
            let mut reminder = n % -2;
            n /= -2;
            if reminder < 0 {
                reminder += 2;
                n += 1;
            }
            ans = reminder.to_string() + &ans;
        }

        ans
    }
}
