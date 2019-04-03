## Climbing Stairs :star:
- 题目地址: [https://leetcode-cn.com/problems/climbing-stairs](https://leetcode-cn.com/problems/climbing-stairs)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-03-05 19:27

### 题目内容
<p>假设你正在爬楼梯。需要 <em>n</em> 阶你才能到达楼顶。</p>

<p>每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？</p>

<p><strong>注意：</strong>给定 <em>n</em> 是一个正整数。</p>

<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong> 2
<strong>输出：</strong> 2
<strong>解释：</strong> 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong> 3
<strong>输出：</strong> 3
<strong>解释：</strong> 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut vec = vec![0; (n + 2) as usize];
        vec[1] = 1;
        vec[2] = 2;
        for i in (3..=n as usize) {
            vec[i] = vec[i - 1] + vec[i - 2];
        }
        vec[n as usize]
    }
}


```
