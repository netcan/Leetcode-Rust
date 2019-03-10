
### 24 Game
- 题目地址/Problem Url: [https://leetcode-cn.com/problems/24-game](https://leetcode-cn.com/problems/24-game)
- 执行时间/Runtime: 4 ms 
- 内存消耗/Mem Usage: 2.3 MB
- 提交日期/Datime: 2019-03-09 18:07

```rust
impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        Solution::judge_point24_(
            nums.iter().map(|&x| { x as f64 }).collect()
            )
    }

    pub fn judge_point24_(nums: Vec<f64>) -> bool {
        if nums.len() == 0 { return false; }
        if nums.len() == 1 && (nums[0] - 24.0).abs() < 1e-6 { return true; }
        // éåº2ä¸ªæ°ï¼è¿è¡è¿ç®ï¼ç¶åæ¾ååè¡¨ä¸­
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    continue;
                }
                for op in &['+', '*', '-', '/'] {
                    let mut nums_: Vec<f64> = nums.iter()
                        .enumerate()
                        .filter(|&(k, _)| k != i && k != j)
                        .map(|(_, v)| v )
                        .cloned().collect();
                    match op {
                        '+'  => if (i < j) { nums_.push(nums[i] + nums[j]) } else { continue; },
                        '*'  => if (i < j) { nums_.push(nums[i] * nums[j]) } else { continue; },
                        '-'  => nums_.push(nums[i] - nums[j]),
                        '/'  => nums_.push(nums[i] / nums[j]),
                        _    => {}
                    };
                    if Solution::judge_point24_(nums_) { return true; }
                }
            }
        }

        false
    }
}
```
