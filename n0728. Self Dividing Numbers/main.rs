// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut ret = vec![];
        for num in left..=right {
            let num_str = num.to_string();
            if let Some(_) = num_str.to_string().find("0") {
                continue;
            }
            let mut dividing: bool = true;
            for n in num_str.as_bytes() {
                if num % (*n - '0' as u8) as i32 != 0 {
                    dividing = false;
                    break;
                }
            }
            if dividing {
                ret.push(num);
            }
        }
        ret
    }
}

