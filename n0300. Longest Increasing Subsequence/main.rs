// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut longest = Vec::with_capacity(nums.len());

        for num in nums {
            match longest.binary_search(&num) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = num;
                    }  else  {
                        longest.push(num);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}

