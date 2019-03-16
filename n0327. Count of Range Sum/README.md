
### Count of Range Sum :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/count-of-range-sum](https://leetcode-cn.com/problems/count-of-range-sum)
- 执行时间/Runtime: 8 ms 
- 内存消耗/Mem Usage: 3 MB
- 通过日期/Accept Datetime: 2019-03-16 14:22

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
