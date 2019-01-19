struct Solution;

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        // 只要本体和目标的距离 < 任何一个 ghost 到目标的距离，就可以获胜
        // 否则的话 ghost 可以选择行动模式：
        //    1.先不管本体，直接走到终点
        //    2.这样不管本体怎么走，最终都没法获胜了
        let target_dist = target[0].abs() + target[1].abs();
        let mut res = true;
        ghosts.iter().for_each(|g|{
            if (g[0]-target[0]).abs() + (g[1]-target[1]).abs() < target_dist {
                res = false;
            }
        });

        res
    }
}

fn main() {
}
