## Ransom Note :star:
- 题目地址: [https://leetcode-cn.com/problems/ransom-note](https://leetcode-cn.com/problems/ransom-note)
- 执行时间: 0 ms 
- 内存消耗: 2.8 MB
- 通过日期: 2019-03-20 14:43

### 题目内容
<p>给定一个赎金信 (ransom) 字符串和一个杂志(magazine)字符串，判断第一个字符串ransom能不能由第二个字符串magazines里面的字符构成。如果可以构成，返回 true ；否则返回 false。</p>

<p>(题目说明：为了不暴露赎金信字迹，要从杂志上搜索各个需要的字母，组成单词来表达意思。)</p>

<p><strong>注意：</strong></p>

<p>你可以假设两个字符串均只含有小写字母。</p>

<pre>
canConstruct("a", "b") -> false
canConstruct("aa", "ab") -> false
canConstruct("aa", "aab") -> true
</pre>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let ransom_note  = ransom_note.into_bytes();
        let mut magazine = magazine.into_bytes();
        for cr in ransom_note {
            let mut find = false;
            for (j, &cm) in magazine.iter().enumerate() {
                if cr == cm {
                    find = true;
                    magazine.remove(j);
                    break;
                }
            }
            if !find {
                return false;
            }
        }

        true
    }
}


```
