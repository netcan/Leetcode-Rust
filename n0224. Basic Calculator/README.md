# Basic Calculator :star::star::star:
- 题目地址: [https://leetcode-cn.com/problems/basic-calculator](https://leetcode-cn.com/problems/basic-calculator)
- 执行时间: 0 ms 
- 内存消耗: 2.4 MB
- 通过日期: 2019-09-08 19:00

## 题目内容
<p>实现一个基本的计算器来计算一个简单的字符串表达式的值。</p>

<p>字符串表达式可以包含左括号 <code>(</code> ，右括号 <code>)</code>，加号 <code>+</code> ，减号 <code>-</code>，<strong>非负</strong>整数和空格 <code> </code>。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> "1 + 1"
<strong>输出:</strong> 2
</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入:</strong> " 2-1 + 2 "
<strong>输出:</strong> 3</pre>

<p><strong>示例 3:</strong></p>

<pre><strong>输入:</strong> "(1+(4+5+2)-3)+(6+8)"
<strong>输出:</strong> 23</pre>

<p><strong>说明：</strong></p>

<ul>
	<li>你可以假设所给定的表达式都是有效的。</li>
	<li>请<strong>不要</strong>使用内置的库函数 <code>eval</code>。</li>
</ul>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

// Expr    ← Value (('+' / '-') Value)*
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

    // Expr    ← Value (('+' / '-') Value)*
    fn expr(ptr: &mut *const u8) -> i32 {
        let mut lhs = Solution::value(ptr);
        loop {
            let op = Solution::peek(ptr);
            match op {
                '+' => {
                    Solution::get(ptr); // eat op
                    lhs += Solution::value(ptr);
                },
                '-' => {
                    Solution::get(ptr); // eat op
                    lhs -= Solution::value(ptr);
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