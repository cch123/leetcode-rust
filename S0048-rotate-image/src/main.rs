struct Solution;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let (row, col) = (matrix.len(), matrix[0].len());
        for i in 0..row {
            for j in i..col {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        for i in 0..row {
            for j in 0..col/2 {
                matrix[i].swap(j, col-j-1);
            }
        }
    }
}

fn main() {
    let mut matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    Solution::rotate(&mut matrix);
    println!("{:?}", matrix);
}
