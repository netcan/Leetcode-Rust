// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::VecDeque;
use std::collections::HashMap;
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // stickers[i] = Map(char, char_count)
        let stickers: Vec<HashMap<u8, i32>> = stickers.into_iter().map(|x| {
            let mut char_count = HashMap::new();
            for c in x.into_bytes() {
                *char_count.entry(c).or_insert(0) += 1;
            }
            char_count
        }).collect();
        let target = target.into_bytes();
        let mut vis = vec![false; 1 << target.len()];
        let mut que = VecDeque::new();
        // que: (i16: 每一位代表是否完成字母, step)
        // 0101:表示第1, 3个字母已经填上
        que.push_back((0u16, 0));
        vis[0] = true;

        // BFS
        while !que.is_empty() {
            let (cur_chars, cur_step) = que.pop_front().unwrap();
            if cur_chars == (1 << target.len()) - 1 {
                return cur_step;
            }

            for sticker in &stickers {
                let mut next_chars = cur_chars;
                let mut char_count = sticker.clone();
                for i in (0..target.len()) {
                    if next_chars & (1 << i) == 0 && *char_count.entry(target[i]).or_insert(0) > 0 {
                        *char_count.get_mut(&target[i]).unwrap() -= 1;
                        next_chars |= (1 << i);
                    }
                }

                if next_chars != cur_chars && !vis[next_chars as usize] {
                    que.push_back((next_chars, cur_step + 1));
                    vis[next_chars as usize] = true;
                }
            }

        }

        -1
    }
}

