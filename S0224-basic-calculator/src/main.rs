struct Solution;

impl Solution {
    fn eval(op_stack: &mut Vec<char>, num_stack: &mut Vec<i32>) -> i32 {
        while op_stack.len() > 0 {
            let cur_op = op_stack.pop().unwrap();
            match cur_op {
                '+' => {
                    let (rop, lop) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
                    num_stack.push(rop + lop);
                }
                '-' => {
                    let (lop, rop) = (num_stack.pop().unwrap(), num_stack.pop().unwrap());
                    num_stack.push(lop - rop);
                }
                _ => unreachable!(),
            }
        }
        num_stack[0]
    }

    pub fn calculate(s: String) -> i32 {
        let mut num_stack = vec![];
        let mut op_stack = vec![];
        let mut cur_num = "".to_string();
        for c in s.chars() {
            if c.is_numeric() {
                cur_num.push(c);
                continue;
            }

            if cur_num.len() > 0 {
                let num: i32 = cur_num.parse().unwrap();
                num_stack.push(num);
                cur_num = "".to_string();
            }

            match c {
                '+' | '-' | '(' => op_stack.push(c),
                ')' => {
                    let mut num_stack_tmp = vec![];
                    let mut op_stack_tmp = vec![];
                    loop {
                        let op = op_stack.pop().unwrap();
                        num_stack_tmp.push(num_stack.pop().unwrap());
                        if op == '(' {
                            break;
                        }
                        op_stack_tmp.push(op);
                    }
                    num_stack.push(Solution::eval(&mut op_stack_tmp, &mut num_stack_tmp));
                }
                _ => continue,
            }
        }

        if cur_num.len() > 0 {
            num_stack.push(cur_num.parse().unwrap())
        }

        num_stack.reverse();
        op_stack.reverse();
        return Solution::eval(&mut op_stack, &mut num_stack);
    }
}

fn main() {
    println!("{}", Solution::calculate("1+2+4".to_string()));
    println!("{}", Solution::calculate("11+2-4".to_string()));
    println!("{}", Solution::calculate("11+2+4".to_string()));
    println!("{}", Solution::calculate("2-1 + 2".to_string()));
    println!("{}", Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
}


/*
impl Solution {
pub fn calculate(s: String) -> i32 {
        let mut res = 0;
        let mut sign = 1;
        let mut stack:Vec<i32> = Vec::new();

        let mut i = 0;
        while i < s.len() {
            let c = s.as_bytes()[i];
            if c>=b'0' && c<= b'9'{
                let mut num = 0;
                while i < s.len() && s.as_bytes()[i]>=b'0' && s.as_bytes()[i]<= b'9' {
                    num = 10 * num + s.as_bytes()[i] as i32 -48;
                    i += 1;
                }

                res += sign * num;
                i -= 1;
            }else if c == b'+' {
                sign = 1
            }else if c == b'-' {
                sign = -1
            }else if c == b'(' {
                stack.push(res);
                stack.push(sign);
                res = 0;
                sign = 1;
            }else if c == b')' {
                res *= stack.pop().unwrap();
                res += stack.pop().unwrap();
            }

            i += 1;
        }
        res
}
}
*/


/*
impl Solution {
    #[inline]
    fn calc(nums: Vec<i32>, ops: Vec<u8>) -> i32 {
        let mut ret = nums[0];
        for (&op, &num) in ops.iter().zip(&nums[1..]) {
            match op {
                b'+' => ret += num,
                _ => ret -= num,
            }
        }
        ret
    }

    pub fn calculate(s: String) -> i32 {
        let mut positive = true;
        let mut nums = vec![];
        let mut ops = vec![];
        let mut num = (false, 0);
        let mut prev = b'+';
        let mut sign = vec![];
        for c in s.bytes().filter(|c| !c.is_ascii_whitespace()) {
            match c {
                b'0'..=b'9' => num = (true, num.1 * 10 + (c - b'0') as i32),
                _ => {
                    if num.0 {
                        nums.push(num.1);
                        num = (false, 0);
                    }
                    match c {
                        b'+' if positive => ops.push(b'+'),
                        b'-' if positive => ops.push(b'-'),
                        b'+' if !positive => ops.push(b'-'),
                        b'-' if !positive => ops.push(b'+'),
                        b'(' => {
                            sign.push(positive);
                            positive = prev == b'+';
                        },
                        b')' => positive = sign.pop().unwrap(),
                        _ => unreachable!(),
                    }
                }
            }
            prev = *ops.last().unwrap_or(&b'+');
        }
        if num.0 {
            nums.push(num.1);
        }
        Solution::calc(nums, ops)
    }
}
*/
