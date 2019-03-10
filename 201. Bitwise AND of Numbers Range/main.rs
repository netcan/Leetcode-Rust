impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let m: u32 = m as u32;
        let n: u32 = n as u32;
        let diff = n - m + 1;
        let mut bit_and = 1;
        while bit_and < diff {
            bit_and <<= 1;
        }
        (m & !(bit_and - 1) & n) as i32
    }
}
