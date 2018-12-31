use std::collections::HashMap;
struct Solution {
    // 因为这两个 map 都是在 new 内部生成的
    // 所以需要放在堆上
    idx_map: Box<HashMap<i32, Box<Vec<usize>>>>,
    pick_map: Box<HashMap<i32, i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        let mut idx_map: HashMap<i32, Box<Vec<usize>>> = HashMap::new();
        let mut pick_map: HashMap<i32, i32> = HashMap::new();
        for (idx, num) in nums.iter().enumerate() {
            if pick_map.get(num).is_none() {
                pick_map.insert(*num, -1);
                let v = vec![idx];
                idx_map.insert(*num, Box::new(v));
            } else {
                // 这里的 clone 能优化掉么？
                let mut v = idx_map.get(num).as_mut().unwrap().clone();
                v.push(idx);
                idx_map.insert(*num, v);
            }
        }

        return Solution {
            idx_map: Box::new(idx_map),
            pick_map: Box::new(pick_map),
        };
    }

    fn pick(&mut self, target: i32) -> i32 {
        let id_arr = self.idx_map.get(&target).unwrap();
        let mut pick_pos = *(self.pick_map.get(&target).unwrap());
        pick_pos += 1;
        self.pick_map.insert(target, pick_pos);
        if pick_pos >= id_arr.len() as i32 {
            pick_pos = 0;
        }
        self.pick_map.insert(target, pick_pos);

        return id_arr[pick_pos as usize] as i32;
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */

fn main() {
    let nums = vec![1, 2, 3, 3, 3];
    let mut obj = Solution::new(nums);
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
    println!("{}", obj.pick(3));
}
