struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut num_stack: Vec<String> = vec![];
        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let rop: i32 = num_stack.pop().unwrap().parse().unwrap();
                    let lop: i32 = num_stack.pop().unwrap().parse().unwrap();
                    let result = match token.as_str() {
                        "+" => lop + rop,
                        "-" => lop - rop,
                        "*" => lop * rop,
                        "/" => lop / rop,
                        _ => 0,
                    };
                    num_stack.push(result.to_string());
                }
                _ => {
                    num_stack.push(token);
                }
            }
        }
        num_stack.pop().unwrap().parse().unwrap()
    }
}

fn main() {
    println!("{}", Solution::eval_rpn(vec!["2".to_string(), "1".to_string(), "+".to_string(), "3".to_string(), "*".to_string()]));
    println!("{}", Solution::eval_rpn(vec!["0".to_string(), "3".to_string(),"/".to_string()]))
}

/*
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut nums = vec![];
        for ref s in tokens {
            let s = s as &str;
            match s {
                "+" | "-" | "*" | "/" => {
                    let (y, x) = (nums.pop().unwrap(), nums.pop().unwrap());
                    let ans = match s {
                        "+" => x + y,
                        "-" => x - y,
                        "*" => x * y,
                        "/" => x / y,
                        _ => unreachable!(),
                    };
                    nums.push(ans);
                },
                _ => nums.push(s.parse::<i32>().unwrap()),
            }
        }
        nums[0]
    }
}
*/