// LeetCode 269. Alient Dictionary

// There is a foreign language which uses the latin alphabet, 
// but the order among letters is not "a", "b", "c" ... "z" as in English.

// You receive a list of non-empty strings words from the dictionary, 
// where the words are sorted lexicographically based on the rules of this new language.

// Derive the order of letters in this language. If the order is invalid, 
// return an empty string. If there are multiple valid order of letters, return any of them.

// A string a is lexicographically smaller than a string b if either of the following is true:

// The first letter where they differ is smaller in a than in b.
// There is no index i such that a[i] != b[i] and a.length < b.length.

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn foreign_dictionary(words: Vec<String>) -> String {
        let mut adj: HashMap<char, HashSet<char>> = HashMap::new();
        for word in &words {
            for c in word.chars() {
                adj.entry(c).or_insert(HashSet::new());
            }
        }

        for i in 0..words.len() - 1 {
            let w1 = &words[i];
            let w2 = &words[i + 1];
            let min_len = std::cmp::min(w1.len(), w2.len());
            if w1.len() > w2.len() && w1[..min_len] == w2[..min_len] {
                return String::new();
            }
            for j in 0..min_len {
                let c1 = w1.chars().nth(j).unwrap();
                let c2 = w2.chars().nth(j).unwrap();
                if c1 != c2 {
                    adj.entry(c1).or_insert(HashSet::new()).insert(c2);
                    break;
                }
            }
        }

        let mut visited: HashMap<char, bool> = HashMap::new();
        let mut res = Vec::new();

        fn dfs(
            char: char,
            adj: &HashMap<char, HashSet<char>>,
            visited: &mut HashMap<char, bool>,
            res: &mut Vec<char>,
        ) -> bool {
            if let Some(&in_path) = visited.get(&char) {
                return in_path;
            }

            visited.insert(char, true);

            if let Some(neighbors) = adj.get(&char) {
                for &neigh in neighbors {
                    if dfs(neigh, adj, visited, res) {
                        return true;
                    }
                }
            }

            visited.insert(char, false);
            res.push(char);
            false
        }

        for &char in adj.keys() {
            if dfs(char, &adj, &mut visited, &mut res) {
                return String::new();
            }
        }

        res.reverse();
        res.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words = vec!["z".to_string(), "o".to_string()];
        let expected = "zo";
        assert_eq!(Solution::foreign_dictionary(words), expected);
    }

    #[test]
    fn example_2() {
        let words = vec![
            "hrn".to_string(),
            "hrf".to_string(),
            "er".to_string(),
            "enn".to_string(),
            "rfnn".to_string(),
        ];
        let expected = "hernf";
        assert_eq!(Solution::foreign_dictionary(words), expected);
    }

    #[test]
    fn test_invalid_order() {
        let words = vec!["abc".to_string(), "ab".to_string()];
        let expected = "";
        assert_eq!(Solution::foreign_dictionary(words), expected);
    }
}
