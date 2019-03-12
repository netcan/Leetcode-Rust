// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut temp = Vec::new();
        Self::dfs(1, n, k, &mut temp, &mut ret);

        ret
    }

    fn dfs(cur: i32, n: i32, k: i32, array: &mut Vec<i32>, ret: &mut Vec<Vec<i32>>) {
        if array.len() == k as usize {
            ret.push(array.clone());
            return;
        }
        for i in cur..n+1 {
            array.push(i);
            Self::dfs(i + 1, n, k, array, ret);
            array.pop();
        }
    }
}

