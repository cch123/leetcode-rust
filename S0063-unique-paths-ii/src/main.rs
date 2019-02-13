
struct Solution;
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {

        let mut res = vec![vec![0;obstacle_grid[0].len()]; obstacle_grid.len()];
        (0..obstacle_grid.len()).for_each(|i| {
            (0..obstacle_grid[0].len()).for_each(|j|{
                if obstacle_grid[i][j] == 0 {
                    if i == 0 && j == 0 {
                        // do nothing
                        if obstacle_grid[0][0] == 0 {
                            res[0][0] = 1;
                        } else {
                            res[0][0] = 0;
                        }
                    } else if i == 0 {
                        res[i][j] = res[i][j-1];
                    } else if j == 0 {
                        res[i][j] = res[i-1][j];
                    } else {
                        res[i][j] = res[i-1][j] + res[i][j-1];
                    }
                } else {
                    res[i][j] = 0;
                }
            })
        });

        let (i,j) = (obstacle_grid.len() -1, obstacle_grid[0].len()-1);

        res[i][j] as i32
    }
}

fn main() {
    let input = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];
    println!("{:?}", Solution::unique_paths_with_obstacles(input));
    let input = vec![vec![1,0]];
    println!("{:?}", Solution::unique_paths_with_obstacles(input));
}