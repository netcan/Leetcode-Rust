### Numbers At Most N Given Digit Set :star::star::star:
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set](https://leetcode-cn.com/problems/numbers-at-most-n-given-digit-set)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 通过日期/Accept Datetime: 2019-03-22 15:33

```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, mut num: i32) -> i32 {
        let digits: Vec<i32> = digits.into_iter().map(|d| d.chars().nth(0).unwrap() as i32 - '0' as i32).collect();
        let num: Vec<usize> = num.to_string().as_bytes().into_iter().map(|b| *b as usize - '0' as usize).collect();
        if num.len() <= 1 { return digits.iter().filter(|&d| *d <= num[0] as i32).count() as i32 }

        // 小于数字n的个数
        let num_times: Vec<i32> = (0..10).map(|d| {
            digits.iter().filter(|&n| *n < d).count() as i32
        }).collect();
        // 子集中是否存在数字d
        let has_digit: Vec<i32> = (0..10).map(|d| {
            digits.iter().find(|&d2| { *d2 == d }).is_some() as i32
        }
        ).collect();

        // num_combination[n] = (digits.len()) ^ n
        let num_combination: Vec<i32> = (0..num.len() as u32).map(|n| digits.len().pow(n) as i32).collect();
        let mut cnt = 0;
        // 先从个位到最高位计算
        cnt += num_combination[1..num.len()].iter().sum::<i32>();

        // 算上与num同长的个数
        let mut cnt2 = 1;
        for (idx, &n) in num[0..num.len()].iter().rev().enumerate() {
            cnt2 = num_times[n] * num_combination[idx] + has_digit[n] * cnt2;
            println!("n = {} idx = {} cnt2 = {}", n, idx, cnt2);
        }

        println!("cnt: {}", cnt);
        println!("cnt2: {}", cnt2);


        println!("num={:?}", num);
        println!("digits={:?}", digits);
        println!("num_times = {:?}", num_times);
        println!("num_combination = {:?}", num_combination);
        println!("has_digit = {:?}", has_digit);
        cnt + cnt2
    }
}


```
