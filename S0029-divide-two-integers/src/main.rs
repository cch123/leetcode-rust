struct Solution;
impl Solution {
    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        let sign = if (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0) {
            -1
        } else {
            1
        };

        let mut adj = 0;
        let mut q = 0;
        let mut r = 0;
        
        if divisor == std::i32::MIN {
            if divisor == dividend {
                return 1
            } else {
                return 0
            }
        }

        if dividend == std::i32::MIN {
            if divisor == 1 {
                return std::i32::MIN;
            }
            if divisor == -1 {
                return std::i32::MAX;
            }
            dividend += divisor.abs();
            adj += 1;
        }
        
        dividend = dividend.abs();
        divisor = divisor.abs();
                       
        for i in (0..32).rev() {
            r <<= 1;
            if dividend & (1 << i) != 0 {
                r |= 1;
            }
            if r >= divisor {
                r -= divisor;
                q = q | (1 << i);
            }
        }
        
        if sign < 0 {
            -q - adj
        } else {
            q + adj
        }
    }
}

fn main() {}
