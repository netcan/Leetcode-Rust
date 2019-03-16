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
