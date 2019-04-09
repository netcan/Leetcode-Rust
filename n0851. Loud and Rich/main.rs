// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut graph = vec![vec![]; quiet.len()]; // 邻接表
        let mut ans = vec![-1; quiet.len()];
        for e in &richer {
            graph[e[1] as usize].push(e[0]);
        }
        for v in 0..quiet.len() {
            ans[v] = Solution::find_min_quiet(v, &mut ans, &graph, &quiet) as i32;
        }

        ans
    }
    fn find_min_quiet(p: usize, memo: &mut Vec<i32>, graph: &Vec<Vec<i32>>, quiet: &Vec<i32>) -> usize {
        if memo[p] != -1 { return memo[p] as usize; }

        let mut min_quiet = p;
        for &v in &graph[p] {
            let q = Solution::find_min_quiet(v as usize, memo, graph, quiet);
            if quiet[min_quiet] > quiet[q] { min_quiet = q; }
        }

        memo[p] = min_quiet as i32;
        min_quiet
    }
}

