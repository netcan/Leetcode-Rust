
### Pow(x, n) :star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/powx-n](https://leetcode-cn.com/problems/powx-n)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 提交日期/Datetime: 2019-03-10 23:16

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn my_pow(x: f64, mut n: i32) -> f64 {
        let mut x = if n < 0 { 1.0 / x } else { x };
        let mut ret: f64 = 1.0;
        while n != 0 {
            if n.abs() % 2 == 1 {
                ret *= x;
            }
            x *= x;
            n /= 2;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_2_10() {
        assert_eq!(Solution::my_pow(2.0, 10), 1024.0);
    }
    #[test]
    fn test_2_2() {
        assert_eq!(Solution::my_pow(2.0, -2), 0.25);
    }
}

```
