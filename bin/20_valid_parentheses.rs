// Leetcode 20. Valid Parentheses
//Given a string s containing just the characters
// '(', ')', '{', '}', '[' and ']', determine if
// the input string is valid.

//An input string is valid if:

//Open brackets must be closed by the same type of brackets.
//Open brackets must be closed in the correct order.
//Every close bracket has a corresponding open bracket of the same type.

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let map: std::collections::HashMap<char, char> = 
            [(')', '('), (']', '['), ('}', '{')].iter().cloned().collect();
        
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            match map.get(&c) {
                Some(&expected) => {
                    if stack.pop() != Some(expected) {
                        return false;
                    }
                },
                None => stack.push(c),
            }
        }

        stack.is_empty()
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_example1() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
    }

    #[test]
    fn test_is_valid_example2() {
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn test_is_valid_example3() {
        assert_eq!(Solution::is_valid(String::from("(]")), false);
    }
}
