# Candy :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/candy](https://leetcode-cn.com/problems/candy)
- 执行时间: 16 ms 
- 内存消耗: 3.5 MB
- 通过日期: 2019-03-11 21:47

## 题目内容
<p>老师想给孩子们分发糖果，有 <em>N</em> 个孩子站成了一条直线，老师会根据每个孩子的表现，预先给他们评分。</p>

<p>你需要按照以下要求，帮助老师给这些孩子分发糖果：</p>

<ul>
	<li>每个孩子至少分配到 1 个糖果。</li>
	<li>相邻的孩子中，评分高的孩子必须获得更多的糖果。</li>
</ul>

<p>那么这样下来，老师至少需要准备多少颗糖果呢？</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> [1,0,2]
<strong>输出:</strong> 5
<strong>解释:</strong> 你可以分别给这三个孩子分发 2、1、2 颗糖果。
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> [1,2,2]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以分别给这三个孩子分发 1、2、1 颗糖果。
     第三个孩子只得到 1 颗糖果，这已满足上述两个条件。</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::BinaryHeap;
use std::cmp::{max, Reverse};
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for (pos, &score) in ratings.iter().enumerate() {
            heap.push(Reverse((score, pos as i32)));
        }

        let mut candies = vec![0; ratings.len()];

        while let Some(Reverse((score, pos))) = heap.pop() {
            let left_score = if pos > 0 {
                ratings[(pos - 1) as usize]
            } else {
                ratings[pos as usize] + 1
            };

            let right_score = if pos + 1 < ratings.len() as i32 {
                ratings[(pos + 1) as usize]
            } else {
                ratings[pos as usize] + 1
            };

            candies[pos as usize] =
                if score <= left_score && score <= right_score {
                    0
                } else if score > left_score && score <= right_score {
                    candies[(pos - 1) as usize]
                } else if score > right_score && score <= left_score {
                    candies[(pos + 1) as usize]
                } else { // score > left_score && score > right_score
                    max(candies[(pos - 1) as usize], candies[(pos + 1) as usize])
                } + 1

        }
        candies.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1_2_2() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }
    #[test]
    fn test_1_0_2() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }
    #[test]
    fn test_1_2_2_1_0() {
        assert_eq!(Solution::candy(vec![1,2,2,1,0]), 9);
    }
    #[test]
    fn test_1_3_4_5_2() {
        assert_eq!(Solution::candy(vec![1, 3, 4, 5, 2]), 11);
    }
}

```