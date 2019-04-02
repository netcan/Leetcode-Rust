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

        for i in 0..num.len() {
            let mut pos: i32 = -1;
            for j in i + 1..num.len() {
                if num[j] > num[i] {
                    pos = j as i32;
                    break;
                }
            }
            if pos == -1 {
                ans[i] = 0;
            } else {
                ans[i] = num[pos as usize];
            }
        }

        ans
    }
}
