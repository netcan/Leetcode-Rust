// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
        let mut ans = vec![];
        let mut sum = 0;

        for n in a {
            sum = (sum * 2 + n) % 5;
            ans.push(if sum == 0 { true }
                     else { false });
        }

        ans
    }
}
