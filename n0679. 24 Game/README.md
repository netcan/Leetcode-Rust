# 24 Game :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/24-game](https://leetcode-cn.com/problems/24-game)
- 执行时间: 4 ms 
- 内存消耗: 2.3 MB
- 通过日期: 2019-03-09 18:07

## 题目内容
<p>你有 4 张写有 1 到 9 数字的牌。你需要判断是否能通过 <code>*</code>，<code>/</code>，<code>+</code>，<code>-</code>，<code>(</code>，<code>)</code> 的运算得到 24。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> [4, 1, 8, 7]
<strong>输出:</strong> True
<strong>解释:</strong> (8-4) * (7-1) = 24
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> [1, 2, 1, 2]
<strong>输出:</strong> False
</pre>

<p><strong>注意:</strong></p>

<ol>
	<li>除法运算符 <code>/</code> 表示实数除法，而不是整数除法。例如 4 / (1 - 2/3) = 12 。</li>
	<li>每个运算符对两个数进行运算。特别是我们不能用 <code>-</code> 作为一元运算符。例如，<code>[1, 1, 1, 1]</code> 作为输入时，表达式 <code>-1 - 1 - 1 - 1</code> 是不允许的。</li>
	<li>你不能将数字连接在一起。例如，输入为 <code>[1, 2, 1, 2]</code> 时，不能写成 12 + 12 。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        Solution::judge_point24_(
            nums.iter().map(|&x| { x as f64 }).collect()
            )
    }

    pub fn judge_point24_(nums: Vec<f64>) -> bool {
        if nums.len() == 0 { return false; }
        if nums.len() == 1 && (nums[0] - 24.0).abs() < 1e-6 { return true; }
        // 选出2个数，进行运算，然后放回列表中
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                for op in &['+', '*', '-', '/'] {
                    let mut nums_: Vec<f64> = nums.iter()
                        .enumerate()
                        .filter(|&(k, _)| k != i && k != j)
                        .map(|(_, v)| v )
                        .cloned().collect();
                    match op {
                        '+'  => if (i < j) { nums_.push(nums[i] + nums[j]) } else { continue; },
                        '*'  => if (i < j) { nums_.push(nums[i] * nums[j]) } else { continue; },
                        '-'  => nums_.push(nums[i] - nums[j]),
                        '/'  => nums_.push(nums[i] / nums[j]),
                        _    => {}
                    };
                    if Solution::judge_point24_(nums_) { return true; }
                }
            }
        }

        false
    }
}

```