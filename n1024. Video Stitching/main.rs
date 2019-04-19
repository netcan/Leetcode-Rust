// Author: Netcan @ https://github.com/netcan/Leetcode-Rust
// Zhihu: https://www.zhihu.com/people/netcan

impl Solution {
    pub fn video_stitching(mut clips: Vec<Vec<i32>>, t: i32) -> i32 {
        clips.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        // dp[t]: [0, t]所需要的最少片段数量
        // dp[t] = min(dp[s..=t] + 1)
        let mut dp = [clips.len() as i32 + 1; 101];
        dp[0] = 0;
        for i in 0..clips.len() {
            let (s, e) = (clips[i][0], clips[i][1]);
            for j in s..=e {
                dp[e as usize] = dp[e as usize].min(
                    dp[j as usize] + 1
                    );
            }
        }
        let mut ans = clips.len() as i32 + 1;
        for i in t..=100 {
            ans = ans.min(dp[i as usize]);
        }

        if ans > clips.len() as i32 { -1 }
        else { ans }
    }
}
