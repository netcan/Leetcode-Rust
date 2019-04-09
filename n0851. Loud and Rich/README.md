# Loud and Rich :star::star:
- 题目地址: [https://leetcode-cn.com/problems/loud-and-rich](https://leetcode-cn.com/problems/loud-and-rich)
- 执行时间: 20 ms 
- 内存消耗: 4.2 MB
- 通过日期: 2019-04-08 19:44

## 题目内容
<p>在一组 N 个人（编号为 <code>0, 1, 2, ..., N-1</code>）中，每个人都有不同数目的钱，以及不同程度的安静（quietness）。</p>

<p>为了方便起见，我们将编号为 <code>x</code> 的人简称为 "person <code>x</code> "。</p>

<p>如果能够肯定 person <code>x</code> 比 person <code>y</code> 更有钱的话，我们会说 <code>richer[i] = [x, y]</code> 。注意 <code>richer</code> 可能只是有效观察的一个子集。</p>

<p>另外，如果 person <code>x</code> 的安静程度为 <code>q</code> ，我们会说 <code>quiet[x] = q</code> 。</p>

<p>现在，返回答案 <code>answer</code> ，其中 <code>answer[x] = y</code> 的前提是，在所有拥有的钱不少于 person <code>x</code> 的人中，person <code>y</code> 是最安静的人（也就是安静值 <code>quiet[y]</code> 最小的人）。</p>

<p><strong>示例：</strong></p>

<pre><strong>输入：</strong>richer = [[1,0],[2,1],[3,1],[3,7],[4,3],[5,3],[6,3]], quiet = [3,2,5,4,6,1,7,0]
<strong>输出：</strong>[5,5,2,5,4,5,6,7]
<strong>解释： </strong>
answer[0] = 5，
person 5 比 person 3 有更多的钱，person 3 比 person 1 有更多的钱，person 1 比 person 0 有更多的钱。
唯一较为安静（有较低的安静值 quiet[x]）的人是 person 7，
但是目前还不清楚他是否比 person 0 更有钱。

answer[7] = 7，
在所有拥有的钱肯定不少于 person 7 的人中(这可能包括 person 3，4，5，6 以及 7)，
最安静(有较低安静值 quiet[x])的人是 person 7。

其他的答案也可以用类似的推理来解释。
</pre>

<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= quiet.length = N <= 500</code></li>
	<li><code>0 <= quiet[i] < N</code>，所有 <code>quiet[i]</code> 都不相同。</li>
	<li><code>0 <= richer.length <= N * (N-1) / 2</code></li>
	<li><code>0 <= richer[i][j] < N</code></li>
	<li><code>richer[i][0] != richer[i][1]</code></li>
	<li><code>richer[i]</code> 都是不同的。</li>
	<li>对 <code>richer</code> 的观察在逻辑上是一致的。</li>
</ol>


## 解法
```rust
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


```