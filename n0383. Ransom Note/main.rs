// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let ransom_note  = ransom_note.into_bytes();
        let mut magazine = magazine.into_bytes();
        for cr in ransom_note {
            let mut find = false;
            for (j, &cm) in magazine.iter().enumerate() {
                if cr == cm {
                    find = true;
                    magazine.remove(j);
                    break;
                }
            }
            if !find {
                return false;
            }
        }

        true
    }
}

