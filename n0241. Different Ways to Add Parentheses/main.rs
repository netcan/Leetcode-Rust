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

