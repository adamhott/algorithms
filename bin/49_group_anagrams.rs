// Leetcode 49. Group Anagrams
//Given an array of strings strs, group the 
//anagrams together. You can return the answer 
//in any order.

//An Anagram is a word or phrase formed by 
//rearranging the letters of a different word 
//or phrase, typically using all the original 
//letters exactly once.

use std::collections::HashMap;

fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<String>> {
    let mut ans: HashMap<[usize; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut count = [0; 26];
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            count[index] += 1;
        }
        ans.entry(count).or_insert_with(Vec::new).push(s.to_string());
    }

    ans.into_values().collect()
}


fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_result(mut result: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for group in &mut result {
            group.sort();
        }
        result.sort();
        result
    }

    #[test]
    fn test_example_1() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut expected = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        let mut result = group_anagrams(strs);
        expected = sort_result(expected);
        result = sort_result(result);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_2() {
        let strs = vec![""];
        let expected = vec![vec!["".to_string()]];
        let result = group_anagrams(strs);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_example_3() {
        let strs = vec!["a"];
        let expected = vec![vec!["a".to_string()]];
        let result = group_anagrams(strs);
        assert_eq!(result, expected);
    }
}