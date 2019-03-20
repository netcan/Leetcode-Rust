// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // Map(char, char_count)
        let mut char_count = HashMap::new();
        for c in magazine.into_bytes() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        for c in ransom_note.into_bytes() {
            match char_count.get_mut(&c) {
                Some(ref mut v) if **v > 0 => { **v -= 1; },
                _ => { return false; }
            }
        }

        true
    }
}

