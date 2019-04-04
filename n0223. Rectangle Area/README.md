## Rectangle Area :star::star:
- 题目地址: [https://leetcode-cn.com/problems/rectangle-area](https://leetcode-cn.com/problems/rectangle-area)
- 执行时间: 16 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-14 23:49

### 题目内容
---
<p>在<strong>二维</strong>平面上计算出两个<strong>由直线构成的</strong>矩形重叠后形成的总面积。</p>

<p>每个矩形由其左下顶点和右上顶点坐标表示，如图所示。</p>

<p><img alt="Rectangle Area" src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2018/10/22/rectangle_area.png"></p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> -3, 0, 3, 4, 0, -1, 9, 2
<strong>输出:</strong> 45</pre>

<p><strong>说明:</strong> 假设矩形面积不会超出 <strong>int </strong>的范围。</p>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::cmp::{max, min};
impl Solution {
    pub fn compute_area(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
        let (a, b, c, d, e, f, g, h): (i64, i64, i64, i64, i64, i64, i64, i64) =
            (a as i64, b as i64, c as i64, d as i64, e as i64, f as i64, g as i64, h as i64);
         // 重叠部分面积
        let dup_area = max(0, (min(h, d) - max(f, b))) * max(0, (min(c, g) - max(a, e)));
        // 两面积之和减去重叠部分
        ((d - b) * (c - a) + (h - f) * (g - e) - dup_area) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, 3, 3, 4, 4), 17);
    }
    #[test]
    fn test_2() {
        assert_eq!(Solution::compute_area(-1500000001, 0, -1500000000, 1, 1500000000, 0, 1500000001, 1), 2);
    }
}

```