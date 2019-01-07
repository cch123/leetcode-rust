struct Solution;
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let v: Vec<i32> = (0..=s.len()).map(|x| x as i32).collect();
        let (mut v_iter, mut v_rev_iter) = (v.iter(), v.iter().rev());
        let mut res = vec![];

        s.chars()
            .map(|c| match c {
                'D' => res.push(*v_rev_iter.next().unwrap()),
                'I' => res.push(*v_iter.next().unwrap()),
                _ => {}
            })
            .for_each(drop);
        res.push(*v_iter.next().unwrap());

        return res;
    }
}

fn main() {
    println!("{:?}", Solution::di_string_match("IDID".to_string()));
}

/*
impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut a = 0;
        let mut b = s.len() as i32;
        let mut result:Vec<i32> = Vec::new();

        for i in s.chars() {
            match i {
                'I'=>{
                    result.push(a);
                    a += 1;
                },
                'D'=>{
                    result.push(b);
                    b -= 1;
                },
                _  =>(),
            }
        }

        result.push(a);

        result


    }
}
*/
