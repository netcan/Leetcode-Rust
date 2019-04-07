# Russian Doll Envelopes :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/russian-doll-envelopes](https://leetcode-cn.com/problems/russian-doll-envelopes)
- 执行时间: 8 ms 
- 内存消耗: 3.3 MB
- 通过日期: 2019-03-21 18:49

## 题目内容
---
<p>给定一些标记了宽度和高度的信封，宽度和高度以整数对形式 <code>(w, h)</code> 出现。当另一个信封的宽度和高度都比这个信封大的时候，这个信封就可以放进另一个信封里，如同俄罗斯套娃一样。</p>

<p>请计算最多能有多少个信封能组成一组“俄罗斯套娃”信封（即可以把一个信封放到另一个信封里面）。</p>

<p><strong>说明:</strong><br>
不允许旋转信封。</p>

<p><strong>示例:</strong></p>

<pre><strong>输入:</strong> envelopes = <code>[[5,4],[6,4],[6,7],[2,3]]</code>
<strong>输出:</strong> 3 
<strong>解释:</strong> 最多信封的个数为 <code>3, 组合为: </code>[2,3] => [5,4] => [6,7]。
</pre>


## 解法
---
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // 按宽度升序排，高度降序排，然后对高度求LIS
        if envelopes.len() <= 0 { return 0; }
        envelopes.sort_by(|a, b| {
            if a[0] != b[0] { return a[0].cmp(&b[0]); }
            return b[1].cmp(&a[1]);
        });

        let mut longest = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            match longest.binary_search(&envelope[1]) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = envelope[1];
                    }  else  {
                        longest.push(envelope[1]);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}


```