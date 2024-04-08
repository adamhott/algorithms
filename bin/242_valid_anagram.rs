// Leetcode 242. Valid Anagram
// Given two strings s and t, return true if t is an anagram of s, 
//and false otherwise.
//An Anagram is a word or phrase formed by rearranging 
//the letters of a different word or phrase, typically using 
//all the original letters exactly once.

use std::collections::HashMap;

fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count_s = HashMap::new();
    let mut count_t = HashMap::new();

    for char_s in s.chars() {
        *count_s.entry(char_s).or_insert(0) += 1;
    }

    for char_t in t.chars() {
        *count_t.entry(char_t).or_insert(0) += 1;
    }

    count_s == count_t
}

fn main() {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_true() {
        assert!(is_anagram("anagram", "nagaram"));
    }

    #[test]
    fn test_anagram_false() {
        assert!(!is_anagram("rat", "car"));
    }
}