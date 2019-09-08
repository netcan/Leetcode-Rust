// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn read_binary_watch(num: i32) -> Vec<String> {
        let mut watch = vec![];
        for led in 0..1<<10 {
            let (mut led_num, mut led_) = (0, led);
            while led_ > 0 {
                led_ &= led_ - 1;
                led_num += 1;
            }

            if led_num == num {
                let (hour, min) = (led >> 6, led & 0x3f);
                if hour < 12 && min < 60 {
                    watch.push(format!("{}:{:02}", led >> 6, led & 0x3f))
                }
            }
        }

        watch
    }
}

