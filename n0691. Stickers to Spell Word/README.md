## Stickers to Spell Word :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/stickers-to-spell-word](https://leetcode-cn.com/problems/stickers-to-spell-word)
- 执行时间: 428 ms 
- 内存消耗: 2.6 MB
- 通过日期: 2019-03-20 12:11

### 题目内容
---
<p>我们给出了 N 种不同类型的贴纸。每个贴纸上都有一个小写的英文单词。</p>

<p>你希望从自己的贴纸集合中裁剪单个字母并重新排列它们，从而拼写出给定的目标字符串 <code>target</code>。</p>

<p>如果你愿意的话，你可以不止一次地使用每一张贴纸，而且每一张贴纸的数量都是无限的。</p>

<p>拼出目标 <code>target</code> 所需的最小贴纸数量是多少？如果任务不可能，则返回 -1。</p>



<p><strong>示例 1：</strong></p>

<p>输入：</p>

<pre>["with", "example", "science"], "thehat"
</pre>

<p>输出：</p>

<pre>3
</pre>

<p>解释：</p>

<pre>我们可以使用 2 个 "with" 贴纸，和 1 个 "example" 贴纸。
把贴纸上的字母剪下来并重新排列后，就可以形成目标 “thehat“ 了。
此外，这是形成目标字符串所需的最小贴纸数量。
</pre>

<p><strong>示例 2：</strong></p>

<p>输入：</p>

<pre>["notice", "possible"], "basicbasic"
</pre>

<p>输出：</p>

<pre>-1
</pre>

<p>解释：</p>

<pre>我们不能通过剪切给定贴纸的字母来形成目标“basicbasic”。
</pre>



<p><strong>提示：</strong></p>

<ul>
	<li><code>stickers</code> 长度范围是 <code>[1, 50]</code>。</li>
	<li><code>stickers</code> 由小写英文单词组成（不带撇号）。</li>
	<li><code>target</code> 的长度在 <code>[1, 15]</code> 范围内，由小写字母组成。</li>
	<li>在所有的测试案例中，所有的单词都是从 1000 个最常见的美国英语单词中随机选取的，目标是两个随机单词的串联。</li>
	<li>时间限制可能比平时更具挑战性。预计 50 个贴纸的测试案例平均可在35ms内解决。</li>
</ul>




### 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // stickers[i] = Map(char, char_count)
        let stickers: Vec<HashMap<u8, i32>> = stickers.into_iter().map(|x| {
            let mut char_count = HashMap::new();
            for c in x.into_bytes() {
                *char_count.entry(c).or_insert(0) += 1;
            }
            char_count
        }).collect();
        let target = target.into_bytes();
        let mut vis = vec![false; 1 << target.len()];
        let mut que = VecDeque::new();
        // que: (i16: 每一位代表是否完成字母, step)
        // 0101:表示第1, 3个字母已经填上
        que.push_back((0u16, 0));
        vis[0] = true;

        // BFS
        while !que.is_empty() {
            let (cur_chars, cur_step) = que.pop_front().unwrap();
            if cur_chars == (1 << target.len()) - 1 {
                return cur_step;
            }

            for sticker in &stickers {
                let mut next_chars = cur_chars;
                let mut char_count = sticker.clone();
                for i in (0..target.len()) {
                    if next_chars & (1 << i) == 0 && *char_count.entry(target[i]).or_insert(0) > 0 {
                        *char_count.get_mut(&target[i]).unwrap() -= 1;
                        next_chars |= (1 << i);
                    }
                }

                if next_chars != cur_chars && !vis[next_chars as usize] {
                    que.push_back((next_chars, cur_step + 1));
                    vis[next_chars as usize] = true;
                }
            }

        }

        -1
    }
}


```