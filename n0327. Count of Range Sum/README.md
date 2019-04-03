## Count of Range Sum :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/count-of-range-sum](https://leetcode-cn.com/problems/count-of-range-sum)
- 执行时间: 8 ms 
- 内存消耗: 3 MB
- 通过日期: 2019-03-16 14:22

### 题目内容
<p>给定一个整数数组 <code>nums</code>，返回区间和在 <code>[lower, upper]</code> 之间的个数，包含 <code>lower</code> 和 <code>upper</code>。<br>
区间和 <code>S(i, j)</code> 表示在 <code>nums</code> 中，位置从 <code>i</code> 到 <code>j</code> 的元素之和，包含 <code>i</code> 和 <code>j</code> (<code>i</code> ≤ <code>j</code>)。</p>

<p><strong>说明:</strong><br>
最直观的算法复杂度是 <em>O</em>(<em>n</em><sup>2</sup>) ，请在此基础上优化你的算法。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入: </strong><em>nums</em> = <code>[-2,5,-1]</code>, <em>lower</em> = <code>-2</code>, <em>upper</em> = <code>2</code>,
<strong>输出: </strong>3 
<strong>解释: </strong>3个区间分别是: <code>[0,0]</code>, <code>[2,2]</code>, <code>[0,2]，</code>它们表示的和分别为: <code>-2, -1, 2。</code>
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        fn count_range_sum_(sum: &mut [i64], lower: i32, upper: i32) -> i32 {
            let n = sum.len();
            if n <= 1 { return 0; }

            let (left, right) = sum.split_at_mut(n / 2);
            let mut count = count_range_sum_(left, lower, upper) +
                count_range_sum_(right, lower, upper);

            let (mut j, mut k) = (0, 0);
            for i in 0..n/2 {
                while j < right.len() && right[j] - left[i] < lower as i64 { j += 1; }
                while k < right.len() && right[k] - left[i] <= upper as i64 { k += 1; }
                count += k as i32 - j as i32; // 求出位于[lower, upper]的个数
            }
            sum.sort(); // 归并排序 left + right
            count
        }

        let mut cnt = 0;
        let mut pre_sum = vec![0i64; nums.len() + 1];
        for (i, num) in nums.iter().enumerate() {
            pre_sum[i + 1] = pre_sum[i] + *num as i64;
        }
        if nums.len() >= 1 { cnt = count_range_sum_(&mut pre_sum, lower, upper); }
        cnt
    }
}

```
