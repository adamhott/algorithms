// Leetcode 22. Generate Parentheses
//Given n pairs of parentheses, write 
//a function to generate all combinations 
//of well-formed parentheses.

struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack = Vec::new();
        let mut res = Vec::new();

        fn backtrack(open_n: i32, closed_n: i32, n: i32, stack: &mut Vec<char>, res: &mut Vec<String>) {
            if open_n == closed_n && open_n == n {
                res.push(stack.iter().collect());
                return;
            }

            if open_n < n {
                stack.push('(');
                backtrack(open_n + 1, closed_n, n, stack, res);
                stack.pop();
            }
            if closed_n < open_n {
                stack.push(')');
                backtrack(open_n, closed_n + 1, n, stack, res);
                stack.pop();
            }
        }

        backtrack(0, 0, n, &mut stack, &mut res);
        res
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_generate_parenthesis_n_3() {
        let mut results = Solution::generate_parenthesis(3);
        let mut expected = vec![
            "((()))", "(()())", "(())()", "()(())", "()()()"
        ];

        // Sort both vectors to ensure the comparison is order-independent
        results.sort();
        expected.sort();

        assert_eq!(results, expected);
    }

    #[test]
    fn test_generate_parenthesis_n_1() {
        let mut results = Solution::generate_parenthesis(1);
        let mut expected = vec!["()"];

        // Sort both vectors to ensure the comparison is order-independent
        results.sort();
        expected.sort();

        assert_eq!(results, expected);
    }
}
