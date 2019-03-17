### Median of Two Sorted Arrays :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/median-of-two-sorted-arrays](https://leetcode-cn.com/problems/median-of-two-sorted-arrays)
- 执行时间/Runtime: 12 ms 
- 内存消耗/Mem Usage: 2.5 MB
- 通过日期/Accept Datetime: 2019-03-17 16:56

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
