# Pow(x, n) :star::star:
- 题目地址: [https://leetcode-cn.com/problems/powx-n](https://leetcode-cn.com/problems/powx-n)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-10 23:16

## 题目内容
<p>实现 <a href="https://www.cplusplus.com/reference/valarray/pow/" target="_blank">pow(<em>x</em>, <em>n</em>)</a> ，即计算 x 的 n 次幂函数。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> 2.00000, 10
<strong>输出:</strong> 1024.00000
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> 2.10000, 3
<strong>输出:</strong> 9.26100
</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> 2.00000, -2
<strong>输出:</strong> 0.25000
<strong>解释:</strong> 2<sup>-2</sup> = 1/2<sup>2</sup> = 1/4 = 0.25</pre>

<p><strong>说明:</strong></p>

<ul>
	<li>-100.0 < <em>x</em> < 100.0</li>
	<li><em>n</em> 是 32 位有符号整数，其数值范围是 [−2<sup>31</sup>, 2<sup>31 </sup>− 1] 。</li>
</ul>


## 解法
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