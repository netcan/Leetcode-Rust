## Assign Cookies :star:
- 题目地址: [https://leetcode-cn.com/problems/assign-cookies](https://leetcode-cn.com/problems/assign-cookies)
- 执行时间: 12 ms 
- 内存消耗: 3.1 MB
- 通过日期: 2019-03-05 20:22

### 题目内容
<p>假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是，每个孩子最多只能给一块饼干。对每个孩子 i ，都有一个胃口值 g<sub>i ，</sub>这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j ，都有一个尺寸 s<sub>j </sub>。如果 s<sub>j</sub> >= g<sub>i </sub>，我们可以将这个饼干 j 分配给孩子 i ，这个孩子会得到满足。你的目标是尽可能满足越多数量的孩子，并输出这个最大数值。</p>

<p><strong>注意：</strong></p>

<p>你可以假设胃口值为正。<br />
一个小朋友最多只能拥有一块饼干。</p>

<p><strong>示例 1:</strong></p>

<pre>
<strong>输入:</strong> [1,2,3], [1,1]

<strong>输出:</strong> 1

<strong>解释:</strong> 
你有三个孩子和两块小饼干，3个孩子的胃口值分别是：1,2,3。
虽然你有两块小饼干，由于他们的尺寸都是1，你只能让胃口值是1的孩子满足。
所以你应该输出1。
</pre>

<p><strong>示例 2:</strong></p>

<pre>
<strong>输入:</strong> [1,2], [1,2,3]

<strong>输出:</strong> 2

<strong>解释:</strong> 
你有两个孩子和三块小饼干，2个孩子的胃口值分别是1,2。
你拥有的饼干数量和尺寸都足以让所有孩子满足。
所以你应该输出2.
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();

        let mut ret = 0;
        let mut j = 0;
        for ss in &s {
            if j >= g.len() {
                break;
            }
            if *ss >= g[j] {
                ret += 1;
                j += 1;
            }
        }

        ret
    }
}



```
