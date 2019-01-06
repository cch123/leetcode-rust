struct Solution;

use std::collections::HashSet;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let mut set = HashSet::new();
        let (xb, yb) = (
            ((bound as f64).log(x as f64) as i32).max(1),
            ((bound as f64).log(y as f64) as i32).max(1),
        );

        (0..=xb)
            .map(|i| {
                (0..=yb)
                    .map(|j| {
                        let num = ((x as f64).powi(i) + (y as f64).powi(j)) as i32;
                        if num <= bound && num > 0 {
                            set.insert(num);
                        }
                    })
                    .for_each(drop);
            })
            .for_each(drop);

        set.iter().map(|&num| num).collect()
    }
}

fn main() {
    println!("{:?}", Solution::powerful_integers(2, 3, 10));
    println!("{:?}", Solution::powerful_integers(90, 90, 1000000));
    println!("{:?}", Solution::powerful_integers(1, 2, 100));
}
