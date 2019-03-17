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
