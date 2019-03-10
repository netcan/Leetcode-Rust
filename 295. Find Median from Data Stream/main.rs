use std::collections::BTreeMap;

struct MedianFinder {
    container: BTreeMap<i32, i32>
}

impl MedianFinder {
    fn new() -> Self {
        MedianFinder{
            container: BTreeMap::new()
        }
    }

    fn add_num(&mut self, num: i32) {
        *self.container.entry(num).or_insert(0) += 1;
    }

    fn find_median(&self) -> f64 {
        let mut iter = self.container.iter();
        let n = self.container.values().sum::<i32>();

        let (mut pos, mid_pos) = (-1, ((n - 1)/ 2) as i32);
        let mut num = 0;

        while pos < mid_pos {
            let (k, v) = iter.next().unwrap();
            let times = *v;
            num = *k;
            if pos + times > mid_pos {
                return num as f64;
            }
            pos += times;
        }

        if n % 2 == 0 {
            (num + *iter.next().unwrap().0) as f64 / 2.0
        } else {
            num as f64
        }
    }
}
