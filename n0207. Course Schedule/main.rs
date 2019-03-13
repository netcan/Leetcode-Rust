// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::iter;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph = iter::repeat(Vec::<i32>::new()).take(num_courses as usize).collect::<Vec<Vec<i32>>>();
        let mut indegree = vec![0; num_courses as usize];

        // 建图
        for edge in &prerequisites {
            let (u, v) = (edge[0], edge[1]);
            graph[v as usize].push(u); // v->u
            indegree[u as usize] += 1;
        }

        // 入度为0的节点
        let mut S = Vec::new();
        for (node, &degree) in indegree.iter().enumerate() {
            if degree == 0 {
                S.push(node as i32);
            }
        }

        while !S.is_empty() {
            let v = S.pop().unwrap();
            for &u in &graph[v as usize] {
                indegree[u as usize] -= 1;
                if indegree[u as usize] == 0 {
                    S.push(u);
                }
            }
        }

        // 图是否为空
        if (indegree.iter().all(|&e| e == 0)) {
            true
        } else {
            false
        }
    }
}

