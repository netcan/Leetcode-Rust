// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring = ring.into_bytes();
        let key = key.into_bytes();
        // 记录各个字母的位置，快速检索用
        let mut alpha_pos = vec![Vec::new(); 26];
        for (idx, &alpha) in ring.iter().enumerate() {
            alpha_pos[(alpha - 'a' as u8) as usize].push(idx);
        }

        // dp[k_idx][ring_pos] = min(dp[k_idx - 1][ring_pos] + step), step是上一步到这一步的距离
        let mut dp: Vec<Vec<i32>> =
            vec![vec![(ring.len() * key.len()) as i32; ring.len() + 1]; key.len() + 1];

        dp[0][0] = 0; // 初始状态，转盘指向0点(ring_pos == 0)
        let ring_len = ring.len() as i32;

        for k_idx in 0..key.len() {
            for &k_pos in &alpha_pos[(key[k_idx] - 'a' as u8) as usize] {
                for ring_pos in 0..ring.len() {
                    let step = ((k_pos as i32 - ring_pos as i32 + ring_len) % ring_len).min(
                        (ring_pos as i32 - k_pos as i32 + ring_len) % ring_len
                    );

                    dp[k_idx + 1][k_pos] = dp[k_idx + 1][k_pos].min(
                        dp[k_idx][ring_pos] + step
                    );
                }
            }
        }

        dp[key.len()].iter().min().unwrap() + key.len() as i32
    }
}
