// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::i32;
impl Solution {
    pub fn my_atoi(str: String) -> i32 {
        let mut str = str.trim();
        let mut ret: i64 = 0;
        let mut sign: bool = false;

        match str.chars().nth(0) {
            Some(c) => {
                match c {
                    '-' => {
                        sign = true;
                        str = &str[1..];
                    }
                    '+' => {
                        str = &str[1..];
                    }
                    c if c >= '0' && c <= '9' => {}
                    _ => { return 0; }
                }
            }
            None => {
                return 0;
            }
        }

        for c in str.chars() {
            if c >= '0' && c <= '9' {
                ret = ret * 10 + c as i64 - '0' as i64;
            } else { break; }

            match sign {
                true if -ret < i32::MIN as i64 => {return i32::MIN;}
                false if ret > i32::MAX as i64 => {return i32::MAX;}
                _ => {}
            }
        }
        if sign {
            ret = -ret;
        }



        ret as i32
    }
}

