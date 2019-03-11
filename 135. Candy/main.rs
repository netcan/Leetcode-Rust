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
