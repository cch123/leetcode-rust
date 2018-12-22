use std::collections::HashSet;
struct Solution;
impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut cnt_set = HashSet::new();
        for m in emails {
            let name_and_domain:Vec<&str> = m.split('@').collect();
            let (name, domain) = (name_and_domain[0], name_and_domain[1]);
            let name_without_dot = name.replace(".","");
            let name_arr:Vec<&str> = name_without_dot .split('+').collect();
            let name = name_arr[0];
            let addr = vec![name, "@", domain].concat();
            if cnt_set.contains(&addr) {
                continue;
            } else {
                cnt_set.insert(addr);
            }
        }
        return cnt_set.len() as i32;
    }
}

fn main() {
    let v = vec!["abc.efg@aaa.com".to_string(), "def+xxx@abc.com".to_string()];
    println!("{}",Solution::num_unique_emails(v));
}
