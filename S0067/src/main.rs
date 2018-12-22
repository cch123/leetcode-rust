struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = (
            a.chars().rev().collect::<String>().into_bytes(),
            b.chars().rev().collect::<String>().into_bytes(),
        );

        let (mut res, mut idx, mut carry) = ("".to_string(), 0, 0);

        while a.get(idx).is_some() || b.get(idx).is_some() {
            let mut sum = carry;
            if a.get(idx).is_some() {
                sum += a.get(idx).unwrap() - 48;
            }
            if b.get(idx).is_some() {
                sum += b.get(idx).unwrap() - 48;
            }

            carry = sum / 2;
            if sum % 2 == 0 {
                res.push('0');
            } else {
                res.push('1');
            }

            idx = idx + 1;
        }

        if carry == 1 {
            res.push('1');
        }
        return res.chars().rev().collect::<String>();
    }
}

fn main() {
    assert_eq!("100", Solution::add_binary("11".to_string(),"1".to_string()));
    println!(
        "{}",
        Solution::add_binary("111".to_string(), "1".to_string())
    );
}
