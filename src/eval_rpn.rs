use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for ele in tokens {
            if let Ok(num) = ele.parse::<i32>() {
                stack.push(num);
            } else {
                let second: i32 = stack.pop().unwrap();
                let first: i32 = stack.pop().unwrap();
                stack.push(match ele.as_str() {
                    "+" => first + second,
                    "-" => first - second,
                    "*" => first * second,
                    "/" => first / second,
                    _ => panic!("Can't calculate"),
                });
            }
        }
        stack[0]
    }
}
