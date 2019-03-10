impl Solution {
    pub fn judge_point24(nums: Vec<i32>) -> bool {
        Solution::judge_point24_(
            nums.iter().map(|&x| { x as f64 }).collect()
            )
    }

    pub fn judge_point24_(nums: Vec<f64>) -> bool {
        if nums.len() == 0 { return false; }
        if nums.len() == 1 && (nums[0] - 24.0).abs() < 1e-6 { return true; }
        // 选出2个数，进行运算，然后放回列表中
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