# Different Ways to Add Parentheses :star::star:
- 题目地址: [https://leetcode-cn.com/problems/different-ways-to-add-parentheses](https://leetcode-cn.com/problems/different-ways-to-add-parentheses)
- 执行时间: 0 ms 
- 内存消耗: 2.5 MB
- 通过日期: 2019-04-20 16:10

## 题目内容
<p>给定一个含有数字和运算符的字符串，为表达式添加括号，改变其运算优先级以求出不同的结果。你需要给出所有可能的组合的结果。有效的运算符号包含 <code>+</code>, <code>-</code> 以及 <code>*</code> 。</p>

<p><strong>示例 1:</strong></p>

<pre><strong>输入:</strong> <code>"2-1-1"</code>
<strong>输出:</strong> <code>[0, 2]</code>
<strong>解释: </strong>
((2-1)-1) = 0 
(2-(1-1)) = 2</pre>

<p><strong>示例 2:</strong></p>

<pre><strong>输入: </strong><code>"2*3-4*5"</code>
<strong>输出:</strong> <code>[-34, -14, -10, -10, 10]</code>
<strong>解释: 
</strong>(2*(3-(4*5))) = -34 
((2*3)-(4*5)) = -14 
((2*(3-4))*5) = -10 
(2*((3-4)*5)) = -10 
(((2*3)-4)*5) = 10</pre>


## 解法
```rust
// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

fn bin_op(a: i32, b: i32, o: char) -> i32 {
    match o {
        '+' => { a + b },
        '-' => { a - b },
        '*' => { a * b },
        _ => { unreachable!() }
    }
}

impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let numbers: Vec<i32> = input
            .split(|c| c == '+' || c == '-' || c == '*')
            .map(|x| x.parse().unwrap())
            .collect();

        let op: Vec<char> = input
            .split(char::is_numeric)
            .filter(|o| o.len() > 0)
            .map(|o| o.as_bytes()[0] as char).collect();

        println!("numbers: {:?} op: {:?}", numbers, op);
        Solution::diff_ways_to_compute_(&numbers, &op)
    }
    pub fn diff_ways_to_compute_(numbers: &[i32], op: &[char]) -> Vec<i32> {
        if numbers.len() == 1 { return vec![numbers[0]]; }

        let mut result = vec![];
        // 计算左部分[0..j) op 右部分[j..end)
        let end = numbers.len();
        for j in 1..end {
            let (left_vals, right_vals) = (
                Solution::diff_ways_to_compute_(&numbers[0..j],   &op[0..j-1]),
                Solution::diff_ways_to_compute_(&numbers[j..end], &op[j..op.len()])
            );
            for &lval in &left_vals {
                for &rval in &right_vals {
                    result.push(bin_op(lval, rval, op[j-1]))
                }
            }
        }

        result
    }
}


```