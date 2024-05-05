//Given a string s, find the length of the longest substring
//without repeating characters.

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_set = std::collections::HashSet::new();
        let mut l = 0;
        let mut res = 0;

        for (r, c) in s.chars().enumerate() {
            while char_set.contains(&c) {
                char_set.remove(&s.chars().nth(l).unwrap());
                l += 1;
            }
            char_set.insert(c);
            res = res.max(r - l + 1);
        }
        res as i32
    }
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let s = String::from("abcabcbb");
        let expected = 3;
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example2() {
        let s = String::from("bbbbb");
        let expected = 1;
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example3() {
        let s = String::from("pwwkew");
        let expected = 3;
        let result = Solution::length_of_longest_substring(s);
        assert_eq!(result, expected);
    }
}
