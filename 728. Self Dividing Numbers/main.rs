impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let ret = (left..=right).filter(|&x| {
            let mut _x = x;
            let mut dividing = true;
            while(_x > 0) {
                if(_x % 10 == 0 || (x % (_x % 10)) != 0) {
                    dividing = false;
                    break;
                }
                _x /= 10;
            }
            dividing
        }).collect();
        ret
    }
}
