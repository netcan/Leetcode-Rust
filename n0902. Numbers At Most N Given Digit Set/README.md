# Numbers At Most N Given Digit Set :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set](https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-22 15:37

## 题目内容
<p>我们有一组<strong>排序的</strong>数字 <code>D</code>，它是  <code>{'1','2','3','4','5','6','7','8','9'}</code> 的非空子集。（请注意，<code>'0'</code> 不包括在内。）</p>

<p>现在，我们用这些数字进行组合写数字，想用多少次就用多少次。例如 <code>D = {'1','3','5'}</code>，我们可以写出像 <code>'13', '551', '1351315'</code> 这样的数字。</p>

<p>返回可以用 <code>D</code> 中的数字写出的小于或等于 <code>N</code> 的正整数的数目。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>D = ["1","3","5","7"], N = 100
<strong>输出：</strong>20
<strong>解释：</strong>
可写出的 20 个数字是：
1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>D = ["1","4","9"], N = 1000000000
<strong>输出：</strong>29523
<strong>解释：</strong>
我们可以写 3 个一位数字，9 个两位数字，27 个三位数字，
81 个四位数字，243 个五位数字，729 个六位数字，
2187 个七位数字，6561 个八位数字和 19683 个九位数字。
总共，可以使用D中的数字写出 29523 个整数。</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>D</code> 是按排序顺序的数字 <code>'1'-'9'</code> 的子集。</li>
	<li><code>1 <= N <= 10^9</code></li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut num: i32) -> i32 {
        let digits: Vec<i32> = digits.into_iter().map(|d| d.chars().nth(0).unwrap() as i32 - '0' as i32).collect();
        let num: Vec<usize> = num.to_string().as_bytes().into_iter().map(|b| *b as usize - '0' as usize).rev().collect();

        // 小于数字n的个数
        let num_times: Vec<i32> = (0..10).map(|d| {
            digits.iter().filter(|&n| *n < d).count() as i32
        }).collect();
        // 子集中是否存在数字d
        let has_digit: Vec<i32> = (0..10).map(|d| {
            digits.iter().find(|&d2| { *d2 == d }).is_some() as i32
        }).collect();

        // num_combination[n] = (digits.len()) ^ n
        let num_combination: Vec<i32> = (0..num.len() as u32).map(|n| digits.len().pow(n) as i32).collect();
        // 先从个位到最高位计算
        let mut cnt = num_combination[1..num.len()].iter().sum::<i32>();

        // 加上与num同长的个数
        let mut cnt2 = 1;
        for (idx, &n) in num[0..num.len()].iter().enumerate() {
            cnt2 = num_times[n] * num_combination[idx] + has_digit[n] * cnt2;
        }

        cnt + cnt2
    }
}


```