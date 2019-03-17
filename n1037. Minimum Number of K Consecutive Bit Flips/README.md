### Minimum Number of K Consecutive Bit Flips :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/minimum-number-of-k-consecutive-bit-flips](https://leetcode-cn.com/problems/minimum-number-of-k-consecutive-bit-flips)
- 执行时间/Runtime: 20 ms 
- 内存消耗/Mem Usage: 3.5 MB
- 通过日期/Accept Datetime: 2019-03-11 23:42

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn min_k_bit_flips(a: Vec<i32>, k: i32) -> i32 {
        let (mut sum, mut ans) = (0, 0);
        // 是否翻转？
        let mut flip = vec![0; a.len()];
        for i in 0..a.len() - k as usize + 1 {
            if (sum + a[i]) % 2 == 0 {
                flip[i] = 1;
                sum += flip[i];
                ans += 1;
            }

            if i as i32 - k + 1 >= 0 {
                sum -= flip[(i as i32 - k + 1) as usize]
            }
        }
        // 检查后面部分，看看是否全部翻转成功
        for i in a.len() - k as usize + 1..a.len() {
            if (sum + a[i]) % 2 == 0 {
                return -1;
            }
            if i as i32 - k + 1 >= 0 {
                sum -= flip[(i as i32 - k + 1) as usize]
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
        assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 3), -1);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::min_k_bit_flips(vec![0,0,0,1,0,1,1,0], 3), 3);
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 1], 2), -1);
    }
    #[test]
    fn test_4() {
        assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
    }
}

```
