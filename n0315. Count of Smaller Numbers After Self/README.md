## Count of Smaller Numbers After Self :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/count-of-smaller-numbers-after-self](https://leetcode-cn.com/problems/count-of-smaller-numbers-after-self)
- 执行时间: 8 ms 
- 内存消耗: 4 MB
- 通过日期: 2019-03-16 12:46

### 题目内容
---
<p>给定一个整数数组 <em>nums</em>，按要求返回一个新数组 <em>counts</em>。数组 <em>counts</em> 有该性质： <code>counts[i]</code> 的值是  <code>nums[i]</code> 右侧小于 <code>nums[i]</code> 的元素的数量。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> [5,2,6,1]
<strong>输出:</strong> <code>[2,1,1,0] 
<strong>解释:</strong></code>
5 的右侧有 <strong>2 </strong>个更小的元素 (2 和 1).
2 的右侧仅有 <strong>1 </strong>个更小的元素 (1).
6 的右侧有 <strong>1 </strong>个更小的元素 (1).
1 的右侧有 <strong>0 </strong>个更小的元素.
</pre>


### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        fn merged_sort(nums: &mut [(usize, i32)], counts: &mut Vec<i32>) -> Vec<(usize, i32)> {
            let n = nums.len();
            if n <= 1 { return vec![nums[0]]; }
            let (left, right) = nums.split_at_mut(n / 2);

            let left = merged_sort(left, counts);
            let right = merged_sort(right, counts);
            let (m, n) = (left.len(), right.len());
            let mut sorted = vec![(0, 0); left.len() + right.len()];
            let (mut i, mut j) = (0, 0);

            while i < m || j < n {
                if j == n || (i < m && left[i].1 <= right[j].1) {
                    sorted[i + j] = left[i];
                    counts[left[i].0] += j as i32; // 位于右边的数排到左边的都是小于这个数
                    i += 1;
                } else {
                    sorted[i + j] = right[j];
                    j += 1;
                }
            }
            sorted
        }

        let n = nums.len();
        let mut counts = vec![0; n];
        if n == 0 { return counts; }
        merged_sort(&mut nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>(), &mut counts);

        counts
    }
}

```