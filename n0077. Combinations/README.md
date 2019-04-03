## Combinations :star::star:
- 题目地址: [https://leetcode-cn.com/problems/combinations](https://leetcode-cn.com/problems/combinations)
- 执行时间: 12 ms 
- 内存消耗: 3.2 MB
- 通过日期: 2019-02-18 16:39

### 题目内容
<p>给定两个整数 <em>n</em> 和 <em>k</em>，返回 1 ... <em>n </em>中所有可能的 <em>k</em> 个数的组合。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> n = 4, k = 2
<strong>输出:</strong>
[
  [2,4],
  [3,4],
  [2,3],
  [1,2],
  [1,3],
  [1,4],
]</pre>


### 解法
```rust
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


```
