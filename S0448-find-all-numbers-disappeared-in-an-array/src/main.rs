struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut v = vec![];

        (0..=nums.len()).map(|x| {
            v.push(x as i32);
        }).for_each(drop);

        nums.iter().map(|n|{
            v[*n as usize] = 0;
        }).for_each(drop);

        let res = v.into_iter().filter(|x| *x != 0).collect();
        return res;
    }
}

fn main() {
    let v = vec![4,3,2,7,8,2,3,1];
    println!("{:?}", Solution::find_disappeared_numbers(v));

}
