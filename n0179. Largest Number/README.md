# Largest Number :star::star:
- 题目地址: [https://leetcode-cn.com/problems/largest-number](https://leetcode-cn.com/problems/largest-number)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-19 19:42

## 题目内容
<p>给定一组非负整数，重新排列它们的顺序使之组成一个最大的整数。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> <code>[10,2]</code>
<strong>输出:</strong> <code>210</code></pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> <code>[3,30,34,5,9]</code>
<strong>输出:</strong> <code>9534330</code></pre>

<p><strong>说明: </strong>输出结果可能非常大，所以你需要返回一个字符串而不是整数。</p>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums: Vec<Vec<u8>> = nums.into_iter().map(|x| x.to_string().into_bytes()).collect();
        nums.sort_by(|a, b| {
            let mut i = 0;
            while i < a.len() || i < b.len() {
                let (ac_end, bc_end) = ( // 当前数字 or 结尾数字
                    a[i.min(a.len() - 1)],
                    b[i.min(b.len() - 1)]
                    );

                let (ac_begin, bc_begin) = ( // 当前数字 or 开头数字
                    a[i.min(a.len()) % a.len()],
                    b[i.min(b.len()) % b.len()]
                    );

                let (ac, bc) = (
                    ac_begin.max(ac_end), 
                    bc_begin.max(bc_end)
                );

                if ac < bc { return Ordering::Greater; }
                else if ac > bc { return Ordering::Less; }
                else { // ac == bc
                    if ac_end < bc_end { return Ordering::Greater; }
                    else if ac_end > bc_end { return Ordering::Less; }
                }
                i += 1;
            }
            Ordering::Equal
        });

        if nums.iter().flatten().all(|x| *x == '0' as u8) { return "0".to_string(); }
        String::from_utf8(nums.into_iter().flatten().collect()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::largest_number(vec![96, 969]), "96996".to_owned());
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::largest_number(vec![12, 121]), "12121".to_owned());
    }
    #[test]
    fn test_3() {
        assert_eq!(Solution::largest_number(vec![824,938,1399,5607,6973,5703,9609,4398,8247]), "9609938824824769735703560743981399".to_owned());
    }

}


```