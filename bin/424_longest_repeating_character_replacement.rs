//Leetcode 424. Longest Repeating Character Replacement

//You are given a string s and an integer k. You can choose any character of the string and change it to 
//any other uppercase English character. You can perform this operation at most k times.

//Return the length of the longest substring containing the same letter you can get after performing the 
//above operations.


use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = HashMap::new();
        let mut l = 0;
        let mut maxf = 0;
        let mut result = 0;

        for (r, c) in s.chars().enumerate() {
            let counter = count.entry(c).or_insert(0);
            *counter += 1;
            maxf = maxf.max(*counter);

            while (r as i32 - l as i32 + 1) - maxf > k {
                let left_char = s.chars().nth(l).unwrap();
                *count.get_mut(&left_char).unwrap() -= 1;
                l += 1;
            }

            result = result.max(r as i32 - l as i32 + 1);
        }

        result
    }
}

fn main() {
    todo!{}
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let s = "XYYX".to_string();
        let k = 2;
        let result = Solution::character_replacement(s, k);
        assert_eq!(result, 4); // Expected output: 4
    }

    #[test]
    fn test_example_2() {
        let s = "AAABABB".to_string();
        let k = 1;
        let result = Solution::character_replacement(s, k);
        assert_eq!(result, 5); // Expected output: 5
    }
}