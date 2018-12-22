
struct Solution;
impl Solution {
    pub fn to_lower_case(str: String) -> String {
        let mut res = String::new();
        for c in str.chars() {
            res.push(c.to_ascii_lowercase());
        }
        return res;
    }
}

fn main() {
    Solution::to_lower_case("aBbc".to_string());
}
