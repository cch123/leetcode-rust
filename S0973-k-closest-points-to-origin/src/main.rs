struct Solution;

impl Solution {
    // 也可以用小顶堆来搞
    // binaryheap

    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points.clone();
        points.sort_by(|a, b| {
            (a[0] * a[0] + a[1] * a[1]).cmp(&(b[0] * b[0] + b[1] * b[1]))
        });

        return points[0..k as usize].to_vec();
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1)
    );
    println!(
        "{:?}",
        Solution::k_closest(vec![vec![3, 3], vec![5, -1], vec![-2, 4]], 2)
    );
}
