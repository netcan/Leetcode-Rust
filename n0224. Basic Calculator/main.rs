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
