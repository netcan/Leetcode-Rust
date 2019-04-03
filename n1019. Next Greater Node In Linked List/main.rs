// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn next_larger_nodes(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut num = vec![];
        while let Some(node) = head {
            num.push(node.val);
            head = node.next;
        }


        let mut ans = vec![0; num.len()];
        let mut stack = vec![];

        for i in 0..num.len() {
            while stack.len() > 0 && num[*stack.last().unwrap()] < num[i] {
                ans[stack.pop().unwrap()] = num[i];
            }
            stack.push(i);
        }

        ans
    }
}
