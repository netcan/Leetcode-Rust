
### Length of Last Word
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/length-of-last-word](https://leetcode-cn.com/problems/length-of-last-word)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 2.4 MB
- 提交日期/Datime: 2019-03-05 19:11

```rust
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() == 0 {
            return 0;
        }
        return words[words.len() - 1].len() as i32;

    }
}

```
