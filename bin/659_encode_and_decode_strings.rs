// Leetcode 659. Encode and Decode Strings
// Design an algorithm to encode a list of
// strings to a string. The encoded string 
// is then sent over the network and is decoded
// back to the original list of strings.
// Please implement "encode" and "decode"
// Solution must be stateless

struct Solution;

impl Solution {
    fn encode(strs: Vec<&str>) -> String {
        let mut res = String::new();
        for s in strs {
            res.push_str(&format!("{}#{}", s.len(), s));
        }
        res
    }

    fn decode(s: String) -> Vec<String> {
        let mut res = Vec::new();
        let mut i = 0;
        let bytes = s.as_bytes();

        while i < s.len() {
            let mut j = i;
            while bytes[j] != b'#' {
                j += 1;
            }
            let length: usize = s[i..j].parse().unwrap();
            i = j + 1;
            j = i + length;
            res.push(s[i..j].to_string());
            i = j;
        }

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
    fn test_encode_decode() {
        let input = vec!["Hello, world!", "Foo", "Bar"];
        let encoded = Solution::encode(input.iter().map(|s| *s).collect());
        let decoded = Solution::decode(encoded);
        let expected = vec!["Hello, world!".to_string(), "Foo".to_string(), "Bar".to_string()];
        
        assert_eq!(decoded, expected);
    }
}


