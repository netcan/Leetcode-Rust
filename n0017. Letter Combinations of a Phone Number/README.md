## Letter Combinations of a Phone Number :star::star:
- 题目地址: [https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number](https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number)
- 执行时间: 0 ms 
- 内存消耗: 745.5 KB
- 通过日期: 2019-02-19 19:37

### 题目内容
<p>给定一个仅包含数字 <code>2-9</code> 的字符串，返回所有它能表示的字母组合。</p>

<p>给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。</p>

<p><img src="http://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png"></p>

<p><strong>示例:</strong></p>

<pre><strong>输入：</strong>"23"
<strong>输出：</strong>["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
</pre>

<p><strong>说明:</strong><br>
尽管上面的答案是按字典序排列的，但是你可以任意选择答案输出的顺序。</p>


### 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {

    const ALPHA: [&'static str; 10] = [
        "", // 0
        "", // 1
        "abc", // 2
        "def", // 3
        "ghi", // 4
        "jkl", // 5
        "mno", // 6
        "pqrs", // 7
        "tuv", // 8
        "wxyz", // 9
    ];
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ret = Vec::new();
        if digits.len() == 0 {
            return ret;
        }
        Self::dfs(&digits, &mut String::new(), &mut ret);
        ret
    }

    fn dfs(digits: &str, comb: &mut String, ret: &mut Vec<String>) {
        if digits.len() == 0 {
            ret.push(comb.clone());
            return;
        }

        if let Some(c) = digits.chars().nth(0) {
            for cc in Self::ALPHA[c as usize - '0' as usize].chars() {
                comb.push(cc);
                Self::dfs(&digits[1..], comb, ret);
                comb.pop();
            }
        }
    }
}


```
