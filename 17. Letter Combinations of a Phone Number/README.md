
### Letter Combinations of a Phone Number
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number](https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number)
- 执行时间/Runtime: 0 ms 
- 内存消耗/Mem Usage: 745.5 KB
- 提交日期/Datime: 2019-02-19 19:37

```rust
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
