fn main() {
    println!("Hello, world!");
}

/*
 * @lc app=leetcode.cn id=165 lang=rust
 *
 * [165] 比较版本号
 */
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (v1, v2) = (
            version1
                .split(".")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
            version2
                .split(".")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );
        for i in 0..(v1.len().max(v2.len())) {
            match (v1.get(i), v2.get(i)) {
                (Some(v1i), Some(v2i)) => {
                    if v1i > v2i {
                        return 1;
                    }
                    if v1i < v2i {
                        return -1;
                    }
                }
                (Some(v1i), None) => {
                    if v1i > &0 {
                        return 1;
                    }
                }
                (None, Some(v2i)) => {
                    if v2i > &0 {
                        return -1;
                    }
                }
                _ => {}
            }
        }

        0
    }
}

