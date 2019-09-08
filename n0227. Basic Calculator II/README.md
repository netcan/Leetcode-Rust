# Basic Calculator II :star::star:
- 题目地址: [https://leetcode-cn.com/problems/basic-calculator-ii](https://leetcode-cn.com/problems/basic-calculator-ii)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-09-08 19:24

## 题目内容
<p>实现一个基本的计算器来计算一个简单的字符串表达式的值。</p>

<p>字符串表达式仅包含非负整数，<code>+</code>， <code>-</code> ，<code>*</code>，<code>/</code> 四种运算符和空格 <code> </code>。 整数除法仅保留整数部分。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入: </strong>"3+2*2"
<strong>输出:</strong> 7
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> " 3/2 "
<strong>输出:</strong> 1</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> " 3+5 / 2 "
<strong>输出:</strong> 5
</pre>

<p><strong>说明：</strong></p>

<ul>
	<li>你可以假设所给定的表达式都是有效的。</li>
	<li>请<strong>不要</strong>使用内置的库函数 <code>eval</code>。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

// Expr    ← Product (('+' / '-') Product)*
// Product ← Value (('*' / '/') Value)*
// Value   ← [0-9]+ / '(' Expr ')'
impl Solution {
    fn peek(ptr: &mut *const u8) -> char {
        unsafe { **ptr as char }
    }
    fn get(ptr: &mut *const u8) -> char {
        unsafe {
        let ch = Solution::peek(ptr);
        loop {
            *ptr = ptr.offset(1);
            if Solution::peek(ptr) != ' ' {
                break;
            }
        }
        ch
        }
    }

    // Value   ← [0-9]+ / '(' Expr ')'
    fn value(ptr: &mut *const u8) -> i32 {
        if Solution::peek(ptr) == '(' {
            Solution::get(ptr); // eat '('
            let val = Solution::expr(ptr);
            Solution::get(ptr); // eat ')'
            return val;
        }
        let mut val: i32 = 0;
        while Solution::peek(ptr) >= '0' && Solution::peek(ptr) <= '9' {
            val = val * 10 + (Solution::get(ptr) as u8 - '0' as u8) as i32;
        }
        val
    }

    // Expr    ← Product (('+' / '-') Product)*
    fn expr(ptr: &mut *const u8) -> i32 {
        let mut lhs = Solution::product(ptr);
        loop {
            let op = Solution::peek(ptr);
            match op {
                '+' => {
                    Solution::get(ptr); // eat op
                    lhs += Solution::product(ptr);
                },
                '-' => {
                    Solution::get(ptr); // eat op
                    lhs -= Solution::product(ptr);
                }
                _ => { break; }
            }
        }
        lhs
    }

    // Product ← Value (('*' / '/') Value)*
    fn product(ptr: &mut *const u8) -> i32 {
        let mut lhs = Solution::value(ptr);
        loop {
            let op = Solution::peek(ptr);
            match op {
                '*' => {
                    Solution::get(ptr); // eat op
                    lhs *= Solution::value(ptr);
                },
                '/' => {
                    Solution::get(ptr); // eat op
                    lhs /= Solution::value(ptr);
                }
                _ => { break; }
            }
        }
        lhs
    }

    pub fn calculate(mut expr: String) -> i32 {
        expr = expr.trim().to_string();
        expr.push(0 as char); // add '\0' to string end
        Solution::expr(&mut expr.as_ptr())
    }
}

```