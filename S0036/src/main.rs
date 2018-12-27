use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // 同一行
        for i in 0..board.len() {
            let mut map = HashSet::new();
            for j in 0..board.len() {
                if board[i][j] != '.' && map.contains(&board[i][j]) {
                    //println!('{:?}, {}', map, board[i][j]);
                    return false;
                }
                map.insert(board[i][j]);
            }
        }

        // 同一列
        for i in 0..board.len() {
            let mut map = HashSet::new();
            for j in 0..board.len() {
                if board[j][i] != '.' && map.contains(&board[j][i]) {
                    //println!('{:?}, {}', map, board[j][i]);
                    return false;
                }
                map.insert(board[j][i]);
            }
        }
        // 同一个九宫格
        for i in 0..board.len() {
            for j in 0..board.len() {
                let (mut inner_i, mut inner_j) = (i - i% 3, j -j% 3);
               // println!('fuck {},{},{},{}', i/3,j/3,inner_i,inner_j);
                for inner_i in inner_i..(inner_i + 3) {
                    for inner_j in inner_j..(inner_j + 3) {
                        if (inner_i != i || inner_j != j) && board[i][j] == board[inner_i][inner_j] {
                            if board[i][j] != '.' {
                                //println!('{},{},{},{}', i,j,inner_i,inner_j);
                                return false;
                            }
                        }
                    }
                }
            }
        }
        return true;
    }
}

fn main() {
    let sudoku = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    assert_eq!(true, Solution::is_valid_sudoku(sudoku));
    //Solution::is_valid_sudoku(sudoku);
    let sudoku = vec![
       vec!['8','3','.','.','7','.','.','.','.'],
       vec!['6','.','.','1','9','5','.','.','.'],
       vec!['.','9','8','.','.','.','.','6','.'],
       vec!['8','.','.','.','6','.','.','.','3'],
       vec!['4','.','.','8','.','3','.','.','1'],
       vec!['7','.','.','.','2','.','.','.','6'],
       vec!['.','6','.','.','.','.','2','8','.'],
       vec!['.','.','.','4','1','9','.','.','5'],
       vec!['.','.','.','.','8','.','.','7','9']

    ];
    assert_eq!(false, Solution::is_valid_sudoku(sudoku));
}
