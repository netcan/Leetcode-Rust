# Camelcase Matching :star::star:
- 题目地址: [https://leetcode-cn.com/problems/camelcase-matching](https://leetcode-cn.com/problems/camelcase-matching)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-04-07 12:23

## 题目内容
<p>如果我们可以将<strong>小写字母</strong>插入模式串 <code>pattern</code> 得到待查询项 <code>query</code>，那么待查询项与给定模式串匹配。（我们可以在任何位置插入每个字符，也可以插入 0 个字符。）</p>

<p>给定待查询列表 <code>queries</code>，和模式串 <code>pattern</code>，返回由布尔值组成的答案列表 <code>answer</code>。只有在待查项 <code>queries[i]</code> 与模式串 <code>pattern</code> 匹配时， <code>answer[i]</code> 才为 <code>true</code>，否则为 <code>false</code>。</p>



<p><strong>示例 1：</strong></p>

<pre><strong>输入：</strong>queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FB"
<strong>输出：</strong>[true,false,true,true,false]
<strong>示例：</strong>
"FooBar" 可以这样生成："F" + "oo" + "B" + "ar"。
"FootBall" 可以这样生成："F" + "oot" + "B" + "all".
"FrameBuffer" 可以这样生成："F" + "rame" + "B" + "uffer".</pre>

<p><strong>示例 2：</strong></p>

<pre><strong>输入：</strong>queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBa"
<strong>输出：</strong>[true,false,true,false,false]
<strong>解释：</strong>
"FooBar" 可以这样生成："Fo" + "o" + "Ba" + "r".
"FootBall" 可以这样生成："Fo" + "ot" + "Ba" + "ll".
</pre>

<p><strong>示例 3：</strong></p>

<pre><strong>输出：</strong>queries = ["FooBar","FooBarTest","FootBall","FrameBuffer","ForceFeedBack"], pattern = "FoBaT"
<strong>输入：</strong>[false,true,false,false,false]
<strong>解释： </strong>
"FooBarTest" 可以这样生成："Fo" + "o" + "Ba" + "r" + "T" + "est".
</pre>



<p><strong>提示：</strong></p>

<ol>
	<li><code>1 <= queries.length <= 100</code></li>
	<li><code>1 <= queries[i].length <= 100</code></li>
	<li><code>1 <= pattern.length <= 100</code></li>
	<li>所有字符串都仅由大写和小写英文字母组成。</li>
</ol>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let pattern = pattern.into_bytes();
        let mut ans = vec![false; queries.len()];
        let queries: Vec<Vec<u8>> = queries.into_iter().map(|x| x.into_bytes()).collect();
        for i in 0..queries.len() {
            let word = &queries[i];
            let (mut j, mut k) = (0, 0);
            while j < word.len() && k < pattern.len() {
                if word[j] == pattern[k] { j += 1; k += 1; }
                else if word[j] >= 'a' as u8 && word[j] <= 'z' as u8 { j += 1; }
                else { break; }
            }
            while j < word.len() {
                if word[j] >= 'a' as u8 && word[j] <= 'z' as u8 { j += 1; }
                else { break; }
            }
            ans[i] = j >= word.len() && k >= pattern.len();
        }

        ans
    }
}

```