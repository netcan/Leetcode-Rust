## Minimum Number of K Consecutive Bit Flips :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/minimum-number-of-k-consecutive-bit-flips](https://leetcode-cn.com/problems/minimum-number-of-k-consecutive-bit-flips)
- 执行时间: 20 ms 
- 内存消耗: 3.5 MB
- 通过日期: 2019-03-11 23:53

### 题目内容
---
<p>在仅包含 <code>0</code> 和 <code>1</code> 的数组 <code>A</code> 中，一次 <em><code>K</code> 位翻转</em>包括选择一个长度为 <code>K</code> 的（连续）子数组，同时将子数组中的每个 <code>0</code> 更改为 <code>1</code>，而每个 <code>1</code> 更改为 <code>0</code>。</p>

<p>返回所需的 <code>K</code> 位翻转的次数，以便数组没有值为 <code>0</code> 的元素。如果不可能，返回 <code>-1</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>A = [0,1,0], K = 1
<strong>输出：</strong>2
<strong>解释：</strong>先翻转 A[0]，然后翻转 A[2]。
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>A = [1,1,0], K = 2
<strong>输出：</strong>-1
<strong>解释：</strong>无论我们怎样翻转大小为 2 的子数组，我们都不能使数组变为 [1,1,1]。
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输入：</strong>A = [0,0,0,1,0,1,1,0], K = 3
<strong>输出：</strong>3
<strong>解释：</strong>
翻转 A[0],A[1],A[2]: A变成 [1,1,1,1,0,1,1,0]
翻转 A[4],A[5],A[6]: A变成 [1,1,1,1,1,0,0,0]
翻转 A[5],A[6],A[7]: A变成 [1,1,1,1,1,1,1,1]
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= A.length <= 30000</code></li>
	<li><code>1 <= K <= A.length</code></li>
</ol>


### 解法
---
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