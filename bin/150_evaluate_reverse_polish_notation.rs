// Leetcode 150. Evaluate Reverse Polish Notation

//You are given an array of strings tokens that 
//represents an arithmetic expression in a 
//Reverse Polish Notation.

//Evaluate the expression. 
//Return an integer that represents the value 
//of the expression.

//Note that:

//The valid operators are '+', '-', '*', and '/'.
//Each operand may be an integer or another expression.
//The division between two integers always truncates toward zero.
//There will not be any division by zero.
//The input represents a valid arithmetic expression in a reverse polish notation.
//The answer and all the intermediate calculations can be represented in a 32-bit integer.

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.as_str() {
                "+" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b + a);
                },
                "-" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b - a);
                },
                "*" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b * a);
                },
                "/" => {
                    let (a, b) = (stack.pop().unwrap(), stack.pop().unwrap());
                    stack.push(b / a); // In Rust, dividing two integers already floors the result
                },
                _ => {
                    stack.push(token.parse::<i32>().unwrap());
                },
            }
        }

        stack.pop().unwrap()
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_eval_rpn_example_1() {
        let tokens = vec![
            "2".to_string(), "1".to_string(), "+".to_string(), 
            "3".to_string(), "*".to_string()
        ];
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn test_eval_rpn_example_2() {
        let tokens = vec![
            "4".to_string(), "13".to_string(), "5".to_string(), 
            "/".to_string(), "+".to_string()
        ];
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn test_eval_rpn_example_3() {
        let tokens = vec![
            "10".to_string(), "6".to_string(), "9".to_string(), "3".to_string(), "+".to_string(),
            "-11".to_string(), "*".to_string(), "/".to_string(), "*".to_string(), 
            "17".to_string(), "+".to_string(), "5".to_string(), "+".to_string()
        ];
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }
}
