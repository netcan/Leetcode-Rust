// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        // 按宽度升序排，高度降序排，然后对高度求LIS
        if envelopes.len() <= 0 { return 0; }
        envelopes.sort_by(|a, b| {
            if a[0] != b[0] { return a[0].cmp(&b[0]); }
            return b[1].cmp(&a[1]);
        });

        let mut longest = Vec::with_capacity(envelopes.len());
        for envelope in envelopes {
            match longest.binary_search(&envelope[1]) {
                Err(pos) => {
                    if pos < longest.len() {
                        longest[pos] = envelope[1];
                    }  else  {
                        longest.push(envelope[1]);
                    }
                },
                _ => {}
            }
        }

        longest.len() as i32
    }
}

