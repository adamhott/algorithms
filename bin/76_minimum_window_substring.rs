//Leetcode 76. Minimum Window Substring

//Given two strings s and t of lengths m and n respectively, return the minimum window 
//substring of s such that every character in t (including duplicates) is included in the window. 

//If there is no such substring, return the empty string "".

//The testcases will be generated such that the answer is unique.

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return "".to_string();
        }

        let mut count_t = HashMap::new();
        let mut window = HashMap::new();

        // Count the characters in t
        for c in t.chars() {
            *count_t.entry(c).or_insert(0) += 1;
        }

        let mut have = 0;
        let need = count_t.len();
        let mut res: (isize, isize) = (-1, -1);
        let mut res_len = std::usize::MAX;
        let mut l = 0;

        for (r, c) in s.chars().enumerate() {
            *window.entry(c).or_insert(0) += 1;

            if count_t.contains_key(&c) && window[&c] == count_t[&c] {
                have += 1;
            }

            while have == need {
                // Update the result if the current window is smaller
                if (r - l + 1) < res_len {
                    res = (l as isize, r as isize);
                    res_len = r - l + 1;
                }

                let left_char = s.chars().nth(l).unwrap();
                if let Some(x) = window.get_mut(&left_char) {
                    *x -= 1;
                }

                if count_t.contains_key(&left_char) && window[&left_char] < count_t[&left_char] {
                    have -= 1;
                }
                l += 1;
            }
        }

        let (start, end) = res;
        if res_len == std::usize::MAX {
            "".to_string()
        } else {
            s.chars().skip(start as usize).take((end - start + 1) as usize).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "BANC");
    }

    #[test]
    fn test_example_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "a");
    }

    #[test]
    fn test_example_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = Solution::min_window(s, t);
        assert_eq!(result, "");
    }
}

fn main() {
    // Running some manual tests if needed
    let s = "ADOBECODEBANC".to_string();
    let t = "ABC".to_string();
    let result = Solution::min_window(s, t);
    println!("{}", result); // Expected: "BANC"
}
