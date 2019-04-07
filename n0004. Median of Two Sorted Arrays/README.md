# Median of Two Sorted Arrays :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/median-of-two-sorted-arrays](https://leetcode-cn.com/problems/median-of-two-sorted-arrays)
- 执行时间: 12 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-03-17 16:56

## 题目内容
<p>给定两个大小为 m 和 n 的有序数组 <code>nums1</code> 和 <code>nums2</code>。</p>

<p>请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。</p>

<p>你可以假设 <code>nums1</code> 和 <code>nums2</code> 不会同时为空。</p>

<p><strong>示例 1:</strong></p>

<pre>nums1 = [1, 3]
nums2 = [2]

则中位数是 2.0
</pre>

<p><strong>示例 2:</strong></p>

<pre>nums1 = [1, 2]
nums2 = [3, 4]

则中位数是 (2 + 3)/2 = 2.5
</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (m, n) = (nums1.len(), nums2.len());
        let mid_idx = (m + n - 1) / 2;

        let (mut i, mut j) = (0, 0);
        let (mut ans, mut cur_num) = (0.0, 0);
        while i < m || j < n {
            if j == n || (i < m && nums1[i] <= nums2[j]) { cur_num = nums1[i]; }
            else { cur_num = nums2[j]; };

            if i + j >= mid_idx && i + j <= mid_idx + 1 {
                if (m + n) % 2 == 1 {
                    return cur_num as f64;
                } else {
                    ans += cur_num as f64;
                    if i + j == mid_idx + 1 { return ans / 2.0; }
                }
            }

            if j == n || (i < m && nums1[i] <= nums2[j]) { i += 1; }
            else { j += 1 };
        }

        ans
    }
}

```